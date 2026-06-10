//! IR → executable Patch. Deterministic: spec'd order, f64 only.
//!
//! # Determinism
//! Execution proceeds in topological order with NodeId tie-breaking.
//! All Compute steps run first, then deferred Update steps.
//! Region execution is purely sequential, left-to-right statement order.

use opengen_ir::*;
use opengen_ops::Registry;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct CompileError(pub String);

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CompileError {}

// ---------------------------------------------------------------------------
// Compiled region IR — resolved from PExpr/PStmt at compile time.
// ---------------------------------------------------------------------------

/// Resolved region expression — all names replaced by indices/kernels.
#[derive(Debug, Clone)]
enum RExpr {
    /// Literal constant.
    Const(f64),
    /// Region-local variable (absolute index into Patch.scratch).
    Local(usize),
    /// Graph value slot (region input, read from Patch.values).
    Slot(usize),
    /// Region persistent state (absolute index into Patch.state).
    State(usize),
    /// Operator call with resolved kernel and state sub-range.
    Call {
        kernel: opengen_ops::Kernel,
        args: Vec<RExpr>,
        state_range: std::ops::Range<usize>,
    },
}

/// Resolved region statement.
#[derive(Debug, Clone)]
enum RStep {
    SetLocal { dst: usize, expr: RExpr },
    SetOut { slot: usize, expr: RExpr },
    SetState { idx: usize, expr: RExpr },
    Eval(RExpr),
    If { cond: RExpr, then_b: Vec<RStep>, else_b: Vec<RStep> },
    While { cond: RExpr, body: Vec<RStep> },
    Break,
    Continue,
}

/// Control flow signals for while/break/continue.
enum Flow {
    Normal,
    Break,
    Continue,
}

// ---------------------------------------------------------------------------
// Patch step kinds
// ---------------------------------------------------------------------------

/// Execution action for one node.
#[derive(Debug)]
enum StepKind {
    /// Copy from process() input arg
    CopyInput { input_index: u16 },
    /// Execute kernel: inputs → value_slot
    Compute {
        kernel: opengen_ops::Kernel,
        inputs: Vec<usize>,
        state_range: std::ops::Range<usize>,
    },
    /// End-of-sample update: gather input slots, call the op's UpdateFn.
    Update {
        update: opengen_ops::UpdateFn,
        inputs: Vec<usize>,
        state_range: std::ops::Range<usize>,
    },
    /// Execute a compiled region — runs set of nested RSteps.
    Region {
        steps: Vec<RStep>,
        /// Range into Patch.scratch for region-local variables (zeroed each sample).
        locals_range: std::ops::Range<usize>,
    },
}

#[derive(Debug)]
struct Step {
    kind: StepKind,
    value_slot: usize,
}

// ---------------------------------------------------------------------------
// Patch — executable representation
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct Patch {
    steps: Vec<Step>,
    /// Value slots: one per graph node (port 0) + extra (region output ports > 0).
    values: Vec<f64>,
    state: Vec<f64>, // flat state arena
    /// Region locals arena — sub-ranges per Region, zeroed each sample.
    scratch: Vec<f64>,
    outputs: Vec<usize>, // value slots feeding Output nodes, by output index
    sr: f64,
    /// Probe storage: maps probe name to (value_slot, recorded_samples)
    probes: HashMap<String, (usize, Vec<f64>)>,
}

