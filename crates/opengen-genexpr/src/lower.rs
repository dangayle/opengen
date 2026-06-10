//! Lower AST to IR Graph

use crate::ast::{*, UnaryOp};
use opengen_ir::{proc, Graph, Node, NodeKind, Port, StateDecl};
use opengen_ops::Registry;
use std::cell::Cell;
use std::collections::HashMap;

/// Builtin constants that GenExpr resolves in identifier position (unless shadowed).
///
/// # Documented
/// GenExpr Language Guide, chapter "Builtins & Constants": lists `pi`, `twopi`, `halfpi`,
/// `invpi`, `e`, `ln2`, `ln10`, `log2e`, `log10e`, `sqrt2`, `sqrt1_2`, `degtorad`, `radtodeg`.
/// Values match `std::f64::consts` where applicable.
///
/// `samplerate` is a separate arity-0 operator (not a constant); `vectorsize` is 1.0
/// per the per-sample engine divergence (see `# Divergence` on those entries).
const BUILTIN_CONSTANTS: &[(&str, f64)] = &[
    ("pi", std::f64::consts::PI),
    ("twopi", std::f64::consts::TAU),
    ("halfpi", std::f64::consts::FRAC_PI_2),
    ("invpi", std::f64::consts::FRAC_1_PI),
    ("e", std::f64::consts::E),
    ("ln2", std::f64::consts::LN_2),
    ("ln10", std::f64::consts::LN_10),
    ("log2e", std::f64::consts::LOG2_E),
    ("log10e", std::f64::consts::LOG10_E),
    ("sqrt2", std::f64::consts::SQRT_2),
    ("sqrt1_2", std::f64::consts::FRAC_1_SQRT_2),
    ("degtorad", std::f64::consts::PI / 180.0),
    ("radtodeg", 180.0 / std::f64::consts::PI),
    // `vectorsize` → 1.0 (per-sample engine; gen~ returns actual vector size).
    // # Divergence
    // opengen uses a per-sample engine (vectorsize is always 1), whereas gen~ can
    // process signal vectors of configurable size (typically 64 or 128 samples).
    ("vectorsize", 1.0),
];

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct LowerError {
    pub msg: String,
    pub loc: Option<SourceLoc>,
}

impl std::fmt::Display for LowerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.loc {
            Some(loc) => write!(f, "{}:{}: {}", loc.line, loc.col, self.msg),
            None => write!(f, "{}", self.msg),
        }
    }
}

impl std::error::Error for LowerError {}

// ---------------------------------------------------------------------------
// Region metadata — collected in a first pass over all statements
// ---------------------------------------------------------------------------

/// Metadata about identifiers used inside a control-flow region.
struct RegionMeta {
    /// All assigned non-output names → local slot index.
    locals: Vec<String>,
    local_idx: HashMap<String, u32>,
    /// Param declarations: (name, default).
    params: Vec<(String, f64)>,
    param_idx: HashMap<String, u16>,
    /// History declarations: (name, init).
    histories: Vec<(String, f64)>,
    hist_idx: HashMap<String, u32>,
    /// Distinct inN references (store the input index).
    in_refs: Vec<u16>,
    /// Distinct output indices.
    outputs: Vec<u16>,
    /// Stateful op calls inside region: each gets a state_base offset.
    /// We track the call-expression and later assign state slots.
    stateful_ops: Vec<StatefulOpInfo>,
    /// Cumulative state offset (after all History slots + previously allocated stateful ops).
    next_state_base: u32,
    /// Data/Buffer declarations: (name, size).
    data_decls: Vec<(String, usize)>,
}

/// Info about a stateful op call inside a region.
#[derive(Debug, Clone)]
struct StatefulOpInfo {
    /// The whole Call expression.
    expr: Expr,
    /// Size of state this call needs, from Registry.
    _state_size: u32,
    /// Assigned state_base offset into region state.
    state_base: u32,
}

impl RegionMeta {
    fn new() -> Self {
        Self {
            locals: Vec::new(),
            local_idx: HashMap::new(),
            params: Vec::new(),
            param_idx: HashMap::new(),
            histories: Vec::new(),
            hist_idx: HashMap::new(),
            in_refs: Vec::new(),
            outputs: Vec::new(),
            stateful_ops: Vec::new(),
            next_state_base: 0,
            data_decls: Vec::new(),
        }
    }

    fn add_local(&mut self, name: &str) {
        if !self.local_idx.contains_key(name)
            && !self.param_idx.contains_key(name)
            && !self.hist_idx.contains_key(name)
        {
            let idx = self.locals.len() as u32;
            self.locals.push(name.to_string());
            self.local_idx.insert(name.to_string(), idx);
        }
    }

    fn add_param(&mut self, name: &str, default: f64) {
        if !self.param_idx.contains_key(name) {
            let idx = self.params.len() as u16;
            self.params.push((name.to_string(), default));
            self.param_idx.insert(name.to_string(), idx);
        }
    }

    fn add_history(&mut self, name: &str, init: f64) -> Result<(), LowerError> {
        if self.hist_idx.contains_key(name) {
            return Err(LowerError {
                msg: format!("duplicate History declaration: {}", name),
                loc: None,
            });
        }
        let idx = self.histories.len() as u32;
        self.histories.push((name.to_string(), init));
        self.hist_idx.insert(name.to_string(), idx);
        Ok(())
    }

    fn add_in_ref(&mut self, n: u16) {
        if !self.in_refs.contains(&n) {
            self.in_refs.push(n);
        }
    }

    fn add_output(&mut self, n: u16) {
        if !self.outputs.contains(&n) {
            self.outputs.push(n);
        }
    }

    fn register_stateful_op(&mut self, expr: &Expr, state_size: u32) {
        let state_base = self.next_state_base;
        self.next_state_base += state_size;
        self.stateful_ops.push(StatefulOpInfo {
            expr: expr.clone(),
            _state_size: state_size,
            state_base,
        });
    }

    /// Total state slots: histories + stateful ops.
    fn n_state(&self) -> u32 {
        self.histories.len() as u32 + self.next_state_base
    }

    /// State init values: history inits first, then zero for stateful ops.
    fn state_init(&self) -> Vec<f64> {
        let mut init = Vec::with_capacity(self.n_state() as usize);
        for (_, v) in &self.histories {
            init.push(*v);
        }
        // stateful ops are zero-initialized
        init.resize(self.n_state() as usize, 0.0);
        init
    }

    /// State index for a History name.
    fn hist_slot(&self, name: &str) -> Option<u32> {
        self.hist_idx.get(name).copied()
    }

    /// Local index for a name.
    fn local_slot(&self, name: &str) -> Option<u32> {
        self.local_idx.get(name).copied()
    }

    /// Return the region input port index for a Param name.
    fn param_port(&self, name: &str) -> Option<u16> {
        self.param_idx.get(name).copied()
    }

    /// Return the region input port index for an inN reference.
    fn in_port(&self, in_index: u16) -> Option<u16> {
        let offset = self.params.len() as u16;
        self.in_refs.iter().position(|&n| n == in_index).map(|p| offset + p as u16)
    }

    fn n_inputs(&self) -> u16 {
        self.params.len() as u16 + self.in_refs.len() as u16
    }

    fn n_outputs(&self) -> u16 {
        self.outputs.iter().max().map(|m| m + 1).unwrap_or(0)
    }
}

// ---------------------------------------------------------------------------
// Lowerer
// ---------------------------------------------------------------------------

/// Info about a declared Delay.
struct DelayInfo {
    /// Name of the synthetic Data node ("__delay_<id>")
    data_name: String,
    /// Whether d.write() has been used (only one writer allowed)
    written: bool,
}

pub struct Lowerer<'a, 'b> {
    graph: &'b mut Graph,
    registry: &'a Registry,
    /// Maps identifier names to their output ports
    bindings: HashMap<String, Port>,
    /// Counter for stateful op call sites during region lowering.
    /// Ensures per-call-site state allocation even when two calls are
    /// structurally identical (same operator, same arguments).
    region_stateful_idx: Cell<usize>,
    /// Delay declarations: name -> delay info
    delay_bindings: HashMap<String, DelayInfo>,
    /// Counter for unique synthetic Data node names
    delay_counter: u32,
}

impl<'a, 'b> Lowerer<'a, 'b> {
    pub fn new(registry: &'a Registry, graph: &'b mut Graph) -> Self {
        Self {
            graph,
            registry,
            bindings: HashMap::new(),
            region_stateful_idx: Cell::new(0),
            delay_bindings: HashMap::new(),
            delay_counter: 0,
        }
    }

    pub fn lower(&mut self, program: &Program) -> Result<(), LowerError> {
        // Check if any statement (including inside blocks) has control flow.
        if has_program_control_flow(&program.statements) {
            return self.lower_to_region(program);
        }
        // M1 path: straight-line lowering (unchanged).
        for stmt in &program.statements {
            self.lower_statement(stmt)?;
        }
        Ok(())
    }

    /// Lower embedded into an existing graph with pre-seeded bindings.
    ///
    /// Unlike `lower()` which creates Input/Output nodes, this method:
    /// - Accepts pre-seeded bindings for inN ports (pointing into the host graph)
    /// - Returns output ports (outN → Port) instead of creating Output nodes
    /// - All other nodes (constants, ops, params, builtins) are added to the host graph
    ///
    /// Control-flow programs produce a Region node added to the host graph.
    /// Straight-line programs produce individual nodes in the host graph.
    pub fn lower_embedded(
        &mut self,
        program: &Program,
        seeded_inputs: HashMap<String, Port>,
    ) -> Result<Vec<(u16, Port)>, LowerError> {
        // Pre-seed bindings with the provided input ports
        for (name, port) in seeded_inputs {
            self.bindings.insert(name, port);
        }

        let out_ports = if has_program_control_flow(&program.statements) {
            // Region path: lower_to_region creates a Region node, wires inputs/outputs.
            // We need an adapted version that doesn't create Input/Output nodes.
            self.lower_embedded_region(program)?
        } else {
            // Straight-line path: lower statements, capture outN ports
            self.lower_embedded_straightline(program)?
        };

        Ok(out_ports)
    }

