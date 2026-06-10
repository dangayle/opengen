//! Compile-level error tests: graphs that fail validation during compilation.
//!
//! These tests build graphs by hand (or via genexpr) and assert that
//! `compile()` returns a CompileError with the expected message fragment.

use opengen_compile::compile;
use opengen_ir::{Graph, Node, NodeId, Port, StateDecl};

// ═══════════════════════════════════════════════════════════════════
//  Duplicate data names
// ═══════════════════════════════════════════════════════════════════

#[test]
fn duplicate_data_names_error() {
    let mut g = Graph::new();
    // Two Data nodes with the same name "a"
    g.add_node(Node::data("a", 4));
    g.add_node(Node::data("a", 8));
    // An output nodes so we have something to look at
    let out = g.add_node(Node::output(0));
    // Connect one data node to out (doesn't matter which)
    g.connect(Port { node: NodeId(0), index: 0 }, Port { node: out, index: 0 });

    let result = compile(&g, &opengen_ops::Registry::core(), 48_000.0);
    let err = result.unwrap_err();
    assert!(
        err.to_string().contains("duplicate"),
        "expected error containing 'duplicate', got: {}",
        err.to_string()
    );
    assert!(
        err.to_string().contains("a"),
        "expected error mentioning 'a', got: {}",
        err.to_string()
    );
}

// ═══════════════════════════════════════════════════════════════════
//  Unknown data ref on an operator
// ═══════════════════════════════════════════════════════════════════

#[test]
fn unknown_data_ref_errors() {
    let mut g = Graph::new();
    // Add a peek node with data_ref "nope" — no Data node with that name exists
    let peek = g.add_node(Node::op_with_data("peek", vec![], StateDecl::None, "nope"));
    // Connect an input to its index argument
    let idx = g.add_node(Node::constant(0.0));
    g.connect(Port { node: idx, index: 0 }, Port { node: peek, index: 0 });
    // Output
    let out = g.add_node(Node::output(0));
    g.connect(Port { node: peek, index: 0 }, Port { node: out, index: 0 });

    let result = compile(&g, &opengen_ops::Registry::core(), 48_000.0);
    let err = result.unwrap_err();
    assert!(
        err.to_string().contains("nope"),
        "expected error containing 'nope', got: {}",
        err.to_string()
    );
}
