//! Recursive-descent parser with precedence climbing
//! Precedence reference: reference/rnbo/genexpr_js/genexpr.pegjs (Vendor provenance)
//! * / bind tighter than + - (C precedence)

use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(input: &str) -> Result<Self, String> {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token()?;
        Ok(Self { lexer, current })
    }

    fn advance(&mut self) -> Result<(), String> {
        self.current = self.lexer.next_token()?;
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

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        
        while self.current != Token::Eof {
            statements.push(self.parse_statement()?);
        }
        
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match &self.current {
            Token::Param => self.parse_param_decl(),
            Token::Ident(_) => self.parse_assign(),
            _ => Err(format!("unexpected token at statement start: {:?}", self.current)),
        }
    }

    fn parse_param_decl(&mut self) -> Result<Statement, String> {
        self.expect(Token::Param)?;
        
        let name = match &self.current {
            Token::Ident(s) => s.clone(),
            _ => return Err(format!("expected identifier after Param, got {:?}", self.current)),
        };
        self.advance()?;
        
        self.expect(Token::LParen)?;
        
        let default = match self.current {
            Token::Number(n) => n,
            _ => return Err(format!("expected number in Param default, got {:?}", self.current)),
        };
        self.advance()?;
        
        self.expect(Token::RParen)?;
        self.expect(Token::Semicolon)?;
        
        Ok(Statement::ParamDecl { name, default })
    }

    fn parse_assign(&mut self) -> Result<Statement, String> {
        let name = match &self.current {
            Token::Ident(s) => s.clone(),
            _ => return Err(format!("expected identifier, got {:?}", self.current)),
        };
        self.advance()?;
        
        self.expect(Token::Equals)?;
        
        let expr = self.parse_expr()?;
        
        self.expect(Token::Semicolon)?;
        
        Ok(Statement::Assign { name, expr })
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_additive()
    }

    // Additive: + -
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

    // Multiplicative: * /
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        
        loop {
            let op = match &self.current {
                Token::Star => BinOpKind::Mul,
                Token::Slash => BinOpKind::Div,
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

    // Unary: - primary
    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.current == Token::Minus {
            self.advance()?;
            let expr = self.parse_unary()?;
            Ok(Expr::UnaryMinus(Box::new(expr)))
        } else {
            self.parse_primary()
        }
    }

    // Primary: number | identifier | call | ( expr )
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current.clone() {
            Token::Number(n) => {
                let num = *n;
                self.advance()?;
                Ok(Expr::Number(num))
            }
            Token::Ident(s) => {
                let name = s.clone();
                self.advance()?;
                
                // Check if this is a function call
                if self.current == Token::LParen {
                    self.parse_call(name)
                } else {
                    Ok(Expr::Ident(name))
                }
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

    fn parse_call(&mut self, name: String) -> Result<Expr, String> {
        self.expect(Token::LParen)?;
        
        let mut args = Vec::new();
        
        // Empty argument list
        if self.current == Token::RParen {
            self.advance()?;
            return Ok(Expr::Call { name, args });
        }
        
        // Parse arguments
        loop {
            args.push(self.parse_expr()?);
            
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
        
        Ok(Expr::Call { name, args })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_assignment() {
        let mut parser = Parser::new("out1 = 42;").unwrap();
        let prog = parser.parse_program().unwrap();
        assert_eq!(prog.statements.len(), 1);
        match &prog.statements[0] {
            Statement::Assign { name, expr } => {
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
        match &prog.statements[0] {
            Statement::Assign { expr, .. } => {
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
        match &prog.statements[0] {
            Statement::ParamDecl { name, default } => {
                assert_eq!(name, "freq");
                assert_eq!(*default, 440.0);
            }
            _ => panic!("expected param declaration"),
        }
    }

    #[test]
    fn parses_function_call() {
        let mut parser = Parser::new("out1 = cycle(freq);").unwrap();
        let prog = parser.parse_program().unwrap();
        match &prog.statements[0] {
            Statement::Assign { expr, .. } => {
                match expr {
                    Expr::Call { name, args } => {
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
}
