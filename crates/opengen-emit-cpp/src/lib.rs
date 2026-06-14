//! C++ emitter — produces self-contained C++17 source from an IR Graph.

use std::collections::HashMap;
use opengen_ir::{Graph, NodeId, NodeKind, Port, StateDecl};
use opengen_ops::Registry;

#[derive(Debug, Clone)]
pub struct CppSource {
    pub header: String,
    pub body: String,
}

/// Emit C++17 source from an IR graph and operator registry.
pub fn emit_cpp(graph: &Graph, reg: &Registry, _sr: f64) -> Result<CppSource, String> {
    let mut emitter = Emitter::new(graph, reg);
    emitter.emit()
}

struct Emitter<'a> {
    graph: &'a Graph,
    reg: &'a Registry,
    /// node_id -> value slot index
    slot_of: HashMap<NodeId, usize>,
    /// node_id -> state arena start offset
    state_of: HashMap<NodeId, usize>,
    /// total number of value slots
    n_values: usize,
    /// total state arena size
    n_state: usize,
    /// topo-sorted node order
    order: Vec<NodeId>,
}

impl<'a> Emitter<'a> {
    fn new(graph: &'a Graph, reg: &'a Registry) -> Self {
        Emitter {
            graph,
            reg,
            slot_of: HashMap::new(),
            state_of: HashMap::new(),
            n_values: 0,
            n_state: 0,
            order: Vec::new(),
        }
    }

    fn emit(&mut self) -> Result<CppSource, String> {
        self.assign_slots()?;
        self.assign_state()?;
        self.topo_sort()?;

        let header = self.emit_header();
        let body = self.emit_body()?;
        Ok(CppSource { header, body })
    }

    /// Assign a value slot index to every node.
    fn assign_slots(&mut self) -> Result<(), String> {
        for (id, _node) in self.graph.nodes() {
            self.slot_of.insert(id, self.n_values);
            self.n_values += 1;
        }
        Ok(())
    }

    /// Assign state arena offsets for stateful ops and data nodes.
    fn assign_state(&mut self) -> Result<(), String> {
        let mut off = 0usize;
        for (id, node) in self.graph.nodes() {
            let slots = match &node.kind {
                NodeKind::Op { state, .. } => match state {
                    StateDecl::Slots(n) => *n as usize,
                    StateDecl::None => 0,
                },
                NodeKind::Data { size, .. } => *size,
                NodeKind::Region(r) => r.n_state as usize,
                _ => 0,
            };
            if slots > 0 {
                self.state_of.insert(id, off);
                off += slots;
            }
        }
        self.n_state = off;
        Ok(())
    }

