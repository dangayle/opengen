//! Structured procedural regions: codebox control flow lowers to one Region
//! node embedded in the dataflow graph (design doc, "structured regions").
//!
//! # Semantics
//! Region locals are zero-initialized at the start of every sample. History
//! inside a region is region state; stateful op calls inside regions get
//! region-state sub-ranges. Probes are graph-level only (region interiors not
//! probeable in M2).

/// Expression tree, fully resolved at lowering time: locals/inputs/state by
/// index, op calls by registry name (kernel pointers resolve at compile).
#[derive(Debug, Clone, PartialEq)]
pub enum PExpr {
    /// Literal constant.
    Const(f64),
    /// Region-local variable slot (zero-initialized every sample).
    Local(u32),
    /// Region input port (fed by graph edges).
    In(u16),
    /// Region persistent state slot (History reads).
    State(u32),
    /// Operator/function call.
    Call {
        /// Operator name, resolved to a kernel at compile time.
        op: String,
        /// Arguments to the call.
        args: Vec<PExpr>,
        /// Offset into the region's state block for this call instance
        /// (stateful ops get unique instances per call site); `u32::MAX` if stateless.
        state_base: u32,
        /// Named data region (peek/poke); resolved to an arena range at compile.
        data_ref: Option<String>,
    },
}

/// A statement in a procedural region.
#[derive(Debug, Clone, PartialEq)]
pub enum PStmt {
    /// Write to a region-local variable.
    SetLocal { dst: u32, expr: PExpr },
    /// Write to a region output port.
    SetOut { index: u16, expr: PExpr },
    /// History writes — immediate, per genlib codebox semantics (a History is a
    /// plain persistent variable; `# Vendor` cite genlib_ops.h; note the
    /// deliberate contrast with graph-level history's deferred update).
    SetState { index: u32, expr: PExpr },
    /// Side-effect expression statement (e.g. poke).
    Eval(PExpr),
    /// Conditional branch.
    If { cond: PExpr, then_body: Vec<PStmt>, else_body: Vec<PStmt> },
    /// Loop with re-checked condition on each iteration.
    While { cond: PExpr, body: Vec<PStmt> },
    /// Break out of the innermost enclosing While.
    Break,
    /// Skip remaining statements in the innermost While body and re-check condition.
    Continue,
}

/// A structured procedural region — the compiled form of a codebox.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProcRegion {
    /// Number of input ports (fed by graph edges).
    pub n_inputs: u16,
    /// Number of output ports (written by SetOut statements).
    pub n_outputs: u16,
    /// Number of region-local variables (zero-initialized each sample).
    pub n_locals: u32,
    /// Number of persistent state slots (History instances inside the region).
    pub n_state: u32,
    /// Initial values for the region state block (History inits), len == n_state.
    pub state_init: Vec<f64>,
    /// The body statements compiled from the codebox source.
    pub body: Vec<PStmt>,
}
