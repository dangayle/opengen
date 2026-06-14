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

/// Types for typed declarations (History, Delay, Data, Buffer, Param)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclType {
    History,
    Delay,
    Data,
    Buffer,
    Param,
}

/// A declarator in a typed declaration: name(init_args) with named attrs.
#[derive(Debug, Clone, PartialEq)]
pub struct Declarator {
    pub name: String,
    pub args: Vec<Expr>,
    pub named_args: Vec<(String, Expr)>,
}

/// Unary prefix operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// Unary minus: -expr
    Neg,
    /// Logical not: !expr
    Not,
}

/// The inner kind of a statement, without location.
#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    // ─── M1-compat variants (kept for existing lowering) ───────
    /// Param name(default);
    ParamDecl { name: String, default: f64 },
    /// ident = expr;
    Assign { name: String, expr: Expr },

    // ─── New typed declaration ─────────────────────────────────
    /// Type(name(args...), ...);   e.g. History h(0), d(1); Param p(0.5, min=0, max=1);
    Decl { ty: DeclType, items: Vec<Declarator> },

    // ─── Control flow ──────────────────────────────────────────
    /// if (cond) then_branch [else else_branch]
    /// Single-statement bodies have no braces in the source.
    If { cond: Expr, then_branch: Box<Statement>, else_branch: Option<Box<Statement>> },
    /// while (cond) body
    While { cond: Expr, body: Box<Statement> },
    /// do body while (cond);
    DoWhile { body: Box<Statement>, cond: Expr },
    /// for (init; cond; step) body
    /// init is always an expression statement (or empty); cond/step optional.
    For { init: Option<Box<Statement>>, cond: Option<Expr>, step: Option<Expr>, body: Box<Statement> },
    /// { statements }
    Block(Vec<Statement>),
    /// break;
    Break,
    /// continue;
    Continue,
    /// return [e1, e2, ...];
    Return(Vec<Expr>),
    /// a, b = expr;
    MultiAssign { names: Vec<String>, expr: Expr },
    /// name(p1, p2) { body }
    FuncDecl { name: String, params: Vec<String>, body: Vec<Statement> },

    // ─── Directives ─────────────────────────────────────────────
    /// require "file";
    Require(String),

    // ─── Expression statement ───────────────────────────────────
    /// expr;
    ExprStmt(Expr),
}

/// Binary operator kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOpKind {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Comparison
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Neq,
    // Logical (short-circuit is NOT a thing in dataflow — all operands always evaluated)
    LogicalAnd,
    LogicalOr,
    LogicalXor,

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
            BinOpKind::LogicalAnd => "and",
            BinOpKind::LogicalOr => "or",
            BinOpKind::LogicalXor => "xor",

        }
    }
}

/// Expression node. Source locations are tracked at [`Statement`] granularity.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Numeric literal: 42, 3.14
    Number(f64),
    /// String literal: "linear", "myBuffer"
    Str(String),
    /// Identifier: x, freq, samplerate
    Ident(String),
    /// Binary operators: +, -, *, /, %, &&, ||, ^^, &, |, ^, <<, >>, etc.
    BinOp {
        op: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Unary prefix operators: -expr, !expr
    Unary(UnaryOp, Box<Expr>),
    /// Function call: name(arg1, arg2, ...)
    /// Named arguments: name(arg, attr=val)
    Call {
        name: String,
        args: Vec<Expr>,
        named_args: Vec<(String, Expr)>,
    },
    /// Member call: object.method(args) with optional named args
    MemberCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
        named_args: Vec<(String, Expr)>,
    },
    /// Ternary: cond ? true_expr : false_expr
    Ternary {
        cond: Box<Expr>,
        true_expr: Box<Expr>,
        false_expr: Box<Expr>,
    },
}

impl Default for Program {
    fn default() -> Self {
        Program { statements: Vec::new() }
    }
}
