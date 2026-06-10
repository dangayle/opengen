//! Typed dataflow IR, operator registry, type/shape checking

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Port { pub node: NodeId, pub index: u16 }

/// Explicit state declaration — state is a visible property of the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateDecl { None, Slots(u32) }

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Constant(f64),
    Param { name: String, default: f64 },
    Input(u16),
    Output(u16),
    Op { name: String, args: Vec<f64>, state: StateDecl },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node { pub kind: NodeKind }

impl Node {
    pub fn constant(v: f64) -> Self { Node { kind: NodeKind::Constant(v) } }
    pub fn output(i: u16) -> Self { Node { kind: NodeKind::Output(i) } }
    pub fn input(i: u16) -> Self { Node { kind: NodeKind::Input(i) } }
    pub fn param(name: &str, default: f64) -> Self {
        Node { kind: NodeKind::Param { name: name.into(), default } }
    }
    pub fn op(name: &str, args: Vec<f64>, state: StateDecl) -> Self {
        Node { kind: NodeKind::Op { name: name.into(), args, state } }
    }
    pub fn op_name(&self) -> Option<&str> {
        match &self.kind { NodeKind::Op { name, .. } => Some(name), _ => None }
    }
    pub fn state(&self) -> StateDecl {
        match &self.kind { NodeKind::Op { state, .. } => *state, _ => StateDecl::None }
    }
}

#[derive(Debug, Default)]
pub struct Graph {
    nodes: Vec<Node>,
    /// dest port -> source port
    edges: std::collections::HashMap<Port, Port>,
    /// User-visible bindings: identifier name -> node
    bindings: std::collections::HashMap<String, NodeId>,
}

impl Graph {
    pub fn new() -> Self { Self::default() }
    pub fn add_node(&mut self, n: Node) -> NodeId {
        self.nodes.push(n);
        NodeId(self.nodes.len() as u32 - 1)
    }
    pub fn connect(&mut self, from: Port, to: Port) { self.edges.insert(to, from); }
    pub fn node(&self, id: NodeId) -> &Node { &self.nodes[id.0 as usize] }
    pub fn nodes(&self) -> impl Iterator<Item = (NodeId, &Node)> {
        self.nodes.iter().enumerate().map(|(i, n)| (NodeId(i as u32), n))
    }
    pub fn input_of(&self, p: Port) -> Option<Port> { self.edges.get(&p).copied() }
    
    /// Bind a name to a node (for user-visible variables)
    pub fn bind(&mut self, name: String, id: NodeId) { self.bindings.insert(name, id); }
    
    /// Look up a binding by name
    pub fn binding(&self, name: &str) -> Option<NodeId> { self.bindings.get(name).copied() }
    
    /// Iterator over all bindings
    pub fn bindings(&self) -> impl Iterator<Item = (&String, &NodeId)> { self.bindings.iter() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_constant_to_output_graph() {
        let mut g = Graph::new();
        let c = g.add_node(Node::constant(0.75));
        let out = g.add_node(Node::output(0));
        g.connect(Port { node: c, index: 0 }, Port { node: out, index: 0 });
        assert_eq!(g.nodes().count(), 2);
        assert_eq!(g.input_of(Port { node: out, index: 0 }), Some(Port { node: c, index: 0 }));
    }

    #[test]
    fn op_node_carries_kind_and_state_decl() {
        let n = Node::op("history", vec![], StateDecl::Slots(1));
        assert_eq!(n.op_name(), Some("history"));
        assert_eq!(n.state(), StateDecl::Slots(1));
    }
}
