//! Control flow lowering tests (Task 15): if/else, for, history+if, params in regions.
//!
//! TDD: all tests fail first (control flow not yet lowered), then pass after implementation.

use opengen_testkit::{render, render_with_inputs};

// ═══════════════════════════════════════════════════════════════════
//  if / else
// ═══════════════════════════════════════════════════════════════════

#[test]
fn if_else_lowers_and_runs() {
    let out = render_with_inputs(
        "x = 0;\nif (in1 > 0.5) { x = in1 * 2; } else { x = 0 - 1; }\nout1 = x;",
        48_000.0,
        &[&[0.75, 0.25]],
    );
    assert_eq!(out.ch(0), &[1.5, -1.0]);
}

#[test]
fn if_without_else_keeps_zero_init() {
    // Locals zero-initialize each sample; x stays 0 when condition false
    let out = render_with_inputs(
        "if (in1 > 0.5) { x = in1 * 2; } else { x = 0.0; }\nout1 = x;",
        48_000.0,
        &[&[0.3]],
    );
    assert_eq!(out.ch(0), &[0.0]);
}

#[test]
fn if_with_only_assign() {
    // Single-statement body without braces
    let out = render_with_inputs(
        "x = 0;\nif (in1 > 0.5) x = in1 * 2;\nout1 = x;",
        48_000.0,
        &[&[0.75, 0.25]],
    );
    assert_eq!(out.ch(0), &[1.5, 0.0]);
}

// ═══════════════════════════════════════════════════════════════════
//  for loop
// ═══════════════════════════════════════════════════════════════════

#[test]
fn for_loop_accumulates() {
    let out = render(
        "acc = 0; for (i = 0; i < 4; i += 1) { acc += i; } out1 = acc;",
        48_000.0,
        1,
    );
    assert_eq!(out.ch(0)[0], 6.0);
}

#[test]
fn for_loop_without_braces() {
    let out = render(
        "acc = 0; for (i = 0; i < 3; i += 1) acc = acc + i; out1 = acc;",
        48_000.0,
        1,
    );
    assert_eq!(out.ch(0)[0], 3.0);
}

// ═══════════════════════════════════════════════════════════════════
//  History inside control flow
// ═══════════════════════════════════════════════════════════════════

#[test]
fn history_inside_control_flow_program() {
    // History decl + if: whole body lowers to one region with persistent state.
    // Immediate-write semantics (D6/genlib): h read for out1 happens AFTER the
    // write in the same sample. Work the example by hand: inputs [1.0, 0.0, 2.0]
    //
    // Sample 0 (in1=1.0): read state[0]=0, cond true, compute 0+1=1, set state[0]=1.
    //                      Then out1 reads state[0]=1 → output 1.0.
    // Sample 1 (in1=0.0): read state[0]=1, cond false. out1 reads state[0]=1 → output 1.0.
    // Sample 2 (in1=2.0): read state[0]=1, cond true, compute 1+2=3, set state[0]=3.
    //                      Then out1 reads state[0]=3 → output 3.0.
    //
    // Expected: [1.0, 1.0, 3.0]
    let src = "History h(0);\nif (in1 > 0) { h = h + in1; }\nout1 = h;";
    let out = render_with_inputs(src, 48_000.0, &[&[1.0, 0.0, 2.0]]);
    assert_eq!(out.ch(0), &[1.0, 1.0, 3.0]);
}

// ═══════════════════════════════════════════════════════════════════
//  Params feeding regions
// ═══════════════════════════════════════════════════════════════════

#[test]
fn params_feed_regions() {
    let src = "Param g(2);\ny = 0;\nif (1) { y = in1 * g; }\nout1 = y;";
    let out = render_with_inputs(src, 48_000.0, &[&[3.0]]);
    assert_eq!(out.ch(0), &[6.0]);
}

// ═══════════════════════════════════════════════════════════════════
//  M1 backward-compatibility regression tests
// ═══════════════════════════════════════════════════════════════════

#[test]
fn straight_line_history_still_works() {
    // Programs WITHOUT control flow must keep the M1 graph path (byte-for-byte).
    // Regression: h = history(h + 1); out1 = h; → [0, 1, 2]
    let out = render("h = history(h + 1); out1 = h;", 48_000.0, 3);
    assert_eq!(out.ch(0), &[0.0, 1.0, 2.0]);
}

#[test]
fn straight_line_probes_still_work() {
    // compile_with_probes on a named binding must still work for straight-line programs.
    let (out, probes) = opengen_testkit::render_with_probes(
        "h = history(h + 1); out1 = h;",
        48_000.0,
        3,
        &["h"],
    );
    assert_eq!(out.ch(0), &[0.0, 1.0, 2.0]);
    assert_eq!(probes["h"], vec![0.0, 1.0, 2.0]);
}

#[test]
fn straight_line_constant_add_still_works() {
    let out = render("out1 = 1.5 + 2.25;", 48_000.0, 1);
    assert_eq!(out.ch(0)[0], 3.75);
}

#[test]
fn straight_line_no_inputs_works() {
    let out = render("out1 = 42;", 48_000.0, 1);
    assert_eq!(out.ch(0)[0], 42.0);
}

#[test]
fn straight_line_history_decl_works() {
    let out = render("History h(5); h = h + 1; out1 = h;", 48_000.0, 3);
    assert_eq!(out.ch(0), &[5.0, 6.0, 7.0]);
}