impl Patch {
    /// Returns the number of output channels.
    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }

    /// Get recorded probe samples by name.
    /// Returns `None` if the probe name was not registered at compile time.
    pub fn probe(&self, name: &str) -> Option<&[f64]> {
        self.probes.get(name).map(|(_, samples)| samples.as_slice())
    }

    /// Names of all probes registered at compile time, sorted ascending.
    pub fn probe_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.probes.keys().map(|s| s.as_str()).collect();
        names.sort_unstable();
        names
    }

    /// Process one sample frame.
    ///
    /// Execution order:
    /// 1. All graph-level steps in topo order (NodeId tie-breaking).
    /// 2. Deferred Update steps in ascending NodeId order.
    /// 3. Region steps are embedded inline — at region execution, locals are
    ///    zeroed, then statements run sequentially left-to-right.
    /// 4. Probe capture and output gathering run after all steps.
    pub fn process(&mut self, inputs: &[f64]) -> Vec<f64> {
        // Execute all steps in order
        for step in &self.steps {
            match &step.kind {
                StepKind::CopyInput { input_index } => {
                    self.values[step.value_slot] =
                        inputs.get(*input_index as usize).copied().unwrap_or(0.0);
                }
                StepKind::Compute {
                    kernel,
                    inputs: input_slots,
                    state_range,
                } => {
                    let input_vals: Vec<f64> =
                        input_slots.iter().map(|&i| self.values[i]).collect();
                    let state_slice = &mut self.state[state_range.clone()];
                    self.values[step.value_slot] = kernel(&input_vals, state_slice, self.sr);
                }
                StepKind::Update {
                    update,
                    inputs: input_slots,
                    state_range,
                } => {
                    let input_vals: Vec<f64> =
                        input_slots.iter().map(|&i| self.values[i]).collect();
                    update(&input_vals, &mut self.state[state_range.clone()], self.sr);
                }
                StepKind::Region {
                    steps,
                    locals_range,
                } => {
                    // Zero-initialize region locals every sample
                    for v in self.scratch[locals_range.clone()].iter_mut() {
                        *v = 0.0;
                    }
                    // Execute region body
                    run_rsteps(steps, &mut self.values, &mut self.state, &mut self.scratch, self.sr);
                }
            }
        }

        // Capture probe samples (after all steps run)
        for (_, (value_slot, samples)) in self.probes.iter_mut() {
            samples.push(self.values[*value_slot]);
        }

        // Gather outputs
        self.outputs.iter().map(|&slot| self.values[slot]).collect()
    }
}

// ---------------------------------------------------------------------------
// Region runtime — eval / step execution
// ---------------------------------------------------------------------------

/// Evaluate a resolved region expression.
fn eval_rexpr(
    e: &RExpr,
    values: &[f64],
    state: &mut [f64],
    scratch: &[f64],
    sr: f64,
) -> f64 {
    match e {
        RExpr::Const(v) => *v,
        RExpr::Local(idx) => scratch[*idx],
        RExpr::Slot(idx) => values[*idx],
        RExpr::State(idx) => state[*idx],
        RExpr::Call {
            kernel,
            args,
            state_range,
        } => {
            // Evaluate arguments FIRST, then borrow the kernel's state range.
            let arg_vals: Vec<f64> = args
                .iter()
                .map(|a| eval_rexpr(a, values, state, scratch, sr))
                .collect();
            kernel(&arg_vals, &mut state[state_range.clone()], sr)
        }
    }
}

/// Run a block of resolved region steps. Returns a control-flow signal.
fn run_rsteps(
    steps: &[RStep],
    values: &mut [f64],
    state: &mut [f64],
    scratch: &mut [f64],
    sr: f64,
) -> Flow {
    for step in steps {
        match step {
            RStep::SetLocal { dst, expr } => {
                scratch[*dst] = eval_rexpr(expr, values, state, scratch, sr);
            }
            RStep::SetOut { slot, expr } => {
                values[*slot] = eval_rexpr(expr, values, state, scratch, sr);
            }
            RStep::SetState { idx, expr } => {
                state[*idx] = eval_rexpr(expr, values, state, scratch, sr);
            }
            RStep::Eval(expr) => {
                eval_rexpr(expr, values, state, scratch, sr);
            }
            RStep::If {
                cond,
                then_b,
                else_b,
            } => {
                let c = eval_rexpr(cond, values, state, scratch, sr);
                let body = if c != 0.0 { then_b } else { else_b };
                match run_rsteps(body, values, state, scratch, sr) {
                    Flow::Normal => {}
                    f => return f,
                }
            }
            RStep::While { cond, body } => {
                // No iteration guard — matches gen~ behaviour (documented).
                while eval_rexpr(cond, values, state, scratch, sr) != 0.0 {
                    match run_rsteps(body, values, state, scratch, sr) {
                        Flow::Normal => {}
                        Flow::Break => break,
                        Flow::Continue => continue,
                    }
                }
            }
            RStep::Break => return Flow::Break,
            RStep::Continue => return Flow::Continue,
        }
    }
    Flow::Normal
}

// ---------------------------------------------------------------------------
// Region lowering — PExpr/PStmt → RExpr/RStep
// ---------------------------------------------------------------------------

