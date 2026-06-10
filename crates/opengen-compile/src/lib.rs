//! IR → executable Patch. Deterministic: spec'd order, f64 only.

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

/// Execution action for one node.
#[derive(Debug)]
enum StepKind {
    /// Copy from process() input arg
    CopyInput { input_index: u16 },
    /// Execute kernel: inputs → value_slot
    Compute { kernel: opengen_ops::Kernel, inputs: Vec<usize>, state_range: std::ops::Range<usize> },
    /// End-of-sample update: gather input slots, call the op's UpdateFn.
    Update { update: opengen_ops::UpdateFn, inputs: Vec<usize>, state_range: std::ops::Range<usize> },
}

#[derive(Debug)]
struct Step {
    kind: StepKind,
    value_slot: usize,
}

#[derive(Debug)]
pub struct Patch {
    steps: Vec<Step>,
    /// One value slot per node, indexed by NodeId.0.
    values: Vec<f64>,
    state: Vec<f64>,        // flat state arena
    outputs: Vec<usize>,    // value slots feeding Output nodes, by output index
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
    /// Execution: all Compute steps in topo order (NodeId ties ascending),
    /// then all Update steps in ascending NodeId order.
    pub fn process(&mut self, inputs: &[f64]) -> Vec<f64> {
        // Execute all steps in order
        for step in &self.steps {
            match &step.kind {
                StepKind::CopyInput { input_index } => {
                    // Missing input defaults to 0.0
                    self.values[step.value_slot] = inputs.get(*input_index as usize).copied().unwrap_or(0.0);
                }
                StepKind::Compute { kernel, inputs: input_slots, state_range } => {
                    // TODO(perf): allocates per sample; replace with a reusable scratch buffer on Patch for realtime use (most ops have arity <= 2).
                    let input_vals: Vec<f64> = input_slots.iter().map(|&i| self.values[i]).collect();
                    let state_slice = &mut self.state[state_range.clone()];
                    self.values[step.value_slot] = kernel(&input_vals, state_slice, self.sr);
                }
                StepKind::Update { update, inputs: input_slots, state_range } => {
                    // TODO(perf): allocates per sample; replace with a reusable scratch buffer on Patch for realtime use (most ops have arity <= 2).
                    let input_vals: Vec<f64> = input_slots.iter().map(|&i| self.values[i]).collect();
                    update(&input_vals, &mut self.state[state_range.clone()], self.sr);
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
    // Resolve probe names to NodeIds
    let mut probes: HashMap<String, (usize, Vec<f64>)> = HashMap::new();
    for &name in probe_names {
        let node_id = g.binding(name).ok_or_else(|| {
            CompileError(format!("probe '{}' not found in graph bindings", name))
        })?;
        let value_slot = node_id.0 as usize;
        probes.insert(name.to_string(), (value_slot, Vec::new()));
    }
    
    let node_count = g.nodes().count();
    
    // Allocate value slots (one per node) and state arena
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
        // Determine arity and deferred ports (single OpDef lookup)
        let (arity, deferred): (u16, &[u16]) = match &node.kind {
            NodeKind::Op { name, .. } => {
                let op_def = reg.get(name)
                    .ok_or_else(|| CompileError(format!("unknown operator: {}", name)))?;
                (op_def.arity, op_def.deferred_ports)
            }
            NodeKind::Output(_) => (1, &[]),
            _ => (0, &[]),
        };
        
        // Check each input port
        for port_idx in 0..arity {
            if let Some(src) = g.input_of(Port { node: id, index: port_idx }) {
                // Edge from src.node to id
                // Only create dependency if this port is NOT deferred
                if !deferred.contains(&port_idx) {
                    *in_degree.get_mut(&id).unwrap() += 1;
                    dependencies.entry(src.node).or_insert_with(Vec::new).push(id);
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
    let mut ready: Vec<NodeId> = in_degree.iter()
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
                    let pos = ready.binary_search_by_key(&dep.0, |id| id.0).unwrap_or_else(|e| e);
                    ready.insert(pos, dep);
                }
            }
        }
    }
    
    // Check for cycles
    if topo_order.len() != node_count {
        return Err(CompileError(
            "cycle detected: feedback requires history or delay".to_string()
        ));
    }
    
    // Build steps in topo order
    let mut steps = Vec::new();
    // Slot index for unconnected deferred ports (always reads 0.0).
    let zero_slot = node_count;
    let mut values = vec![0.0; node_count + 1];
    let mut state = vec![0.0; state_offset];
    let mut outputs_map: HashMap<u16, usize> = HashMap::new();
    let mut stateful_updates: Vec<(NodeId, Step)> = Vec::new(); // Defer state updates to end, with NodeId for sorting
    
    for &id in &topo_order {
        let node = g.node(id);
        let value_slot = id.0 as usize;
        
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
                if let Some(src) = g.input_of(Port { node: id, index: 0 }) {
                    outputs_map.insert(*i, src.node.0 as usize);
                }
                // No step needed; just marks what to return
            }
            NodeKind::Op { name, .. } => {
                let op_def = reg.get(name)
                    .ok_or_else(|| CompileError(format!("unknown operator: {}", name)))?;
                
                // Gather input slots
                let mut input_slots = Vec::new();
                for port_idx in 0..op_def.arity {
                    if let Some(src) = g.input_of(Port { node: id, index: port_idx }) {
                        input_slots.push(src.node.0 as usize);
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
                        g.input_of(Port { node: id, index: p }).is_none()
                    });
                    if !all_deferred_unconnected {
                        stateful_updates.push((id, Step {
                            kind: StepKind::Update {
                                update,
                                inputs: input_slots,
                                state_range,
                            },
                            value_slot, // Not actually written to, but needed for struct
                        }));
                    }
                }
            }
        }
    }
    
