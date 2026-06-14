//! Filter operators: dcblock, slide — one-pole highpass, logarithmic smoother.
//!
//! Both operators manage their state directly in the kernel (kernel-managed state).

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// DC blocking filter (one-pole highpass).
///
/// # Definition
/// A one-pole high-pass filter to remove DC components. The time-domain recurrence:
///
/// ```text
/// y[n] = x[n] - x[n-1] + y[n-1] * 0.9997
/// ```
///
/// where `x[n-1]` and `y[n-1]` are kernel-managed state slots, both
/// initialized to 0. This is the classic DC blocker (matching genlib
/// `reference/genlib/gen_dsp/genlib_ops.h` struct DCBlock). Slots(2):
/// x1, y1.
///
/// Impulse response: `y[0]` = 1, `y[n]` = -0.9993 * 0.9997^(n-1) for n ≥ 1
/// (confirmed against gen~ golden `conformance/golden/dcblock_impulse.ch0.wav`,
/// 2026-06-13).
///
/// # Observed
/// M2 conformance harness (2026-06-11), updated 2026-06-13 with follow-up
/// impulse probe (`conformance/patches/dcblock_impulse.genexpr`): the
/// impulse-response golden starts at `y[0]` = 1.0, confirming the genlib
/// x1=0 init. The prior hypothesis of "lazy x1-init to first input"
/// (which would give `y[0]` = 0) is rejected.
///
/// # Divergence
/// `conformance/patches/dcblock_step.genexpr` feeds a compile-time-constant
/// input (`dcblock(1.0)`). gen~'s JIT compiler constant-folds this to the
/// steady-state value (all zeros). opengen has no constant folder — it runs
/// the per-sample kernel, producing the classic highpass step response
/// (`y[0]` = 1.0, then decay). The golden for this patch measures gen~'s
/// **optimizer**, not the dcblock **kernel**; opengen's kernel output for a
/// step is correct. dcblock_step is a known divergence of the
/// constant-folder-vs-kernel class identified in M2 (see CLAUDE.md).
///
/// `reference/genlib/gen_dsp/genlib_ops.h` (struct DCBlock) establishes
/// x1 = 0 init — consistent with the impulse golden and with this
/// implementation.
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_dcblock.maxref.xml`
///
/// ```
/// use opengen_testkit::render_with_inputs;
/// // A step arriving at sample 2 produces the classic highpass response
/// let step: Vec<f64> = [0.0, 0.0, 1.0, 1.0, 1.0].to_vec();
/// let out = render_with_inputs("out1 = dcblock(in1);", 48000.0, &[&step]);
/// assert_eq!(out.ch(0)[2], 1.0);               // edge passes through
/// assert_eq!(out.ch(0)[3], 0.9997);            // then decays
/// ```
pub fn dcblock(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    let x1 = state[0]; // previous input (init 0)
    let y1 = state[1]; // previous output (init 0)
    let y = x - x1 + y1 * 0.9997;
    state[0] = x; // update x1
    state[1] = y; // update y1
    y
}

