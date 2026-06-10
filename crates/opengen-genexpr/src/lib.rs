//! .genexpr lexer/parser → AST → lowering to IR

mod ast;
mod lexer;
mod parser;
mod lower;

pub use ast::{Program, Statement, Expr, BinOpKind};
pub use lower::{lower, LowerError};

use opengen_ir::Graph;

/// Parse error
#[derive(Debug)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseError {}

/// Parse GenExpr source code into an AST
pub fn parse(src: &str) -> Result<Program, ParseError> {
    let mut parser = parser::Parser::new(src).map_err(|e| ParseError(e))?;
    parser.parse_program().map_err(|e| ParseError(e))
}

/// Combined parse and lower
pub fn parse_and_lower(src: &str) -> Result<Graph, String> {
    let ast = parse(src).map_err(|e| e.to_string())?;
    lower(&ast).map_err(|e| e.to_string())
}
