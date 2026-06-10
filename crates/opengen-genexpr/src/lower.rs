//! Lower AST to IR Graph

use crate::ast::*;
use opengen_ir::{Graph, Node, Port, StateDecl};
use opengen_ops::Registry;
use std::collections::HashMap;

#[derive(Debug)]
pub struct LowerError(pub String);

impl std::fmt::Display for LowerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
        match stmt {
            Statement::ParamDecl { name, default } => {
                let node_id = self.graph.add_node(Node::param(name, *default));
                let port = Port { node: node_id, index: 0 };
                self.bindings.insert(name.clone(), port);
                Ok(())
            }
            Statement::Assign { name, expr } => {
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
                .ok_or_else(|| LowerError(format!("unknown operator: {}", op_name)))?;
            
            // Create the op node
            let op_node = self.graph.add_node(Node::op(op_name, vec![], op_def.state));
            let op_port = Port { node: op_node, index: 0 };
            
            // Pre-bind the name
            self.bindings.insert(name.to_string(), op_port);
            
            // Now lower arguments (which can reference the name)
            if args.len() != op_def.arity as usize {
                return Err(LowerError(format!(
                    "operator '{}' expects {} arguments, got {}",
                    op_name, op_def.arity, args.len()
                )));
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
            unreachable!("is_stateful_self_reference should only return true for Call exprs")
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
                    let node_id = self.graph.add_node(Node::input(input_index));
                    return Ok(Port { node: node_id, index: 0 });
                }
                
                // Look up in bindings
                self.bindings.get(name)
                    .copied()
                    .ok_or_else(|| LowerError(format!("undefined identifier: {}", name)))
            }
            Expr::BinOp { op, left, right } => {
                let op_name = op.op_name();
                let op_def = self.registry.get(op_name)
                    .ok_or_else(|| LowerError(format!("unknown binary operator: {}", op_name)))?;
                
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
                    .ok_or_else(|| LowerError("'sub' operator not available (needed for unary minus)".to_string()))?;
                
                let sub_node = self.graph.add_node(Node::op("sub", vec![], sub_def.state));
                self.graph.connect(zero_port, Port { node: sub_node, index: 0 });
                self.graph.connect(expr_port, Port { node: sub_node, index: 1 });
                
                Ok(Port { node: sub_node, index: 0 })
            }
            Expr::Call { name, args } => {
                let op_def = self.registry.get(name)
                    .ok_or_else(|| LowerError(format!("unknown function: {}", name)))?;
                
                if args.len() != op_def.arity as usize {
                    return Err(LowerError(format!(
                        "function '{}' expects {} arguments, got {}",
                        name, op_def.arity, args.len()
                    )));
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
}