    /// Lower straight-line code and capture outN ports.
    fn lower_embedded_straightline(
        &mut self,
        program: &Program,
    ) -> Result<Vec<(u16, Port)>, LowerError> {
        let mut out_ports: Vec<(u16, Port)> = Vec::new();

        for stmt in &program.statements {
            match &stmt.kind {
                StatementKind::Assign { name, expr } => {
                    if let Some(output_idx) = parse_output_name(name) {
                        // outN assignment: lower expr, capture port (no Output node)
                        let port = self.lower_expr(expr)?;
                        out_ports.push((output_idx, port));
                        self.bindings.insert(name.clone(), port);
                    } else {
                        // Normal assignment
                        self.try_lower_statement(stmt)?;
                    }
                }
                _ => {
                    self.try_lower_statement(stmt)?;
                }
            }
        }

        Ok(out_ports)
    }

    /// Lower program with control flow into a Region node embedded in the host graph.
    fn lower_embedded_region(
        &mut self,
        program: &Program,
    ) -> Result<Vec<(u16, Port)>, LowerError> {
        self.region_stateful_idx.set(0);

        // ---- Pass 1: collect metadata ----
        let mut meta = RegionMeta::new();
        for stmt in &program.statements {
            self.collect_region_meta(stmt, &mut meta)?;
        }

        // ---- Build graph-level nodes - BUT don't create Input/Output nodes ----

        // Param nodes: create a node for each Param decl, wire to region input.
        let mut param_ports: Vec<(u16, Port)> = Vec::new();
        for (param_idx, (name, default)) in meta.params.iter().enumerate() {
            let node_id = self.graph.add_node(Node::param(name, *default));
            let port = Port { node: node_id, index: 0 };
            // Don't override pre-seeded bindings (e.g., from param boxes in the patcher)
            self.bindings.entry(name.clone()).or_insert(port);
            self.graph.bind(name.clone(), node_id);
            param_ports.push((param_idx as u16, port));
        }

        // Input nodes: check if pre-seeded, don't create new ones.
        // The meta.in_refs tells us which inN are referenced; we seed them
        // from self.bindings (which was pre-populated by the caller).
        let mut input_ports: Vec<(u16, Port)> = Vec::new();
        let base = meta.params.len() as u16;
        for (i, &in_idx) in meta.in_refs.iter().enumerate() {
            let name = format!("in{}", in_idx + 1);
            if let Some(&port) = self.bindings.get(&name) {
                // Pre-seeded by caller — use as-is
                input_ports.push((base + i as u16, port));
            } else {
                // Not seeded — create an Input node in the host graph
                let node_id = self.graph.add_node(Node::input(in_idx));
                let port = Port { node: node_id, index: 0 };
                self.bindings.insert(name, port);
                input_ports.push((base + i as u16, port));
            }
        }

        // Data/Buffer nodes: create a graph-level Data node for each decl.
        for (name, size) in &meta.data_decls {
            let node_id = self.graph.add_node(Node::data(name, *size));
            let port = Port { node: node_id, index: 0 };
            // Don't override pre-seeded bindings
            self.bindings.entry(name.clone()).or_insert(port);
            self.graph.bind(name.clone(), node_id);
        }

        // ---- Build ProcRegion - same as lower_to_region ----

        let n_inputs = meta.n_inputs();
        let n_outputs = meta.n_outputs();
        let n_locals = meta.locals.len() as u32;
        let n_state = meta.n_state();
        let state_init = meta.state_init();

        let mut input_port_of: HashMap<String, u16> = HashMap::new();
        for (name, _) in &meta.params {
            input_port_of.insert(name.clone(), meta.param_port(name).unwrap());
        }
        for &in_idx in &meta.in_refs {
            let name = format!("in{}", in_idx + 1);
            if let Some(port) = meta.in_port(in_idx) {
                input_port_of.insert(name, port);
            }
        }

        let mut body: Vec<proc::PStmt> = Vec::new();
        for stmt in &program.statements {
            let stmts = self.lower_region_stmt(stmt, &meta, &input_port_of)?;
            body.extend(stmts);
        }

        let region = proc::ProcRegion {
            n_inputs,
            n_outputs,
            n_locals,
            n_state,
            state_init,
            body,
        };

        let region_id = self.graph.add_node(Node::region(region));

        // ---- Wire graph edges ----
        for (port_idx, param_port) in &param_ports {
            self.graph.connect(*param_port, Port { node: region_id, index: *port_idx });
        }
        for (port_idx, in_port) in &input_ports {
            self.graph.connect(*in_port, Port { node: region_id, index: *port_idx });
        }

        // ---- Collect output ports (don't create Output nodes) ----
        let out_ports: Vec<(u16, Port)> = meta.outputs.iter().map(|&out_idx| {
            (out_idx, Port { node: region_id, index: out_idx })
        }).collect();

        Ok(out_ports)
    }

    // ─────────────────────────────────────────────────────────
    //  Region lowering path
    // ─────────────────────────────────────────────────────────

    /// Lower the entire program into one Region node.
    ///
    /// Architecture (D6): when the program contains any control-flow statement,
    /// the ENTIRE program body lowers into ONE Region node. Graph-level nodes
    /// remain only: Input nodes (for inN), Param nodes (for params), the Region,
    /// and Output nodes.
    fn lower_to_region(&mut self, program: &Program) -> Result<(), LowerError> {
        self.region_stateful_idx.set(0);

        // ---- Pass 1: collect metadata ----
        let mut meta = RegionMeta::new();
        for stmt in &program.statements {
            self.collect_region_meta(stmt, &mut meta)?;
        }

        // ---- Build graph-level nodes ----

        // Param nodes: create a node for each Param decl, wire to region input.
        let mut param_ports: Vec<(u16, Port)> = Vec::new(); // (region_port_idx, graph_node_port)
        for (param_idx, (name, default)) in meta.params.iter().enumerate() {
            let node_id = self.graph.add_node(Node::param(name, *default));
            let port = Port { node: node_id, index: 0 };
            self.bindings.insert(name.clone(), port);
            self.graph.bind(name.clone(), node_id);
            param_ports.push((param_idx as u16, port));
        }

        // Input nodes: create one per distinct inN reference.
        let mut input_ports: Vec<(u16, Port)> = Vec::new(); // (region_in_port_idx, graph_node_port)
        let base = meta.params.len() as u16;
        for (i, &in_idx) in meta.in_refs.iter().enumerate() {
            let name = format!("in{}", in_idx + 1);
            let node_id = self.graph.add_node(Node::input(in_idx));
            let port = Port { node: node_id, index: 0 };
            self.bindings.insert(name, port);
            // Input nodes are NOT added to graph.bind() — they aren't user bindings.
            input_ports.push((base + i as u16, port));
        }

        // Data/Buffer nodes: create a graph-level Data node for each decl.
        for (name, size) in &meta.data_decls {
            let node_id = self.graph.add_node(Node::data(name, *size));
            let port = Port { node: node_id, index: 0 };
            self.bindings.insert(name.clone(), port);
            self.graph.bind(name.clone(), node_id);
        }

        // ---- Build ProcRegion ----

        let n_inputs = meta.n_inputs();
        let n_outputs = meta.n_outputs();
        let n_locals = meta.locals.len() as u32;
        let n_state = meta.n_state();
        let state_init = meta.state_init();

        // Pre-populate input_port_of: Param name → region input port, inN → region input port.
        let mut input_port_of: HashMap<String, u16> = HashMap::new();
        for (name, _) in &meta.params {
            // param_port returns Some for any name that was registered by add_param.
            input_port_of.insert(name.clone(), meta.param_port(name).unwrap());
        }
        for &in_idx in &meta.in_refs {
            let name = format!("in{}", in_idx + 1);
            if let Some(port) = meta.in_port(in_idx) {
                input_port_of.insert(name, port);
            }
        }

        // We also need: stateful_op_info for matching during lowering.
        // The meta.stateful_ops contains the info.

        // ---- Pass 2: lower body statements to PStmt ----
        let mut body: Vec<proc::PStmt> = Vec::new();
        for stmt in &program.statements {
            let stmts = self.lower_region_stmt(
                stmt,
                &meta,
                &input_port_of,
            )?;
            body.extend(stmts);
        }

        let region = proc::ProcRegion {
            n_inputs,
            n_outputs,
            n_locals,
            n_state,
            state_init,
            body,
        };

        let region_id = self.graph.add_node(Node::region(region));

        // ---- Wire graph edges ----
        // Param nodes → region input ports.
        for (port_idx, param_port) in &param_ports {
            self.graph.connect(*param_port, Port { node: region_id, index: *port_idx });
        }
        // Input nodes → region input ports.
        for (port_idx, in_port) in &input_ports {
            self.graph.connect(*in_port, Port { node: region_id, index: *port_idx });
        }
        // Region output ports → Output nodes.
        for &out_idx in &meta.outputs {
            let out_node = self.graph.add_node(Node::output(out_idx));
            self.graph.connect(
                Port { node: region_id, index: out_idx },
                Port { node: out_node, index: 0 },
            );
        }

        Ok(())
    }

    // ─────────────────────────────────────────────────────────
    //  Metadata collection helpers
    // ─────────────────────────────────────────────────────────

    /// First pass: collect assigned names, params, histories, outputs, stateful ops.
    fn collect_region_meta(&mut self, stmt: &Statement, meta: &mut RegionMeta) -> Result<(), LowerError> {
        let stmt_loc = stmt.loc;
        self.try_collect_region_meta(stmt, meta)
            .map_err(|e| LowerError { msg: e.msg, loc: Some(stmt_loc) })
    }

