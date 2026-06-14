//! .genexpr lexer/parser → AST → lowering to IR

pub mod ast;
mod lexer;
mod parser;
mod lower;
pub mod inline;

pub use ast::{Program, Statement, StatementKind, Expr, BinOpKind, SourceLoc,
    DeclType, Declarator, UnaryOp};
pub use lower::{lower, lower_embedded, lower_embedded_with_resolver, AbstractionResolver, LowerError};

use opengen_ir::Graph;

/// Parse error with optional source location.
#[derive(Debug)]
pub struct ParseError {
    pub msg: String,
    pub loc: Option<SourceLoc>,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.loc {
            Some(loc) => write!(f, "{}:{}: {}", loc.line, loc.col, self.msg),
            None => write!(f, "{}", self.msg),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parse a single GenExpr expression (no trailing semicolon).
///
/// Wraps the expression as a synthetic assignment and extracts the
/// expression from the resulting statement. Supports all expression
/// forms: binary ops, function calls, member calls, ternary, unary.
///
/// ```
/// use opengen_genexpr::{Expr, parse_expression};
/// let expr = parse_expression("1 + 2 * 3").unwrap();
/// assert!(matches!(expr, Expr::BinOp { .. }));
/// ```
pub fn parse_expression(src: &str) -> Result<Expr, ParseError> {
    // Wrap in a synthetic statement to reuse existing parser
    let wrapped = format!("out1 = {};", src);
    let prog = parse(&wrapped)?;
    match &prog.statements[0].kind {
        StatementKind::Assign { expr, .. } => Ok(expr.clone()),
        _ => Err(ParseError { msg: "internal: expected assignment from parse_expression wrap".into(), loc: None }),
    }
}

/// Parse GenExpr source code into an AST
pub fn parse(src: &str) -> Result<Program, ParseError> {
    let mut parser = parser::Parser::new(src).map_err(|e| ParseError { msg: e, loc: None })?;
    match parser.parse_program() {
        Ok(prog) => Ok(prog),
        Err(msg) => Err(ParseError {
            msg,
            loc: Some(parser.current_loc()),
        }),
    }
}

/// Combined parse and lower
pub fn parse_and_lower(src: &str) -> Result<Graph, String> {
    let ast = parse(src).map_err(|e| e.to_string())?;
    lower(&ast).map_err(|e| e.to_string())
}

/// Parse with gen~ strict-mode validation: declarations must precede expressions.
///
/// Real gen~ rejects declarations written after expression statements with
/// "declarations must come before expressions" (observed Max 9).
/// `parse_strict` parses normally, then validates the ordering rule.
pub fn parse_strict(src: &str) -> Result<Program, ParseError> {
    let program = parse(src)?;
    let mut seen_expr = false;
    for stmt in &program.statements {
        let is_decl = is_declaration(stmt);
        let is_expr = is_expression(stmt);

        // Self-referential history check: h = history(h + ...) is invalid.
        if let StatementKind::Assign { name, expr } = &stmt.kind {
            // Check if expr is a history(...) call whose args reference name
            if let Expr::Call { name: op_name, args, .. } = expr {
                if op_name == "history" {
                    if args.iter().any(|a| expr_references_ident(a, name)) {
                        return Err(ParseError {
                            msg: format!("variable '{}' is not defined", name),
                            loc: Some(stmt.loc),
                        });
                    }
                }
            }
        }

        if seen_expr && is_decl {
            return Err(ParseError {
                msg: "declaration after expression".to_string(),
                loc: Some(stmt.loc),
            });
        }
        if is_expr {
            seen_expr = true;
        }
    }
    Ok(program)
}

/// Returns true if the statement is a declaration (creates state/params).
fn is_declaration(stmt: &Statement) -> bool {
    match &stmt.kind {
        StatementKind::ParamDecl { .. }
        | StatementKind::Decl { .. }
        | StatementKind::FuncDecl { .. }
        | StatementKind::Require(_)
        | StatementKind::MultiAssign { .. } => true,
        StatementKind::Assign { name, .. } => {
            // Assignments are declarations unless they write to an output.
            !is_output_name(name)
        }
        _ => false,
    }
}

/// Returns true if the statement is a runtime expression/computation.
fn is_expression(stmt: &Statement) -> bool {
    match &stmt.kind {
        StatementKind::ExprStmt(_)
        | StatementKind::If { .. }
        | StatementKind::While { .. }
        | StatementKind::For { .. }
        | StatementKind::DoWhile { .. }
        | StatementKind::Break
        | StatementKind::Continue
        | StatementKind::Return(_)
        | StatementKind::Block(_) => true,
        StatementKind::Assign { name, .. } => {
            // Output assignments (out1 = ...) are expressions.
            is_output_name(name)
        }
        _ => false,
    }
}

/// Returns true if the name looks like an output port (outN).
fn is_output_name(name: &str) -> bool {
    name.starts_with("out") && name[3..].parse::<u32>().is_ok()
}

/// Recursively check if an expression tree references a given identifier.
fn expr_references_ident(expr: &Expr, name: &str) -> bool {
    match expr {
        Expr::Ident(n) => n == name,
        Expr::BinOp { left, right, .. } => {
            expr_references_ident(left, name) || expr_references_ident(right, name)
        }
        Expr::Unary(_, e) => expr_references_ident(e, name),
        Expr::Call { args, .. } | Expr::MemberCall { args, .. } => {
            args.iter().any(|a| expr_references_ident(a, name))
        }
        Expr::Ternary { cond, true_expr, false_expr } => {
            expr_references_ident(cond, name)
                || expr_references_ident(true_expr, name)
                || expr_references_ident(false_expr, name)
        }
        Expr::Number(_) | Expr::Str(_) => false,
    }
}