/// Lower a PExpr to an RExpr, resolving operator names and state ranges.
///
/// `input_slots` maps region input port index → graph value slot.
fn lower_region_expr(
    expr: &proc::PExpr,
    reg: &Registry,
    slot_of: &HashMap<(NodeId, u16), usize>,
    region_state_base: usize,
    locals_base: usize,
    sr: f64,
    input_slots: &[usize],
) -> Result<RExpr, CompileError> {
    match expr {
        proc::PExpr::Const(v) => Ok(RExpr::Const(*v)),
        proc::PExpr::Local(idx) => Ok(RExpr::Local(locals_base + *idx as usize)),
        proc::PExpr::In(idx) => Ok(RExpr::Slot(input_slots[*idx as usize])),
        proc::PExpr::State(idx) => {
            Ok(RExpr::State(region_state_base + *idx as usize))
        }
        proc::PExpr::Call {
            op,
            args,
            state_base,
            data_ref,
        } => {
            // Data regions land in Task 17.
            if data_ref.is_some() {
                return Err(CompileError(
                    "data regions (peek/poke) land in Task 17".into(),
                ));
            }

            let op_def = reg.get(op).ok_or_else(|| {
                CompileError(format!("unknown operator in region: '{}'", op))
            })?;

            // Reject stateful ops with deferred updates — they belong to
            // graph-level declarations, not region call sites.
            if op_def.update.is_some() || !op_def.deferred_ports.is_empty() {
                return Err(CompileError(format!(
                    "stateful operator '{}' cannot be called inside a region; \
                     use state slots (PExpr::State/PStmt::SetState) instead",
                    op
                )));
            }

            let lowered_args: Vec<RExpr> = args
                .iter()
                .map(|a| lower_region_expr(a, reg, slot_of, region_state_base, locals_base, sr, input_slots))
                .collect::<Result<Vec<_>, _>>()?;

            let state_range = if *state_base != u32::MAX {
                let size = match op_def.state {
                    StateDecl::Slots(n) => n as usize,
                    StateDecl::None => 0,
                };
                region_state_base + *state_base as usize
                    ..region_state_base + *state_base as usize + size
            } else {
                0..0
            };

            Ok(RExpr::Call {
                kernel: op_def.kernel,
                args: lowered_args,
                state_range,
            })
        }
    }
}

/// Lower a PStmt to one or more RSteps.
fn lower_region_stmt(
    stmt: &proc::PStmt,
    reg: &Registry,
    slot_of: &HashMap<(NodeId, u16), usize>,
    region_state_base: usize,
    locals_base: usize,
    sr: f64,
    out_slots: &[usize],
    input_slots: &[usize],
) -> Result<RStep, CompileError> {
    match stmt {
        proc::PStmt::SetLocal { dst, expr } => {
            let e = lower_region_expr(expr, reg, slot_of, region_state_base, locals_base, sr, input_slots)?;
            Ok(RStep::SetLocal {
                dst: locals_base + *dst as usize,
                expr: e,
            })
        }
        proc::PStmt::SetOut { index, expr } => {
            let slot = out_slots[*index as usize];
            let e = lower_region_expr(expr, reg, slot_of, region_state_base, locals_base, sr, input_slots)?;
            Ok(RStep::SetOut { slot, expr: e })
        }
        proc::PStmt::SetState { index, expr } => {
            let idx = region_state_base + *index as usize;
            let e = lower_region_expr(expr, reg, slot_of, region_state_base, locals_base, sr, input_slots)?;
            Ok(RStep::SetState { idx, expr: e })
        }
        proc::PStmt::Eval(expr) => {
            let e = lower_region_expr(expr, reg, slot_of, region_state_base, locals_base, sr, input_slots)?;
            Ok(RStep::Eval(e))
        }
        proc::PStmt::If {
            cond,
            then_body,
            else_body,
        } => {
            let cond = lower_region_expr(cond, reg, slot_of, region_state_base, locals_base, sr, input_slots)?;
            let then_b = lower_region_stmts(then_body, reg, slot_of, region_state_base, locals_base, sr, out_slots, input_slots)?;
            let else_b = lower_region_stmts(else_body, reg, slot_of, region_state_base, locals_base, sr, out_slots, input_slots)?;
            Ok(RStep::If {
                cond,
                then_b,
                else_b,
            })
        }
        proc::PStmt::While { cond, body } => {
            let cond = lower_region_expr(cond, reg, slot_of, region_state_base, locals_base, sr, input_slots)?;
            let steps = lower_region_stmts(body, reg, slot_of, region_state_base, locals_base, sr, out_slots, input_slots)?;
            Ok(RStep::While { cond, body: steps })
        }
        proc::PStmt::Break => Ok(RStep::Break),
        proc::PStmt::Continue => Ok(RStep::Continue),
    }
}