    /// Compute topological sort order (NodeId tie-break).
    fn topo_sort(&mut self) -> Result<(), String> {
        // Build adjacency: edges go source -> dest
        let mut successors: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();

        for (id, _) in self.graph.nodes() {
            in_degree.entry(id).or_insert(0);
            successors.entry(id).or_default();
        }

        // Edges map dest_port -> source_port, so source → dest
        // We need to iterate over edges to find all (source, dest) pairs.
        // The Graph's edges are HashMap<Port, Port> where key=dest, value=source.
        // We need to reverse look up: find all dests for each source.
        // Instead, we can iterate all node output ports and check connections.
        for (src_id, src_node) in self.graph.nodes() {
            let n_outs = match &src_node.kind {
                NodeKind::Output(_) => 0,
                NodeKind::Op { .. } | NodeKind::Constant(_) | NodeKind::Param { .. } | NodeKind::Input(_) | NodeKind::Region(_) => 1,
                NodeKind::Data { .. } => 1,
            };
            for port_idx in 0..n_outs {
                let src_port = Port { node: src_id, index: port_idx as u16 };
                // Find all destinations by scanning all nodes' input ports
                for (dest_id, _dest_node) in self.graph.nodes() {
                    let dest_port = Port { node: dest_id, index: 0 }; // most nodes have single input at index 0
                    if self.graph.input_of(dest_port) == Some(src_port) {
                        successors.entry(src_id).or_default().push(dest_id);
                        *in_degree.entry(dest_id).or_insert(0) += 1;
                    }
                    // Also check higher input port indices for multi-input ops
                    for idx in 1..=8u16 {
                        let dp = Port { node: dest_id, index: idx };
                        if self.graph.input_of(dp) == Some(src_port) {
                            successors.entry(src_id).or_default().push(dest_id);
                            *in_degree.entry(dest_id).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        // Kahn's algorithm
        let mut queue: Vec<NodeId> = in_degree
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(&id, _)| id)
            .collect();
        queue.sort_by_key(|id| id.0); // tie-break by NodeId

        while let Some(id) = queue.pop() {
            self.order.push(id);
            if let Some(succs) = successors.get(&id) {
                let mut sorted_succs: Vec<NodeId> = succs.clone();
                sorted_succs.sort_by_key(|id| id.0);
                for succ in &sorted_succs {
                    let deg = in_degree.get_mut(succ).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push(*succ);
                    }
                }
            }
        }

        if self.order.len() != self.graph.nodes().count() {
            return Err("graph contains a cycle".to_string());
        }
        Ok(())
    }

    fn emit_header(&self) -> String {
        format!(
            r#"#include <vector>
#include <string>
#include <cmath>
#include <cstdint>

struct Patch {{
    int n_inputs, n_outputs;
    std::vector<double> state;
    std::vector<double> v;

    Patch() : n_inputs({n_in}), n_outputs({n_out}), state({n_state}), v({n_vals}) {{}}
    void process(const double* in, double* out);
    void set_param(const std::string& name, double value);
}};
"#,
            n_in = self.count_inputs(),
            n_out = self.count_outputs(),
            n_state = self.n_state,
            n_vals = self.n_values,
        )
    }

    fn emit_body(&self) -> Result<String, String> {
        let mut out = String::new();
        out.push_str(&format!(
            "void Patch::process(const double* in, double* out) {{\n"
        ));

        for &id in &self.order {
            let node = self.graph.node(id);
            let slot = self.slot_of[&id];
            match &node.kind {
                NodeKind::Constant(v) => {
                    out.push_str(&format!("    v[{}] = {};\n", slot, format_f64(*v)));
                }
                NodeKind::Input(idx) => {
                    out.push_str(&format!("    v[{}] = in[{}];\n", slot, idx));
                }
                NodeKind::Output(idx) => {
                    // Find what feeds this output
                    let out_port = Port { node: id, index: 0 };
                    if let Some(src) = self.graph.input_of(out_port) {
                        let src_slot = self.slot_of[&src.node];
                        out.push_str(&format!("    out[{}] = v[{}];\n", idx, src_slot));
                    }
                }
                NodeKind::Op { name, .. } => {
                    if let Some(op_def) = self.reg.get(name) {
                        // Collect input slots from graph edges
                        let mut in_slots = Vec::new();
                        for port_idx in 0..op_def.arity {
                            let in_port = Port { node: id, index: port_idx as u16 };
                            if let Some(src) = self.graph.input_of(in_port) {
                                in_slots.push(self.slot_of[&src.node]);
                            } else {
                                in_slots.push(0); // unconnected → 0.0
                            }
                        }
                        // Emit as a call to kernel_<name>(in0, in1, ...)
                        let in_args: Vec<String> = in_slots.iter()
                            .map(|s| format!("v[{}]", s))
                            .collect();
                        out.push_str(&format!(
                            "    v[{}] = kernel_{}({});\n",
                            slot, name, in_args.join(", ")
                        ));
                    } else {
                        out.push_str(&format!("    // unknown op: {}\n", name));
                    }
                }
                NodeKind::Param { .. } => {
                    out.push_str(&format!("    // param at v[{}]\n", slot));
                }
                NodeKind::Data { .. } => {
                    out.push_str(&format!("    // data buffer at state[{}]\n",
                        self.state_of.get(&id).copied().unwrap_or(0)));
                }
                NodeKind::Region(_) => {
                    out.push_str(&format!("    // region at v[{}]\n", slot));
                }
            }
        }

        out.push_str("}\n");

        // set_param stub
        out.push_str(
            "void Patch::set_param(const std::string& name, double value) {\n    (void)name; (void)value;\n}\n"
        );

        Ok(out)
    }

    fn count_inputs(&self) -> usize {
        self.graph.nodes()
            .filter(|(_, n)| matches!(n.kind, NodeKind::Input(_)))
            .count()
    }

    fn count_outputs(&self) -> usize {
        self.graph.nodes()
            .filter(|(_, n)| matches!(n.kind, NodeKind::Output(_)))
            .count()
    }
}

fn format_f64(v: f64) -> String {
    if v == 0.0 { return "0.0".into(); }
    if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{:.1}", v)
    } else {
        format!("{:.15}", v)
    }
}