    // Sort update steps by NodeId (determinism contract), then append.
    // Execution model: all Compute steps run first (in topo order with NodeId ties),
    // then Update steps run in ascending NodeId order. This split implements
    // y[n] = x[n-1] delay: compute reads old state, update writes current input.
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
        // No outputs: empty graph is valid
        vec![]
    };
    
    // Run state initializers
    for (id, node) in g.nodes() {
        if let NodeKind::Op { name, args, .. } = &node.kind {
            if let Some(range) = state_ranges.get(&id) {
                if let Some(init) = reg.get(name).and_then(|d| d.init) {
                    init(args, &mut state[range.clone()], sr);
                }
            }
        }
    }
    
    Ok(Patch { steps, values, state, outputs, sr, probes })
}

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
        g.connect(Port { node: a, index: 0 }, Port { node: add, index: 0 });
        g.connect(Port { node: b, index: 0 }, Port { node: add, index: 1 });
        g.connect(Port { node: add, index: 0 }, Port { node: out, index: 0 });
        g
    }

    #[test]
    fn compiles_and_processes_constant_add() {
        let mut patch = compile(&const_add_graph(), &opengen_ops::Registry::core(), 48_000.0).unwrap();
        let out = patch.process(&[]);
        assert_eq!(out, vec![3.75]);
    }

    #[test]
    fn rejects_cycle_without_history() {
        let mut g = Graph::new();
        let add = g.add_node(Node::op("add", vec![], StateDecl::None));
        let c = g.add_node(Node::constant(1.0));
        g.connect(Port { node: add, index: 0 }, Port { node: add, index: 0 }); // self-loop
        g.connect(Port { node: c, index: 0 }, Port { node: add, index: 1 }); // other input
        let out = g.add_node(Node::output(0));
        g.connect(Port { node: add, index: 0 }, Port { node: out, index: 0 });
        let err = compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap_err();
        assert!(err.to_string().contains("feedback requires history or delay"));
    }

    #[test]
    fn rejects_non_contiguous_outputs() {
        // Graph with Output(0) and Output(2), but no Output(1)
        let mut g = Graph::new();
        let c1 = g.add_node(Node::constant(1.0));
        let c2 = g.add_node(Node::constant(2.0));
        let out0 = g.add_node(Node::output(0));
        let out2 = g.add_node(Node::output(2)); // Gap: missing output 1
        g.connect(Port { node: c1, index: 0 }, Port { node: out0, index: 0 });
        g.connect(Port { node: c2, index: 0 }, Port { node: out2, index: 0 });
        
        let err = compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap_err();
        assert!(err.to_string().contains("missing output index"));
    }

    #[test]
    fn compiles_empty_graph_with_no_outputs() {
        // Empty graph should compile successfully with zero outputs
        let g = Graph::new();
        let mut patch = compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap();
        let out = patch.process(&[]);
        assert_eq!(out, vec![]);
    }

    #[test]
    fn stateful_node_breaks_feedback_cycle() {
        // A feedback loop through history must compile (cycle broken by deferred port 0).
        // history implements true y[n] = x[n-1] delay semantics.
        let mut g = Graph::new();
        let add = g.add_node(Node::op("add", vec![], StateDecl::None));
        let h = g.add_node(Node::op("history", vec![], StateDecl::Slots(1)));
        let one = g.add_node(Node::constant(1.0));
        let out = g.add_node(Node::output(0));
        // h = history(h + 1); out1 = h;
        g.connect(Port { node: h, index: 0 }, Port { node: add, index: 0 }); // h feeds add
        g.connect(Port { node: one, index: 0 }, Port { node: add, index: 1 }); // 1 feeds add
        g.connect(Port { node: add, index: 0 }, Port { node: h, index: 0 }); // add feeds history (deferred)
        g.connect(Port { node: h, index: 0 }, Port { node: out, index: 0 }); // h feeds output
        let mut patch = compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap();
        // Each sample: read old h (starts at 0), compute h+1, update h for next sample
        assert_eq!(patch.process(&[]), vec![0.0]);
        assert_eq!(patch.process(&[]), vec![1.0]);
        assert_eq!(patch.process(&[]), vec![2.0]);
    }

    #[test]
    fn rejects_unregistered_op() {
        let mut g = Graph::new();
        let bogus = g.add_node(Node::op("bogus", vec![], StateDecl::None));
        let out = g.add_node(Node::output(0));
        g.connect(Port { node: bogus, index: 0 }, Port { node: out, index: 0 });
        let err = compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap_err();
        assert!(err.to_string().contains("bogus"));
    }

    #[test]
    fn one_source_feeding_two_ports_schedules_once() {
        // out1 = in1 + in1 — same source node on both ports of add
        let graph = opengen_genexpr::parse_and_lower("out1 = in1 + in1;").unwrap();
        let mut patch = compile(&graph, &opengen_ops::Registry::core(), 48_000.0).unwrap();
        assert_eq!(patch.process(&[2.0]), vec![4.0]);
        assert_eq!(patch.process(&[3.0]), vec![6.0]);
    }
}
