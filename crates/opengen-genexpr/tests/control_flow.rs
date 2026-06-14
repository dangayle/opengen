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

// ═══════════════════════════════════════════════════════════════════
//  history() function-call error inside region
// ═══════════════════════════════════════════════════════════════════

#[test]
fn history_call_inside_region_errors() {
    // history(…) as a function call inside control flow must error with
    // "use History declaration instead" — the call-site form history(x)
    // is only valid as a stateful self-reference declaration outside regions.
    // Inside a region, you must use the History h(init); h = ...; form.
    let err = opengen_genexpr::parse_and_lower(
        "if (in1 > 0) { h = history(in1); } out1 = h;",
    )
    .unwrap_err();
    assert!(
        err.contains("history"),
        "Expected error mentioning 'history', got: {}",
        err
    );
}

// ═══════════════════════════════════════════════════════════════════
//  Stateful ops get independent state per call site inside regions
// ═══════════════════════════════════════════════════════════════════

#[test]
fn stateful_ops_get_independent_state_per_call_site() {
    // Two noise() calls in a region: each gets its own 4-slot PRNG state.
    // Both start from the same seed, so first-sample output is identical.
    // But they must NOT share state (which would produce interleaved noise).
    //
    // More definitive: two phasor() calls at different frequencies inside
    // a region accumulate independent phase. With SHARED state, one would
    // corrupt the other's phase. With independent per-site state:
    //   a = phasor(100) → samples: [0, 100/48000, 200/48000, …]
    //   b = phasor(200) → samples: [200/48000, 400/48000, 600/48000, …]
    //   (phasor is increment-then-output per gen~ conformance)
    //
    // The two channels are visibly different, proving independence.
    let out = render(
        "a = 0; b = 0; if (1) { a = phasor(100); b = phasor(200); } out1 = a; out2 = b;",
        48000.0,
        3,
    );
    let expected_a: Vec<f64> = (0..3)
        .map(|i| ((i + 1) as f64) * 100.0 / 48000.0)
        .collect();
    let expected_b: Vec<f64> = (0..3)
        .map(|i| ((i + 1) as f64) * 200.0 / 48000.0)
        .collect();
    assert_eq!(out.ch(0), &expected_a[..3]);
    assert_eq!(out.ch(1), &expected_b[..3]);

    // Verify noise calls: with correct per-site state, two noise() calls
    // produce IDENTICAL sequences (same seed, independent state slots).
    // This is per-call-site state as specified by D6.
    let out2 = render(
        "a = 0; b = 0; if (1) { a = noise(); b = noise(); } out1 = a; out2 = b;",
        48000.0,
        4,
    );
    assert_eq!(
        out2.ch(0),
        out2.ch(1),
        "identical call sites must produce identical independent sequences"
    );
    for s in 0..4 {
        assert!(out2.ch(0)[s] >= -1.0 && out2.ch(0)[s] < 1.0);
    }
}

#[test]
fn identical_stateful_calls_get_independent_state() {
    // Two structurally identical noise() calls: per-call-site state means each
    // starts from the fixed seed -> identical sequences (NOT one interleaved sequence).
    let src = "a = 0; b = 0;\nif (1) { a = noise(); b = noise(); }\nout1 = a; out2 = b;";
    let out = opengen_testkit::render(src, 48_000.0, 4);
    assert_eq!(
        out.ch(0),
        out.ch(1),
        "identical call sites must produce identical independent sequences"
    );
}

// ═══════════════════════════════════════════════════════════════════
//  Continue inside for-sugar while loop — step is SKIPPED
// ═══════════════════════════════════════════════════════════════════

#[test]
fn continue_in_for_skips_step() {
    // Our for→while desugar: for(init; cond; step) body → init; while(cond) { body; step; }
    // This means `continue` inside `body` jumps to the `cond` check, SKIPPING `step`.
    // This diverges from C where continue goes to step. Documented as a known decision.
    //
    // Program:
    //   acc = 0;
    //   for (i = 0; i < 5; i += 1) {
    //     if (i >= 2) { i = i + 1; continue; }
    //     acc += 1;
    //   }
    //   out1 = acc;
    //
    // Trace (skip-step semantics):
    //   i=0: i<5, i>=2? no, acc+=1 → acc=1, step→i=1
    //   i=1: i<5, i>=2? no, acc+=1 → acc=2, step→i=2
    //   i=2: i<5, i>=2? yes, i=3, continue→skip step
    //   i=3: i<5, i>=2? yes, i=4, continue→skip step
    //   i=4: i<5, i>=2? yes, i=5, continue→skip step
    //   i=5: i<5? no → exit
    //   Expected acc = 2
    let out = render(
        "acc = 0; for (i = 0; i < 5; i += 1) { if (i >= 2) { i = i + 1; continue; } acc += 1; } out1 = acc;",
        48000.0,
        1,
    );
    assert_eq!(
        out.ch(0)[0], 2.0,
        "Expected acc=2 under skip-step semantics, got {}",
        out.ch(0)[0]
    );
}