    fn try_collect_region_meta(&mut self, stmt: &Statement, meta: &mut RegionMeta) -> Result<(), LowerError> {
        match &stmt.kind {
            StatementKind::ParamDecl { name, default } => {
                meta.add_param(name, *default);
                Ok(())
            }
            StatementKind::Decl { ty: DeclType::Param, items } => {
                for item in items {
                    let default = item.args.first().map(|e| match e {
                        Expr::Number(n) => *n,
                        _ => 0.0,
                    }).unwrap_or(0.0);
                    meta.add_param(&item.name, default);
                }
                Ok(())
            }
            StatementKind::Decl { ty: DeclType::History, items } => {
                for item in items {
                    let init = if let Some(init_expr) = item.args.first() {
                        Lowerer::const_fold(init_expr).ok_or_else(|| LowerError {
                            msg: format!(
                                "History '{}' init must be a constant (literal number)",
                                item.name
                            ),
                            loc: None,
                        })?
                    } else {
                        0.0
                    };
                    meta.add_history(&item.name, init)?;
                }
                Ok(())
            }
            StatementKind::Decl { ty: DeclType::Data | DeclType::Buffer, items } => {
                for item in items {
                    let size = match item.args.first() {
                        Some(Expr::Number(n)) => *n as usize,
                        Some(_) => {
                            return Err(LowerError {
                                msg: format!(
                                    "Buffer '{}': external buffer~ references are unsupported in M2 \
                                     (buffer is a data alias); declare with a numeric size",
                                    item.name
                                ),
                                loc: None,
                            });
                        }
                        None => 512,
                    };
                    meta.data_decls.push((item.name.clone(), size));
                }
                Ok(())
            }
            StatementKind::Decl { ty: DeclType::Delay, items } => {
                for item in items {
                    // In regions, we just allocate the state for the ring buffer.
                    // Member calls (d.write, d.read) inside regions are rejected with
                    // a clear error in lower_region_expr.
                    let size = match item.args.first() {
                        Some(Expr::Number(n)) => {
                            let s = *n as usize;
                            if s == 0 {
                                return Err(LowerError {
                                    msg: format!(
                                        "Delay '{}': size must be >= 1 (got 0)",
                                        item.name
                                    ),
                                    loc: None,
                                });
                            }
                            s
                        }
                        Some(_) => {
                            return Err(LowerError {
                                msg: format!(
                                    "Delay '{}': size must be a numeric literal",
                                    item.name
                                ),
                                loc: None,
                            });
                        }
                        None => {
                            return Err(LowerError {
                                msg: format!(
                                    "Delay '{}': size is required (got default)",
                                    item.name
                                ),
                                loc: None,
                            });
                        }
                    };
                    // Push a synthetic Data node to data_decls for state allocation.
                    // The data_decls loop in lower_to_region creates the Data node.
                    let data_name = format!("__delay_{}", self.delay_counter);
                    self.delay_counter += 1;
                    meta.data_decls.push((data_name.clone(), size + 1));
                    self.delay_bindings.insert(item.name.clone(), DelayInfo {
                        data_name,
                        written: false,
                    });
                }
                Ok(())
            }
            #[allow(unreachable_patterns)]
            StatementKind::Decl { ty, .. } => {
                Err(LowerError {
                    msg: format!("{:?} declarations not yet implemented in regions", ty),
                    loc: None,
                })
            }
            StatementKind::Assign { name, expr } => {
                if let Some(output_idx) = parse_output_name(name) {
                    meta.add_output(output_idx);
                } else if !meta.param_idx.contains_key(name) && !meta.hist_idx.contains_key(name) {
                    // If it's a param or history, the write goes to the param/history, not a local.
                    // History writes are handled in statement lowering via SetState.
                    // Param writes? Params are read-only in GenExpr. But we still need to register
                    // the name as "known" so it doesn't get an undefined-identifier error.
                    // Actually, if the name is already a param, it's not a local.
                }
                // For all non-output names, also register as local if not already a Param/History.
                if parse_output_name(name).is_none() && !meta.param_idx.contains_key(name) && !meta.hist_idx.contains_key(name) {
                    meta.add_local(name);
                }
                // Walk expression for inN refs, param refs, stateful ops, etc.
                self.collect_meta_from_expr(expr, meta);
                Ok(())
            }
            StatementKind::If { cond, then_branch, else_branch } => {
                self.collect_meta_from_expr(cond, meta);
                self.collect_region_meta(then_branch, meta)?;
                if let Some(else_b) = else_branch {
                    self.collect_region_meta(else_b, meta)?;
                }
                Ok(())
            }
            StatementKind::While { cond, body } => {
                self.collect_meta_from_expr(cond, meta);
                self.collect_region_meta(body, meta)?;
                Ok(())
            }
            StatementKind::DoWhile { body, cond } => {
                // Walk order MUST match lowering: body first, then cond.
                // The lowering desugar visits body expressions before cond expressions
                // (body; while(cond) { body; }), so metadata must collect in the same
                // order to keep positional state_base indices aligned.
                self.collect_region_meta(body, meta)?;
                self.collect_meta_from_expr(cond, meta);
                Ok(())
            }
            StatementKind::For { init, cond, step, body } => {
                // Walk order MUST match lowering: init, cond, body, step.
                // The lowering desugar is init; while(cond) { body; step; } —
                // it visits init, cond, body, then step. Metadata must collect
                // in this exact order for positional state_base alignment.
                if let Some(init_stmt) = init {
                    self.collect_region_meta(init_stmt, meta)?;
                }
                if let Some(cond_expr) = cond {
                    self.collect_meta_from_expr(cond_expr, meta);
                }
                self.collect_region_meta(body, meta)?;
                if let Some(step_expr) = step {
                    // For the step expression, treat it as a potential compound-assignment
                    // to a local: if step is a BinOp with Ident on left, the target gets a local slot.
                    if let Expr::BinOp { left, .. } = step_expr {
                        if let Expr::Ident(name) = left.as_ref() {
                            if parse_output_name(name).is_none() && !meta.param_idx.contains_key(name) && !meta.hist_idx.contains_key(name) {
                                meta.add_local(name);
                            }
                        }
                    }
                    self.collect_meta_from_expr(step_expr, meta);
                }
                Ok(())
            }
            StatementKind::Block(stmts) => {
                for s in stmts {
                    self.collect_region_meta(s, meta)?;
                }
                Ok(())
            }
            StatementKind::Break | StatementKind::Continue => {
                // No metadata to collect from break/continue.
                Ok(())
            }
            StatementKind::Return(_) => {
                Err(LowerError { msg: "return not yet implemented (Task 16)".to_string(), loc: None })
            }
            StatementKind::MultiAssign { .. } => {
                Err(LowerError { msg: "multi-assign not yet implemented (Task 16)".to_string(), loc: None })
            }
            StatementKind::FuncDecl { .. } => {
                Err(LowerError { msg: "function declarations not yet implemented (Task 16)".to_string(), loc: None })
            }
            StatementKind::Require(_) => {
                Err(LowerError { msg: "require unsupported in M2".to_string(), loc: None })
            }
            StatementKind::ExprStmt(expr) => {
                self.collect_meta_from_expr(expr, meta);
                Ok(())
            }
        }
    }

    /// Walk an expression looking for inN/param/history refs, builtin constants,
    /// samplerate, and stateful op calls.
    fn collect_meta_from_expr(&self, expr: &Expr, meta: &mut RegionMeta) {
        match expr {
            Expr::Number(_) | Expr::Str(_) => {}
            Expr::Ident(name) => {
                if let Some(in_idx) = parse_input_name(name) {
                    meta.add_in_ref(in_idx);
                }
                // Param references are discovered via the ident lookup in lowering;
                // we don't need to pre-collect them here since the param is already
                // in meta.params. But we check if the name matches a param/history.
                // Already handled by param/history registration.
            }
            Expr::BinOp { left, right, .. } => {
                self.collect_meta_from_expr(left, meta);
                self.collect_meta_from_expr(right, meta);
            }
            Expr::Unary(_, e) => {
                self.collect_meta_from_expr(e, meta);
            }
            Expr::Call { name, args, .. } => {
                // Check for stateful ops that need per-call-site state.
                if let Some(op_def) = self.registry.get(name) {
                    if op_def.state != StateDecl::None && op_def.deferred_ports.is_empty() && op_def.update.is_none() {
                        // Kernel-managed stateful op (phasor, cycle, noise) — allocate state.
                        let state_size = match op_def.state {
                            StateDecl::Slots(n) => n,
                            StateDecl::None => 0,
                        };
                        if state_size > 0 {
                            meta.register_stateful_op(expr, state_size);
                        }
                    }
                }
                for arg in args {
                    self.collect_meta_from_expr(arg, meta);
                }
            }
            Expr::MemberCall { args, .. } => {
                for arg in args {
                    self.collect_meta_from_expr(arg, meta);
                }
            }
            Expr::Ternary { cond, true_expr, false_expr } => {
                self.collect_meta_from_expr(cond, meta);
                self.collect_meta_from_expr(true_expr, meta);
                self.collect_meta_from_expr(false_expr, meta);
            }
        }
    }

    // ─────────────────────────────────────────────────────────
    //  Region statement lowering
    // ─────────────────────────────────────────────────────────

    /// Lower a statement (inside a region) to zero or more PStmts.
    fn lower_region_stmt(
        &self,
        stmt: &Statement,
        meta: &RegionMeta,
        input_port_of: &HashMap<String, u16>,
    ) -> Result<Vec<proc::PStmt>, LowerError> {
        let stmt_loc = stmt.loc;
        self.try_lower_region_stmt(stmt, meta, input_port_of)
            .map_err(|e| LowerError { msg: e.msg, loc: Some(stmt_loc) })
    }

