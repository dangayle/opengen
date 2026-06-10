//! AST for GenExpr language

/// Source location (1-indexed line and column)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLoc {
    pub line: u32,
    pub col: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// A statement with its source location.
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub loc: SourceLoc,
}

/// The inner kind of a statement, without location.
#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    /// Param name(default);
    ParamDecl { name: String, default: f64 },
    /// ident = expr;
    Assign { name: String, expr: Expr },
}

/// Expression node. Source locations are tracked at [`Statement`] granularity, not per-expression (M2 decision).
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Ident(String),
    /// Binary operators: +, -, *, /
    BinOp { op: BinOpKind, left: Box<Expr>, right: Box<Expr> },
    /// Unary minus
    UnaryMinus(Box<Expr>),
    /// Function call: name(arg1, arg2, ...)
    Call { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Comparison operators
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Neq,
}

impl BinOpKind {
    /// Map to registry operator name
    pub fn op_name(&self) -> &'static str {
        match self {
            BinOpKind::Add => "add",
            BinOpKind::Sub => "sub",
            BinOpKind::Mul => "mul",
            BinOpKind::Div => "div",
            BinOpKind::Mod => "mod",
            BinOpKind::Gt => "gt",
            BinOpKind::Gte => "gte",
            BinOpKind::Lt => "lt",
            BinOpKind::Lte => "lte",
            BinOpKind::Eq => "eq",
            BinOpKind::Neq => "neq",
        }
    }
}
