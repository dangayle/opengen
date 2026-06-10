//! Integration tests for Delay declarations and member calls.
//!
//! These tests exercise the full pipeline: parse → lower → compile → process.
//! Lower-level kernel tests live in `opengen-ops/src/memory.rs`.

use opengen_testkit::render;

// ═══════════════════════════════════════════════════════════════════
//  1-sample echo (write + read with tap=1)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn one_sample_echo() {
    // Delay d(4); d.write(in1); out1 = d.read(1);
    // in1 = [1,0,0] -> out1 == [0,1,0] (1-sample echo; reads precede update-phase write)
    let out = opengen_testkit::render_with_inputs(
        "Delay d(4); d.write(in1); out1 = d.read(1);",
        48000.0,
        &[&[1.0, 0.0, 0.0]],
    );
    assert_eq!(out.ch(0), &[0.0, 1.0, 0.0]);
}

// ═══════════════════════════════════════════════════════════════════
//  Linear interpolation at tap=1.5
// ═══════════════════════════════════════════════════════════════════

#[test]
fn linear_interp_half_sample() {
    // Delay d(8); d.write(in1); out1 = d.read(1.5);
    // in1 = [1,0,0] -> out1 == [0, 0.5, 0.5]
    let out = opengen_testkit::render_with_inputs(
        "Delay d(8); d.write(in1); out1 = d.read(1.5);",
        48000.0,
        &[&[1.0, 0.0, 0.0]],
    );
    assert_eq!(out.ch(0), &[0.0, 0.5, 0.5]);
}

// ═══════════════════════════════════════════════════════════════════
//  Default interp is linear
// ═══════════════════════════════════════════════════════════════════

#[test]
fn default_interp_is_linear() {
    let out = opengen_testkit::render_with_inputs(
        "Delay d(8); d.write(in1); out1 = d.read(1.5);",
        48000.0,
        &[&[2.0, 0.0, 0.0]],
    );
    assert_eq!(out.ch(0), &[0.0, 1.0, 1.0]);
}

// ═══════════════════════════════════════════════════════════════════
//  None interpolation (nearest sample via half-sample rounding)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn none_interp_nearest_sample() {
    // genlib read_step uses ceil(d-0.5) rounding (ties DOWN).
    // tap=1.5: ceil(1.5-0.5)=ceil(1.0)=1 → reads 1 sample ago (k=1).
    //
    // Sample 0: cursor=0, in=1, k=1, ring[(0-1+8)%8]=ring[7]=0. out=0.
    //   Update: write 1 at ring[0], cursor=1.
    // Sample 1: cursor=1, in=0, k=1, ring[(1-1+8)%8]=ring[0]=1. out=1.
    //   Update: write 0 at ring[1], cursor=2.
    // Sample 2: cursor=2, in=0, k=1, ring[(2-1+8)%8]=ring[1]=0. out=0.
    //
    // Result: [0, 1, 0] (1-sample echo, same as tap=1 integer).
    let out = opengen_testkit::render_with_inputs(
        r#"Delay d(8); d.write(in1); out1 = d.read(1.5, interp="none");"#,
        48000.0,
        &[&[1.0, 0.0, 0.0]],
    );
    assert_eq!(out.ch(0), &[0.0, 1.0, 0.0]);

    // Also verify genlib tie-down for ceil(d-0.5) at tap=1.51:
    // ceil(1.51-0.5)=ceil(1.01)=2 → reads 2 samples ago.
    let out2 = opengen_testkit::render_with_inputs(
        r#"Delay d(8); d.write(in1); out1 = d.read(1.51, interp="none");"#,
        48000.0,
        &[&[1.0, 0.0, 0.0]],
    );
    assert_eq!(out2.ch(0), &[0.0, 0.0, 1.0]);
}

// ═══════════════════════════════════════════════════════════════════
//  Multiple taps (multi-tap delay — REQUIRED)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn multi_tap_delay() {
    // Two separate reads from the same delay buffer
    let out = opengen_testkit::render_with_inputs(
        "Delay d(8); d.write(in1); out1 = d.read(1); out2 = d.read(2);",
        48000.0,
        &[&[1.0, 0.0, 0.0]],
    );
    // Sample 0: in=1. tap=1: 0, tap=2: 0. Out = [0, 0]
    // Sample 1: in=0. tap=1: 1, tap=2: 0. Out = [1, 0]
    // Sample 2: in=0. tap=1: 0, tap=2: 1. Out = [0, 1]
    assert_eq!(out.ch(0), &[0.0, 1.0, 0.0]); // tap=1
    assert_eq!(out.ch(1), &[0.0, 0.0, 1.0]); // tap=2
}

