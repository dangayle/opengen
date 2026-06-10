//! Graph builder — convert a `Patcher` (parsed `.gendsp` JSON) into an `opengen_ir::Graph`.
//!
//! # Architecture
//!
//! The builder processes boxes and lines in phases:
//!
//! 1. **Classification** — Each box's `text` field is classified via [`crate::boxtext::classify_box_text`]
//!    into a [`BoxKind`]. IR nodes are created per box.
//! 2. **Wiring** — Lines between box outlets and inlets become graph edges.
//! 3. **Arg filling** — Numeric positional args from box text fill trailing inlets (rightmost first).
//! 4. **Default constants** — Any unwired, unfilled inlet gets constant 0.0.
//! 5. **Bus resolution** — Send/receive pairs resolved per-patcher scope (D9).
//! 6. **setparam** — Param consumers rewired to the setparam's input source (D13).
//! 7. **Codebox splicing** — Codeboxes are lowered into the host graph via
//!    `opengen_genexpr::lower_embedded`.
//!
//! # Design Decisions
//!
//! - D9: param @min/@max parsed but ignored at runtime
//! - D13: setparam rewires param consumers to the setparam's input source
//! - D14: mc_channel → constant 1.0
//! - Expression args (e.g., `* twopi/samplerate`) are parsed and lowered into graph nodes
//! - Param-name args resolve to the patcher's Param node

use std::collections::HashMap;
use opengen_ir::NodeId;  

use opengen_genexpr::{Expr, parse_expression, lower_embedded};
use opengen_ir::{Graph, Node, Port, StateDecl};
use opengen_ops::Registry;

use crate::boxtext::{BoxKind, classify_box_text};
use crate::model::{Patcher, GBox, Line};

/// Build an `opengen_ir::Graph` from a parsed `Patcher`.
///
/// The resulting graph can be compiled directly via `opengen_compile::compile`.
///
/// # Errors
///
/// Returns an error string for:
/// - Unknown operator names (not in the registry)
/// - Codebox parse/lower errors
/// - Missing param bindings for expression args
pub fn build_graph(patcher: &Patcher, registry: &Registry) -> Result<Graph, String> {
    let mut ctx = BuildCtx {
        graph: Graph::new(),
        registry,
        box_map: HashMap::new(),

        out_ports: HashMap::new(),
        bus_sends: HashMap::new(),
        receive_names: HashMap::new(),
        param_ports: HashMap::new(),
        setparam_nodes: HashMap::new(),
        delay_writes: HashMap::new(),
    };

    // ── Phase 1: Classify boxes and create IR nodes ───────────────
    ctx.classify_and_create_nodes(patcher)?;

    // ── Phase 2: Wire edges from lines ────────────────────────────
    ctx.wire_lines(&patcher.lines)?;

    // ── Phase 3: Fill expression args from box text ───────────────
    ctx.fill_operator_expr_args(patcher)?;

    // ── Phase 4: Default 0.0 for unfilled inlets ──────────────────
    ctx.fill_defaults()?;

    // ── Phase 4a: Fill delay box defaults ─────────────────────────
    ctx.fill_delay_defaults()?;

    // ── Phase 5: Bus resolution (send/receive) ────────────────────
    ctx.resolve_buses()?;

    // ── Phase 6: setparam rewiring (D13) ──────────────────────────
    ctx.rewire_setparams()?;

    // ── Phase 7: Codebox splicing ─────────────────────────────────
    ctx.splice_codeboxes(patcher)?;

    Ok(ctx.graph)
}

// ---------------------------------------------------------------------------
// Build context
// ---------------------------------------------------------------------------

struct BuildCtx<'a> {
    graph: Graph,
    registry: &'a Registry,
    /// Box ID → BoxInfo
    box_map: HashMap<String, BoxInfo>,

    /// Box ID → (outlet_idx, port) output ports
    out_ports: HashMap<String, Vec<(u16, Port)>>,
    /// Bus name → list of (send_placeholder_node, inlet_idx)
    bus_sends: HashMap<String, Vec<(NodeId, u16)>>,
    /// Receive box ID → bus name (for aliasing)
    receive_names: HashMap<String, String>,
    /// Param name → Param node port
    param_ports: HashMap<String, Port>,
    /// Param name → setparam box's placeholder node ID
    setparam_nodes: HashMap<String, NodeId>,
    /// Box ID → delay_write node ID (for special inlet wiring)
    delay_writes: HashMap<String, NodeId>,
}

/// Information about a classified box.
struct BoxInfo {
    #[allow(dead_code)]
    kind: BoxKind,
    /// Primary node ID for this box
    node_id: opengen_ir::NodeId,
    #[allow(dead_code)]
    text: String,
    /// Whether each inlet is wired (true) or not
    wired_inlets: Vec<bool>,
    /// Operator arity (for Operator boxes)
    arity: u16,
    /// Inlet indices that have been filled by args
    arg_filled: Vec<u16>,
}

// ─── Phase implementations ───────────────────────────────────────────────────

impl<'a> BuildCtx<'a> {
    /// Phase 1: Classify every box and create the corresponding IR node(s).
    fn classify_and_create_nodes(&mut self, patcher: &Patcher) -> Result<(), String> {
        for bx in &patcher.boxes {
            if bx.maxclass == "codebox" {
                // Codeboxes: create a placeholder node so wiring works.
                // Phase 7 (splice_codeboxes) will replace the outlet.
                let id = self.graph.add_node(Node::constant(0.0));
                let port = Port { node: id, index: 0 };
                self.add_box_out_port(&bx.id, 0, port);
                self.insert_box_info(&bx.id, &BoxKind::Constant(0.0), id, bx.numinlets);
                continue;
            }
            let kind = classify_box_text(&bx.text);
            self.create_node_for_box(bx, &kind)?;
        }
        Ok(())
    }

