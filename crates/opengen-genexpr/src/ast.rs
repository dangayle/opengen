//! AST for GenExpr language (M1 subset)

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Param name(default);
    ParamDecl { name: String, default: f64 },
    /// ident = expr;
    Assign { name: String, expr: Expr },
}

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
