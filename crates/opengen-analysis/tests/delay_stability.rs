//! Stability assertions for delay-line feedback loops.
//!
//! A delay with feedback gain < 1.0 should produce finite, bounded output
//! (exponentially decaying impulse response).  These tests verify that the
//! deferred-write semantic (reads before writes) interacts correctly with
//! feedback by rendering many samples and checking finiteness/denormals.

use opengen_analysis::assert_stable;

#[test]
fn feedback_loop_with_gain_half_does_not_blow_up() {
    // y = d.read(10) * 0.5 + in1;  d.write(y);  out1 = y;
    // Feed a unit impulse then 0 for the remaining samples.
    // The output should be finite, denormal-free, and eventually decay to 0.
    let out = opengen_testkit::render_with_inputs_n(
        "Delay d(64); y = d.read(10) * 0.5 + in1; d.write(y); out1 = y;",
        48_000.0,
        &[&[1.0]],
        /* n = */ 2048,
    );
    assert_stable!(out.ch(0));
}

#[test]
fn feedback_loop_impulse_decays_exponentially() {
    // Same patch: after the first 64 samples the feedback dominates.
    // Sample 0:  in=1 → y=1.  Tail starts sample 10 with value 0.5.
    // At N=2048 the tail should be at 0.5 * (0.5)^((2048-10)/64) ≈ 2.3e-10,
    // well above denormal threshold (~2.2e-308).  Also check RMS bound.
    let out = opengen_testkit::render_with_inputs_n(
        "Delay d(64); y = d.read(10) * 0.5 + in1; d.write(y); out1 = y;",
        48_000.0,
        &[&[1.0]],
        /* n = */ 2048,
    );
    assert_stable!(out.ch(0), max_rms = 1.0, max_dc = 0.02);

    // Verify the envelope is decaying (last sample much smaller than peak)
    let peak = out.ch(0).iter().cloned().fold(0.0_f64, f64::max);
    let last = out.ch(0)[out.ch(0).len() - 1];
    assert!(last.abs() < peak * 0.001, "last={last} should be very small relative to peak={peak}");
}

#[test]
fn feedback_loop_zero_input_produces_zero_output() {
    // No input impulse → everything stays at 0.
    let out = opengen_testkit::render("Delay d(64); y = d.read(10) * 0.5; d.write(y); out1 = y;", 48_000.0, 128);
    assert_stable!(out.ch(0));
    assert_eq!(out.ch(0), &[0.0; 128]);
}