    fn try_lower_region_stmt(
        &self,
        stmt: &Statement,
        meta: &RegionMeta,
        input_port_of: &HashMap<String, u16>,
    ) -> Result<Vec<proc::PStmt>, LowerError> {
        match &stmt.kind {
            StatementKind::ParamDecl { .. } => {
                // Params are already registered; inside region they read as In(port).
                Ok(vec![])
            }
            StatementKind::Decl { ty: DeclType::Param, .. } => {
                Ok(vec![])
            }
            StatementKind::Decl { ty: DeclType::History, .. } => {
                // History decls are already registered in state; no PStmt needed.
                Ok(vec![])
            }
            StatementKind::Decl { ty: DeclType::Data | DeclType::Buffer, .. } => {
                // Data/Buffer decls are handled in collect_region_meta; no PStmt needed.
                Ok(vec![])
            }
            StatementKind::Decl { ty: DeclType::Delay, .. } => {
                // Delay decls are handled in collect_region_meta; no PStmt needed.
                Ok(vec![])
            }
            #[allow(unreachable_patterns)]
            StatementKind::Decl { .. } => {
                Err(LowerError {
                    msg: "declarations not yet supported inside region".to_string(),
                    loc: None,
                })
            }
            StatementKind::Assign { name, expr } => {
                if let Some(output_idx) = parse_output_name(name) {
                    let e = self.lower_region_expr(expr, meta, input_port_of)?;
                    Ok(vec![proc::PStmt::SetOut { index: output_idx, expr: e }])
                } else if let Some(state_idx) = meta.hist_slot(name) {
                    // History write: immediate (D6/genlib codebox semantics).
                    let e = self.lower_region_expr(expr, meta, input_port_of)?;
                    Ok(vec![proc::PStmt::SetState { index: state_idx, expr: e }])
                } else if let Some(local_idx) = meta.local_slot(name) {
                    let e = self.lower_region_expr(expr, meta, input_port_of)?;
                    Ok(vec![proc::PStmt::SetLocal { dst: local_idx, expr: e }])
                } else {
                    Err(LowerError {
                        msg: format!("internal: unknown name '{}' in region assign", name),
                        loc: None,
                    })
                }
            }
            StatementKind::If { cond, then_branch, else_branch } => {
                let cond_expr = self.lower_region_expr(cond, meta, input_port_of)?;
                let then_body = self.lower_region_stmts_for_block(
                    &[then_branch.as_ref().clone()],
                    meta,
                    input_port_of,
                )?;
                let else_body = if let Some(else_b) = else_branch {
                    self.lower_region_stmts_for_block(
                        &[else_b.as_ref().clone()],
                        meta,
                        input_port_of,
                    )?
                } else {
                    vec![]
                };
                Ok(vec![proc::PStmt::If {
                    cond: cond_expr,
                    then_body,
                    else_body,
                }])
            }
            StatementKind::While { cond, body } => {
                let cond_expr = self.lower_region_expr(cond, meta, input_port_of)?;
                let body_stmts = self.lower_region_stmts_for_block(
                    &[body.as_ref().clone()],
                    meta,
                    input_port_of,
                )?;
                Ok(vec![proc::PStmt::While {
                    cond: cond_expr,
                    body: body_stmts,
                }])
            }
            StatementKind::DoWhile { body, cond } => {
                // Desugar: do { body } while (cond); → body; while (cond) { body; }
                let body_stmts = self.lower_region_stmts_for_block(
                    &[body.as_ref().clone()],
                    meta,
                    input_port_of,
                )?;
                let cond_expr = self.lower_region_expr(cond, meta, input_port_of)?;
                let mut result: Vec<proc::PStmt> = Vec::new();
                // Execute body once unconditionally
                result.extend(body_stmts.clone());
                // Then while (cond) { body }
                result.push(proc::PStmt::While {
                    cond: cond_expr,
                    body: body_stmts,
                });
                Ok(result)
            }
            StatementKind::For { init, cond, step, body } => {
                // Desugar: for (init; cond; step) body → init; while (cond) { body; step; }
                let mut result: Vec<proc::PStmt> = Vec::new();

                // Init: statements to execute before the while
                if let Some(init_stmt) = init {
                    let init_stmts = self.lower_region_stmt(init_stmt, meta, input_port_of)?;
                    result.extend(init_stmts);
                }

                // Condition (default true)
                let cond_expr = if let Some(c) = cond {
                    self.lower_region_expr(c, meta, input_port_of)?
                } else {
                    proc::PExpr::Const(1.0)
                };

                // Body of the while: original body + step
                let mut while_body = Vec::new();
                let body_stmts = self.lower_region_stmts_for_block(
                    &[body.as_ref().clone()],
                    meta,
                    input_port_of,
                )?;
                while_body.extend(body_stmts);

                // Step: convert step expression to an assignment statement
                // #continue-in-for
                // The desugar `for (init; cond; step) body → init; while (cond) { body; step; }`
                // means `continue` inside the for body SKIPS the step. This diverges from C
                // where `continue` still executes the step. Documented as a known decision
                // (the alternative — emitting `do { if(cond){body;continue?}}` — was rejected
                // as too complex; the dang-tools corpus does not use continue-in-for).
                if let Some(step_expr) = step {
                    let step_stmts = self.for_step_as_assign(step_expr, meta, input_port_of)?;
                    while_body.extend(step_stmts);
                }

                result.push(proc::PStmt::While {
                    cond: cond_expr,
                    body: while_body,
                });

                Ok(result)
            }
            StatementKind::Block(stmts) => {
                let mut result = Vec::new();
                for s in stmts {
                    result.extend(self.lower_region_stmt(s, meta, input_port_of)?);
                }
                Ok(result)
            }
            StatementKind::Break => {
                Ok(vec![proc::PStmt::Break])
            }
            StatementKind::Continue => {
                Ok(vec![proc::PStmt::Continue])
            }
            StatementKind::Return(_) => {
                Err(LowerError { msg: "return not yet implemented (Task 16)".to_string(), loc: None })
            }
            StatementKind::MultiAssign { .. } => {
                Err(LowerError { msg: "multi-assign not yet implemented (Task 16)".to_string(), loc: None })
            }
            StatementKind::FuncDecl { .. } => {
                Err(LowerError { msg: "function declarations not yet implemented (Task 16)".to_string(), loc: None })
            }
            StatementKind::Require(_) => {
                Err(LowerError { msg: "require unsupported in M2".to_string(), loc: None })
            }
            StatementKind::ExprStmt(expr) => {
                let e = self.lower_region_expr(expr, meta, input_port_of)?;
                Ok(vec![proc::PStmt::Eval(e)])
            }
        }
    }

    /// Lower a list of statements as a block (for if/while bodies).
    fn lower_region_stmts_for_block(
        &self,
        stmts: &[Statement],
        meta: &RegionMeta,
        input_port_of: &HashMap<String, u16>,
    ) -> Result<Vec<proc::PStmt>, LowerError> {
        let mut result = Vec::new();
        for s in stmts {
            result.extend(self.lower_region_stmt(s, meta, input_port_of)?);
        }
        Ok(result)
    }

    /// Convert a for-loop step expression into assignment statements.
    /// The parser desugars compound assignment `i += 1` into `BinOp(Add, Ident("i"), Num(1))`,
    /// but this is just an expression, not an assignment. We re-interpret it here.
    fn for_step_as_assign(
        &self,
        step: &Expr,
        meta: &RegionMeta,
        input_port_of: &HashMap<String, u16>,
    ) -> Result<Vec<proc::PStmt>, LowerError> {
        // If step is a BinOp with Ident on the left, produce SetLocal { name }.
        match step {
            Expr::BinOp { left, .. } if matches!(left.as_ref(), Expr::Ident(_)) => {
                if let Expr::Ident(name) = left.as_ref() {
                    if let Some(local_idx) = meta.local_slot(name) {
                        let e = self.lower_region_expr(step, meta, input_port_of)?;
                        return Ok(vec![proc::PStmt::SetLocal { dst: local_idx, expr: e }]);
                    }
                }
                // Fall through: evaluate as expression statement
                let e = self.lower_region_expr(step, meta, input_port_of)?;
                Ok(vec![proc::PStmt::Eval(e)])
            }
            _ => {
                let e = self.lower_region_expr(step, meta, input_port_of)?;
                Ok(vec![proc::PStmt::Eval(e)])
            }
        }
    }

    // ─────────────────────────────────────────────────────────
    //  Region expression lowering
    // ─────────────────────────────────────────────────────────

