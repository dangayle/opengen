//! Lower AST to IR Graph

use crate::ast::*;
use opengen_ir::{Graph, Node, Port, StateDecl};
use opengen_ops::Registry;
use std::collections::HashMap;

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

pub struct Lowerer<'a> {
    graph: Graph,
    registry: &'a Registry,
    /// Maps identifier names to their output ports
    bindings: HashMap<String, Port>,
}

impl<'a> Lowerer<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self {
            graph: Graph::new(),
            registry,
            bindings: HashMap::new(),
        }
    }

    pub fn lower(mut self, program: &Program) -> Result<Graph, LowerError> {
        for stmt in &program.statements {
            self.lower_statement(stmt)?;
        }
        Ok(self.graph)
    }

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
                // Record user-visible param binding in graph
                self.graph.bind(name.clone(), node_id);
                Ok(())
            }
            StatementKind::Assign { name, expr } => {
                // Check if this is a stateful self-reference pattern
                let is_stateful_self_ref = self.is_stateful_self_reference(name, expr);
                
                if is_stateful_self_ref {
                    // Pre-bind the name to enable self-reference
                    // We'll create the node and bind it before lowering arguments
                    self.lower_stateful_assign(name, expr)
                } else {
                    // Normal assignment: lower expr, then bind
                    let port = self.lower_expr(expr)?;
                    
                    // Handle output nodes specially
                    if let Some(output_index) = parse_output_name(name) {
                        let out_node = self.graph.add_node(Node::output(output_index));
                        self.graph.connect(port, Port { node: out_node, index: 0 });
                    }
                    
                    // Bind the name (allows re-use in later expressions)
                    self.bindings.insert(name.clone(), port);
                    
                    // Record user-visible binding in graph (exclude outputs and synthetic names)
                    if !is_synthetic_name(name) && parse_output_name(name).is_none() {
                        self.graph.bind(name.clone(), port.node);
                    }
                    Ok(())
                }
            }
        }
    }

    fn is_stateful_self_reference(&self, name: &str, expr: &Expr) -> bool {
        // Check if expr is a call to a stateful op that references 'name'
        if let Expr::Call { name: op_name, args } = expr {
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
            Expr::UnaryMinus(e) => self.expr_references(e, name),
            Expr::Call { args, .. } => args.iter().any(|arg| self.expr_references(arg, name)),
            Expr::Number(_) => false,
        }
    }

    fn lower_stateful_assign(&mut self, name: &str, expr: &Expr) -> Result<(), LowerError> {
        // Pre-create the op node and pre-bind it
        if let Expr::Call { name: op_name, args } = expr {
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
            return Err(LowerError {
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
                    // regardless of how many times it's referenced. This keeps the IR minimal and
                    // ensures NodeId assignment is deterministic (doesn't vary with reference count).
                    if let Some(port) = self.bindings.get(name) {
                        return Ok(*port);
                    }
                    
                    // Create the Input node and cache it in bindings for future references
                    let node_id = self.graph.add_node(Node::input(input_index));
                    let port = Port { node: node_id, index: 0 };
                    self.bindings.insert(name.clone(), port);
                    return Ok(port);
                }
                
                // Look up in bindings
                self.bindings.get(name)
                    .copied()
                    .ok_or_else(|| LowerError { msg: format!("undefined identifier: {}", name), loc: None })
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
            Expr::UnaryMinus(e) => {
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
            Expr::Call { name, args } => {
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
        }
    }
}

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

pub fn lower(program: &Program) -> Result<Graph, LowerError> {
    let registry = Registry::core();
    Lowerer::new(&registry).lower(program)
}

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
}
