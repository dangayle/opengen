//! .genexpr lexer/parser → AST → lowering to IR

pub mod ast;
mod lexer;
mod parser;
mod lower;
pub mod inline;

pub use ast::{Program, Statement, StatementKind, Expr, BinOpKind, SourceLoc,
    DeclType, Declarator, UnaryOp};
pub use lower::{lower, LowerError};

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