    /// Lower an expression to a PExpr inside a region context.
    fn lower_region_expr(
        &self,
        expr: &Expr,
        meta: &RegionMeta,
        input_port_of: &HashMap<String, u16>,
    ) -> Result<proc::PExpr, LowerError> {
        match expr {
            Expr::Number(n) => Ok(proc::PExpr::Const(*n)),
            Expr::Ident(name) => {
                // Input references
                if let Some(in_idx) = parse_input_name(name) {
                    if let Some(port) = meta.in_port(in_idx) {
                        return Ok(proc::PExpr::In(port));
                    }
                    return Err(LowerError {
                        msg: format!("input '{}' referenced but not registered in region meta", name),
                        loc: None,
                    });
                }

                // History state reads
                if let Some(state_idx) = meta.hist_slot(name) {
                    return Ok(proc::PExpr::State(state_idx));
                }

                // Param reads → region input port
                if let Some(port) = meta.param_port(name) {
                    return Ok(proc::PExpr::In(port));
                }

                // Local reads
                if let Some(local_idx) = meta.local_slot(name) {
                    return Ok(proc::PExpr::Local(local_idx));
                }

                // Check for builtin constants
                if let Some(&val) = BUILTIN_CONSTANTS.iter().find(|(k, _)| *k == name).map(|(_, v)| v) {
                    return Ok(proc::PExpr::Const(val));
                }

                // Samplerate
                if name == "samplerate" {
                    return Ok(proc::PExpr::Call {
                        op: "samplerate".to_string(),
                        args: vec![],
                        state_base: u32::MAX,
                        data_ref: None,
                    });
                }

                Err(LowerError {
                    msg: format!("undefined identifier: {}", name),
                    loc: None,
                })
            }
            Expr::BinOp { op, left, right } => {
                let op_name = op.op_name();
                let left_expr = self.lower_region_expr(left, meta, input_port_of)?;
                let right_expr = self.lower_region_expr(right, meta, input_port_of)?;
                Ok(proc::PExpr::Call {
                    op: op_name.to_string(),
                    args: vec![left_expr, right_expr],
                    state_base: u32::MAX,
                    data_ref: None,
                })
            }
            Expr::Unary(UnaryOp::Neg, e) => {
                // -x → 0 - x
                let expr_port = self.lower_region_expr(e, meta, input_port_of)?;
                Ok(proc::PExpr::Call {
                    op: "sub".to_string(),
                    args: vec![proc::PExpr::Const(0.0), expr_port],
                    state_base: u32::MAX,
                    data_ref: None,
                })
            }
            Expr::Unary(UnaryOp::Not, e) => {
                let expr_port = self.lower_region_expr(e, meta, input_port_of)?;
                Ok(proc::PExpr::Call {
                    op: "not".to_string(),
                    args: vec![expr_port],
                    state_base: u32::MAX,
                    data_ref: None,
                })
            }
            Expr::Call { name, args, named_args } => {
                if !named_args.is_empty() {
                    return Err(LowerError {
                        msg: format!("named arguments not yet implemented for '{}'", name),
                        loc: None,
                    });
                }

                // Special case: peek/poke — first arg is a data name (identifier), not a value.
                if name == "peek" || name == "poke" {
                    if args.is_empty() {
                        return Err(LowerError {
                            msg: format!("function '{}' requires a data name as first argument", name),
                            loc: None,
                        });
                    }
                    let data_name = match &args[0] {
                        Expr::Ident(n) => n.clone(),
                        _ => return Err(LowerError {
                            msg: format!("first argument to '{}' must be a data name (identifier), got expression", name),
                            loc: None,
                        }),
                    };
                    // Verify the data name exists as a Data/Buffer binding in the graph.
                    // We check only self.bindings (graph-level Data/Buffer nodes, including those
                    // declared inside the region and promoted during region lowering). We do NOT
                    // check meta.local_slot — a region-local variable is NOT a valid data buffer.
                    if self.bindings.get(&data_name).is_none() {
                        return Err(LowerError {
                            msg: format!("unknown data buffer '{}' in '{}'", data_name, name),
                            loc: None,
                        });
                    }

                    let op_def = self.registry.get(name).ok_or_else(|| LowerError {
                        msg: format!("unknown function: {}", name),
                        loc: None,
                    })?;

                    let signal_args = &args[1..];
                    if signal_args.len() != op_def.arity as usize {
                        return Err(LowerError {
                            msg: format!(
                                "function '{}' expects {} signal arguments, got {}",
                                name, op_def.arity, signal_args.len()
                            ),
                            loc: None,
                        });
                    }

                    let mut lowered_args: Vec<proc::PExpr> = Vec::new();
                    for arg in signal_args {
                        lowered_args.push(self.lower_region_expr(arg, meta, input_port_of)?);
                    }

                    // Check for stateful ops — peek/poke are stateless (state comes from data_ref).
                    if op_def.update.is_some() || !op_def.deferred_ports.is_empty() {
                        return Err(LowerError {
                            msg: format!(
                                "operator '{}' with deferred updates cannot be called inside a region",
                                name
                            ),
                            loc: None,
                        });
                    }

                    return Ok(proc::PExpr::Call {
                        op: name.clone(),
                        args: lowered_args,
                        state_base: u32::MAX,
                        data_ref: Some(data_name),
                    });
                }

                // Check for history(…) CALL inside region — this is an error.
                if name == "history" {
                    return Err(LowerError {
                        msg: "history() function call inside region: use History declaration instead"
                            .to_string(),
                        loc: None,
                    });
                }

                let op_def = self.registry.get(name).ok_or_else(|| LowerError {
                    msg: format!("unknown function: {}", name),
                    loc: None,
                })?;

                if args.len() != op_def.arity as usize {
                    return Err(LowerError {
                        msg: format!(
                            "function '{}' expects {} arguments, got {}",
                            name,
                            op_def.arity,
                            args.len()
                        ),
                        loc: None,
                    });
                }

                // Check for stateful ops with deferred_ports/update (history/delay)
                // — they belong to decls, not region call sites.
                if op_def.update.is_some() || !op_def.deferred_ports.is_empty() {
                    return Err(LowerError {
                        msg: format!(
                            "operator '{}' with deferred updates cannot be called inside a region",
                            name
                        ),
                        loc: None,
                    });
                }

                // Determine state_base: look up in meta.stateful_ops by POSITION.
                // Both the metadata collection pass and this lowering pass walk the
                // AST in the same DFS order, so the nth stateful call encountered
                // during lowering corresponds to the nth entry in meta.stateful_ops.
                // This avoids the bug of matching structurally identical calls
                // (e.g. two noise() calls) via `find` + structural equality, which
                // would share state across call sites.
                //
                // The positional guard below is a BEST-EFFORT name check, not
                // structural proof. Two calls to the same operator (e.g. two phasor
                // calls at different frequencies) have the same op name and would
                // slip past the check but still get the correct state_base slot
                // (the positional index determines the slot, not the name). The
                // check catches the programmer error where an entirely different
                // operator appears at a given position (e.g. metadata collected
                // noise but lowering visits phasor), which indicates a structural
                // walk-order mismatch between the two passes.
                let state_base = if op_def.state != StateDecl::None {
                    let idx = self.region_stateful_idx.get();
                    self.region_stateful_idx.set(idx + 1);
                    if let Some(info) = meta.stateful_ops.get(idx) {
                        // Best-effort guard: verify walk-order agreement by checking the op name.
                        if let Expr::Call { name: info_name, .. } = &info.expr {
                            if info_name != name {
                                return Err(LowerError {
                                    msg: format!(
                                        "internal: stateful op walk-order mismatch at position {}: \
                                         expected '{}', found '{}'",
                                        idx, info_name, name
                                    ),
                                    loc: None,
                                });
                            }
                        }
                        meta.histories.len() as u32 + info.state_base
                    } else {
                        u32::MAX
                    }
                } else {
                    u32::MAX
                };

                let lower_args: Result<Vec<proc::PExpr>, LowerError> = args
                    .iter()
                    .map(|a| self.lower_region_expr(a, meta, input_port_of))
                    .collect();

                Ok(proc::PExpr::Call {
                    op: name.clone(),
                    args: lower_args?,
                    state_base,
                    data_ref: None,
                })
            }
            Expr::Ternary { cond, true_expr, false_expr } => {
                // Ternary → switch op
                let cond_expr = self.lower_region_expr(cond, meta, input_port_of)?;
                let true_ex = self.lower_region_expr(true_expr, meta, input_port_of)?;
                let false_ex = self.lower_region_expr(false_expr, meta, input_port_of)?;
                Ok(proc::PExpr::Call {
                    op: "switch".to_string(),
                    args: vec![cond_expr, true_ex, false_ex],
                    state_base: u32::MAX,
                    data_ref: None,
                })
            }
            Expr::MemberCall { object, .. } => {
                let obj_name = match object.as_ref() {
                    Expr::Ident(name) => name.clone(),
                    _ => return Err(LowerError {
                        msg: "member calls on non-identifier expressions are not supported".to_string(),
                        loc: None,
                    }),
                };
                if self.delay_bindings.contains_key(&obj_name) {
                    Err(LowerError {
                        msg: format!(
                            "Delay member calls inside control flow are not supported in M2"
                        ),
                        loc: None,
                    })
                } else {
                    Err(LowerError {
                        msg: format!("member calls not supported on '{}'", obj_name),
                        loc: None,
                    })
                }
            }
            Expr::Str(_) => {
                Err(LowerError {
                    msg: "string literals not supported in runtime expressions".to_string(),
                    loc: None,
                })
            }
        }
    }

    // ─────────────────────────────────────────────────────────
    //  M1 statement lowering (unchanged)
    // ─────────────────────────────────────────────────────────

    fn lower_statement(&mut self, stmt: &Statement) -> Result<(), LowerError> {
        let stmt_loc = stmt.loc;
        self.try_lower_statement(stmt)
            .map_err(|e| LowerError { msg: e.msg, loc: Some(stmt_loc) })
    }