// ═══════════════════════════════════════════════════════════════════
//  Feedback loop (write port deferred breaks the cycle)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn feedback_loop_compiles_and_runs() {
    // y = d.read(10) * 0.5 + in1; d.write(y); out1 = y;
    // With in1 = [1.0, 0.0, 0.0]:
    // Sample 0: y = 0*0.5 + 1.0 = 1.0. Write(1.0). out = 1.0
    // Sample 1: y = 0*0.5 + 0.0 = 0.0. Write(0.0). out = 0.0
    // Sample 2: y = 0*0.5 + 0.0 = 0.0. Write(0.0). out = 0.0
    let out = opengen_testkit::render_with_inputs(
        "Delay d(64); y = d.read(10) * 0.5 + in1; d.write(y); out1 = y;",
        48000.0,
        &[&[1.0, 0.0, 0.0]],
    );
    assert_eq!(out.ch(0)[0], 1.0);
    assert_eq!(out.ch(0)[1], 0.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Unused Delay decl compiles silently
// ═══════════════════════════════════════════════════════════════════

#[test]
fn unused_delay_decl_compiles() {
    // No write, no read — just a Delay decl. Must compile silently.
    let out = render("Delay d(32); out1 = 42;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 42.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Tap > size clamps to size (reads oldest sample)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn tap_clamped_to_size() {
    // Delay d(4); tap=10 clamps to 4 (maxdelay).
    // Write [1,0,0,0,0] (5 samples), tap=10 clamped to 4.
    //
    // Ring math (N=4, cursor at state[0], ring in state[1..4]):
    // delay_read linear-interp: tap=4 clamped to 4 (maxdelay).
    // i = floor(4) = 4, frac = 0. ring_read(cursor, 4, 4) = oldest sample.
    //
    //   Sample 0 (cursor=0, in=1):
    //     ring_read(0, 4, 4) = state[(0+4-4)%4+1] = state[1] = 0
    //     out = 0.  Write at ring[0]=state[1]=1, cursor→1
    //   Sample 1 (cursor=1, in=0):
    //     ring_read(1, 4) = state[(1+4-4)%4+1] = state[2] = 0
    //     out = 0.  Write at ring[1]=state[2]=0, cursor→2
    //   Sample 2 (cursor=2, in=0):
    //     ring_read(2, 4) = state[(2+4-4)%4+1] = state[3] = 0
    //     out = 0.  Write at ring[2]=state[3]=0, cursor→3
    //   Sample 3 (cursor=3, in=0):
    //     ring_read(3, 4) = state[(3+4-4)%4+1] = state[4] = 0
    //     out = 0.  Write at ring[3]=state[4]=0, cursor→0
    //   Sample 4 (cursor=0, in=0):
    //     ring_read(0, 4) = state[(0+4-4)%4+1] = state[1] = 1 ← the 1!
    //     out = 1.  Write at ring[0]=state[1]=0, cursor→1
    //
    // Result: [0, 0, 0, 0, 1] — full 4-sample delay (tap=4 = size).
    let out = opengen_testkit::render_with_inputs(
        "Delay d(4); d.write(in1); out1 = d.read(10);",
        48000.0,
        &[&[1.0, 0.0, 0.0, 0.0, 0.0]],
    );
    assert_eq!(out.ch(0), &[0.0, 0.0, 0.0, 0.0, 1.0]);
}

// ═══════════════════════════════════════════════════════════════════
//  Multiple delay declarations
// ═══════════════════════════════════════════════════════════════════

#[test]
fn multiple_delays_independent() {
    let out = opengen_testkit::render_with_inputs(
        "Delay a(4); Delay b(4); \
         a.write(in1); b.write(in2); \
         out1 = a.read(1); out2 = b.read(1);",
        48000.0,
        &[&[1.0, 0.0, 0.0], &[10.0, 0.0, 0.0]],
    );
    assert_eq!(out.ch(0), &[0.0, 1.0, 0.0]); // delay a echo
    assert_eq!(out.ch(1), &[0.0, 10.0, 0.0]); // delay b echo
}

// ═══════════════════════════════════════════════════════════════════
//  Error: double write on same delay
// ═══════════════════════════════════════════════════════════════════

#[test]
fn double_write_errors() {
    let err = opengen_genexpr::parse_and_lower(
        "Delay d(4); d.write(1); d.write(2); out1 = d.read(1);"
    ).unwrap_err();
    assert!(
        err.to_string().contains("already called"),
        "expected error about multiple writes, got: {}",
        err.to_string()
    );
}

// ═══════════════════════════════════════════════════════════════════
//  Error: member call on non-Delay binding
// ═══════════════════════════════════════════════════════════════════

#[test]
fn member_call_on_non_delay_errors() {
    let err = opengen_genexpr::parse_and_lower(
        "Param p(1); p.write(2); out1 = 0;"
    ).unwrap_err();
    assert!(
        err.to_string().contains("member calls"),
        "expected error about member calls, got: {}",
        err.to_string()
    );
}

// ═══════════════════════════════════════════════════════════════════
//  Error: delay member calls inside regions
// ═══════════════════════════════════════════════════════════════════

#[test]
fn delay_inside_region_errors() {
    let err = opengen_genexpr::parse_and_lower(
        "Delay d(4); if (in1 > 0) { d.write(in1); } out1 = d.read(1);"
    ).unwrap_err();
    assert!(
        err.to_string().contains("control flow"),
        "expected error about control flow, got: {}",
        err.to_string()
    );
}

// ═══════════════════════════════════════════════════════════════════
//  M3: Zero-size delay is rejected
// ═══════════════════════════════════════════════════════════════════

#[test]
fn zero_size_delay_rejected() {
    let err = opengen_genexpr::parse_and_lower(
        "Delay d(0); d.write(in1); out1 = d.read(1);"
    ).unwrap_err();
    assert!(
        err.to_string().contains("size must be >= 1"),
        "expected error about size >= 1, got: {}",
        err.to_string()
    );

    // Also test with empty argument
    let err2 = opengen_genexpr::parse_and_lower(
        "Delay d(0); out1 = 0;"
    ).unwrap_err();
    assert!(
        err2.to_string().contains("size must be >= 1"),
        "expected error about size >= 1 for unused delay, got: {}",
        err2.to_string()
    );
}

// ═══════════════════════════════════════════════════════════════════
//  Error: unknown interpolation mode

#[test]
fn unknown_interp_errors() {
    let err = opengen_genexpr::parse_and_lower(
        r##"Delay d(8); d.write(in1); out1 = d.read(1, interp="cubic");"##
    ).unwrap_err();
    assert!(
        err.to_string().contains("unknown interpolation"),
        "expected error about unknown interpolation, got: {}",
        err.to_string()
    );
}

// ═══════════════════════════════════════════════════════════════════
//  M1: Negative tap clamps to 1
// ═══════════════════════════════════════════════════════════════════

#[test]
fn negative_tap_clamps_to_one() {
    // d.read(0 - 100) is equivalent to d.read(-100) which is clamped to 1.
    // With tap=1, k=1, reads 1 sample ago (the most recent write).
    let out = opengen_testkit::render_with_inputs(
        "Delay d(8); d.write(in1); out1 = d.read(0 - 100);",
        48000.0,
        &[&[1.0, 0.0]],
    );
    // Same as out1 = d.read(1)
    let out_ref = opengen_testkit::render_with_inputs(
        "Delay d(8); d.write(in1); out1 = d.read(1);",
        48000.0,
        &[&[1.0, 0.0]],
    );
    assert_eq!(out.ch(0), out_ref.ch(0));
    // Also verify explicitly negative literal
    let out2 = opengen_testkit::render_with_inputs(
        "Delay d(8); d.write(in1); out1 = d.read(-100);",
        48000.0,
        &[&[1.0, 0.0]],
    );
    assert_eq!(out2.ch(0), out_ref.ch(0));
}

// ═══════════════════════════════════════════════════════════════════
//  M4: Read-before-write source order yields same output
// ═══════════════════════════════════════════════════════════════════

#[test]
fn read_before_write_same_as_write_before_read() {
    // Source order: read before write in the code. But compute phase
    // runs all reads before update-phase writes, so the output must be
    // identical to the write-before-read source order.
    let read_first = opengen_testkit::render_with_inputs(
        "Delay d(8); out1 = d.read(1); d.write(in1);",
        48000.0,
        &[&[1.0, 2.0, 3.0]],
    );
    let write_first = opengen_testkit::render_with_inputs(
        "Delay d(8); d.write(in1); out1 = d.read(1);",
        48000.0,
        &[&[1.0, 2.0, 3.0]],
    );
    assert_eq!(read_first.ch(0), write_first.ch(0));
}

// ═══════════════════════════════════════════════════════════════════
//  M2: NaN tap with linear interpolation → NaN output
// ═══════════════════════════════════════════════════════════════════

#[test]
fn nan_tap_produces_nan_linear() {
    // NaN tap with linear interp should produce NaN on the output.
    // Pass NaN as the tap signal (second input) and verify propagation.
    let out = opengen_testkit::render_with_inputs(
        "Delay d(8); d.write(in1); out1 = d.read(in2);",
        48000.0,
        &[&[1.0], &[f64::NAN]],
    );
    assert!(out.ch(0)[0].is_nan(), "expected NaN output for NaN tap");
}

// ═══════════════════════════════════════════════════════════════════
//  Determinism: multi-sample echo ring
// ═══════════════════════════════════════════════════════════════════

#[test]
fn multi_sample_ring_echo_works() {
    // Feed [1, 2, 3, 4] one per sample, read back tap=1 each sample
    let out = opengen_testkit::render_with_inputs(
        "Delay d(8); d.write(in1); out1 = d.read(1);",
        48000.0,
        &[&[1.0, 2.0, 3.0, 4.0]],
    );
    // Sample 0: in=1, read=0. out=0
    // Sample 1: in=2, read=1. out=1
    // Sample 2: in=3, read=2. out=2
    // Sample 3: in=4, read=3. out=3
    assert_eq!(out.ch(0), &[0.0, 1.0, 2.0, 3.0]);
}
