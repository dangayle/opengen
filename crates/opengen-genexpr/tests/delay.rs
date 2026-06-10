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
    // tap=1.5 with @interp none: nearest sample is tap=2 (1.5+0.5=2.0)
    // out1 = d.read(1.5, interp="none")
    let out = opengen_testkit::render_with_inputs(
        r#"Delay d(8); d.write(in1); out1 = d.read(1.5, interp="none");"#,
        48000.0,
        &[&[1.0, 0.0, 0.0]],
    );
    // tap=2: 2 samples ago. Sample 0: in1=1 -> out=0.
    // Sample 1: in1=0, tap=2 reads 1 from 2 samples ago (ring[(1-2+8)%8]=ring[7]... 
    // Actually wait: cursor goes 0->1 after sample 0 write. At sample 1, cursor=1.
    // tap=2: ring[(1-2+8)%8] = ring[7] = 0. Hmm, that's 0 not 1.
    // Let me think again...
    // Sample 0: cursor=0. Compute: tap=2 -> ring[(0-2+8)%8]=ring[6]=0. Output: 0.
    //   Update: write 1 at ring[0], cursor=1.
    // Sample 1: cursor=1. Compute: tap=2 -> ring[(1-2+8)%8]=ring[7]=0. Output: 0.
    //   Update: write 0 at ring[1], cursor=2.
    // Sample 2: cursor=2. Compute: tap=2 -> ring[(2-2+8)%8]=ring[0]=1. Output: 1.
    //   Update: write 0 at ring[2], cursor=3.
    // So: [0, 0, 1] not [0, 1, 0].
    assert_eq!(out.ch(0), &[0.0, 0.0, 1.0]);
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
    // delay_read linear-interp clamps the interpolation index to n-1=3
    // (to keep both i and i+1 in bounds).  So tap=4 reads nearest index 3.
    //
    //   Sample 0 (cursor=0, in=1):
    //     ring_read(0, 3, 4) = state[(0+4-3)%4+1] = state[2] = 0
    //     ring_read(0, 4, 4) = state[(0+4-4)%4+1] = state[1] = 0
    //     out = 0.  Write at ring[0]=state[1]=1, cursor→1
    //   Sample 1 (cursor=1, in=0):
    //     ring_read(1, 3) = state[(1+4-3)%4+1] = state[3] = 0
    //     ring_read(1, 4) = state[(1+4-4)%4+1] = state[2] = 0
    //     out = 0.  Write at ring[1]=state[2]=0, cursor→2
    //   Sample 2 (cursor=2, in=0):
    //     ring_read(2, 3) = state[(2+4-3)%4+1] = state[4] = 0
    //     ring_read(2, 4) = state[(2+4-4)%4+1] = state[3] = 0
    //     out = 0.  Write at ring[2]=state[3]=0, cursor→3
    //   Sample 3 (cursor=3, in=0):
    //     ring_read(3, 3) = state[(3+4-3)%4+1] = state[1] = 1 ← from sample 0!
    //     ring_read(3, 4) = state[(3+4-4)%4+1] = state[4] = 0
    //     out = 1 + 0*(0-1) = 1.  Write at ring[3]=state[4]=0, cursor→0
    //   Sample 4 (cursor=0, in=0):
    //     ring_read(0, 3) = state[(0+4-3)%4+1] = state[2] = 0
    //     ring_read(0, 4) = state[(0+4-4)%4+1] = state[1] = 1
    //     out = 0 + 0*(1-0) = 0.
    //
    // Result: [0, 0, 0, 1, 0] — the 1 appears at sample 3 because
    // linear interpolation clamps index to n-1, so tap=4 reads 3 samples ago.
    let out = opengen_testkit::render_with_inputs(
        "Delay d(4); d.write(in1); out1 = d.read(10);",
        48000.0,
        &[&[1.0, 0.0, 0.0, 0.0, 0.0]],
    );
    assert_eq!(out.ch(0), &[0.0, 0.0, 0.0, 1.0, 0.0]);
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