    /// Inner helper that uses `?` freely; `lower_statement` wraps errors with statement location.
    fn try_lower_statement(&mut self, stmt: &Statement) -> Result<(), LowerError> {
        match &stmt.kind {
            StatementKind::ParamDecl { name, default } => {
                let node_id = self.graph.add_node(Node::param(name, *default));
                let port = Port { node: node_id, index: 0 };
                self.bindings.insert(name.clone(), port);
                self.graph.bind(name.clone(), node_id);
                Ok(())
            }
            StatementKind::Decl { ty: DeclType::Param, items } => {
                // Fold Param declarations into lowerable nodes
                for item in items {
                    let default = item.args.first().map(|e| match e {
                        Expr::Number(n) => *n,
                        _ => 0.0, // fallback; named args ignored at runtime in M2
                    }).unwrap_or(0.0);
                    let node_id = self.graph.add_node(Node::param(&item.name, default));
                    let port = Port { node: node_id, index: 0 };
                    self.bindings.insert(item.name.clone(), port);
                    self.graph.bind(item.name.clone(), node_id);
                }
                Ok(())
            }
            StatementKind::Decl { ty: DeclType::History, items } => {
                for item in items {
                    // Const-fold the init expression to f64
                    let init = if let Some(init_expr) = item.args.first() {
                        Self::const_fold(init_expr).ok_or_else(|| LowerError {
                            msg: format!(
                                "History '{}' init must be a constant (literal number)",
                                item.name
                            ),
                            loc: None,
                        })?
                    } else {
                        0.0
                    };
                    // Create a history op node with args=[init] for the init hook to consume
                    let node_id = self.graph.add_node(Node::op(
                        "history",
                        vec![init],
                        StateDecl::Slots(1),
                    ));
                    // Connect port 0 (write port) — initially unconnected, stays at init value
                    // until an Assign writes to it.
                    let port = Port { node: node_id, index: 0 };
                    self.bindings.insert(item.name.clone(), port);
                    self.graph.bind(item.name.clone(), node_id);
                }
                Ok(())
            }
            StatementKind::Decl { ty: DeclType::Data | DeclType::Buffer, items } => {
                for item in items {
                    let size = match item.args.first() {
                        Some(Expr::Number(n)) => *n as usize,
                        Some(_) => {
                            return Err(LowerError {
                                msg: format!(
                                    "Buffer '{}': external buffer~ references are unsupported in M2 \
                                     (buffer is a data alias); declare with a numeric size",
                                    item.name
                                ),
                                loc: None,
                            });
                        }
                        None => 512,
                    };
                    let data_node = self.graph.add_node(Node::data(&item.name, size));
                    let port = Port { node: data_node, index: 0 };
                    self.bindings.insert(item.name.clone(), port);
                    self.graph.bind(item.name.clone(), data_node);
                }
                Ok(())
            }
            StatementKind::Decl { ty: DeclType::Delay, items } => {
                for item in items {
                    let size = match item.args.first() {
                        Some(Expr::Number(n)) => {
                            let s = *n as usize;
                            if s == 0 {
                                return Err(LowerError {
                                    msg: format!(
                                        "Delay '{}': size must be >= 1 (got 0)",
                                        item.name
                                    ),
                                    loc: None,
                                });
                            }
                            s
                        }
                        Some(_) => {
                            return Err(LowerError {
                                msg: format!(
                                    "Delay '{}': size must be a numeric literal",
                                    item.name
                                ),
                                loc: None,
                            });
                        }
                        None => {
                            return Err(LowerError {
                                msg: format!(
                                    "Delay '{}': size is required (got default)",
                                    item.name
                                ),
                                loc: None,
                            });
                        }
                    };
                    // Synthetic Data node: size+1 (slot 0 = cursor, slots 1..size = ring)
                    let data_name = format!("__delay_{}", self.delay_counter);
                    self.delay_counter += 1;
                    let data_node = self.graph.add_node(Node::data(&data_name, size + 1));
                    self.bindings.insert(item.name.clone(), Port { node: data_node, index: 0 });
                    self.graph.bind(item.name.clone(), data_node);
                    self.delay_bindings.insert(item.name.clone(), DelayInfo {
                        data_name,
                        written: false,
                    });
                }
                Ok(())
            }
            #[allow(unreachable_patterns)]
            StatementKind::Decl { ty, .. } => {
                Err(LowerError {
                    msg: format!("{:?} declarations not yet implemented", ty),
                    loc: None,
                })
            }
            StatementKind::Assign { name, expr } => {
                // Check if this is a write to an existing history/stateful node
                // (name already bound to a node with deferred port 0)
                if let Some(&existing_port) = self.bindings.get(name) {
                    let node = self.graph.node(existing_port.node);
                    if let NodeKind::Op { name: op_name, .. } = &node.kind {
                        if let Some(op_def) = self.registry.get(op_name) {
                            if op_def.deferred_ports.contains(&0) {
                                // Write to history/delay: connect RHS to port 0
                                let rhs_port = self.lower_expr(expr)?;
                                self.graph.connect(rhs_port, Port {
                                    node: existing_port.node,
                                    index: 0,
                                });
                                return Ok(());
                            }
                        }
                    }
                }

                // Otherwise, fall through to the normal assign-or-stateful-self-ref logic
                let is_stateful_self_ref = self.is_stateful_self_reference(name, expr);
                if is_stateful_self_ref {
                    self.lower_stateful_assign(name, expr)
                } else {
                    let port = self.lower_expr(expr)?;
                    if let Some(output_index) = parse_output_name(name) {
                        let out_node = self.graph.add_node(Node::output(output_index));
                        self.graph.connect(port, Port { node: out_node, index: 0 });
                    }
                    self.bindings.insert(name.clone(), port);
                    if !is_synthetic_name(name) && parse_output_name(name).is_none() {
                        self.graph.bind(name.clone(), port.node);
                    }
                    Ok(())
                }
            }
            // Control flow statements — handled by region lowering path.
            StatementKind::If { .. } => {
                Err(LowerError { msg: "if statements not yet implemented (Task 14–15)".to_string(), loc: None })
            }
            StatementKind::While { .. } => {
                Err(LowerError { msg: "while loops not yet implemented (Task 14–15)".to_string(), loc: None })
            }
            StatementKind::DoWhile { .. } => {
                Err(LowerError { msg: "do-while loops not yet implemented (Task 14–15)".to_string(), loc: None })
            }
            StatementKind::For { .. } => {
                Err(LowerError { msg: "for loops not yet implemented (Task 14–15)".to_string(), loc: None })
            }
            StatementKind::Block(_) => {
                Err(LowerError { msg: "block statements not yet implemented (Task 14–15)".to_string(), loc: None })
            }
            StatementKind::Break => {
                Err(LowerError { msg: "break not yet implemented (Task 14–15)".to_string(), loc: None })
            }
            StatementKind::Continue => {
                Err(LowerError { msg: "continue not yet implemented (Task 14–15)".to_string(), loc: None })
            }
            StatementKind::Return(_) => {
                Err(LowerError { msg: "return not yet implemented (Task 14–16)".to_string(), loc: None })
            }
            StatementKind::MultiAssign { .. } => {
                Err(LowerError { msg: "multi-assign not yet implemented (Task 16)".to_string(), loc: None })
            }
            StatementKind::FuncDecl { .. } => {
                Err(LowerError { msg: "function declarations not yet implemented (Task 16)".to_string(), loc: None })
            }
            StatementKind::Require(_) => {
                Err(LowerError { msg: "require unsupported in M2".to_string(), loc: None })
            }
            StatementKind::ExprStmt(expr) => {
                // Expression statement: lower expr but don't bind (side-effect only, e.g. poke)
                self.lower_expr(expr)?;
                Ok(())
            }
        }
    }

    /// Try to const-fold an expression to a simple f64 literal.
    /// Supports number literals and unary negation of a constant (e.g., `-5`, `-(-1)`).
    /// Returns None if not foldable.
    fn const_fold(expr: &Expr) -> Option<f64> {
        match expr {
            Expr::Number(n) => Some(*n),
            Expr::Unary(UnaryOp::Neg, e) => Self::const_fold(e).map(|v| -v),
            _ => None,
        }
    }

    fn is_stateful_self_reference(&self, name: &str, expr: &Expr) -> bool {
        // Check if expr is a call to a stateful op that references 'name'
        if let Expr::Call { name: op_name, args, .. } = expr {
            // Check if the op is stateful
            if let Some(op_def) = self.registry.get(op_name) {
                if op_def.state != StateDecl::None {
                    // Check if any argument references 'name'
                    return args.iter().any(|arg| self.expr_references(arg, name));
                }
            }
        }
        false
    }

    fn expr_references(&self, expr: &Expr, name: &str) -> bool {
        match expr {
            Expr::Ident(s) => s == name,
            Expr::BinOp { left, right, .. } => {
                self.expr_references(left, name) || self.expr_references(right, name)
            }
            Expr::Unary(_, e) => self.expr_references(e, name),
            Expr::Call { args, .. } => args.iter().any(|arg| self.expr_references(arg, name)),
            Expr::Number(_) | Expr::Str(_) => false,
            Expr::MemberCall { args, .. } => args.iter().any(|arg| self.expr_references(arg, name)),
            Expr::Ternary { cond, true_expr, false_expr } => {
                self.expr_references(cond, name)
                    || self.expr_references(true_expr, name)
                    || self.expr_references(false_expr, name)
            }
        }
    }

    fn lower_stateful_assign(&mut self, name: &str, expr: &Expr) -> Result<(), LowerError> {
        // Pre-create the op node and pre-bind it
        if let Expr::Call { name: op_name, args, .. } = expr {
            let op_def = self.registry.get(op_name)
                .ok_or_else(|| LowerError { msg: format!("unknown operator: {}", op_name), loc: None })?;

            // Create the op node
            let op_node = self.graph.add_node(Node::op(op_name, vec![], op_def.state));
            let op_port = Port { node: op_node, index: 0 };

            // Pre-bind the name
            self.bindings.insert(name.to_string(), op_port);

            // Record user-visible binding in graph (exclude outputs and synthetic names)
            if !is_synthetic_name(name) && parse_output_name(name).is_none() {
                self.graph.bind(name.to_string(), op_node);
            }

            // Now lower arguments (which can reference the name)
            if args.len() != op_def.arity as usize {
                return Err(LowerError {
                    msg: format!(
                        "operator '{}' expects {} arguments, got {}",
                        op_name, op_def.arity, args.len()
                    ),
                    loc: None,
                });
            }

            for (i, arg) in args.iter().enumerate() {
                let arg_port = self.lower_expr(arg)?;
                self.graph.connect(arg_port, Port { node: op_node, index: i as u16 });
            }

            // Handle output nodes
            if let Some(output_index) = parse_output_name(name) {
                let out_node = self.graph.add_node(Node::output(output_index));
                self.graph.connect(op_port, Port { node: out_node, index: 0 });
            }

            Ok(())
        } else {
            Err(LowerError {
                msg: "internal: stateful self-reference on non-call expression".into(),
                loc: None,
            })
        }
    }

