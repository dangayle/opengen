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
    /// Update state for a stateful node: read input value → state arena
    StateUpdate { input_slot: usize, state_range: std::ops::Range<usize> },
}

#[derive(Debug)]
struct Step {
    kind: StepKind,
    value_slot: usize,
}

#[derive(Debug)]
pub struct Patch {
    steps: Vec<Step>,
    values: Vec<f64>,       // one slot per node
    state: Vec<f64>,        // flat state arena
    outputs: Vec<usize>,    // value slots feeding Output nodes, by output index
    sr: f64,
}

impl Patch {
    /// Process one sample frame. Deterministic order = topo order (ties broken by NodeId).
    pub fn process(&mut self, inputs: &[f64]) -> Vec<f64> {
        // Execute all steps in order
        for step in &self.steps {
            match &step.kind {
                StepKind::CopyInput { input_index } => {
                    // Missing input defaults to 0.0
                    self.values[step.value_slot] = inputs.get(*input_index as usize).copied().unwrap_or(0.0);
                }
                StepKind::Compute { kernel, inputs: input_slots, state_range } => {
                    let input_vals: Vec<f64> = input_slots.iter().map(|&i| self.values[i]).collect();
                    let state_slice = &mut self.state[state_range.clone()];
                    self.values[step.value_slot] = kernel(&input_vals, state_slice, self.sr);
                }
                StepKind::StateUpdate { input_slot, state_range } => {
                    // Copy current input value into state for next sample
                    let val = self.values[*input_slot];
                    self.state[state_range.start] = val;
                }
            }
        }
        
        // Gather outputs
        self.outputs.iter().map(|&slot| self.values[slot]).collect()
    }
}

pub fn compile(g: &Graph, reg: &Registry, sr: f64) -> Result<Patch, CompileError> {
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
        // Determine arity (how many inputs to check)
        let arity = match &node.kind {
            NodeKind::Op { name, .. } => {
                let op_def = reg.get(name)
                    .ok_or_else(|| CompileError(format!("unknown operator: {}", name)))?;
                op_def.arity
            }
            NodeKind::Output(_) => 1,
            _ => 0,
        };
        
        // Check each input port
        for port_idx in 0..arity {
            if let Some(src) = g.input_of(Port { node: id, index: port_idx }) {
                // Edge from src.node to id
                // If id is stateful, this edge doesn't create a dependency (breaks cycles)
                if node.state() == StateDecl::None {
                    *in_degree.get_mut(&id).unwrap() += 1;
                    dependencies.entry(src.node).or_insert_with(Vec::new).push(id);
                }
            } else if matches!(node.kind, NodeKind::Op { .. }) {
                // Missing required input for an op
                return Err(CompileError(format!(
                    "missing input {} for op node {:?}",
                    port_idx, id
                )));
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
    let mut values = vec![0.0; node_count];
    let state = vec![0.0; state_offset];
    let mut outputs_map: HashMap<u16, usize> = HashMap::new();
    let mut stateful_updates = Vec::new(); // Defer state updates to end
    
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
            NodeKind::Op { name, state, .. } => {
                let op_def = reg.get(name)
                    .ok_or_else(|| CompileError(format!("unknown operator: {}", name)))?;
                
                // Gather input slots
                let mut input_slots = Vec::new();
                for port_idx in 0..op_def.arity {
                    if let Some(src) = g.input_of(Port { node: id, index: port_idx }) {
                        input_slots.push(src.node.0 as usize);
                    } else {
                        return Err(CompileError(format!(
                            "missing input {} for operator '{}'",
                            port_idx, name
                        )));
                    }
                }
                
                let state_range = state_ranges.get(&id).cloned().unwrap_or(0..0);
                
                if *state != StateDecl::None {
                    // Stateful node: emit compute step now (reads old state),
                    // defer state update to end (writes current input)
                    steps.push(Step {
                        kind: StepKind::Compute {
                            kernel: op_def.kernel,
                            inputs: input_slots.clone(),
                            state_range: state_range.clone(),
                        },
                        value_slot,
                    });
                    
                    // Defer state update: copy input[0] to state[0]
                    stateful_updates.push(Step {
                        kind: StepKind::StateUpdate {
                            input_slot: input_slots[0],
                            state_range,
                        },
                        value_slot, // Not actually written to, but needed for struct
                    });
                } else {
                    // Stateless op: just compute
                    steps.push(Step {
                        kind: StepKind::Compute {
                            kernel: op_def.kernel,
                            inputs: input_slots,
                            state_range,
                        },
                        value_slot,
                    });
                }
            }
        }
    }
    
    // Append state updates at the end
    steps.extend(stateful_updates);
    
    // Build outputs list in order by output index
    let max_output = outputs_map.keys().max().copied().unwrap_or(0);
    let mut outputs = vec![0; max_output as usize + 1];
    for (&idx, &slot) in &outputs_map {
        outputs[idx as usize] = slot;
    }
    
    Ok(Patch { steps, values, state, outputs, sr })
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
}
