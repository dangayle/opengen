//! Recursive-descent parser with precedence climbing.
//!
//! Precedence ladder (low → high):
//!   1.  Ternary `?:`                 (right-associative)
//!   2.  `||`                         (logical or)
//!   3.  `^^`                         (logical xor)
//!   4.  `&&`                         (logical and)
//!   5.  `|`                          (bitwise or)
//!   6.  `^`                          (bitwise xor)
//!   7.  `&`                          (bitwise and)
//!   8.  `==` `!=`                    (equality)
//!   9.  `<` `>` `<=` `>=`            (relational)
//!  10.  `<<` `>>`                    (shifts)
//!  11.  `+` `-`                      (additive)
//!  12.  `*` `/` `%`                  (multiplicative)
//!  13.  unary `-` `!`                (prefix)
//!  14.  postfix (call, member-call)  (suffix)
//!  15.  primary
//!
//! Provenance: ladder per `reference/rnbo/genexpr_js/genexpr.pegjs` operator_precedence
//! (Vendor; the M2 plan's task text mis-transcribed `^^`'s tier — corrected during execution;
//! conformance cross-check tracked for M3 "bitwise ^^ semantics confirmation").
//! Also cross-checked with `docs/research/gen_docs/genexpr_ebnf.md`.

use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current: Token,
    current_loc: SourceLoc,
}

impl Parser {
    pub fn new(input: &str) -> Result<Self, String> {
        let mut lexer = Lexer::new(input);
        let spanned = lexer.next_token()?;
        Ok(Self { lexer, current: spanned.tok, current_loc: spanned.loc })
    }

    pub fn current_loc(&self) -> SourceLoc {
        self.current_loc
    }

    fn advance(&mut self) -> Result<(), String> {
        let spanned = self.lexer.next_token()?;
        self.current = spanned.tok;
        self.current_loc = spanned.loc;
        Ok(())
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current == expected {
            self.advance()?;
            Ok(())
        } else {
            Err(format!("expected {:?}, got {:?}", expected, self.current))
        }
    }



    // ═══════════════════════════════════════════════════════════════════
    //  Program-level parsing
    // ═══════════════════════════════════════════════════════════════════

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while self.current != Token::Eof {
            statements.push(self.parse_statement()?);
        }

        // Bare-final-expression sugar is handled at parse-expr time:
        // when parse_expr_or_assign encounters a bare expression at EOF
        // (no trailing semicolon), it directly emits Assign { name: "out1", ... }.
        // Statements with explicit semicolons stay as ExprStmt.
        // No additional desugaring needed here.