/// Lower a block of PStmts.
fn lower_region_stmts(
    stmts: &[proc::PStmt],
    reg: &Registry,
    slot_of: &HashMap<(NodeId, u16), usize>,
    region_state_base: usize,
    locals_base: usize,
    sr: f64,
    out_slots: &[usize],
    input_slots: &[usize],
) -> Result<Vec<RStep>, CompileError> {
    stmts
        .iter()
        .map(|s| lower_region_stmt(s, reg, slot_of, region_state_base, locals_base, sr, out_slots, input_slots))
        .collect()
}

// ---------------------------------------------------------------------------
// Compile entry points
// ---------------------------------------------------------------------------

pub fn compile(g: &Graph, reg: &Registry, sr: f64) -> Result<Patch, CompileError> {
    compile_impl(g, reg, sr, &[])
}

/// Compile a graph with named probes that record interior wire values.
/// Returns an error if any probe name is not found in the graph's bindings.
/// Use `Patch::probe(name)` to retrieve recorded samples after processing.
pub fn compile_with_probes(
    g: &Graph,
    reg: &Registry,
    sr: f64,
    probe_names: &[&str],
) -> Result<Patch, CompileError> {
    compile_impl(g, reg, sr, probe_names)
}

fn compile_impl(
    g: &Graph,
    reg: &Registry,
    sr: f64,
    probe_names: &[&str],
) -> Result<Patch, CompileError> {
    let node_count = g.nodes().count();

    // Build the slot_of map: (NodeId, port_index) → value slot.
    // Port 0 of every node maps to node.0.
    // Region output ports 1..n_outputs get extra slots appended after the zero_sentinel.
    let mut slot_of: HashMap<(NodeId, u16), usize> = HashMap::new();
    let mut next_extra_slot = node_count + 1; // node_count is the zero_sentinel

    for (id, _) in g.nodes() {
        slot_of.insert((id, 0), id.0 as usize);
    }

    // First pass: allocate extra slots for region output ports beyond index 0.
    for (id, node) in g.nodes() {
        if let NodeKind::Region(r) = &node.kind {
            for out_idx in 1..r.n_outputs {
                slot_of.insert((id, out_idx), next_extra_slot);
                next_extra_slot += 1;
            }
        }
    }

    let extra_slots = next_extra_slot - (node_count + 1);

    // Resolve probe names to NodeIds — after slot_of is built so we can
    // use slot_of lookups for consistency.
    let mut probes: HashMap<String, (usize, Vec<f64>)> = HashMap::new();
    for &name in probe_names {
        let node_id = g.binding(name).ok_or_else(|| {
            CompileError(format!("probe '{}' not found in graph bindings", name))
        })?;
        let value_slot = slot_of[&(node_id, 0)];
        probes.insert(name.to_string(), (value_slot, Vec::new()));
    }

    // Allocate value slots and state arena
    let mut state_offset = 0;
    let mut state_ranges: HashMap<NodeId, std::ops::Range<usize>> = HashMap::new();

    for (id, node) in g.nodes() {
        if let StateDecl::Slots(n) = node.state() {
            let start = state_offset;
            state_offset += n as usize;
            state_ranges.insert(id, start..state_offset);
        }
    }

    // Topological sort with Kahn's algorithm
    // Stateful nodes break cycles: edges INTO stateful nodes are deferred (non-blocking)
    let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
    let mut dependencies: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

    // Initialize in-degrees
    for (id, _) in g.nodes() {
        in_degree.insert(id, 0);
    }

    // Build dependency graph (who depends on whom)
    // For each node, find what nodes feed into it
    for (id, node) in g.nodes() {
        // Determine arity and deferred ports
        let (arity, deferred): (u16, &[u16]) = match &node.kind {
            NodeKind::Region(r) => (r.n_inputs, &[]), // all ports blocking
            NodeKind::Op { name, .. } => {
                let op_def = reg
                    .get(name)
                    .ok_or_else(|| CompileError(format!("unknown operator: {}", name)))?;
                (op_def.arity, op_def.deferred_ports)
            }
            NodeKind::Output(_) => (1, &[]),
            _ => (0, &[]),
        };

        // Check each input port
        for port_idx in 0..arity {
            if let Some(src) = g.input_of(Port {
                node: id,
                index: port_idx,
            }) {
                // Edge from src.node to id
                // Only create dependency if this port is NOT deferred
                if !deferred.contains(&port_idx) {
                    *in_degree.get_mut(&id).unwrap() += 1;
                    dependencies
                        .entry(src.node)
                        .or_insert_with(Vec::new)
                        .push(id);
                }
            } else if matches!(node.kind, NodeKind::Op { .. }) {
                // Missing required input for an op.
                // Only error if the port is NOT deferred (deferred ports may be
                // unconnected, e.g. an un-written History write port).
                if !deferred.contains(&port_idx) {
                    return Err(CompileError(format!(
                        "missing input {} for op node {:?}",
                        port_idx, id
                    )));
                }
            }
        }
    }

    // Kahn's algorithm with deterministic ordering (sort ready nodes by NodeId)
    let mut ready: Vec<NodeId> = in_degree
        .iter()
        .filter(|(_, deg)| **deg == 0)
        .map(|(&id, _)| id)
        .collect();
    ready.sort_by_key(|id| id.0);

    let mut topo_order = Vec::new();
    let mut processed = HashSet::new();

    while !ready.is_empty() {
        // Take minimum NodeId for determinism
        let id = ready.remove(0);
        topo_order.push(id);
        processed.insert(id);

        // Decrease in-degree of dependents
        if let Some(deps) = dependencies.get(&id) {
            for &dep in deps {
                let deg = in_degree.get_mut(&dep).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    // Insert in sorted position for determinism
                    let pos = ready
                        .binary_search_by_key(&dep.0, |id| id.0)
                        .unwrap_or_else(|e| e);
                    ready.insert(pos, dep);
                }
            }
        }
    }

    // Check for cycles
    if topo_order.len() != node_count {
        return Err(CompileError(
            "cycle detected: feedback requires history or delay".to_string(),
        ));
    }

    // Build steps in topo order
    let mut steps = Vec::new();
    // Slot index for unconnected deferred ports (always reads 0.0).
    let zero_slot = node_count;
    let mut values = vec![0.0; node_count + 1 + extra_slots];
    let mut state = vec![0.0; state_offset];
    let mut scratch = Vec::new();
    let mut scratch_offset = 0;
    let mut outputs_map: HashMap<u16, usize> = HashMap::new();
    let mut stateful_updates: Vec<(NodeId, Step)> = Vec::new();

    for &id in &topo_order {
        let node = g.node(id);
        let value_slot = slot_of[&(id, 0)];

        match &node.kind {
            NodeKind::Constant(v) => {
                values[value_slot] = *v;
                // No step needed; prefilled
            }
            NodeKind::Param { default, .. } => {
                values[value_slot] = *default;
                // No step needed for M1; prefilled with default
            }
            NodeKind::Input(i) => {
                // Copy from process() input args; missing defaults to 0.0
                steps.push(Step {
                    kind: StepKind::CopyInput { input_index: *i },
                    value_slot,
                });
            }
            NodeKind::Output(i) => {
                // Output nodes gather from their input port
                if let Some(src) = g.input_of(Port {
                    node: id,
                    index: 0,
                }) {
                    outputs_map.insert(*i, slot_of[&(src.node, src.index)]);
                }
                // No step needed; just marks what to return
            }
            NodeKind::Region(r) => {
                // Region state base
                let region_state_base = state_ranges.get(&id).map(|r| r.start).unwrap_or(0);

                // Allocate locals in scratch arena
                let locals_start = scratch_offset;
                let n_locals = r.n_locals as usize;
                scratch_offset += n_locals;
                scratch.resize(scratch_offset, 0.0);
                let locals_range = locals_start..scratch_offset;

                // Build input slot map: PExpr::In(idx) → graph value slot
                let mut input_slots: Vec<usize> = Vec::new();
                for port_idx in 0..r.n_inputs {
                    if let Some(src) = g.input_of(Port { node: id, index: port_idx }) {
                        input_slots.push(slot_of[&(src.node, src.index)]);
                    } else {
                        input_slots.push(zero_slot);
                    }
                }

                // Output slots for this region
                let out_slots: Vec<usize> = (0..r.n_outputs)
                    .map(|i| slot_of[&(id, i)])
                    .collect();

                // Lower the body statements
                let region_steps = lower_region_stmts(
                    &r.body,
                    reg,
                    &slot_of,
                    region_state_base,
                    locals_start,
                    sr,
                    &out_slots,
                    &input_slots,
                )?;

                steps.push(Step {
                    kind: StepKind::Region {
                        steps: region_steps,
                        locals_range,
                    },
                    value_slot,
                });
            }
            NodeKind::Op { name, .. } => {
                let op_def = reg
                    .get(name)
                    .ok_or_else(|| CompileError(format!("unknown operator: {}", name)))?;

                // Gather input slots
                let mut input_slots = Vec::new();
                for port_idx in 0..op_def.arity {
                    if let Some(src) = g.input_of(Port {
                        node: id,
                        index: port_idx,
                    }) {
                        input_slots.push(slot_of[&(src.node, src.index)]);
                    } else if op_def.deferred_ports.contains(&port_idx) {
                        // Unconnected deferred port: use the zero sentinel slot.
                        input_slots.push(zero_slot);
                    } else {
                        return Err(CompileError(format!(
                            "missing input {} for operator '{}'",
                            port_idx, name
                        )));
                    }
                }

                let state_range = state_ranges.get(&id).cloned().unwrap_or(0..0);

                // Always emit compute step
                steps.push(Step {
                    kind: StepKind::Compute {
                        kernel: op_def.kernel,
                        inputs: input_slots.clone(),
                        state_range: state_range.clone(),
                    },
                    value_slot,
                });

                // If op has an update function, defer it for end-of-sample execution.
                // Skip update if all deferred ports are unconnected (e.g. unwritten History
                // holds its init value forever — no state mutation needed).
                if let Some(update) = op_def.update {
                    let all_deferred_unconnected = op_def.deferred_ports.iter().all(|&p| {
                        g.input_of(Port {
                            node: id,
                            index: p,
                        })
                        .is_none()
                    });
                    if !all_deferred_unconnected {
                        stateful_updates.push((
                            id,
                            Step {
                                kind: StepKind::Update {
                                    update,
                                    inputs: input_slots,
                                    state_range,
                                },
                                value_slot, // Not actually written to, but needed for struct
                            },
                        ));
                    }
                }
            }
        }
    }

    // Sort update steps by NodeId (determinism contract), then append.
    stateful_updates.sort_by_key(|(id, _)| id.0);
    steps.extend(stateful_updates.into_iter().map(|(_, step)| step));

    // Build outputs list in order by output index
    // Validate that output indices are contiguous (no gaps)
    let outputs = if let Some(&max_output) = outputs_map.keys().max() {
        for i in 0..=max_output {
            if !outputs_map.contains_key(&i) {
                return Err(CompileError(format!("missing output index {}", i)));
            }
        }
        let mut outputs = vec![0; max_output as usize + 1];
        for (&idx, &slot) in &outputs_map {
            outputs[idx as usize] = slot;
        }
        outputs
    } else {
        vec![]
    };

    // Run state initializers
    for (id, node) in g.nodes() {
        match &node.kind {
            NodeKind::Op { name, args, .. } => {
                if let Some(range) = state_ranges.get(&id) {
                    if let Some(init) = reg.get(name).and_then(|d| d.init) {
                        init(args, &mut state[range.clone()], sr);
                    }
                }
            }
            NodeKind::Region(r) => {
                if let Some(range) = state_ranges.get(&id) {
                    let n_state = range.end - range.start;
                    if r.state_init.len() != n_state {
                        return Err(CompileError(format!(
                            "region node {} state_init length {} does not match n_state {}",
                            id.0,
                            r.state_init.len(),
                            n_state,
                        )));
                    }
                    for (i, &v) in r.state_init.iter().enumerate() {
                        state[range.start + i] = v;
                    }
                }
            }
            _ => {}
        }
    }

    Ok(Patch {
        steps,
        values,
        state,
        scratch,
        outputs,
        sr,
        probes,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use opengen_ir::{Graph, Node, Port, StateDecl};

    fn const_add_graph() -> Graph {
        let mut g = Graph::new();
        let a = g.add_node(Node::constant(1.5));
        let b = g.add_node(Node::constant(2.25));
        let add = g.add_node(Node::op("add", vec![], StateDecl::None));
        let out = g.add_node(Node::output(0));
        g.connect(
            Port { node: a, index: 0 },
            Port {
                node: add,
                index: 0,
            },
        );
        g.connect(
            Port { node: b, index: 0 },
            Port {
                node: add,
                index: 1,
            },
        );
        g.connect(
            Port {
                node: add,
                index: 0,
            },
            Port {
                node: out,
                index: 0,
            },
        );
        g
    }

    #[test]
    fn compiles_and_processes_constant_add() {
        let mut patch =
            compile(&const_add_graph(), &opengen_ops::Registry::core(), 48_000.0).unwrap();
        let out = patch.process(&[]);
        assert_eq!(out, vec![3.75]);
    }

    #[test]
    fn rejects_cycle_without_history() {
        let mut g = Graph::new();
        let add = g.add_node(Node::op("add", vec![], StateDecl::None));
        let c = g.add_node(Node::constant(1.0));
        g.connect(
            Port {
                node: add,
                index: 0,
            },
            Port {
                node: add,
                index: 0,
            },
        ); // self-loop
        g.connect(
            Port { node: c, index: 0 },
            Port {
                node: add,
                index: 1,
            },
        ); // other input
        let out = g.add_node(Node::output(0));
        g.connect(
            Port {
                node: add,
                index: 0,
            },
            Port {
                node: out,
                index: 0,
            },
        );
        let err = compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap_err();
        assert!(err
            .to_string()
            .contains("feedback requires history or delay"));
    }

    #[test]
    fn rejects_non_contiguous_outputs() {
        let mut g = Graph::new();
        let c1 = g.add_node(Node::constant(1.0));
        let c2 = g.add_node(Node::constant(2.0));
        let out0 = g.add_node(Node::output(0));
        let out2 = g.add_node(Node::output(2)); // Gap: missing output 1
        g.connect(
            Port { node: c1, index: 0 },
            Port {
                node: out0,
                index: 0,
            },
        );
        g.connect(
            Port { node: c2, index: 0 },
            Port {
                node: out2,
                index: 0,
            },
        );

        let err = compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap_err();
        assert!(err.to_string().contains("missing output index"));
    }

    #[test]
    fn compiles_empty_graph_with_no_outputs() {
        let g = Graph::new();
        let mut patch = compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap();
        let out = patch.process(&[]);
        assert_eq!(out, vec![]);
    }

    #[test]
    fn stateful_node_breaks_feedback_cycle() {
        let mut g = Graph::new();
        let add = g.add_node(Node::op("add", vec![], StateDecl::None));
        let h = g.add_node(Node::op("history", vec![], StateDecl::Slots(1)));
        let one = g.add_node(Node::constant(1.0));
        let out = g.add_node(Node::output(0));
        g.connect(
            Port { node: h, index: 0 },
            Port {
                node: add,
                index: 0,
            },
        );
        g.connect(
            Port { node: one, index: 0 },
            Port {
                node: add,
                index: 1,
            },
        );
        g.connect(
            Port {
                node: add,
                index: 0,
            },
            Port {
                node: h,
                index: 0,
            },
        );
        g.connect(
            Port { node: h, index: 0 },
            Port {
                node: out,
                index: 0,
            },
        );
        let mut patch =
            compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap();
        assert_eq!(patch.process(&[]), vec![0.0]);
        assert_eq!(patch.process(&[]), vec![1.0]);
        assert_eq!(patch.process(&[]), vec![2.0]);
    }

    #[test]
    fn rejects_unregistered_op() {
        let mut g = Graph::new();
        let bogus = g.add_node(Node::op("bogus", vec![], StateDecl::None));
        let out = g.add_node(Node::output(0));
        g.connect(
            Port {
                node: bogus,
                index: 0,
            },
            Port {
                node: out,
                index: 0,
            },
        );
        let err = compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap_err();
        assert!(err.to_string().contains("bogus"));
    }

    #[test]
    fn one_source_feeding_two_ports_schedules_once() {
        let graph =
            opengen_genexpr::parse_and_lower("out1 = in1 + in1;").unwrap();
        let mut patch =
            compile(&graph, &opengen_ops::Registry::core(), 48_000.0).unwrap();
        assert_eq!(patch.process(&[2.0]), vec![4.0]);
        assert_eq!(patch.process(&[3.0]), vec![6.0]);
    }
}