    /// Create IR node(s) for one box based on its classification.
    fn create_node_for_box(&mut self, bx: &GBox, kind: &BoxKind) -> Result<(), String> {
        match kind {
            BoxKind::Inlet(n) => {
                let idx = n.checked_sub(1).ok_or_else(||
                    format!("inlet index must be >= 1, got {} in box '{}'", n, bx.id)
                )?;
                let id = self.graph.add_node(Node::input(idx));
                let port = Port { node: id, index: 0 };
                self.add_box_out_port(&bx.id, 0, port);
                self.insert_box_info(&bx.id, kind, id, 0);
            }
            BoxKind::Outlet(n) => {
                let idx = n.checked_sub(1).ok_or_else(||
                    format!("outlet index must be >= 1, got {} in box '{}'", n, bx.id)
                )?;
                let id = self.graph.add_node(Node::output(idx));
                // Track the port — outlet has no outlet; it's a sink
                // The inlet (index 0) will be wired from lines
                self.insert_box_info(&bx.id, kind, id, 1);
            }
            BoxKind::Param(name, default) => {
                let id = self.graph.add_node(Node::param(name, *default));
                let port = Port { node: id, index: 0 };
                self.param_ports.insert(name.clone(), port);
                self.add_box_out_port(&bx.id, 0, port);
                self.insert_box_info(&bx.id, kind, id, 0);
            }
            BoxKind::SetParam(name) => {
                // setparam: pass-through. Create a constant placeholder.
                // The outlet drives param consumers (wired in Phase 6).
                let id = self.graph.add_node(Node::constant(0.0));
                let port = Port { node: id, index: 0 };
                self.add_box_out_port(&bx.id, 0, port);
                self.insert_box_info(&bx.id, kind, id, 1);
                // Inlet 0 will be wired from the driving signal
                self.setparam_nodes.insert(name.clone(), id);
            }
            BoxKind::Constant(v) => {
                let id = self.graph.add_node(Node::constant(*v));
                let port = Port { node: id, index: 0 };
                self.add_box_out_port(&bx.id, 0, port);
                self.insert_box_info(&bx.id, kind, id, 0);
            }
            BoxKind::Send(name) => {
                // Send: outlet drives the bus. Placeholder constant node.
                let id = self.graph.add_node(Node::constant(0.0));
                self.insert_box_info(&bx.id, kind, id, 1);
                // Inlet 0 will be wired from the signal to send. Record
                // the placeholder node so Phase 5 can look up the source.
                self.bus_sends.entry(name.clone()).or_default().push((id, 0));
            }
            BoxKind::Receive(name) => {
                // Receive: bus drives the outlet. Placeholder constant node.
                let id = self.graph.add_node(Node::constant(0.0));
                let port = Port { node: id, index: 0 };
                self.add_box_out_port(&bx.id, 0, port);
                self.insert_box_info(&bx.id, kind, id, 0);
                self.receive_names.insert(bx.id.clone(), name.clone());
            }
            BoxKind::History(name) => {
                let init = 0.0;
                let id = self.graph.add_node(Node::op(
                    "history",
                    vec![init],
                    StateDecl::Slots(1),
                ));
                let port = Port { node: id, index: 0 };
                self.add_box_out_port(&bx.id, 0, port);
                self.insert_box_info(&bx.id, kind, id, 1);
                // Named history: bind in graph for probes
                if let Some(n) = name {
                    self.graph.bind(n.clone(), id);
                }
            }
            BoxKind::Delay(_size, taps) => {
                if *taps > 1 {
                    return Err(format!(
                        "delay box '{}': multi-tap (TAPS={}) not yet supported (M3)",
                        bx.id, taps
                    ));
                }
                let box_id = bx.id.clone();
                let data_name = format!("__delaybox_{}", box_id);
                // Synthetic Data node: size+1 (slot 0 = cursor, 1..N = ring)
                self.graph.add_node(Node::data(&data_name, (*_size as usize) + 1));
                // delay_write: deferred write, inlet 0 = signal
                let write_id = self.graph.add_node(Node::op_with_data(
                    "delay_write", vec![], StateDecl::None, &data_name,
                ));
                // delay_read: inlet 0 = tap time, default linear interp
                let read_id = self.graph.add_node(Node::op_with_data(
                    "delay_read", vec![], StateDecl::None, &data_name,
                ));
                // Outlet 0 = read's output
                self.add_box_out_port(&box_id, 0, Port { node: read_id, index: 0 });
                // Store box_info: node_id = read_id, inlets = 2 (signal, tap)
                self.insert_box_info(&box_id, kind, read_id, 2);
                // Track delay_write for inlet 0 remapping
                self.delay_writes.insert(box_id, write_id);
            }
            BoxKind::Data(_name) => {
                let id = self.graph.add_node(Node::data(_name, 512));
                let port = Port { node: id, index: 0 };
                self.add_box_out_port(&bx.id, 0, port);
                self.insert_box_info(&bx.id, kind, id, 0);
            }
            BoxKind::McChannel => {
                let id = self.graph.add_node(Node::constant(1.0));
                let port = Port { node: id, index: 0 };
                self.add_box_out_port(&bx.id, 0, port);
                self.insert_box_info(&bx.id, kind, id, 0);
            }
            BoxKind::Subpatcher(name) => {
                return Err(format!(
                    "subpatcher/abstraction '{}' not yet supported (M3)",
                    name
                ));
            }
            BoxKind::Expr(expr) => {
                // Expression boxes: lower the expression into the graph.
                let port = self.lower_expr_inline(expr)?;
                self.add_box_out_port(&bx.id, 0, port);
                // For Expr boxes, we don't store a meaningful node_id in box_map
                // because the expression may consist of multiple nodes.
                // Wire_lines will use out_ports to find the source.
            }
            BoxKind::Operator { name: op_name_val, args, expr_args, attrs: _ } => {
                let op_reg_name = map_op_name(op_name_val);
                let op_def = self.registry.get(&op_reg_name).ok_or_else(|| {
                    format!("unknown operator '{}' in box '{}'", op_reg_name, bx.id)
                })?;

                let arity = op_def.arity;
                let id = self.graph.add_node(Node::op(
                    &op_reg_name, vec![], op_def.state,
                ));
                let port = Port { node: id, index: 0 };
                self.add_box_out_port(&bx.id, 0, port);

                // Store box info for later arg filling and inlet tracking
                let num_wired = bx.numinlets.max(arity);
                self.box_map.insert(bx.id.clone(), BoxInfo {
                    kind: kind.clone(),
                    node_id: id,
                    text: bx.text.clone(),
                    wired_inlets: vec![false; num_wired as usize],
                    arity,
                    arg_filled: Vec::new(),
                });

                // Fill numeric args into trailing inlets
                let n_args = args.len();
                let n_expr_args = expr_args.len();
                let total_args = n_args + n_expr_args;
                for (i, &val) in args.iter().enumerate() {
                    if arity > 0 {
                        let inlet = arity as u16 - 1 - (total_args - 1 - i) as u16;
                        let const_node = self.graph.add_node(Node::constant(val));
                        let const_port = Port { node: const_node, index: 0 };
                        self.graph.connect(const_port, Port { node: id, index: inlet });
                        if let Some(info) = self.box_map.get_mut(&bx.id) {
                            info.arg_filled.push(inlet);
                        }
                    }
                }

                // Track which inlets will be filled by expression args
                for i in 0..n_expr_args {
                    if arity > 0 {
                        let inlet = arity as u16 - 1 - (total_args - 1 - n_args - i) as u16;
                        if let Some(info) = self.box_map.get_mut(&bx.id) {
                            info.arg_filled.push(inlet);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Phase 2: Wire edges from lines.
    fn wire_lines(&mut self, lines: &[Line]) -> Result<(), String> {
        for line in lines {
            let (src_id, src_idx) = &line.src;
            let (dst_id, dst_idx) = &line.dst;

            // Find source port
            let src_port = self.get_box_outlet_port(src_id, *src_idx)?;

            // Check for delay box special inlet wiring
            if let Some(&write_node) = self.delay_writes.get(dst_id) {
                // Delay box: inlet 0 (signal) → delay_write.in0, inlet 1 (tap) → delay_read.in0
                let real_node = if *dst_idx == 0 { write_node } else { self.get_box_node_id(dst_id)? };
                let dst_port = Port { node: real_node, index: 0 };
                self.graph.connect(src_port, dst_port);
            } else {
                // Find destination: the box's node_id + inlet
                let dst_node = self.get_box_node_id(dst_id)?;
                let dst_port = Port { node: dst_node, index: *dst_idx };
                self.graph.connect(src_port, dst_port);
            }

            // Track wired inlet
            if let Some(info) = self.box_map.get_mut(dst_id) {
                let idx = *dst_idx as usize;
                if idx < info.wired_inlets.len() {
                    info.wired_inlets[idx] = true;
                }
            }
        }
        Ok(())
    }

    /// Phase 3: Fill expression args for operator boxes.
    fn fill_operator_expr_args(&mut self, patcher: &Patcher) -> Result<(), String> {
        for bx in &patcher.boxes {
            if bx.maxclass != "newobj" {
                continue;
            }
            let kind = classify_box_text(&bx.text);
            if let BoxKind::Operator { name: op_name_val, args, expr_args, .. } = &kind {
                if expr_args.is_empty() {
                    continue;
                }
                let node_id = self.get_box_node_id(&bx.id)?;
                // Derive arity from the operator definition
                let op_reg_name = map_op_name(op_name_val);
                let op_def = self.registry.get(&op_reg_name).ok_or_else(|| {
                    format!("unknown operator '{}' in box '{}'", op_reg_name, bx.id)
                })?;
                let arity = op_def.arity;
                if arity == 0 {
                    continue;
                }

                // Lower each expression arg to the correct trailing inlet
                let total_numeric = args.len();
                let total_expr = expr_args.len();
                let total_args = total_numeric + total_expr;
                for (i, expr_text) in expr_args.iter().enumerate() {
                    let inlet = arity as u16 - 1 - (total_args - 1 - total_numeric - i) as u16;
                    let port = self.lower_box_expr_arg(expr_text)?;
                    self.graph.connect(port, Port { node: node_id, index: inlet });
                }
            }
        }
        Ok(())
    }

    /// Phase 4: Fill default 0.0 constants for unwired, unfilled inlets.
    fn fill_defaults(&mut self) -> Result<(), String> {
        // Collect all box entries with arity > 0 into a separate vec to avoid borrow issues.
        // Skip delay boxes — they're handled separately in _fill_delay_defaults.
        let entries: Vec<(NodeId, Vec<bool>, Vec<u16>)> = self.box_map.iter()
            .filter(|(bid, info)| {
                info.arity > 0 && !info.wired_inlets.is_empty() && !self.delay_writes.contains_key(bid.as_str())
            })
            .map(|(_, info)| (info.node_id, info.wired_inlets.clone(), info.arg_filled.clone()))
            .collect();
        for (node_id, wired_inlets, arg_filled) in entries {
            let arity = wired_inlets.len() as u16;
            for inlet in 0..arity {
                let idx = inlet as usize;
                let is_wired = idx < wired_inlets.len() && wired_inlets[idx];
                let is_filled = arg_filled.contains(&inlet);
                if !is_wired && !is_filled {
                    let p = Port { node: node_id, index: inlet };
                    if self.graph.input_of(p).is_none() {
                        let const_node = self.graph.add_node(Node::constant(0.0));
                        self.graph.connect(Port { node: const_node, index: 0 }, p);
                    }
                }
            }
        }
        Ok(())
    }

    /// Phase 4a: Fill default 0.0 for delay box unwired inlets.
    ///
    /// Delay boxes have two inlets (signal, tap) mapped to two different IR nodes
    /// (delay_write and delay_read). The regular fill_defaults skips delay boxes;
    /// this method handles their special wiring:
    /// - Inlet 0 (signal) unwired → 0.0 to delay_write.in0 (silent delay line)
    /// - Inlet 1 (tap) unwired → 0.0 to delay_read.in0 → clamped to 1.0 by kernel
    fn fill_delay_defaults(&mut self) -> Result<(), String> {
        let ids: Vec<String> = self.delay_writes.keys().cloned().collect();
        for box_id in &ids {
            let write_node = self.delay_writes[box_id];
            let read_node = self.get_box_node_id(box_id)?;
            let info = self.box_map.get(box_id).unwrap();

            // Inlet 0 (signal) → delay_write.in0
            if !info.wired_inlets[0] {
                let p = Port { node: write_node, index: 0 };
                if self.graph.input_of(p).is_none() {
                    let cnst = self.graph.add_node(Node::constant(0.0));
                    self.graph.connect(Port { node: cnst, index: 0 }, p);
                }
            }
            // Inlet 1 (tap time) → delay_read.in0
            if !info.wired_inlets[1] {
                let p = Port { node: read_node, index: 0 };
                if self.graph.input_of(p).is_none() {
                    let cnst = self.graph.add_node(Node::constant(0.0));
                    self.graph.connect(Port { node: cnst, index: 0 }, p);
                }
            }
        }
        Ok(())
    }

    /// Phase 5: Bus resolution.
    ///
    /// For each bus, multiple sends sum via an `add` chain.
    /// Each receive aliases its output to the bus's summed signal.
    fn resolve_buses(&mut self) -> Result<(), String> {
        let bus_names: Vec<String> = self.bus_sends.keys().cloned().collect();
        for bus_name in &bus_names {
            let send_inlets = self.bus_sends.get(bus_name).cloned().unwrap_or_default();
            if send_inlets.is_empty() {
                continue;
            }

            // For each send, the bus signal is what's wired to the send's inlet
            // (looked up from the graph edges after Phase 2 wiring).
            let bus_signal_ports: Vec<Port> = send_inlets.iter()
                .filter_map(|&(placeholder_node, inlet_idx)| {
                    self.graph.input_of(Port { node: placeholder_node, index: inlet_idx })
                })
                .collect();

            if bus_signal_ports.is_empty() {
                // No sources wired to any send — skip
                continue;
            }

            // Build an add chain: (((send0 + send1) + send2) + ...)
            let bus_port = if bus_signal_ports.len() == 1 {
                bus_signal_ports[0]
            } else {
                let mut acc = bus_signal_ports[0];
                for &send_port in &bus_signal_ports[1..] {
                    let op_def = self.registry.get("add")
                        .ok_or_else(|| "add operator not registered".to_string())?;
                    let add_id = self.graph.add_node(Node::op("add", vec![], op_def.state));
                    self.graph.connect(acc, Port { node: add_id, index: 0 });
                    self.graph.connect(send_port, Port { node: add_id, index: 1 });
                    acc = Port { node: add_id, index: 0 };
                }
                acc
            };

            // Wire all receives to the bus signal
            let receive_box_ids: Vec<String> = self.receive_names.iter()
                .filter(|(_, name)| *name == bus_name)
                .map(|(bid, _)| bid.clone())
                .collect();

            for bid in &receive_box_ids {
                // Replace the receive placeholder's outlet with the bus signal
                self.replace_box_outlet(bid, 0, bus_port);
            }
        }
        Ok(())
    }

    /// Phase 6: setparam rewiring (D13).
    ///
    /// A `setparam <name>` box drives the named param from a signal.
    /// Consumers of param `<name>` are rewired to the setparam box's input source.
    fn rewire_setparams(&mut self) -> Result<(), String> {
        let keys: Vec<String> = self.setparam_nodes.keys().cloned().collect();
        for param_name in &keys {
            if let Some(&placeholder_node) = self.setparam_nodes.get(param_name) {
                // The setparam's inlet 0 source = the driving signal
                let setparam_source = match self.graph.input_of(Port { node: placeholder_node, index: 0 }) {
                    Some(p) => p,
                    None => continue, // no driving signal wired
                };

                // Find the param node's port
                if let Some(&param_port) = self.param_ports.get(param_name) {
                    // Collect all destination ports that consume this param
                    let consumers = self.find_consumers(param_port);

                    // Rewire each consumer from the param to the setparam's input source
                    for &consumer_port in &consumers {
                        self.graph.connect(setparam_source, consumer_port);
                    }
                }
            }
        }
        Ok(())
    }

    /// Phase 7: Splice codeboxes into the host graph.
    fn splice_codeboxes(&mut self, patcher: &Patcher) -> Result<(), String> {
        for bx in &patcher.boxes {
            if bx.maxclass != "codebox" || bx.code.is_empty() {
                continue;
            }

            // Find the old placeholder node_id from out_ports or box_map
            let old_placeholder_id = self.box_map.get(&bx.id).map(|info| info.node_id);

            // Parse the codebox source
            let program = opengen_genexpr::parse(&bx.code)
                .map_err(|e| format!("codebox '{}': {}", bx.id, e))?;

            // Build seeded input bindings: inN → host graph ports
            let mut seeded_inputs: HashMap<String, Port> = HashMap::new();
            for inlet_idx in 0..bx.numinlets {
                let in_name = format!("in{}", inlet_idx + 1);
                if let Some(p) = self.find_wired_input(&bx.id, inlet_idx) {
                    seeded_inputs.insert(in_name, p);
                } else {
                    let const_id = self.graph.add_node(Node::constant(0.0));
                    seeded_inputs.insert(in_name, Port { node: const_id, index: 0 });
                }
            }

            // Lower the codebox into the host graph
            let new_ports = lower_embedded(
                &program, &seeded_inputs, &mut self.graph, self.registry,
            ).map_err(|e| format!("codebox '{}': {}", bx.id, e))?;

            // Replace the old placeholder outlet(s) with new lowered ports
            // and rewire downstream consumers
            for &(out_idx, new_port) in &new_ports {
                // First, find any consumers of the old placeholder outlet
                if let Some(old_id) = old_placeholder_id {
                    let old_src = Port { node: old_id, index: out_idx };
                    let consumers = self.find_consumers(old_src);
                    for &consumer in &consumers {
                        self.graph.connect(new_port, consumer);
                    }
                }
                // Replace the out_ports entry
                self.replace_out_port(&bx.id, out_idx, new_port);
            }
        }
        Ok(())
    }

    // ─── Helper methods ───────────────────────────────────────────

    /// Add an output port for a box.
    fn add_box_out_port(&mut self, box_id: &str, outlet: u16, port: Port) {
        self.out_ports
            .entry(box_id.to_string())
            .or_default()
            .push((outlet, port));
    }

    /// Insert or update box_info for a box with a known node_id and inlet count.
    fn insert_box_info(&mut self, box_id: &str, kind: &BoxKind, node_id: opengen_ir::NodeId, inlets: u16) {
        self.box_map.insert(box_id.to_string(), BoxInfo {
            kind: kind.clone(),
            node_id,
            text: String::new(),
            wired_inlets: if inlets > 0 { vec![false; inlets as usize] } else { vec![] },
            arity: inlets,
            arg_filled: Vec::new(),
        });
    }

    /// Get the output port for a box's outlet.
    fn get_box_outlet_port(&self, box_id: &str, outlet: u16) -> Result<Port, String> {
        self.out_ports
            .get(box_id)
            .and_then(|ports| ports.iter().find(|(idx, _)| *idx == outlet).map(|(_, p)| *p))
            .ok_or_else(|| format!("box '{}' has no outlet {}", box_id, outlet))
    }

    /// Get the primary node ID for a box.
    fn get_box_node_id(&self, box_id: &str) -> Result<opengen_ir::NodeId, String> {
        // For outlet/param/inlet boxes, the node_id is the corresponding IR node.
        // For operator boxes, it's the op node.
        // For codeboxes, we use the first out_ports entry's node.
        if let Some(info) = self.box_map.get(box_id) {
            return Ok(info.node_id);
        }
        // Fallback: try to find any out_port for this box
        if let Some(ports) = self.out_ports.get(box_id) {
            if let Some((_, port)) = ports.first() {
                return Ok(port.node);
            }
        }
        Err(format!("unknown box '{}'", box_id))
    }

    /// Find the port wired to a box's inlet (for codebox input binding).
    fn find_wired_input(&self, box_id: &str, inlet: u16) -> Option<Port> {
        let info = self.box_map.get(box_id)?;
        let p = Port { node: info.node_id, index: inlet };
        self.graph.input_of(p)
    }

    /// Replace a box's outlet to point to a different port and rewire graph consumers.
    fn replace_box_outlet(&mut self, box_id: &str, outlet: u16, new_port: Port) {
        // Find the old port for this outlet, then rewire graph consumers
        if let Some(ports) = self.out_ports.get_mut(box_id) {
            if let Some((_, old_port)) = ports.iter_mut().find(|(idx, _)| *idx == outlet) {
                let old = *old_port;
                *old_port = new_port;
                // Rewire any graph consumers of the old port to the new port
                let consumers = self.find_consumers(old);
                for &consumer in &consumers {
                    self.graph.connect(new_port, consumer);
                }
            }
        }
    }

    /// Replace an out_port entry for a box, updating existing entry or appending.
    fn replace_out_port(&mut self, box_id: &str, outlet: u16, port: Port) {
        let ports = self.out_ports.entry(box_id.to_string()).or_default();
        if let Some(existing) = ports.iter_mut().find(|(idx, _)| *idx == outlet) {
            *existing = (outlet, port);
        } else {
            ports.push((outlet, port));
        }
    }

    /// Find all graph edges that consume from a given source port.
    /// Brute-force: check all inlets of every node.
    /// NOTE: gen~ boxes never exceed 16 inlets in practice; bound revisited if abstraction
    /// inlining needs more.
    fn find_consumers(&self, src: Port) -> Vec<Port> {
        let mut result = Vec::new();
        for (node_id, _) in self.graph.nodes() {
            for inlet in 0..16u16 {
                let p = Port { node: node_id, index: inlet };
                if let Some(s) = self.graph.input_of(p) {
                    if s == src {
                        result.push(p);
                        break;
                    }
                }
            }
        }
        result
    }

    /// Lower an expression argument into the host graph and return its output port.
    fn lower_box_expr_arg(&mut self, expr_text: &str) -> Result<Port, String> {
        // Try to parse as numeric literal first
        if let Ok(v) = expr_text.parse::<f64>() {
            let id = self.graph.add_node(Node::constant(v));
            return Ok(Port { node: id, index: 0 });
        }

        // Check if it's a param name (resolve to param port)
        if self.param_ports.contains_key(expr_text) {
            return Ok(self.param_ports[expr_text]);
        }

        // Try to parse as a GenExpr expression
        if let Ok(expr) = parse_expression(expr_text) {
            return self.lower_expr_inline(&expr);
        }

        // Treat as a builtin constant or samplerate
        if let Some(val) = resolve_builtin(expr_text) {
            let id = self.graph.add_node(Node::constant(val));
            return Ok(Port { node: id, index: 0 });
        }

        if expr_text == "samplerate" {
            let op_def = self.registry.get("samplerate")
                .ok_or_else(|| "'samplerate' operator not registered".to_string())?;
            let id = self.graph.add_node(Node::op("samplerate", vec![], op_def.state));
            return Ok(Port { node: id, index: 0 });
        }

        Err(format!("could not resolve box expression arg '{}'", expr_text))
    }

    /// Lower a GenExpr expression into the host graph and return its output port.
    fn lower_expr_inline(&mut self, expr: &Expr) -> Result<Port, String> {
        match expr {
            Expr::Number(n) => {
                let id = self.graph.add_node(Node::constant(*n));
                Ok(Port { node: id, index: 0 })
            }
            Expr::Str(s) => {
                Err(format!("string literal in expression not supported: '{}'", s))
            }
            Expr::Ident(name) => {
                // Check param bindings
                if let Some(&port) = self.param_ports.get(name) {
                    return Ok(port);
                }
                // Check builtin constants
                if let Some(val) = resolve_builtin(name) {
                    let id = self.graph.add_node(Node::constant(val));
                    return Ok(Port { node: id, index: 0 });
                }
                if name == "samplerate" {
                    let op_def = self.registry.get("samplerate")
                        .ok_or_else(|| "'samplerate' not registered".to_string())?;
                    let id = self.graph.add_node(Node::op("samplerate", vec![], op_def.state));
                    return Ok(Port { node: id, index: 0 });
                }
                // Check inN references
                if parse_input_name(name).is_some() {
                    let id = self.graph.add_node(Node::input(0)); // will be wired later
                    return Ok(Port { node: id, index: 0 });
                }
                Err(format!("undefined identifier in expression: '{}'", name))
            }
            Expr::BinOp { op, left, right } => {
                let op_name = op.op_name();
                let left_port = self.lower_expr_inline(left)?;
                let right_port = self.lower_expr_inline(right)?;
                let op_def = self.registry.get(op_name)
                    .ok_or_else(|| format!("unknown operator '{}' in expression", op_name))?;
                let id = self.graph.add_node(Node::op(op_name, vec![], op_def.state));
                self.graph.connect(left_port, Port { node: id, index: 0 });
                self.graph.connect(right_port, Port { node: id, index: 1 });
                Ok(Port { node: id, index: 0 })
            }
            Expr::Unary(opengen_genexpr::UnaryOp::Neg, e) => {
                let expr_port = self.lower_expr_inline(e)?;
                let op_def = self.registry.get("sub")
                    .ok_or_else(|| "'sub' not registered".to_string())?;
                let zero_id = self.graph.add_node(Node::constant(0.0));
                let sub_id = self.graph.add_node(Node::op("sub", vec![], op_def.state));
                self.graph.connect(Port { node: zero_id, index: 0 }, Port { node: sub_id, index: 0 });
                self.graph.connect(expr_port, Port { node: sub_id, index: 1 });
                Ok(Port { node: sub_id, index: 0 })
            }
            Expr::Unary(opengen_genexpr::UnaryOp::Not, e) => {
                let expr_port = self.lower_expr_inline(e)?;
                let op_def = self.registry.get("not")
                    .ok_or_else(|| "'not' not registered".to_string())?;
                let id = self.graph.add_node(Node::op("not", vec![], op_def.state));
                self.graph.connect(expr_port, Port { node: id, index: 0 });
                Ok(Port { node: id, index: 0 })
            }
            Expr::Call { name, args, .. } => {
                let op_def = self.registry.get(name)
                    .ok_or_else(|| format!("unknown function '{}' in expression", name))?;
                if args.len() != op_def.arity as usize {
                    return Err(format!(
                        "function '{}' expects {} args, got {}",
                        name, op_def.arity, args.len()
                    ));
                }
                let id = self.graph.add_node(Node::op(name, vec![], op_def.state));
                for (i, arg) in args.iter().enumerate() {
                    let arg_port = self.lower_expr_inline(arg)?;
                    self.graph.connect(arg_port, Port { node: id, index: i as u16 });
                }
                Ok(Port { node: id, index: 0 })
            }
            Expr::Ternary { cond, true_expr, false_expr } => {
                let cond_port = self.lower_expr_inline(cond)?;
                let true_port = self.lower_expr_inline(true_expr)?;
                let false_port = self.lower_expr_inline(false_expr)?;
                let op_def = self.registry.get("switch")
                    .ok_or_else(|| "'switch' not registered".to_string())?;
                let id = self.graph.add_node(Node::op("switch", vec![], op_def.state));
                self.graph.connect(cond_port, Port { node: id, index: 0 });
                self.graph.connect(true_port, Port { node: id, index: 1 });
                self.graph.connect(false_port, Port { node: id, index: 2 });
                Ok(Port { node: id, index: 0 })
            }
            Expr::MemberCall { .. } => {
                Err("member calls not supported in expression args".to_string())
            }
        }
    }
}

// ─── Free functions ──────────────────────────────────────────────────────────

/// Map box text operator name to registry operator name.
fn map_op_name(name: &str) -> String {
    match name {
        "+" => "add",
        "-" => "sub",
        "*" => "mul",
        "/" => "div",
        "%" => "mod",
        "!-" => "rsub",
        "!/" => "rdiv",
        "==" => "eq",
        "!=" => "neq",
        ">" => "gt",
        ">=" => "gte",
        "<" => "lt",
        "<=" => "lte",
        "&&" => "and",
        "||" => "or",
        "^^" => "xor",
        "?" => "switch",
        _ => name,
    }
    .to_string()
}

/// Resolve a builtin constant name to its f64 value.
fn resolve_builtin(name: &str) -> Option<f64> {
    match name {
        "pi" => Some(std::f64::consts::PI),
        "twopi" => Some(std::f64::consts::TAU),
        "halfpi" => Some(std::f64::consts::FRAC_PI_2),
        "invpi" => Some(std::f64::consts::FRAC_1_PI),
        "e" => Some(std::f64::consts::E),
        "ln2" => Some(std::f64::consts::LN_2),
        "ln10" => Some(std::f64::consts::LN_10),
        "log2e" => Some(std::f64::consts::LOG2_E),
        "log10e" => Some(std::f64::consts::LOG10_E),
        "sqrt2" => Some(std::f64::consts::SQRT_2),
        "sqrt1_2" => Some(std::f64::consts::FRAC_1_SQRT_2),
        "degtorad" => Some(std::f64::consts::PI / 180.0),
        "radtodeg" => Some(180.0 / std::f64::consts::PI),
        "vectorsize" => Some(1.0),
        _ => None,
    }
}

/// Parse "inN" to input index (0-based).
fn parse_input_name(name: &str) -> Option<u16> {
    if name.starts_with("in") {
        name[2..].parse::<u16>().ok().map(|n| n - 1)
    } else {
        None
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json;
    use crate::model::Patcher;

    /// Helper: build graph and render with inputs.
    fn build_and_render_with_inputs(
        gendsp_json: &str,
        sr: f64,
        inputs: &[&[f64]],
    ) -> opengen_testkit::Render {
        let j = json::parse(gendsp_json).unwrap();
        let patcher = Patcher::from_json(&j).unwrap();
        let graph = build_graph(&patcher, &opengen_ops::Registry::core()).unwrap();
        let n = inputs.iter().map(|c| c.len()).max().unwrap_or(0);
        opengen_testkit::render_graph_with_inputs(&graph, sr, inputs, n)
    }

    // ── TDD Anchor Tests ───────────────────────────────────────────

    /// minimal: `in 1` → `* 0.5` → `out 1` — exact halving.
    #[test]
    fn minimal_in_times_point5_out() {
        let src = r#"{
            "patcher": {
                "fileversion": 1,
                "classnamespace": "dsp.gen",
                "rect": [0, 0, 400, 300],
                "boxes": [
                    {"box": {"id": "obj-1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                    {"box": {"id": "obj-2", "maxclass": "newobj", "numinlets": 2, "numoutlets": 1, "text": "* 0.5"}},
                    {"box": {"id": "obj-3", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
                ],
                "lines": [
                    {"patchline": {"source": ["obj-1", 0], "destination": ["obj-2", 0]}},
                    {"patchline": {"source": ["obj-2", 0], "destination": ["obj-3", 0]}}
                ]
            }
        }"#;

        let out = build_and_render_with_inputs(src, 48000.0, &[&[2.0, 4.0, 6.0]]);
        assert_eq!(out.ch(0), &[1.0, 2.0, 3.0]); // in1 * 0.5
    }

    /// param-arg: `param g 3` + `* g` box → in1 * 3.
    #[test]
    fn param_times_param_arg() {
        let src = r#"{
            "patcher": {
                "fileversion": 1,
                "classnamespace": "dsp.gen",
                "rect": [0, 0, 400, 300],
                "boxes": [
                    {"box": {"id": "obj-1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                    {"box": {"id": "obj-2", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "param g 3"}},
                    {"box": {"id": "obj-3", "maxclass": "newobj", "numinlets": 2, "numoutlets": 1, "text": "*"}},
                    {"box": {"id": "obj-4", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
                ],
                "lines": [
                    {"patchline": {"source": ["obj-1", 0], "destination": ["obj-3", 0]}},
                    {"patchline": {"source": ["obj-2", 0], "destination": ["obj-3", 1]}},
                    {"patchline": {"source": ["obj-3", 0], "destination": ["obj-4", 0]}}
                ]
            }
        }"#;

        let out = build_and_render_with_inputs(src, 48000.0, &[&[5.0]]);
        assert_eq!(out.ch(0), &[15.0]); // in1 * 3
    }

    /// expression arg: `* twopi/samplerate` matches genexpr render of same formula.
    #[test]
    fn expression_arg_twopi_over_samplerate() {
        let src = r#"{
            "patcher": {
                "fileversion": 1,
                "classnamespace": "dsp.gen",
                "rect": [0, 0, 400, 300],
                "boxes": [
                    {"box": {"id": "obj-1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                    {"box": {"id": "obj-2", "maxclass": "newobj", "numinlets": 2, "numoutlets": 1, "text": "* twopi/samplerate"}},
                    {"box": {"id": "obj-3", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
                ],
                "lines": [
                    {"patchline": {"source": ["obj-1", 0], "destination": ["obj-2", 0]}},
                    {"patchline": {"source": ["obj-2", 0], "destination": ["obj-3", 0]}}
                ]
            }
        }"#;

        let expected_factor = std::f64::consts::TAU / 48000.0;
        let out = build_and_render_with_inputs(src, 48000.0, &[&[1.0]]);
        assert!((out.ch(0)[0] - expected_factor).abs() < 1e-15);
    }

    /// bus: send/receive roundtrip.
    #[test]
    fn bus_send_receive_roundtrip() {
        let src = r#"{
            "patcher": {
                "fileversion": 1,
                "classnamespace": "dsp.gen",
                "rect": [0, 0, 400, 300],
                "boxes": [
                    {"box": {"id": "obj-1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                    {"box": {"id": "obj-2", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "send mybus"}},
                    {"box": {"id": "obj-3", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "receive mybus"}},
                    {"box": {"id": "obj-4", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
                ],
                "lines": [
                    {"patchline": {"source": ["obj-1", 0], "destination": ["obj-2", 0]}},
                    {"patchline": {"source": ["obj-3", 0], "destination": ["obj-4", 0]}}
                ]
            }
        }"#;

        let out = build_and_render_with_inputs(src, 48000.0, &[&[42.0]]);
        assert_eq!(out.ch(0), &[42.0]); // send/receive roundtrip
    }

    /// codebox: a fixture embedding `out1 = in1 + 1;` codebox.
    #[test]
    fn codebox_out1_plus_1() {
        let src = r#"{
            "patcher": {
                "fileversion": 1,
                "classnamespace": "dsp.gen",
                "rect": [0, 0, 400, 300],
                "boxes": [
                    {"box": {"id": "obj-1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                    {"box": {"id": "obj-2", "maxclass": "codebox", "numinlets": 1, "numoutlets": 1, "code": "out1 = in1 + 1;"}},
                    {"box": {"id": "obj-3", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
                ],
                "lines": [
                    {"patchline": {"source": ["obj-1", 0], "destination": ["obj-2", 0]}},
                    {"patchline": {"source": ["obj-2", 0], "destination": ["obj-3", 0]}}
                ]
            }
        }"#;

        let out = build_and_render_with_inputs(src, 48000.0, &[&[5.0, 10.0]]);
        assert_eq!(out.ch(0), &[6.0, 11.0]); // in1 + 1
    }

    /// setparam: param consumers see the driven signal.
    #[test]
    fn setparam_rewiring() {
        let src = r#"{
            "patcher": {
                "fileversion": 1,
                "classnamespace": "dsp.gen",
                "rect": [0, 0, 400, 300],
                "boxes": [
                    {"box": {"id": "obj-1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                    {"box": {"id": "obj-2", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "param g 3"}},
                    {"box": {"id": "obj-3", "maxclass": "newobj", "numinlets": 1, "numoutlets": 1, "text": "setparam g"}},
                    {"box": {"id": "obj-4", "maxclass": "newobj", "numinlets": 2, "numoutlets": 1, "text": "*"}},
                    {"box": {"id": "obj-5", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
                ],
                "lines": [
                    {"patchline": {"source": ["obj-1", 0], "destination": ["obj-3", 0]}},
                    {"patchline": {"source": ["obj-1", 0], "destination": ["obj-4", 0]}},
                    {"patchline": {"source": ["obj-2", 0], "destination": ["obj-4", 1]}},
                    {"patchline": {"source": ["obj-4", 0], "destination": ["obj-5", 0]}}
                ]
            }
        }"#;

        // With in1 = 2 and setparam driving param g from in1:
        // param g default was 3, but setparam rewires: g is driven by in1 (= 2)
        // out1 = in1 * g = 2 * 2 = 4
        let out = build_and_render_with_inputs(src, 48000.0, &[&[2.0]]);
        assert_eq!(out.ch(0), &[4.0]); // in1 * setparam_driven_g = 2 * 2
    }

    /// A line referencing a non-existent source box should produce Err, not a panic.
    #[test]
    fn build_rejects_line_to_nonexistent_box() {
        let patcher = Patcher {
            boxes: vec![
                crate::model::GBox {
                    id: "obj-1".to_string(),
                    maxclass: "newobj".to_string(),
                    text: "in 1".to_string(),
                    code: String::new(),
                    numinlets: 0,
                    numoutlets: 1,
                    subpatcher: None,
                },
                crate::model::GBox {
                    id: "obj-2".to_string(),
                    maxclass: "newobj".to_string(),
                    text: "out 1".to_string(),
                    code: String::new(),
                    numinlets: 1,
                    numoutlets: 0,
                    subpatcher: None,
                },
            ],
            lines: vec![
                crate::model::Line {
                    src: ("obj-99".to_string(), 0),
                    dst: ("obj-2".to_string(), 0),
                },
            ],
        };
        let result = build_graph(&patcher, &opengen_ops::Registry::core());
        assert!(result.is_err(), "expected Err for line to non-existent box, got Ok");
        let err = result.unwrap_err();
        assert!(err.contains("obj-99"), "error should mention the missing box id: {}", err);
    }
}