        Ok(Program { statements })
    }

    // ═══════════════════════════════════════════════════════════════════
    //  Statement parsing
    // ═══════════════════════════════════════════════════════════════════

    /// Parse one statement.
    /// All jump statements (break/continue/return) parse successfully anywhere;
    /// lowering enforces placement restrictions (e.g., return only valid in function body).
    fn parse_statement(&mut self) -> Result<Statement, String> {
        let loc = self.current_loc;

        match &self.current.clone() {
            // Blocks
            Token::LBrace => self.parse_block(loc),

            // Control flow
            Token::If => self.parse_if(loc),
            Token::While => self.parse_while(loc),
            Token::Do => self.parse_do_while(loc),
            Token::For => self.parse_for(loc),

            // Jump statements
            Token::Break => {
                self.advance()?;
                self.expect(Token::Semicolon)?;
                Ok(Statement { kind: StatementKind::Break, loc })
            }
            Token::Continue => {
                self.advance()?;
                self.expect(Token::Semicolon)?;
                Ok(Statement { kind: StatementKind::Continue, loc })
            }
            Token::Return => {
                self.advance()?;
                // Check for empty return
                if self.current == Token::Semicolon {
                    self.advance()?;
                    Ok(Statement { kind: StatementKind::Return(vec![]), loc })
                } else {
                    let exprs = self.parse_expression_list()?;
                    self.expect(Token::Semicolon)?;
                    Ok(Statement { kind: StatementKind::Return(exprs), loc })
                }
            }

            // Require directive
            Token::Require => {
                self.advance()?;
                let path = match self.current.clone() {
                    Token::Str(s) => {
                        self.advance()?;
                        s
                    }
                    Token::LParen => {
                        self.advance()?;
                        let s = match self.current.clone() {
                            Token::Str(s) => {
                                self.advance()?;
                                s
                            }
                            _ => return Err(format!("expected string in require(), got {:?}", self.current)),
                        };
                        self.expect(Token::RParen)?;
                        s
                    }
                    _ => return Err(format!("expected string or (string) after require, got {:?}", self.current)),
                };
                // Optional semicolon per PEG: `require "file" [;]`
                if self.current == Token::Semicolon {
                    self.advance()?;
                }
                Ok(Statement { kind: StatementKind::Require(path), loc })
            }

            // Empty statement
            Token::Semicolon => {
                self.advance()?;
                // Empty statement is a no-op: expression with no expression
                Ok(Statement { kind: StatementKind::ExprStmt(Expr::Number(0.0)), loc })
            }

            // Typed declaration from keyword: Param name(args);
            Token::Param => self.parse_typed_decl(loc, DeclType::Param),

            // Expression/assignment/declaration/function — ident-starting constructs
            Token::Ident(_) => {
                self.parse_statement_from_ident(loc)
            }

            // Expression statement — any expression followed by semicolon (or bare)
            _ => {
                let expr = self.parse_expr()?;

                // If followed by ';', it's an expression statement
                if self.current == Token::Semicolon {
                    self.advance()?;
                    Ok(Statement { kind: StatementKind::ExprStmt(expr), loc })
                } else if self.current == Token::Eof {
                    // Bare expression at end of input (no semicolon) — desugar to out1 = expr
                    Ok(Statement {
                        kind: StatementKind::Assign {
                            name: "out1".to_string(),
                            expr,
                        },
                        loc,
                    })
                } else {
                    Err(format!("unexpected token {:?} after expression", self.current))
                }
            }
        }
    }

    /// Try to match an identifier as a DeclType keyword.
    fn decl_type(ident: &str) -> Option<DeclType> {
        match ident {
            "History" => Some(DeclType::History),
            "Delay" => Some(DeclType::Delay),
            "Data" => Some(DeclType::Data),
            "Buffer" => Some(DeclType::Buffer),
            "Param" => Some(DeclType::Param),
            _ => None,
        }
    }

    /// Parse a typed declaration: TypeName decl1, decl2, ...;
    fn parse_typed_decl(&mut self, loc: SourceLoc, ty: DeclType) -> Result<Statement, String> {
        // Consume the type name identifier
        self.advance()?;

        // Parse declarator list (at least one required)
        let mut items = Vec::new();
        loop {
            let name = match &self.current {
                Token::Ident(s) => s.clone(),
                _ => return Err(format!("expected identifier in declaration, got {:?}", self.current)),
            };
            self.advance()?;

            // Optional call-style initializer
            let (args, named_args) = if self.current == Token::LParen {
                self.parse_call_args()?
            } else {
                (vec![], vec![])
            };

            items.push(Declarator { name, args, named_args });

            // Comma-separated declarators
            if self.current == Token::Comma {
                self.advance()?;
            } else {
                break;
            }
        }

        self.expect(Token::Semicolon)?;

        Ok(Statement {
            kind: StatementKind::Decl { ty, items },
            loc,
        })
    }

    /// Parse statements starting with an identifier.
    fn parse_statement_from_ident(&mut self, loc: SourceLoc) -> Result<Statement, String> {
        let name = match &self.current {
            Token::Ident(s) => s.clone(),
            _ => unreachable!(),
        };

        // Check for typed declarations: History, Delay, Data, Buffer, Param
        if let Some(ty) = Self::decl_type(&name) {
            return self.parse_typed_decl(loc, ty);
        }

        // Check for function declaration: name( ... ) { ... }
        if self.is_func_decl() {
            self.advance()?; // consume name
            return self.parse_func_decl(loc, name);
        }

        // Check for multi-assign: ident, ident = expr
        if self.is_multi_assign_lhs() {
            return self.parse_multi_assign(loc);
        }

        // Parse as expression/assignment statement
        self.parse_expr_or_assign(loc)
    }

    /// Peek ahead to detect function declaration pattern: ident ( params? ) { block }
    fn is_func_decl(&self) -> bool {
        let mut peek = self.lexer.clone();
        match peek.next_token() {
            Ok(sp) if sp.tok == Token::LParen => {
                // Climb parens to see if `{` follows
                let mut depth = 1;
                loop {
                    match peek.next_token() {
                        Ok(sp) => match sp.tok {
                            Token::LParen => depth += 1,
                            Token::RParen => {
                                depth -= 1;
                                if depth == 0 {
                                    return match peek.next_token() {
                                        Ok(sp) => sp.tok == Token::LBrace,
                                        _ => false,
                                    };
                                }
                            }
                            Token::Eof => return false,
                            _ => {}
                        },
                        _ => return false,
                    }
                }
            }
            _ => false,
        }
    }

    /// Peek ahead to detect multi-assign: ident COMMA ident ... EQUALS
    fn is_multi_assign_lhs(&self) -> bool {
        let mut peek = self.lexer.clone();
        let mut seen_comma = false;
        loop {
            match peek.next_token() {
                Ok(sp) => match sp.tok {
                    Token::Comma => {
                        seen_comma = true;
                        // After comma must be another identifier
                        match peek.next_token() {
                            Ok(sp2) if matches!(sp2.tok, Token::Ident(_)) => continue,
                            _ => return false,
                        }
                    }
                    Token::Equals => return seen_comma,
                    _ => return false,
                }
                _ => return false,
            }
        }
    }

    /// Parse multi-assign: a, b = expr;
    fn parse_multi_assign(&mut self, loc: SourceLoc) -> Result<Statement, String> {
        let mut names = Vec::new();
        loop {
            match &self.current {
                Token::Ident(s) => {
                    names.push(s.clone());
                    self.advance()?;
                }
                _ => return Err(format!("expected identifier in multi-assign, got {:?}", self.current)),
            }
            if self.current == Token::Comma {
                self.advance()?;
            } else {
                break;
            }
        }
        self.expect(Token::Equals)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        Ok(Statement {
            kind: StatementKind::MultiAssign { names, expr },
            loc,
        })
    }

    /// Parse expression statement or assignment starting from an expression.
    fn parse_expr_or_assign(&mut self, loc: SourceLoc) -> Result<Statement, String> {
        let expr = self.parse_expr()?;

        // Check for assignment operators
        match &self.current {
            Token::Equals => {
                self.advance()?;
                let rhs = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                let name = match &expr {
                    Expr::Ident(s) => s.clone(),
                    _ => return Err(format!("expected identifier on left of =, got {:?}", expr)),
                };
                Ok(Statement {
                    kind: StatementKind::Assign { name, expr: rhs },
                    loc,
                })
            }
            // Compound assignment: += -= *= /= %=
            Token::PlusEq => self.parse_compound_assign(loc, &expr, BinOpKind::Add),
            Token::MinusEq => self.parse_compound_assign(loc, &expr, BinOpKind::Sub),
            Token::StarEq => self.parse_compound_assign(loc, &expr, BinOpKind::Mul),
            Token::SlashEq => self.parse_compound_assign(loc, &expr, BinOpKind::Div),
            Token::PercentEq => self.parse_compound_assign(loc, &expr, BinOpKind::Mod),

            _ => {
                // Expression statement
                if self.current == Token::Semicolon {
                    self.advance()?;
                    Ok(Statement { kind: StatementKind::ExprStmt(expr), loc })
                } else if self.current == Token::Eof {
                    // Bare expression at end of input (no semicolon) — desugar to out1 = expr
                    Ok(Statement {
                        kind: StatementKind::Assign {
                            name: "out1".to_string(),
                            expr,
                        },
                        loc,
                    })
                } else {
                    Err(format!("unexpected token {:?} after expression", self.current))
                }
            }
        }
    }

    fn parse_compound_assign(&mut self, loc: SourceLoc, lhs: &Expr, op: BinOpKind) -> Result<Statement, String> {
        let name = match lhs {
            Expr::Ident(s) => s.clone(),
            _ => return Err(format!("expected identifier on left of compound assignment, got {:?}", lhs)),
        };
        let name_for_rhs = name.clone();
        self.advance()?; // consume compound op
        let rhs = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        Ok(Statement {
            kind: StatementKind::Assign {
                name,
                expr: Expr::BinOp {
                    op,
                    left: Box::new(Expr::Ident(name_for_rhs)),
                    right: Box::new(rhs),
                },
            },
            loc,
        })
    }

    /// Parse a block: { statement; statement; ... }
    fn parse_block(&mut self, loc: SourceLoc) -> Result<Statement, String> {
        self.expect(Token::LBrace)?;
        let mut statements = Vec::new();
        while self.current != Token::RBrace && self.current != Token::Eof {
            statements.push(self.parse_statement()?);
        }
        self.expect(Token::RBrace)?;
        Ok(Statement { kind: StatementKind::Block(statements), loc })
    }

    /// Parse if/else if/else
    fn parse_if(&mut self, loc: SourceLoc) -> Result<Statement, String> {
        self.advance()?; // consume 'if'
        self.expect(Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(Token::RParen)?;

        let then_branch = Box::new(self.parse_statement()?);

        let else_branch = if self.current == Token::Else {
            self.advance()?; // consume 'else'
            // Check for 'else if'
            let else_stmt = self.parse_statement()?;
            Some(Box::new(else_stmt))
        } else {
            None
        };

        Ok(Statement {
            kind: StatementKind::If { cond, then_branch, else_branch },
            loc,
        })
    }

    /// Parse while (cond) body
    fn parse_while(&mut self, loc: SourceLoc) -> Result<Statement, String> {
        self.advance()?; // consume 'while'
        self.expect(Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(Token::RParen)?;
        let body = Box::new(self.parse_statement()?);
        Ok(Statement { kind: StatementKind::While { cond, body }, loc })
    }

    /// Parse do body while (cond);
    fn parse_do_while(&mut self, loc: SourceLoc) -> Result<Statement, String> {
        self.advance()?; // consume 'do'
        let body = Box::new(self.parse_statement()?);
        self.expect(Token::While)?;
        self.expect(Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(Token::RParen)?;
        self.expect(Token::Semicolon)?;
        Ok(Statement { kind: StatementKind::DoWhile { body, cond }, loc })
    }

    /// Parse for (init; cond; step) body
    fn parse_for(&mut self, loc: SourceLoc) -> Result<Statement, String> {
        self.advance()?; // consume 'for'
        self.expect(Token::LParen)?;

        // Init: parse expression statement or empty
        let init = if self.current != Token::Semicolon {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        // The parse_statement for init consumed the semicolon, so we just need
        // to handle the case where init is empty.
        if init.is_none() {
            self.expect(Token::Semicolon)?;
        }

        // Condition
        let cond = if self.current != Token::Semicolon {
            let e = self.parse_expr()?;
            Some(e)
        } else {
            None
        };
        self.expect(Token::Semicolon)?;

        // Step (may include compound assignment: i += 1)
        let step = self.parse_for_step()?;
        self.expect(Token::RParen)?;

        let body = Box::new(self.parse_statement()?);

        Ok(Statement {
            kind: StatementKind::For { init, cond, step, body },
            loc,
        })
    }

    /// Parse the step expression of a for-loop, handling compound assignment.
    fn parse_for_step(&mut self) -> Result<Option<Expr>, String> {
        if self.current == Token::RParen {
            return Ok(None);
        }
        let lhs = self.parse_expr()?;
        // Check for compound assignment operators
        let result = match &self.current {
            Token::PlusEq => {
                let name = match &lhs {
                    Expr::Ident(s) => s.clone(),
                    _ => return Err(format!("expected identifier before +=, got {:?}", lhs)),
                };
                self.advance()?;
                let rhs = self.parse_expr()?;
                Expr::BinOp {
                    op: BinOpKind::Add,
                    left: Box::new(Expr::Ident(name)),
                    right: Box::new(rhs),
                }
            }
            Token::MinusEq => {
                let name = match &lhs {
                    Expr::Ident(s) => s.clone(),
                    _ => return Err(format!("expected identifier before -=, got {:?}", lhs)),
                };
                self.advance()?;
                let rhs = self.parse_expr()?;
                Expr::BinOp {
                    op: BinOpKind::Sub,
                    left: Box::new(Expr::Ident(name)),
                    right: Box::new(rhs),
                }
            }
            Token::StarEq => {
                let name = match &lhs {
                    Expr::Ident(s) => s.clone(),
                    _ => return Err(format!("expected identifier before *=, got {:?}", lhs)),
                };
                self.advance()?;
                let rhs = self.parse_expr()?;
                Expr::BinOp {
                    op: BinOpKind::Mul,
                    left: Box::new(Expr::Ident(name)),
                    right: Box::new(rhs),
                }
            }
            Token::SlashEq => {
                let name = match &lhs {
                    Expr::Ident(s) => s.clone(),
                    _ => return Err(format!("expected identifier before /=, got {:?}", lhs)),
                };
                self.advance()?;
                let rhs = self.parse_expr()?;
                Expr::BinOp {
                    op: BinOpKind::Div,
                    left: Box::new(Expr::Ident(name)),
                    right: Box::new(rhs),
                }
            }
            Token::PercentEq => {
                let name = match &lhs {
                    Expr::Ident(s) => s.clone(),
                    _ => return Err(format!("expected identifier before %=, got {:?}", lhs)),
                };
                self.advance()?;
                let rhs = self.parse_expr()?;
                Expr::BinOp {
                    op: BinOpKind::Mod,
                    left: Box::new(Expr::Ident(name)),
                    right: Box::new(rhs),
                }
            }
            _ => lhs,
        };
        Ok(Some(result))
    }

    /// Parse function declaration: name(params) { body }
    fn parse_func_decl(&mut self, loc: SourceLoc, name: String) -> Result<Statement, String> {
        // name already consumed; parse params: (p1, p2, ...)
        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        if self.current != Token::RParen {
            loop {
                match &self.current {
                    Token::Ident(s) => {
                        params.push(s.clone());
                        self.advance()?;
                    }
                    _ => return Err(format!("expected parameter name, got {:?}", self.current)),
                }
                if self.current == Token::Comma {
                    self.advance()?;
                } else {
                    break;
                }
            }
        }
        self.expect(Token::RParen)?;
        self.expect(Token::LBrace)?;

        let mut body = Vec::new();
        while self.current != Token::RBrace && self.current != Token::Eof {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RBrace)?;

        Ok(Statement {
            kind: StatementKind::FuncDecl { name, params, body },
            loc,
        })
    }

    // ═══════════════════════════════════════════════════════════════════
    //  Expression parsing (precedence climbing)
    // ═══════════════════════════════════════════════════════════════════

    /// Top-level expression: ternary
    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_ternary()
    }

    /// Ternary: cond ? true_expr : false_expr  (right-associative)
    fn parse_ternary(&mut self) -> Result<Expr, String> {
        let expr = self.parse_logical_or()?;

        if self.current == Token::Question {
            self.advance()?;
            let true_expr = self.parse_expr()?; // parse everything as the true branch
            self.expect(Token::Colon)?;
            let false_expr = self.parse_ternary()?; // right-assoc: parse only ternary-level on right
            Ok(Expr::Ternary {
                cond: Box::new(expr),
                true_expr: Box::new(true_expr),
                false_expr: Box::new(false_expr),
            })
        } else {
            Ok(expr)
        }
    }

    /// Logical OR: ||
    fn parse_logical_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_xor_logical()?;

        while self.current == Token::OrOr {
            self.advance()?;
            let right = self.parse_xor_logical()?;
            left = Expr::BinOp {
                op: BinOpKind::LogicalOr,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Logical AND: &&
    fn parse_logical_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;

        while self.current == Token::AndAnd {
            self.advance()?;
            let right = self.parse_equality()?;
            left = Expr::BinOp {
                op: BinOpKind::LogicalAnd,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Logical XOR: ^^  (between || and && per vendor PEG)
    fn parse_xor_logical(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_logical_and()?;

        while self.current == Token::CaretCaret {
            self.advance()?;
            let right = self.parse_logical_and()?;
            left = Expr::BinOp {
                op: BinOpKind::LogicalXor,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Equality: == !=
    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_relational()?;

        loop {
            let op = match &self.current {
                Token::EqualEqual => BinOpKind::Eq,
                Token::BangEqual => BinOpKind::Neq,
                _ => break,
            };
            self.advance()?;

            let right = self.parse_relational()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Relational: < > <= >=
    fn parse_relational(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;

        loop {
            let op = match &self.current {
                Token::Lt => BinOpKind::Lt,
                Token::Gt => BinOpKind::Gt,
                Token::Lte => BinOpKind::Lte,
                Token::Gte => BinOpKind::Gte,
                _ => break,
            };
            self.advance()?;

            let right = self.parse_additive()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Additive: + -
    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = match &self.current {
                Token::Plus => BinOpKind::Add,
                Token::Minus => BinOpKind::Sub,
                _ => break,
            };
            self.advance()?;

            let right = self.parse_multiplicative()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Multiplicative: * / %
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        loop {
            let op = match &self.current {
                Token::Star => BinOpKind::Mul,
                Token::Slash => BinOpKind::Div,
                Token::Percent => BinOpKind::Mod,
                _ => break,
            };
            self.advance()?;

            let right = self.parse_unary()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Unary prefix: - !
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.current == Token::Minus {
            self.advance()?;
            let expr = self.parse_unary()?;
            Ok(Expr::Unary(UnaryOp::Neg, Box::new(expr)))
        } else if self.current == Token::Bang {
            self.advance()?;
            let expr = self.parse_unary()?;
            Ok(Expr::Unary(UnaryOp::Not, Box::new(expr)))
        } else {
            self.parse_postfix()
        }
    }

    /// Postfix: call, member call, subscript (subscript not yet in AST)
    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match &self.current {
                Token::LParen => {
                    // Function call on the current expression.
                    // If current expr is Ident, this is name(args).
                    // If current expr is MemberCall-like, we'd have parsed it already.
                    match expr {
                        Expr::Ident(name) => {
                            let (args, named_args) = self.parse_call_args()?;
                            expr = Expr::Call { name, args, named_args };
                        }
                        _ => {
                            // Call on non-identifier expression — not standard GenExpr
                            let (_args, _named) = self.parse_call_args()?;
                            return Err(format!("unexpected call on non-identifier expression"));
                        }
                    }
                }
                Token::Dot => {
                    // Member access: expr.method or expr.method(args)
                    self.advance()?; // consume '.'
                    let method = match &self.current {
                        Token::Ident(s) => s.clone(),
                        _ => return Err(format!("expected method name after '.', got {:?}", self.current)),
                    };
                    self.advance()?; // consume method name

                    if self.current == Token::LParen {
                        let (args, named_args) = self.parse_call_args()?;
                        expr = Expr::MemberCall {
                            object: Box::new(expr),
                            method,
                            args,
                            named_args,
                        };
                    } else {
                        // Bare field access without call — not standard in GenExpr.
                        // The EBNF lists member access as a postfix op, but in practice
                        // it's always followed by a call in GenExpr. Treat as error.
                        return Err(format!("expected '(' after method name '{}'", method));
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// Primary: number, string, identifier, ( expr )
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current.clone() {
            Token::Number(n) => {
                let num = *n;
                self.advance()?;
                Ok(Expr::Number(num))
            }
            Token::Str(s) => {
                let val = s.clone();
                self.advance()?;
                Ok(Expr::Str(val))
            }
            Token::Ident(s) => {
                let name = s.clone();
                self.advance()?;
                Ok(Expr::Ident(name))
            }
            Token::LParen => {
                self.advance()?;
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => Err(format!("unexpected token in expression: {:?}", self.current)),
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    //  Argument lists
    // ═══════════════════════════════════════════════════════════════════

    /// Parse a parenthesized argument list: (arg1, arg2, name=val, ...)
    /// Consumes the opening LParen internally.
    /// Returns (positional_args, named_args).
    fn parse_call_args(&mut self) -> Result<(Vec<Expr>, Vec<(String, Expr)>), String> {
        self.expect(Token::LParen)?;
        let mut args = Vec::new();
        let mut named_args = Vec::new();

        if self.current == Token::RParen {
            self.advance()?;
            return Ok((args, named_args));
        }

        loop {
            // Peek ahead for named arg: identifier = expression
            // We need to check if the next token is `ident = expr`
            let maybe_named = self.try_parse_named_arg()?;
            match maybe_named {
                Some((name, expr)) => {
                    named_args.push((name, expr));
                }
                None => {
                    // Positional argument
                    args.push(self.parse_expr()?);
                }
            }

            match self.current {
                Token::Comma => {
                    self.advance()?;
                }
                Token::RParen => {
                    self.advance()?;
                    break;
                }
                _ => return Err(format!("expected ',' or ')' in argument list, got {:?}", self.current)),
            }
        }

        Ok((args, named_args))
    }

    /// Try to parse a named argument `ident = expr`.
    /// Returns None if the current token doesn't start a named argument.
    fn try_parse_named_arg(&mut self) -> Result<Option<(String, Expr)>, String> {
        // Check if we have: Identifier then Equals
        let saved_current = self.current.clone();

        match &saved_current {
            Token::Ident(name) => {
                // Peek ahead with a clone so we don't consume
                let mut peek = self.lexer.clone();
                match peek.next_token() {
                    Ok(sp) if sp.tok == Token::Equals => {
                        // It IS a named arg. Consume the identifier and '='.
                        self.advance()?; // consume identifier
                        self.expect(Token::Equals)?;
                        let value = self.parse_expr()?;
                        Ok(Some((name.clone(), value)))
                    }
                    _ => {
                        // Not a named arg — restore state
                        // (state is unchanged since we only peeked with saved copies)
                        Ok(None)
                    }
                }
            }
            _ => Ok(None),
        }
    }

    /// Parse a comma-separated expression list: expr, expr, ...
    fn parse_expression_list(&mut self) -> Result<Vec<Expr>, String> {
        let mut exprs = Vec::new();
        if self.current == Token::Semicolon || self.current == Token::Eof {
            return Ok(exprs);
        }
        loop {
            exprs.push(self.parse_expr()?);
            match self.current {
                Token::Comma => {
                    self.advance()?;
                }
                _ => break,
            }
        }
        Ok(exprs)
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  Tests
// ═══════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_assignment() {
        let mut parser = Parser::new("out1 = 42;").unwrap();
        let prog = parser.parse_program().unwrap();
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0].kind {
            StatementKind::Assign { name, expr } => {
                assert_eq!(name, "out1");
                assert!(matches!(expr, Expr::Number(42.0)));
            }
            _ => panic!("expected assignment"),
        }
    }

    #[test]
    fn parses_binary_expression_with_precedence() {
        let mut parser = Parser::new("out1 = 1 + 2 * 3;").unwrap();
        let prog = parser.parse_program().unwrap();
        match &prog.statements[0].kind {
            StatementKind::Assign { expr, .. } => {
                // Should be Add(1, Mul(2, 3))
                match expr {
                    Expr::BinOp { op: BinOpKind::Add, left, right } => {
                        assert!(matches!(**left, Expr::Number(1.0)));
                        match &**right {
                            Expr::BinOp { op: BinOpKind::Mul, left, right } => {
                                assert!(matches!(**left, Expr::Number(2.0)));
                                assert!(matches!(**right, Expr::Number(3.0)));
                            }
                            _ => panic!("expected multiplication on right side"),
                        }
                    }
                    _ => panic!("expected addition at top level"),
                }
            }
            _ => panic!("expected assignment"),
        }
    }

    #[test]
    fn parses_param_declaration() {
        let mut parser = Parser::new("Param freq(440);").unwrap();
        let prog = parser.parse_program().unwrap();
        match &prog.statements[0].kind {
            StatementKind::Decl { ty, items } => {
                assert_eq!(*ty, DeclType::Param);
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].name, "freq");
                assert_eq!(items[0].args.len(), 1);
                assert!(matches!(items[0].args[0], Expr::Number(440.0)));
            }
            _ => panic!("expected decl"),
        }
    }

    #[test]
    fn parses_function_call() {
        let mut parser = Parser::new("out1 = cycle(freq);").unwrap();
        let prog = parser.parse_program().unwrap();
        match &prog.statements[0].kind {
            StatementKind::Assign { expr, .. } => {
                match expr {
                    Expr::Call { name, args, .. } => {
                        assert_eq!(name, "cycle");
                        assert_eq!(args.len(), 1);
                        assert!(matches!(args[0], Expr::Ident(ref s) if s == "freq"));
                    }
                    _ => panic!("expected call expression"),
                }
            }
            _ => panic!("expected assignment"),
        }
    }

    #[test]
    fn parses_modulo_operator() {
        let mut parser = Parser::new("out1 = 5 % 2;").unwrap();
        let prog = parser.parse_program().unwrap();
        match &prog.statements[0].kind {
            StatementKind::Assign { expr, .. } => {
                match expr {
                    Expr::BinOp { op: BinOpKind::Mod, .. } => {}
                    _ => panic!("expected modulo operation"),
                }
            }
            _ => panic!("expected assignment"),
        }
    }

    #[test]
    fn parses_comparison_operators() {
        let cases = vec![
            ("out1 = 2 > 1;", BinOpKind::Gt),
            ("out1 = 2 >= 1;", BinOpKind::Gte),
            ("out1 = 1 < 2;", BinOpKind::Lt),
            ("out1 = 1 <= 2;", BinOpKind::Lte),
            ("out1 = 1 == 1;", BinOpKind::Eq),
            ("out1 = 1 != 2;", BinOpKind::Neq),
        ];

        for (src, expected_op) in cases {
            let mut parser = Parser::new(src).unwrap();
            let prog = parser.parse_program().unwrap();
            match &prog.statements[0].kind {
                StatementKind::Assign { expr, .. } => {
                    match expr {
                        Expr::BinOp { op, .. } => {
                            assert_eq!(*op, expected_op, "Failed for: {}", src);
                        }
                        _ => panic!("expected binary operation for: {}", src),
                    }
                }
                _ => panic!("expected assignment for: {}", src),
            }
        }
    }

    #[test]
    fn comparison_has_lower_precedence_than_arithmetic() {
        let mut parser = Parser::new("out1 = 1 + 2 > 3;").unwrap();
        let prog = parser.parse_program().unwrap();
        match &prog.statements[0].kind {
            StatementKind::Assign { expr, .. } => {
                // Should be Gt(Add(1, 2), 3)
                match expr {
                    Expr::BinOp { op: BinOpKind::Gt, left, right } => {
                        assert!(matches!(**left, Expr::BinOp { op: BinOpKind::Add, .. }));
                        assert!(matches!(**right, Expr::Number(3.0)));
                    }
                    _ => panic!("expected comparison at top level"),
                }
            }
            _ => panic!("expected assignment"),
        }
    }
}
