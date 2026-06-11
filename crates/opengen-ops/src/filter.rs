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
/// where `x[n-1]` and `y[n-1]` are kernel-managed state slots. The input
/// history `x[n-1]` initializes to the FIRST input sample (so `y[0] = 0`
/// always — no startup step); `y[n-1]` initializes to 0. Slots(3): x1, y1,
/// first-sample flag.
///
/// # Observed
/// M2 conformance harness (2026-06-11): real gen~ outputs EXACT silence for
/// a constant input present from sample 0 (golden:
/// `conformance/golden/dcblock_step.ch0.wav`, patch
/// `conformance/patches/dcblock_step.genexpr`) — i.e. the input history
/// starts equal to the first input rather than 0. A follow-up probe
/// (dcblock driven by a t=0 impulse) is queued in `conformance/CHECKLIST.md`
/// to distinguish lazy x1-init from compiler constant-folding.
///
/// # Divergence
/// `reference/genlib/gen_dsp/genlib_ops.h` (struct DCBlock) resets x1 = 0,
/// which would output 1.0 at sample 0 for a unit step — the genlib code
/// EXPORT runtime demonstrably differs from gen~ inside Max here. We match
/// in-Max gen~ (the conformance reference).
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_dcblock.maxref.xml`
///
/// ```
/// use opengen_testkit::render_with_inputs;
/// // Constant (pure DC) input from sample 0 → exact silence (gen~ observed)
/// let dc: Vec<f64> = vec![1.0; 64];
/// let out = render_with_inputs("out1 = dcblock(in1);", 48000.0, &[&dc]);
/// assert!(out.ch(0).iter().all(|&v| v == 0.0));
///
/// // A step AFTER sample 0 produces the classic highpass response
/// let step: Vec<f64> = [0.0, 0.0, 1.0, 1.0, 1.0].to_vec();
/// let out2 = render_with_inputs("out1 = dcblock(in1);", 48000.0, &[&step]);
/// assert_eq!(out2.ch(0)[2], 1.0);               // edge passes through
/// assert_eq!(out2.ch(0)[3], 0.9997);            // then decays
/// ```
pub fn dcblock(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    if state[2] == 0.0 {
        // First sample: input history starts at the first input (gen~
        // observed — see # Observed above). y[0] is therefore always 0.
        state[2] = 1.0;
        state[0] = x;
    }
    let x1 = state[0]; // previous input
    let y1 = state[1]; // previous output
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
            state: StateDecl::Slots(3),
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: dcblock,
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
        },
    ]
}

#[cfg(test)]
mod tests {
    use opengen_testkit::render_with_inputs_n;
    use super::*;

    // ── dcblock ─────────────────────────────────────────────────

    #[test]
    fn dcblock_blocks_dc_over_time() {
        // Constant DC from sample 0: lazy x1-init absorbs it — EXACT silence
        // (gen~ observed; golden conformance/golden/dcblock_step.ch0.wav).
        let dc: Vec<f64> = vec![1.0; 10000];
        let out = render_with_inputs_n("out1 = dcblock(in1);", 48000.0, &[&dc], 10000);
        assert!(out.ch(0).iter().all(|&v| v == 0.0),
            "dcblock of constant input should be exactly silent");
    }

    #[test]
    fn dcblock_impulse_response_decays_to_zero() {
        // Impulse at sample 0, zeros thereafter. With gen~'s lazy x1-init,
        // y[0] = 0 (x1 starts at the impulse value); the falling edge at
        // sample 1 produces -1, then the magnitude decays toward zero.
        let impulse: Vec<f64> = vec![1.0];
        let out = render_with_inputs_n("out1 = dcblock(in1);", 48000.0, &[&impulse], 1000);
        assert_eq!(out.ch(0)[0], 0.0);
        assert_eq!(out.ch(0)[1], -1.0);
        // Magnitude decays monotonically after the edge
        for i in 2..out.ch(0).len() {
            assert!(out.ch(0)[i].abs() <= out.ch(0)[i-1].abs() + 1e-15,
                "magnitude decay failed at {}: {} → {}",
                i, out.ch(0)[i-1], out.ch(0)[i]);
        }
        // After 1000 samples, magnitude is very close to 0
        assert!(out.ch(0)[999].abs() < 0.75,
            "dcblock impulse after 1000 samples: got {}", out.ch(0)[999]);
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