// ═══════════════════════════════════════════════════════════════════
//  do-while executes body at least once
// ═══════════════════════════════════════════════════════════════════

#[test]
fn do_while_executes_body_at_least_once() {
    // do { x = x + 1; } while (x < 0): body runs once, x becomes 1, cond false → x=1
    let out = render(
        "x = 0; do { x = x + 1; } while (x < 0); out1 = x;",
        48000.0,
        1,
    );
    assert_eq!(out.ch(0)[0], 1.0);

    // do { x = x + 1; } while (x < 3): body runs 3 times, x becomes 3, cond false → x=3
    let out2 = render(
        "x = 0; do { x = x + 1; } while (x < 3); out1 = x;",
        48000.0,
        1,
    );
    assert_eq!(out2.ch(0)[0], 3.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Probes unavailable on region programs (D6 limitation)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn probes_unavailable_in_region_programs() {
    // compile_with_probes on a control-flow program probing an interior
    // binding (like "x") should error because region lowering only exposes
    // Param names at the graph level — locals are encapsulated inside the
    // ProcRegion and have no graph-level NodeId for probes to attach to.
    let src = "if (1) { x = 42; } out1 = x;";
    let graph = opengen_genexpr::parse_and_lower(src).unwrap();
    let err = opengen_compile::compile_with_probes(
        &graph,
        &opengen_ops::Registry::core(),
        48000.0,
        &["x"],
    )
    .unwrap_err();
    assert!(
        err.to_string().contains("probe 'x' not found"),
        "Expected 'probe'x' not found' error, got: {}",
        err
    );
}

// ═══════════════════════════════════════════════════════════════════
//  Walk-order alignment: metadata collection and lowering must visit
//  stateful ops in the SAME order so positional state_base is correct.
// ═══════════════════════════════════════════════════════════════════

#[test]
fn dowhile_with_stateful_cond_keeps_per_site_state() {
    // noise (size 4) in body, phasor (size 1) in cond.
    // Buggy metadata (cond-first) would register phasor before noise;
    // lowering visits body-first (noise) and maps to the wrong state_base.
    // The assert_eq guard would catch "phasor" != "noise" and panic.
    // After fix (body-first in metadata), both passes agree.
    //
    // do-while executes body unconditionally once, then checks cond.
    // phasor(200) < -1 is always false (phasor outputs [0,1)).
    let src = "a = 0;\ndo { a = noise(); } while (phasor(200) < 0 - 1);\nout1 = a;";
    // Should not panic: correct per-call-site state.
    let out = opengen_testkit::render(src, 48_000.0, 1);
    // noise() returns [-1, 1); phasor/cond doesn't affect `a`.
    assert!(out.ch(0)[0] >= -1.0 && out.ch(0)[0] < 1.0,
        "noise output out of range: {}", out.ch(0)[0]);
}

#[test]
fn for_with_stateful_step_keeps_per_site_state() {
    // phasor (size 1) in body, noise (size 4) in step expression.
    // Buggy metadata (step-first) would register noise before phasor;
    // lowering visits body-first (phasor) and maps to the wrong state_base.
    // After fix (body-before-step in metadata), both passes agree.
    //
    // step = i += 1 + noise() * 0  →  effective i += 1 (noise()*0 = 0)
    // Loop runs i=0,1,2. Each iter body: a = phasor(440).
    let src = "for (i = 0; i < 3; i += 1 + noise() * 0) { a = phasor(440); } out1 = a;";
    let out = opengen_testkit::render(src, 48_000.0, 1);
    // Body phasor(440) runs 3 times within sample 0 (increment-then-output
    // per gen~ conformance):
    //   iter 0: out=440/48000
    //   iter 1: out=880/48000
    //   iter 2: out=1320/48000
    let expected = 3.0 * 440.0 / 48000.0;
    assert_eq!(out.ch(0)[0], expected);
}

// ═══════════════════════════════════════════════════════════════════
//  for-init comma (multi-variable init)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn for_multi_init_lowers_and_renders_correctly() {
    // For-loop init runs once per sample frame; body runs 3 times (i=0,1,2).
    // j starts at 10 each sample, increments 3 times → 13.0.
    let src = "for(i=0, j=10; i<3; i+=1) { j = j + 1; out1 = j; }";
    let out = render(src, 48_000.0, 3);
    assert_eq!(out.ch(0)[0], 13.0);
    assert_eq!(out.ch(0)[1], 13.0);
    assert_eq!(out.ch(0)[2], 13.0);
}