    fn lower_expr(&mut self, expr: &Expr) -> Result<Port, LowerError> {
        match expr {
            Expr::Number(n) => {
                let node_id = self.graph.add_node(Node::constant(*n));
                Ok(Port { node: node_id, index: 0 })
            }
            Expr::Ident(name) => {
                // Check for input nodes (in1, in2, ...)
                if let Some(input_index) = parse_input_name(name) {
                    // Deduplicate Input nodes: each inN identifier maps to exactly one IR Input node,
                    // regardless of how many times it's referenced.
                    if let Some(port) = self.bindings.get(name) {
                        return Ok(*port);
                    }

                    // Create the Input node and cache it in bindings for future references
                    let node_id = self.graph.add_node(Node::input(input_index));
                    let port = Port { node: node_id, index: 0 };
                    self.bindings.insert(name.clone(), port);
                    return Ok(port);
                }

                // Look up in bindings (locals/params/declarations shadow builtins)
                if let Some(port) = self.bindings.get(name) {
                    return Ok(*port);
                }

                // Check for builtin constants (including vectorsize)
                if let Some(&val) = BUILTIN_CONSTANTS.iter().find(|(k, _)| *k == name).map(|(_, v)| v) {
                    let node_id = self.graph.add_node(Node::constant(val));
                    return Ok(Port { node: node_id, index: 0 });
                }

                // Check for samplerate (arity-0 op)
                if name == "samplerate" {
                    if let Some(op_def) = self.registry.get("samplerate") {
                        let node_id = self.graph.add_node(Node::op("samplerate", vec![], op_def.state));
                        return Ok(Port { node: node_id, index: 0 });
                    }
                    return Err(LowerError {
                        msg: "'samplerate' operator not registered".to_string(),
                        loc: None,
                    });
                }

                // Not found anywhere
                Err(LowerError { msg: format!("undefined identifier: {}", name), loc: None })
            }
            Expr::BinOp { op, left, right } => {
                let op_name = op.op_name();
                let op_def = self.registry.get(op_name)
                    .ok_or_else(|| LowerError { msg: format!("unknown binary operator: {}", op_name), loc: None })?;

                let left_port = self.lower_expr(left)?;
                let right_port = self.lower_expr(right)?;

                let op_node = self.graph.add_node(Node::op(op_name, vec![], op_def.state));
                self.graph.connect(left_port, Port { node: op_node, index: 0 });
                self.graph.connect(right_port, Port { node: op_node, index: 1 });

                Ok(Port { node: op_node, index: 0 })
            }
            Expr::Unary(UnaryOp::Neg, e) => {
                // Unary minus: multiply by -1
                let zero_node = self.graph.add_node(Node::constant(0.0));
                let zero_port = Port { node: zero_node, index: 0 };
                let expr_port = self.lower_expr(e)?;
                let sub_def = self.registry.get("sub")
                    .ok_or_else(|| LowerError { msg: "'sub' operator not available (needed for unary minus)".to_string(), loc: None })?;
                let sub_node = self.graph.add_node(Node::op("sub", vec![], sub_def.state));
                self.graph.connect(zero_port, Port { node: sub_node, index: 0 });
                self.graph.connect(expr_port, Port { node: sub_node, index: 1 });
                Ok(Port { node: sub_node, index: 0 })
            }
            Expr::Unary(UnaryOp::Not, e) => {
                // !expr → not(expr)
                let expr_port = self.lower_expr(e)?;
                let op_def = self.registry.get("not")
                    .ok_or_else(|| LowerError { msg: "'not' operator not available".to_string(), loc: None })?;
                let op_node = self.graph.add_node(Node::op("not", vec![], op_def.state));
                self.graph.connect(expr_port, Port { node: op_node, index: 0 });
                Ok(Port { node: op_node, index: 0 })
            }
            Expr::Call { name, args, named_args } => {
                if !named_args.is_empty() {
                    return Err(LowerError {
                        msg: format!("named arguments not yet implemented for '{}'", name),
                        loc: None,
                    });
                }

                // Special case: peek/poke — first arg is a data name (identifier), not a value.
                if name == "peek" || name == "poke" {
                    if args.is_empty() {
                        return Err(LowerError {
                            msg: format!("function '{}' requires a data name as first argument", name),
                            loc: None,
                        });
                    }
                    let data_name = match &args[0] {
                        Expr::Ident(n) => n.clone(),
                        _ => return Err(LowerError {
                            msg: format!("first argument to '{}' must be a data name (identifier), got expression", name),
                            loc: None,
                        }),
                    };
                    // Verify the data name exists in bindings
                    if !self.bindings.contains_key(&data_name) {
                        return Err(LowerError {
                            msg: format!("unknown data buffer '{}' in '{}'", data_name, name),
                            loc: None,
                        });
                    }

                    let op_def = self.registry.get(name)
                        .ok_or_else(|| LowerError { msg: format!("unknown function: {}", name), loc: None })?;

                    let signal_args = &args[1..]; // skip data name
                    if signal_args.len() != op_def.arity as usize {
                        return Err(LowerError {
                            msg: format!(
                                "function '{}' expects {} signal arguments, got {}",
                                name, op_def.arity, signal_args.len()
                            ),
                            loc: None,
                        });
                    }

                    let op_node = self.graph.add_node(Node::op_with_data(
                        name, vec![], op_def.state, &data_name
                    ));

                    for (i, arg) in signal_args.iter().enumerate() {
                        let arg_port = self.lower_expr(arg)?;
                        self.graph.connect(arg_port, Port { node: op_node, index: i as u16 });
                    }

                    return Ok(Port { node: op_node, index: 0 });
                }

                let op_def = self.registry.get(name)
                    .ok_or_else(|| LowerError { msg: format!("unknown function: {}", name), loc: None })?;

                if args.len() != op_def.arity as usize {
                    return Err(LowerError {
                        msg: format!(
                            "function '{}' expects {} arguments, got {}",
                            name, op_def.arity, args.len()
                        ),
                        loc: None,
                    });
                }

                let op_node = self.graph.add_node(Node::op(name, vec![], op_def.state));

                for (i, arg) in args.iter().enumerate() {
                    let arg_port = self.lower_expr(arg)?;
                    self.graph.connect(arg_port, Port { node: op_node, index: i as u16 });
                }

                Ok(Port { node: op_node, index: 0 })
            }
            Expr::MemberCall { object, method, args, named_args } => {
                // Only delay member calls are supported.
                let obj_name = match object.as_ref() {
                    Expr::Ident(name) => name.clone(),
                    _ => return Err(LowerError {
                        msg: "member calls on non-identifier expressions are not supported".to_string(),
                        loc: None,
                    }),
                };
                
                // Check if this is a delay binding.
                if let Some(delay_info) = self.delay_bindings.get(&obj_name) {
                    match method.as_str() {
                        "write" => {
                            if delay_info.written {
                                return Err(LowerError {
                                    msg: format!("Delay '{}': write() already called (only one writer allowed)", obj_name),
                                    loc: None,
                                });
                            }
                            if args.len() != 1 {
                                return Err(LowerError {
                                    msg: format!("Delay '{}'.write() expects 1 argument, got {}", obj_name, args.len()),
                                    loc: None,
                                });
                            }
                            if !named_args.is_empty() {
                                return Err(LowerError {
                                    msg: format!("Delay '{}'.write() does not accept named arguments", obj_name),
                                    loc: None,
                                });
                            }
                            let data_ref = delay_info.data_name.clone();
                            let arg_port = self.lower_expr(&args[0])?;
                            let op_node = self.graph.add_node(Node::op_with_data(
                                "delay_write", vec![], StateDecl::None, &data_ref
                            ));
                            self.graph.connect(arg_port, Port { node: op_node, index: 0 });
                            // Mark as written
                            self.delay_bindings.get_mut(&obj_name).unwrap().written = true;
                            Ok(Port { node: op_node, index: 0 })
                        }
                        "read" => {
                            if args.len() != 1 {
                                return Err(LowerError {
                                    msg: format!("Delay '{}'.read() expects 1 argument (tap time), got {}", obj_name, args.len()),
                                    loc: None,
                                });
                            }
                            let data_ref = delay_info.data_name.clone();
                            
                            // Determine interpolation mode from named args
                            let interp = named_args.iter().find(|(k, _)| k == "interp");
                            let op_name = match interp {
                                None => "delay_read", // default linear
                                Some((_, Expr::Str(s))) => {
                                    match s.as_str() {
                                        "linear" => "delay_read",
                                        "none" => "delay_read_none",
                                        other => return Err(LowerError {
                                            msg: format!(
                                                "unknown interpolation mode '{}' for delay read; supported: linear, none",
                                                other
                                            ),
                                            loc: None,
                                        }),
                                    }
                                }
                                Some((_, _)) => return Err(LowerError {
                                    msg: "interpolation mode must be a string literal".to_string(),
                                    loc: None,
                                }),
                            };
                            
                            let arg_port = self.lower_expr(&args[0])?;
                            let op_node = self.graph.add_node(Node::op_with_data(
                                op_name, vec![], StateDecl::None, &data_ref
                            ));
                            self.graph.connect(arg_port, Port { node: op_node, index: 0 });
                            Ok(Port { node: op_node, index: 0 })
                        }
                        other => Err(LowerError {
                            msg: format!("unknown method '{}' for Delay '{}'; supported: write, read", other, obj_name),
                            loc: None,
                        }),
                    }
                } else if self.bindings.contains_key(&obj_name) {
                    // Known binding but not a delay
                    Err(LowerError {
                        msg: format!("member calls not supported on '{}' (only Delay supports member calls)", obj_name),
                        loc: None,
                    })
                } else {
                    Err(LowerError {
                        msg: format!("undefined identifier: '{}'", obj_name),
                        loc: None,
                    })
                }
            }
            Expr::Ternary { cond, true_expr, false_expr } => {
                // Lower to switch(cond, true_expr, false_expr)
                let cond_port = self.lower_expr(cond)?;
                let true_port = self.lower_expr(true_expr)?;
                let false_port = self.lower_expr(false_expr)?;

                // switch is arity 3
                let op_def = self.registry.get("switch")
                    .ok_or_else(|| LowerError { msg: "'switch' operator not available (needed for ternary)".to_string(), loc: None })?;

                let switch_node = self.graph.add_node(Node::op("switch", vec![], op_def.state));
                self.graph.connect(cond_port, Port { node: switch_node, index: 0 });
                self.graph.connect(true_port, Port { node: switch_node, index: 1 });
                self.graph.connect(false_port, Port { node: switch_node, index: 2 });

                Ok(Port { node: switch_node, index: 0 })
            }
            Expr::Str(_) => {
                Err(LowerError { msg: "string literals not supported in runtime expressions".to_string(), loc: None })
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Free functions
// ---------------------------------------------------------------------------

/// Parse "outN" to output index (0-based)
fn parse_output_name(name: &str) -> Option<u16> {
    if name.starts_with("out") {
        name[3..].parse::<u16>().ok().map(|n| n - 1)
    } else {
        None
    }
}

/// Parse "inN" to input index (0-based)
fn parse_input_name(name: &str) -> Option<u16> {
    if name.starts_with("in") {
        name[2..].parse::<u16>().ok().map(|n| n - 1)
    } else {
        None
    }
}

/// Check if a name is synthetic (internal, not user-visible)
fn is_synthetic_name(name: &str) -> bool {
    name.starts_with("__")
}

/// Check if a program contains any control-flow statements.
pub fn has_program_control_flow(stmts: &[Statement]) -> bool {
    stmts.iter().any(|s| has_stmt_control_flow(s))
}

fn has_stmt_control_flow(stmt: &Statement) -> bool {
    match &stmt.kind {
        StatementKind::If { .. }
        | StatementKind::While { .. }
        | StatementKind::DoWhile { .. }
        | StatementKind::For { .. }
        | StatementKind::Block(_)
        | StatementKind::Break
        | StatementKind::Continue => true,
        StatementKind::Return(_)
        | StatementKind::MultiAssign { .. }
        | StatementKind::FuncDecl { .. } => true, // These also need region path
        _ => false,
    }
}

pub fn lower(program: &Program) -> Result<Graph, LowerError> {
    let mut program = program.clone();
    // Run inline pass first: user-defined functions, return, multi-assign.
    crate::inline::inline_functions(&mut program)?;
    let registry = Registry::core();
    let mut graph = Graph::new();
    let mut lowerer = Lowerer::new(&registry, &mut graph);
    lowerer.lower(&program)?;
    Ok(graph)
}

/// Lower a GenExpr program into an existing graph without creating Input or Output nodes.
///
/// Takes pre-seeded bindings for inN ports (pointing into the host graph).
/// Returns a list of (out_index_0based, port) pairs for each `outN` assignment.
///
/// This is used for codebox splicing in the `.gendsp` graph builder: a codebox's
/// `in1`/`in2` references map to wired input ports from the host graph, and
/// `out1`/`out2` assignments produce ports wired into the host graph.
///
/// Control-flow programs produce a [`NodeKind::Region`] node in the host graph.
/// Straight-line programs produce individual nodes in the host graph.
///
/// ```
/// use opengen_genexpr::{parse, lower_embedded};
/// use opengen_ir::{Graph, Node, Port, NodeId};
/// use std::collections::HashMap;
///
/// let src = "out1 = in1 + 1;";
/// let prog = parse(src).unwrap();
///
/// let mut graph = Graph::new();
/// let in1_node = graph.add_node(Node::input(0));
/// let in1_port = Port { node: in1_node, index: 0 };
///
/// let mut inputs = HashMap::new();
/// inputs.insert("in1".to_string(), in1_port);
///
/// let outputs = lower_embedded(&prog, &inputs, &mut graph, &opengen_ops::Registry::core()).unwrap();
/// assert_eq!(outputs.len(), 1);
/// assert_eq!(outputs[0].0, 0); // out1 → index 0
/// ```
pub fn lower_embedded(
    program: &Program,
    seeded_inputs: &HashMap<String, Port>,
    graph: &mut Graph,
    registry: &Registry,
) -> Result<Vec<(u16, Port)>, LowerError> {
    let mut program = program.clone();
    crate::inline::inline_functions(&mut program)?;
    let mut lowerer = Lowerer::new(registry, graph);
    lowerer.lower_embedded(&program, seeded_inputs.clone())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_and_lower(src: &str) -> Result<Graph, String> {
        let mut parser = crate::parser::Parser::new(src)
            .map_err(|e| format!("parse error: {}", e))?;
        let ast = parser.parse_program()
            .map_err(|e| format!("parse error: {}", e))?;
        lower(&ast).map_err(|e| e.to_string())
    }

    #[test]
    fn lowers_constant_to_output() {
        let graph = parse_and_lower("out1 = 42;").unwrap();
        assert_eq!(graph.nodes().count(), 2); // constant + output
    }

    #[test]
    fn lowers_binary_expression() {
        let graph = parse_and_lower("out1 = 1.5 + 2.5;").unwrap();
        // Should have: 1.5 constant, 2.5 constant, add op, output
        assert_eq!(graph.nodes().count(), 4);
    }

    #[test]
    fn rejects_undefined_identifier() {
        let err = parse_and_lower("out1 = undefined_var;").unwrap_err();
        assert!(err.contains("undefined identifier"));
    }

    #[test]
    fn resolves_param_reference() {
        let graph = parse_and_lower("Param freq(440); out1 = freq;").unwrap();
        assert_eq!(graph.nodes().count(), 2); // param + output
    }

    #[test]
    fn reuses_input_node_across_references() {
        let graph = parse_and_lower("out1 = in1 + in1;").unwrap();

        // Count Input nodes in the graph
        let input_count = graph.nodes()
            .filter(|(_, node)| matches!(node.kind, opengen_ir::NodeKind::Input(_)))
            .count();

        // Should have exactly ONE Input node, not two
        assert_eq!(input_count, 1, "Expected 1 Input node, got {}", input_count);

        // Total nodes: 1 input + 1 add op + 1 output = 3
        assert_eq!(graph.nodes().count(), 3);
    }

    #[test]
    fn lowers_subtraction_and_division() {
        // Test subtraction
        let graph = parse_and_lower("out1 = 5.0 - 2.0;").unwrap();
        assert!(graph.nodes().count() > 0);

        // Test division
        let graph = parse_and_lower("out1 = 10.0 / 2.0;").unwrap();
        assert!(graph.nodes().count() > 0);
    }

    #[test]
    fn lowers_modulo_operator() {
        let graph = parse_and_lower("out1 = 5.0 % 2.0;").unwrap();
        assert!(graph.nodes().count() > 0);
    }

    #[test]
    fn lowers_comparison_operators() {
        let cases = vec![
            "out1 = 2.0 > 1.0;",
            "out1 = 2.0 >= 1.0;",
            "out1 = 1.0 < 2.0;",
            "out1 = 1.0 <= 2.0;",
            "out1 = 1.0 == 1.0;",
            "out1 = 1.0 != 2.0;",
        ];

        for src in cases {
            let graph = parse_and_lower(src).unwrap();
            assert!(graph.nodes().count() > 0, "Failed to lower: {}", src);
        }
    }

    #[test]
    fn rejects_stateless_self_reference() {
        // Direct self-reference without stateful operator (e.g., history) must error
        let err = parse_and_lower("x = x + 1; out1 = x;").unwrap_err();
        assert!(err.contains("undefined identifier"), "Expected 'undefined identifier', got: {}", err);
    }

    #[test]
    fn allows_stateful_self_reference() {
        // Self-reference through stateful operator (history) is valid
        let graph = parse_and_lower("h = history(h + 1); out1 = h;").unwrap();
        assert!(graph.nodes().count() > 0);
    }

    #[test]
    fn history_decl_with_runtime() {
        // Full pipeline test: History h(5); h = h + 1; out1 = h;
        use opengen_testkit::render;
        let out = render("History h(5); h = h + 1; out1 = h;", 48000.0, 3);
        assert_eq!(out.ch(0), &[5.0, 6.0, 7.0]);
    }

    #[test]
    fn constant_pi_resolves() {
        use opengen_testkit::render;
        let out = render("out1 = pi;", 48000.0, 1);
        assert_eq!(out.ch(0)[0], std::f64::consts::PI);
    }

    #[test]
    fn ternary_via_switch() {
        use opengen_testkit::render;
        let out = render("out1 = 1 ? 100 : 200;", 48000.0, 1);
        assert_eq!(out.ch(0)[0], 100.0);
        let out2 = render("out1 = 0 ? 100 : 200;", 48000.0, 1);
        assert_eq!(out2.ch(0)[0], 200.0);
    }

    #[test]
    fn samplerate_via_op() {
        use opengen_testkit::render;
        let out = render("out1 = samplerate;", 48000.0, 1);
        assert_eq!(out.ch(0)[0], 48000.0);
    }

    #[test]
    fn expression_statement_works() {
        use opengen_testkit::render;
        let out = render("1 + 2; out1 = 42;", 48000.0, 1);
        assert_eq!(out.ch(0)[0], 42.0);
    }
}
