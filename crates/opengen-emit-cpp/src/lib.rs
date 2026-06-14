//! C++ emitter — produces self-contained C++17 source from an IR Graph.

use std::collections::{HashMap, BTreeSet};
use opengen_ir::{Graph, NodeId, NodeKind, Port, StateDecl};
use opengen_ops::Registry;

#[derive(Debug, Clone)]
pub struct CppSource {
    pub header: String,
    pub body: String,
}

/// Emit C++17 source from an IR graph and operator registry.
pub fn emit_cpp(graph: &Graph, reg: &Registry, sr: f64) -> Result<CppSource, String> {
    let mut emitter = Emitter::new(graph, reg, sr);
    emitter.emit()
}

struct Emitter<'a> {
    graph: &'a Graph,
    reg: &'a Registry,
    sr: f64,
    slot_of: HashMap<NodeId, usize>,
    state_of: HashMap<NodeId, usize>,
    n_values: usize,
    n_state: usize,
    order: Vec<NodeId>,
    used_ops: BTreeSet<String>,
}

impl<'a> Emitter<'a> {
    fn new(graph: &'a Graph, reg: &'a Registry, sr: f64) -> Self {
        Emitter {
            graph, reg, sr,
            slot_of: HashMap::new(),
            state_of: HashMap::new(),
            n_values: 0, n_state: 0,
            order: Vec::new(),
            used_ops: BTreeSet::new(),
        }
    }

    fn emit(&mut self) -> Result<CppSource, String> {
        self.assign_slots()?;
        self.assign_state()?;
        self.topo_sort()?;
        self.collect_used_ops();

        let header = self.emit_header();
        let body = self.emit_body()?;
        Ok(CppSource { header, body })
    }

    fn assign_slots(&mut self) -> Result<(), String> {
        for (id, _) in self.graph.nodes() {
            self.slot_of.insert(id, self.n_values);
            self.n_values += 1;
        }
        Ok(())
    }

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