/// Logarithmic signal smoother (slide).
///
/// # Definition
/// Exponential approach filter with separate up/down time constants.
/// For input `x`, previous output `y_prev`, and time constants `up` (rising)
/// / `down` (falling):
///
/// ```text
/// rate = 1.0 / max(slide, 1.0)   // choosing `up` when rising, `down` when falling
/// y = y_prev + rate * (x - y_prev)
/// ```
///
/// The slide time constants are in samples — larger values produce slower slewing.
/// The `@init` attribute sets the initial value of the held state (default 0.0).
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_slide.maxref.xml`
///
/// # Vendor
/// `reference/rnbo/operators/slide.js` (paraphrased — EULA file, facts only):
/// `iup = safediv(1., maximum(1., abs(up)))`, `idown = safediv(1., maximum(1., abs(down)))`,
/// `prev = prev + (((x > prev) ? iup : idown) * (x - prev))`.
///
/// ```
/// use opengen_testkit::render_with_inputs;
/// // slide(x, 1, 1) is identity: rate = 1/1 = 1.0 → y = y_prev + 1.0*(x - y_prev) = x
/// let xs: Vec<f64> = vec![0.0, 0.0, 1.0, 2.0];
/// let out = render_with_inputs("out1 = slide(in1, in2, in3);", 48000.0,
///     &[&xs, &[1.0, 1.0, 1.0, 1.0], &[1.0, 1.0, 1.0, 1.0]]);
/// assert_eq!(out.ch(0), &[0.0, 0.0, 1.0, 2.0]);
/// ```
pub fn slide(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    let up = inputs[1];
    let down = inputs[2];
    let y_prev = state[0];

    let rate = if x > y_prev {
        1.0 / up.abs().max(1.0)
    } else {
        1.0 / down.abs().max(1.0)
    };

    let y = y_prev + rate * (x - y_prev);
    state[0] = y;
    y
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "dcblock",
            arity: 1,
            state: StateDecl::Slots(2),
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: dcblock,
            cpp_kernel: None,
            emit_cpp_call: None,
        },
        OpDef {
            name: "slide",
            arity: 3,
            state: StateDecl::Slots(1),
            deferred_ports: &[],
            update: None,
            // @init attribute: args[0] sets the initial held value
            init: Some(|args, state, _sr| {
                if let Some(&v) = args.first() {
                    state[0] = v;
                }
            }),
            kernel: slide,
            cpp_kernel: None,
            emit_cpp_call: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use opengen_testkit::render_with_inputs_n;
    use super::*;

    // ── dcblock ─────────────────────────────────────────────────

    #[test]
    fn dcblock_step_is_classic_highpass_genlib_form() {
        // Constant DC from sample 0: genlib form (x1=0 init) produces classic
        // IR: y[0] = 1, then decays. gen~'s JIT constant-folds this to all
        // zeros at compile time — that's a folder-vs-kernel divergence, not
        // an operator discrepancy. See # Divergence in the dcblock rustdoc.
        let dc: Vec<f64> = vec![1.0; 100];
        let out = render_with_inputs_n("out1 = dcblock(in1);", 48000.0, &[&dc], 100);
        assert_eq!(out.ch(0)[0], 1.0, "genlib-form dcblock: y[0] = x[0] - 0 + 0*R = 1");
        assert!((out.ch(0)[1] - 0.9997).abs() < 1e-12);
        // Decays toward zero: magnitude decreases monotonically
        for i in 2..out.ch(0).len() {
            assert!(out.ch(0)[i].abs() <= out.ch(0)[i-1].abs() + 1e-15);
        }
    }

    #[test]
    fn dcblock_impulse_response_matches_gen_golden() {
        // Impulse at sample 0, zeros thereafter. gen~ golden
        // (conformance/golden/dcblock_impulse.ch0.wav, 2026-06-13)
        // starts y[0] = 1.0, confirming genlib x1=0 init.
        let impulse: Vec<f64> = vec![1.0];
        let out = render_with_inputs_n("out1 = dcblock(in1);", 48000.0, &[&impulse], 100);
        assert_eq!(out.ch(0)[0], 1.0);
        // y[1] = 0 - 1 + 1*0.9997 = -0.0003
        assert!((out.ch(0)[1] + 0.0003).abs() < 1e-7);
        // y[2] = 0 - 0 + (-0.0003)*0.9997 ≈ -0.0002999
        assert!((out.ch(0)[2] + 0.0002999).abs() < 1e-7);
    }

    // ── slide ────────────────────────────────────────────────────

    #[test]
    fn slide_identity_with_unit_time_constants() {
        let xs: Vec<f64> = vec![0.0, 0.0, 1.0, 1.0, 2.0, 2.0];
        let ups: Vec<f64> = vec![1.0; 6];
        let downs: Vec<f64> = vec![1.0; 6];
        let out = render_with_inputs_n("out1 = slide(in1, in2, in3);", 48000.0,
            &[&xs, &ups, &downs], 6);
        assert_eq!(out.ch(0), &[0.0, 0.0, 1.0, 1.0, 2.0, 2.0]);
    }

    #[test]
    fn slide_step_up_asymptotic_approach() {
        // Step 0→1 with up=2: y0 = 0+(1-0)/2 = 0.5, y1 = 0.5+0.5/2=0.75, y2 = 0.875
        let xs: Vec<f64> = vec![1.0; 10];
        let ups: Vec<f64> = vec![2.0; 10];
        let downs: Vec<f64> = vec![1.0; 10];
        let out = render_with_inputs_n("out1 = slide(in1, in2, in3);", 48000.0,
            &[&xs, &ups, &downs], 10);
        assert!((out.ch(0)[0] - 0.5).abs() < 1e-15);
        assert!((out.ch(0)[1] - 0.75).abs() < 1e-15);
        assert!((out.ch(0)[2] - 0.875).abs() < 1e-15);
    }

    #[test]
    fn slide_clamps_sub_one_time_constants() {
        // up/down < 1.0 → clamped to 1.0 → identity
        // All 3 samples should equal the input (5.0) since rate = 1/1 = 1.0
        let out = render_with_inputs_n("out1 = slide(in1, in2, in3);", 48000.0,
            &[&[5.0; 3], &[0.5; 3], &[0.5; 3]], 3);
        assert_eq!(out.ch(0), &[5.0, 5.0, 5.0]);
    }

    #[test]
    fn slide_init_sets_initial_value() {
        // Direct kernel test: init state[0] = 1.0, then input 0.0 with down=2
        // y = 1.0 + (0.0 - 1.0)/2 = 0.5
        let mut state = [1.0]; // pre-initialized
        let result = slide(&[0.0, 1.0, 2.0], &mut state, 48000.0);
        assert!((result - 0.5).abs() < 1e-15);
        assert!((state[0] - 0.5).abs() < 1e-15);
    }

    #[test]
    fn slide_rising_uses_up_parameter() {
        // x > prev: use up parameter
        let mut state = [0.0];
        let result = slide(&[1.0, 4.0, 2.0], &mut state, 48000.0);
        // rate = 1/4 = 0.25, y = 0 + 0.25*(1-0) = 0.25
        assert!((result - 0.25).abs() < 1e-15);
    }

    #[test]
    fn slide_falling_uses_down_parameter() {
        // x < prev: use down parameter
        let mut state = [1.0];
        let result = slide(&[0.0, 1.0, 4.0], &mut state, 48000.0);
        // rate = 1/4 = 0.25, y = 1.0 + 0.25*(0-1.0) = 0.75
        assert!((result - 0.75).abs() < 1e-15);
    }

    #[test]
    fn slide_negative_times_use_abs() {
        // Negative up/down should use abs(): slide(x, -5, -5) behaves like slide(x, 5, 5)
        // Step 0→1 with up=-5 → abs(-5)=5 → rate = 1/5 = 0.2 → y = 0 + 0.2*(1-0) = 0.2
        let mut state = [0.0];
        let result = slide(&[1.0, -5.0, -5.0], &mut state, 48000.0);
        assert!((result - 0.2).abs() < 1e-15,
            "slide(-5) should behave like slide(5): expected 0.2, got {}", result);
    }
}
