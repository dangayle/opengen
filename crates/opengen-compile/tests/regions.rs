//! Tests for Region node compilation and execution.
//!
//! These tests construct ProcRegion IR by hand (no parser dependency) to verify
//! the T4-M2 implementation of structured procedural regions.

use opengen_ir::proc::*;
use opengen_ir::{Graph, Node, Port};

/// `out0 = (in0 > 0.5) ? in0 * 2 : -1;` via If statement.
///
/// The region has one input and one output. The body selects between
/// `in0 * 2` and `-1` depending on `gt(in0, 0.5)`.
#[test]
fn region_if_else_selects_branch() {
    let region = ProcRegion {
        n_inputs: 1,
        n_outputs: 1,
        n_locals: 0,
        n_state: 0,
        state_init: vec![],
        body: vec![PStmt::If {
            cond: PExpr::Call {
                op: "gt".into(),
                args: vec![PExpr::In(0), PExpr::Const(0.5)],
                state_base: u32::MAX,
                data_ref: None,
            },
            then_body: vec![PStmt::SetOut {
                index: 0,
                expr: PExpr::Call {
                    op: "mul".into(),
                    args: vec![PExpr::In(0), PExpr::Const(2.0)],
                    state_base: u32::MAX,
                    data_ref: None,
                },
            }],
            else_body: vec![PStmt::SetOut {
                index: 0,
                expr: PExpr::Const(-1.0),
            }],
        }],
    };
    let mut g = Graph::new();
    let inp = g.add_node(Node::input(0));
    let r = g.add_node(Node::region(region));
    let out = g.add_node(Node::output(0));
    g.connect(Port { node: inp, index: 0 }, Port { node: r, index: 0 });
    g.connect(Port { node: r, index: 0 }, Port { node: out, index: 0 });
    let mut p =
        opengen_compile::compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap();
    assert_eq!(p.process(&[0.75]), vec![1.5]);
    assert_eq!(p.process(&[0.25]), vec![-1.0]);
}

/// `count = 0; while(1) { count = count + 1; if (count >= in0) break; }`
/// — expect out == in0 for in0 in 1..5.
///
/// Uses Local for count, While with Const(1.0) cond, nested If containing Break.
#[test]
fn region_while_with_break() {
    // count = 0 (Local 0)
    // while(1):
    //   count = count + 1
    //   if (count >= in0) break
    // out0 = count
    for input_val in 1u16..5 {
        let region = ProcRegion {
            n_inputs: 1,
            n_outputs: 1,
            n_locals: 1, // count in slot 0
            n_state: 0,
            state_init: vec![],
            body: vec![
                PStmt::SetLocal {
                    dst: 0,
                    expr: PExpr::Const(0.0),
                },
                PStmt::While {
                    cond: PExpr::Const(1.0), // always true
                    body: vec![
                        PStmt::SetLocal {
                            dst: 0,
                            expr: PExpr::Call {
                                op: "add".into(),
                                args: vec![PExpr::Local(0), PExpr::Const(1.0)],
                                state_base: u32::MAX,
                                data_ref: None,
                            },
                        },
                        PStmt::If {
                            cond: PExpr::Call {
                                op: "gte".into(),
                                args: vec![
                                    PExpr::Local(0),
                                    PExpr::Const(input_val as f64),
                                ],
                                state_base: u32::MAX,
                                data_ref: None,
                            },
                            then_body: vec![PStmt::Break],
                            else_body: vec![],
                        },
                    ],
                },
                PStmt::SetOut {
                    index: 0,
                    expr: PExpr::Local(0),
                },
            ],
        };
        let mut g = Graph::new();
        let inp = g.add_node(Node::input(0));
        let r = g.add_node(Node::region(region));
        let out = g.add_node(Node::output(0));
        g.connect(Port { node: inp, index: 0 }, Port { node: r, index: 0 });
        g.connect(Port { node: r, index: 0 }, Port { node: out, index: 0 });
        let mut p =
            opengen_compile::compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap();
        assert_eq!(p.process(&[input_val as f64]), vec![input_val as f64]);
    }
}

/// History counter inside region: out = state[0] (pre-increment read), then
/// SetState state[0] = state[0] + 1; expect 0, 1, 2 across 3 samples.
#[test]
fn region_state_persists_across_samples() {
    let region = ProcRegion {
        n_inputs: 0,
        n_outputs: 1,
        n_locals: 0,
        n_state: 1,
        state_init: vec![0.0],
        body: vec![
            // out = state[0] (read before increment)
            PStmt::SetOut {
                index: 0,
                expr: PExpr::State(0),
            },
            // state[0] = state[0] + 1
            PStmt::SetState {
                index: 0,
                expr: PExpr::Call {
                    op: "add".into(),
                    args: vec![PExpr::State(0), PExpr::Const(1.0)],
                    state_base: u32::MAX,
                    data_ref: None,
                },
            },
        ],
    };
    let mut g = Graph::new();
    let r = g.add_node(Node::region(region));
    let out = g.add_node(Node::output(0));
    g.connect(Port { node: r, index: 0 }, Port { node: out, index: 0 });
    let mut p =
        opengen_compile::compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap();
    assert_eq!(p.process(&[]), vec![0.0]);
    assert_eq!(p.process(&[]), vec![1.0]);
    assert_eq!(p.process(&[]), vec![2.0]);
}

/// Unknown op inside region should produce a CompileError mentioning "bogus".
#[test]
fn region_unknown_op_errors() {
    let region = ProcRegion {
        n_inputs: 0,
        n_outputs: 1,
        n_locals: 0,
        n_state: 0,
        state_init: vec![],
        body: vec![PStmt::SetOut {
            index: 0,
            expr: PExpr::Call {
                op: "bogus".into(),
                args: vec![],
                state_base: u32::MAX,
                data_ref: None,
            },
        }],
    };
    let mut g = Graph::new();
    let r = g.add_node(Node::region(region));
    let out = g.add_node(Node::output(0));
    g.connect(Port { node: r, index: 0 }, Port { node: out, index: 0 });
    let err =
        opengen_compile::compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap_err();
    assert!(
        err.to_string().contains("bogus"),
        "error should mention 'bogus': {}",
        err
    );
}