    fn topo_sort(&mut self) -> Result<(), String> {
        let mut successors: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
        for (id, _) in self.graph.nodes() {
            in_degree.entry(id).or_insert(0);
            successors.entry(id).or_default();
        }
        for (src_id, src_node) in self.graph.nodes() {
            let n_outs: u16 = match &src_node.kind {
                NodeKind::Output(_) => 0,
                _ => 1,
            };
            for port_idx in 0..n_outs {
                let src_port = Port { node: src_id, index: port_idx };
                for (dest_id, _) in self.graph.nodes() {
                    for idx in 0..=8u16 {
                        let dp = Port { node: dest_id, index: idx };
                        if self.graph.input_of(dp) == Some(src_port) {
                            successors.entry(src_id).or_default().push(dest_id);
                            *in_degree.entry(dest_id).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
        let mut queue: Vec<NodeId> = in_degree.iter()
            .filter(|(_, d)| **d == 0)
            .map(|(&id, _)| id)
            .collect();
        queue.sort_by_key(|id| id.0);
        while let Some(id) = queue.pop() {
            self.order.push(id);
            if let Some(succs) = successors.get(&id) {
                let mut sorted: Vec<NodeId> = succs.clone();
                sorted.sort_by_key(|id| id.0);
                for succ in &sorted {
                    let deg = in_degree.get_mut(succ).unwrap();
                    *deg -= 1;
                    if *deg == 0 { queue.push(*succ); }
                }
            }
        }
        if self.order.len() != self.graph.nodes().count() {
            return Err("graph contains a cycle".to_string());
        }
        Ok(())
    }

    fn collect_used_ops(&mut self) {
        for &id in &self.order {
            if let NodeKind::Op { name, .. } = &self.graph.node(id).kind {
                if self.reg.get(name).is_some() {
                    self.used_ops.insert(name.clone());
                }
            }
        }
    }

    // ── header ────────────────────────────────────────────────

    fn emit_header(&self) -> String {
        format!(
            "#pragma once\n\
             #include <cstdint>\n\
             #include <string>\n\
             #include <vector>\n\
             \n\
             constexpr double samplerate = {sr:.1};\n\
             struct Patch {{\n\
                 static constexpr int n_inputs = {n_in};\n\
                 static constexpr int n_outputs = {n_out};\n\
                 std::vector<double> state;\n\
                 std::vector<double> v;\n\
             \n\
                 Patch();\n\
                 void process(const double* in, double* out);\n\
                 void set_param(const std::string& name, double value);\n\
             }};\n",
            sr = self.sr,
            n_in = self.count_inputs(),
            n_out = self.count_outputs(),
        )
    }

    // ── body ──────────────────────────────────────────────────

    fn emit_body(&self) -> Result<String, String> {
        let mut out = String::new();
        out.push_str("#include \"opengen_patch.h\"\n");
        out.push_str("#include <cmath>\n\n");

        // ── constructor ──
        out.push_str(&format!(
            "Patch::Patch() : state({n_state}), v({n_vals}) {{}}\n\n",
            n_state = self.n_state, n_vals = self.n_values,
        ));

        // ── kernel functions ──
        self.emit_kernel_functions(&mut out);

        // ── process() ──
        out.push_str("void Patch::process(const double* in, double* out) {\n");
        for &id in &self.order {
            out.push_str(&self.emit_node_stmt(id));
        }
        out.push_str("}\n\n");

        // ── set_param stub ──
        out.push_str(
            "void Patch::set_param(const std::string& name, double value) {\n    (void)name; (void)value;\n}\n"
        );

        Ok(out)
    }

    fn emit_kernel_functions(&self, out: &mut String) {
        // Emit a static inline function for each used operator.
        for name in &self.used_ops {
            if let Some(op_def) = self.reg.get(name) {
                let params: Vec<String> = (0..op_def.arity)
                    .map(|i| format!("double a{}", i))
                    .collect();
                let body = if let Some(template) = op_def.cpp_kernel {
                    let mut b = template.to_string();
                    for i in 0..op_def.arity {
                        b = b.replace(&format!("{{a{}}}", i), &format!("a{}", i));
                    }
                    b
                } else if op_def.emit_cpp_call.is_some() {
                    "return 0.0; // stateful — inline in process()".to_string()
                } else {
                    "return 0.0; // TODO: cpp_kernel".to_string()
                };
                out.push_str(&format!(
                    "static inline double kernel_{name}({params}) {{\n    {body}\n}}\n\n",
                    name = name, params = params.join(", "), body = body,
                ));
            }
        }
    }

    fn emit_node_stmt(&self, id: NodeId) -> String {
        let node = self.graph.node(id);
        let slot = self.slot_of[&id];
        match &node.kind {
            NodeKind::Constant(v) => {
                format!("    v[{}] = {};\n", slot, format_f64(*v))
            }
            NodeKind::Input(idx) => {
                format!("    v[{}] = in[{}];\n", slot, idx)
            }
            NodeKind::Output(idx) => {
                let out_port = Port { node: id, index: 0 };
                if let Some(src) = self.graph.input_of(out_port) {
                    let src_slot = self.slot_of[&src.node];
                    format!("    out[{}] = v[{}];\n", idx, src_slot)
                } else {
                    format!("    // output {} unconnected\n", idx)
                }
            }
            NodeKind::Op { name, .. } => {
                if let Some(op_def) = self.reg.get(name) {
                    let mut in_slots = Vec::new();
                    for port_idx in 0..op_def.arity {
                        let in_port = Port { node: id, index: port_idx as u16 };
                        if let Some(src) = self.graph.input_of(in_port) {
                            in_slots.push(self.slot_of[&src.node]);
                        } else {
                            in_slots.push(usize::MAX); // unconnected → 0.0
                        }
                    }
                    if let Some(emit_fn) = op_def.emit_cpp_call {
                        let state_off = self.state_of.get(&id).copied().unwrap_or(0);
                        let resolved: Vec<usize> = in_slots.iter()
                            .map(|&s| if s == usize::MAX { 0 } else { s })
                            .collect();
                        format!("    {}\n", emit_fn(slot, &resolved, state_off, self.sr))
                    } else {
                        let in_args: Vec<String> = in_slots.iter()
                            .map(|&s| if s == usize::MAX { "0.0".into() } else { format!("v[{}]", s) })
                            .collect();
                        format!("    v[{}] = kernel_{}({});\n", slot, name, in_args.join(", "))
                    }
                } else {
                    format!("    // unknown op: {}\n", name)
                }
            }
            NodeKind::Param { .. } => {
                format!("    // param at v[{}]\n", slot)
            }
            NodeKind::Data { .. } => {
                format!("    // data buffer at state[{}]\n",
                    self.state_of.get(&id).copied().unwrap_or(0))
            }
            NodeKind::Region(_) => {
                format!("    // region at v[{}]\n", slot)
            }
        }
    }

    fn count_inputs(&self) -> usize {
        self.graph.nodes().filter(|(_, n)| matches!(n.kind, NodeKind::Input(_))).count()
    }

    fn count_outputs(&self) -> usize {
        self.graph.nodes().filter(|(_, n)| matches!(n.kind, NodeKind::Output(_))).count()
    }
}

fn format_f64(v: f64) -> String {
    // Ensure full precision for bit-identical output
    if v == 0.0 { return "0.0".into(); }
    format!("{:.15}", v)
}
