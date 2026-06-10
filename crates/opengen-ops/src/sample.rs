//! Sample-and-hold / sample operators: sah, latch, delta, change, accum.
//!
//! All operators manage their state directly in the kernel (kernel-managed state).

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Sample and hold (Schmitt trigger): `out = sah(input, ctrl, thresh)`.
///
/// # Definition
/// Samples the input when the control signal crosses the threshold upward:
///
/// ```text
/// trigger = (prev_ctrl <= thresh && ctrl > thresh)
/// output = trigger ? input : held_value
/// prev_ctrl = ctrl
/// held_value = output
/// ```
///
/// The threshold argument is the third inlet (default 0.0). The `@init` attribute
/// sets the initial held value (default 0.0).
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_sah.maxref.xml`
///
/// Description confirms the Schmitt trigger behavior: the input is sampled when
/// the control signal transitions from at-or-below the threshold to strictly above it.
///
/// ```
/// use opengen_testkit::render_with_inputs;
/// // ctrl ramp crossing 0.5: ctrl [0, 0.3, 0.6, 0.9], thresh=0.5
/// // Sample triggers when prev_ctrl <= 0.5 && ctrl > 0.5:
/// //   sample 0: prev=0, ctrl=0 → no trigger (0 <= 0.5, but 0 is not > 0.5) → held=0
/// //   sample 1: prev=0, ctrl=0.3 → no trigger (0.3 not > 0.5) → held=0
/// //   sample 2: prev=0.3, ctrl=0.6 → trigger (0.3 <= 0.5 && 0.6 > 0.5) → held=input=42
/// //   sample 3: prev=0.6, ctrl=0.9 → no trigger (0.6 > 0.5) → held=42
/// let out = render_with_inputs("out1 = sah(in1, in2, in3);", 48000.0,
///     &[&[42.0, 42.0, 42.0, 42.0],
///       &[0.0, 0.3, 0.6, 0.9],
///       &[0.5, 0.5, 0.5, 0.5]]);
/// assert_eq!(out.ch(0), &[0.0, 0.0, 42.0, 42.0]);
/// ```
pub fn sah(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];      // input signal
    let ctrl = inputs[1];    // control signal
    let thresh = inputs[2];  // threshold
    let prev_ctrl = state[0]; // previous control value
    let held = state[1];      // currently held value

    // Trigger: prev_ctrl <= thresh && ctrl > thresh
    let trigger = prev_ctrl <= thresh && ctrl > thresh;
    let output = if trigger { x } else { held };

    state[0] = ctrl;    // update prev_ctrl
    state[1] = output;   // update held value
    output
}

/// Conditional pass/hold: `out = latch(input, ctrl)`.
///
/// # Definition
/// When control is non-zero, the input is passed through and stored.
/// When control is zero, the last stored input is output.
///
/// ```text
/// output = (ctrl != 0) ? input : held
/// held = output        // always update held (even when passing through)
/// ```
///
/// The `@init` attribute sets the initial held value (default 0.0).
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_latch.maxref.xml`
///
/// ```
/// use opengen_testkit::render_with_inputs;
/// // input [5, 7, 9], ctrl [1, 0, 1] → output [5, 5, 9]
/// let out = render_with_inputs("out1 = latch(in1, in2);", 48000.0,
///     &[&[5.0, 7.0, 9.0], &[1.0, 0.0, 1.0]]);
/// assert_eq!(out.ch(0), &[5.0, 5.0, 9.0]);
/// ```
pub fn latch(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    let ctrl = inputs[1];
    let held = state[0];

    let output = if ctrl != 0.0 { x } else { held };
    state[0] = output;
    output
}

/// Discrete derivative: `out = delta(x)`.
///
/// # Definition
/// Returns the difference between the current and previous input:
///
/// ```text
/// output = x - prev
/// prev = x
/// ```
///
/// The `@init` attribute sets the initial value of `prev` against which the
/// first input is compared (default 0.0).
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_delta.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = delta(1.0);", 48000.0, 1);
/// // No previous input (default 0) → 1.0 - 0 = 1.0
/// assert_eq!(out.ch(0)[0], 1.0);
/// ```
pub fn delta(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    let prev = state[0];
    let output = x - prev;
    state[0] = x;
    output
}

/// Sign of derivative: `out = change(x)`.
///
/// # Definition
/// Returns the sign of the difference between the current and previous input:
///
/// ```text
/// output = 1  if x > prev
///          0  if x == prev
///          -1 if x < prev
/// prev = x
/// ```
///
/// The `@init` attribute sets the initial value of `prev` (default 0.0).
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_change.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// // First sample: change(1) with prev=0 → 1>0 → 1
/// let out = render("out1 = change(1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// ```
pub fn change(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    let prev = state[0];
    let output = if x > prev {
        1.0
    } else if x < prev {
        -1.0
    } else {
        0.0
    };
    state[0] = x;
    output
}

/// Additive accumulator: `out = accum(increment, reset)`.
///
/// # Definition
/// Accumulates the input by addition. The reset signal (non-zero) resets the
/// internal sum to the minimum value (default 0.0) **after** the accumulation
/// (`resetmode='post'`, the default per refpage):
///
/// ```text
/// output = sum + increment
/// sum = (reset != 0) ? 0.0 : output
/// ```
///
/// The `@init` attribute sets the initial sum value (default 0.0).
///
/// # M2 Scope
/// Only the default `resetmode='post'` is implemented. The `@min` and `@max`
/// attributes (for non-zero reset values and wrap behavior) and `resetmode='pre'`
/// are M3+ backlog items.
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_accum.maxref.xml`
///
/// The operator increments an internal sum by the input value and outputs the
/// result. Default
/// `resetmode='post'`: reset happens after the accumulation step.
///
/// ```
/// use opengen_testkit::render_with_inputs;
/// // accum(1, 0) three times → [1, 2, 3]
/// let out = render_with_inputs("out1 = accum(in1, in2);", 48000.0,
///     &[&[1.0, 1.0, 1.0], &[0.0, 0.0, 0.0]]);
/// assert_eq!(out.ch(0), &[1.0, 2.0, 3.0]);
/// ```
pub fn accum(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    let incr = inputs[0];
    let reset = inputs[1];
    let sum = state[0];

    let output = sum + incr;
    state[0] = if reset != 0.0 { 0.0 } else { output };
    output
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "sah",
            arity: 3,
            state: StateDecl::Slots(2),
            deferred_ports: &[],
            update: None,
            init: Some(|args, state, _sr| {
                // @init: args[0] = initial held value
                if let Some(&v) = args.first() {
                    state[1] = v;
                }
            }),
            kernel: sah,
        },
        OpDef {
            name: "latch",
            arity: 2,
            state: StateDecl::Slots(1),
            deferred_ports: &[],
            update: None,
            init: Some(|args, state, _sr| {
                if let Some(&v) = args.first() {
                    state[0] = v;
                }
            }),
            kernel: latch,
        },
        OpDef {
            name: "delta",
            arity: 1,
            state: StateDecl::Slots(1),
            deferred_ports: &[],
            update: None,
            init: Some(|args, state, _sr| {
                if let Some(&v) = args.first() {
                    state[0] = v;
                }
            }),
            kernel: delta,
        },
        OpDef {
            name: "change",
            arity: 1,
            state: StateDecl::Slots(1),
            deferred_ports: &[],
            update: None,
            init: Some(|args, state, _sr| {
                if let Some(&v) = args.first() {
                    state[0] = v;
                }
            }),
            kernel: change,
        },
        OpDef {
            name: "accum",
            arity: 2,
            state: StateDecl::Slots(1),
            deferred_ports: &[],
            update: None,
            init: Some(|args, state, _sr| {
                if let Some(&v) = args.first() {
                    state[0] = v;
                }
            }),
            kernel: accum,
        },
    ]
}

#[cfg(test)]
mod tests {
    use opengen_testkit::{render_with_inputs, render_with_inputs_n};
    use super::*;

    // ── sah ─────────────────────────────────────────────────────

    #[test]
    fn sah_does_not_sample_below_threshold() {
        let out = render_with_inputs_n("out1 = sah(in1, in2, in3);", 48000.0,
            &[&[100.0; 5], &[0.0, 0.2, 0.4, 0.3, 0.0], &[0.5; 5]], 5);
        // Never crosses 0.5 → never samples → output stays 0 (init)
        assert_eq!(out.ch(0), &[0.0, 0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn sah_crosses_exactly_at_threshold() {
        // prev <= thresh && ctrl > thresh
        // prev=0.5, ctrl=0.5: NOT a trigger (ctrl is not > thresh)
        let out = render_with_inputs_n("out1 = sah(in1, in2, in3);", 48000.0,
            &[&[42.0; 5], &[0.0, 0.5, 0.5, 0.6, 0.0], &[0.5; 5]], 5);
        // sample 0: prev=0, ctrl=0 → no trigger (0 not > 0.5)
        // sample 1: prev=0, ctrl=0.5 → no trigger (0.5 not > 0.5)
        // sample 2: prev=0.5, ctrl=0.5 → no trigger (0.5 not > 0.5)
        // sample 3: prev=0.5, ctrl=0.6 → trigger! (0.5 <= 0.5 && 0.6 > 0.5)
        // sample 4: prev=0.6, ctrl=0.0 → no trigger
        assert_eq!(out.ch(0), &[0.0, 0.0, 0.0, 42.0, 42.0]);
    }

    #[test]
    fn sah_trigger_on_exact_threshold_match() {
        // prev <= thresh: prev=0.5, thresh=0.5 → prev <= thresh ✓
        // ctrl > thresh: ctrl=0.51 → trigger
        let out = render_with_inputs_n("out1 = sah(in1, in2, in3);", 48000.0,
            &[&[99.0; 5], &[0.5, 0.51, 0.0, 0.0, 0.0], &[0.5; 5]], 5);
        assert_eq!(out.ch(0)[0], 0.0);   // init held
        assert_eq!(out.ch(0)[1], 99.0);  // trigger
        assert_eq!(out.ch(0)[2], 99.0);  // held
    }

    #[test]
    fn sah_holds_value_across_multiple_triggers() {
        // sah is edge-triggered: rising crossing thresh from below.
        // ctrl dips below thresh then rises again to trigger.
        let out = render_with_inputs("out1 = sah(in1, in2, in3);", 48000.0,
            &[&[10.0, 10.0, 20.0, 20.0],
              &[0.0, 0.6, 0.0, 0.6],
              &[0.5, 0.5, 0.5, 0.5]]);
        // sample 0: prev=0, ctrl=0 → no trigger (0 not > 0.5) → held=0
        // sample 1: prev=0, ctrl=0.6 → trigger! (0 <= 0.5 && 0.6 > 0.5) → sample 10
        // sample 2: prev=0.6, ctrl=0 → no trigger (0 not > 0.5) → held=10
        // sample 3: prev=0, ctrl=0.6 → trigger! (0 <= 0.5 && 0.6 > 0.5) → sample 20
        assert_eq!(out.ch(0), &[0.0, 10.0, 10.0, 20.0]);
    }

    // ── latch ───────────────────────────────────────────────────

    #[test]
    fn latch_passes_and_holds() {
        let out = render_with_inputs("out1 = latch(in1, in2);", 48000.0,
            &[&[5.0, 7.0, 9.0, 11.0], &[1.0, 0.0, 0.0, 1.0]]);
        assert_eq!(out.ch(0), &[5.0, 5.0, 5.0, 11.0]);
    }

    #[test]
    fn latch_starts_from_held_value_when_ctrl_is_zero() {
        let out = render_with_inputs("out1 = latch(in1, in2);", 48000.0,
            &[&[42.0, 42.0], &[0.0, 1.0]]);
        // ctrl=0 → held=0 (init) → output 0
        // ctrl=1 → pass through 42
        assert_eq!(out.ch(0), &[0.0, 42.0]);
    }

    #[test]
    fn latch_with_continuous_nonzero_ctrl() {
        let out = render_with_inputs("out1 = latch(in1, in2);", 48000.0,
            &[&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0]]);
        // All non-zero → passes through each sample
        assert_eq!(out.ch(0), &[1.0, 2.0, 3.0]);
    }

    // ── delta ───────────────────────────────────────────────────

    #[test]
    fn delta_sequential_values() {
        let out = render_with_inputs("out1 = delta(in1);", 48000.0,
            &[&[1.0, 4.0, 9.0]]);
        assert_eq!(out.ch(0), &[1.0, 3.0, 5.0]);
    }

    #[test]
    fn delta_constant_input_produces_zero() {
        let out = render_with_inputs("out1 = delta(in1);", 48000.0,
            &[&[5.0, 5.0, 5.0]]);
        // first: 5-0=5, then 5-5=0, 5-5=0
        assert_eq!(out.ch(0), &[5.0, 0.0, 0.0]);
    }

    #[test]
    fn delta_negative_values() {
        let out = render_with_inputs("out1 = delta(in1);", 48000.0,
            &[&[0.0, -5.0, -5.0]]);
        // 0-0=0, -5-0=-5, -5-(-5)=0
        assert_eq!(out.ch(0), &[0.0, -5.0, 0.0]);
    }

    // ── change ──────────────────────────────────────────────────

    #[test]
    fn change_detects_increasing_decreasing_unchanging() {
        let out = render_with_inputs("out1 = change(in1);", 48000.0,
            &[&[1.0, 4.0, 4.0, 2.0]]);
        assert_eq!(out.ch(0), &[1.0, 1.0, 0.0, -1.0]);
    }

    #[test]
    fn change_constant_input_all_zeros() {
        let out = render_with_inputs("out1 = change(in1);", 48000.0,
            &[&[3.0, 3.0, 3.0]]);
        // 3>0 → 1, then 3=3 → 0, 3=3 → 0
        assert_eq!(out.ch(0), &[1.0, 0.0, 0.0]);
    }

    #[test]
    fn change_alternating() {
        let out = render_with_inputs("out1 = change(in1);", 48000.0,
            &[&[0.0, 1.0, 0.0, 1.0]]);
        assert_eq!(out.ch(0), &[0.0, 1.0, -1.0, 1.0]);
    }

    // ── accum ───────────────────────────────────────────────────

    #[test]
    fn accum_accumulates() {
        let out = render_with_inputs("out1 = accum(in1, in2);", 48000.0,
            &[&[1.0, 1.0, 1.0], &[0.0, 0.0, 0.0]]);
        assert_eq!(out.ch(0), &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn accum_reset_post_accumulation() {
        // Default resetmode='post': accumulator adds then resets.
        // Sample 0: sum=0, incr=1, reset=0 → out=1, sum=1
        // Sample 1: sum=1, incr=1, reset=1 → out=2, sum=0 (reset after)
        // Sample 2: sum=0, incr=1, reset=0 → out=1, sum=1
        let out = render_with_inputs("out1 = accum(in1, in2);", 48000.0,
            &[&[1.0, 1.0, 1.0], &[0.0, 1.0, 0.0]]);
        assert_eq!(out.ch(0), &[1.0, 2.0, 1.0]);
    }

    #[test]
    fn accum_simultaneous_incr_reset() {
        // When reset is nonzero, output happens BEFORE reset ('post' mode)
        let out = render_with_inputs("out1 = accum(in1, in2);", 48000.0,
            &[&[5.0, 5.0], &[1.0, 0.0]]);
        // Sample 0: sum=0, incr=5, reset=1 → out=5, sum=0 (post-reset)
        // Sample 1: sum=0, incr=5, reset=0 → out=5, sum=5
        assert_eq!(out.ch(0), &[5.0, 5.0]);
    }

    #[test]
    fn accum_large_values_grow_linearly() {
        let out = render_with_inputs("out1 = accum(in1, in2);", 48000.0,
            &[&[10.0; 5], &[0.0; 5]]);
        assert_eq!(out.ch(0), &[10.0, 20.0, 30.0, 40.0, 50.0]);
    }

    // ── Direct kernel tests ─────────────────────────────────────

    #[test]
    fn sah_init_sets_held_value() {
        // Direct test: init sets state[1] = held value
        let mut state = [0.0, 5.0]; // prev_ctrl=0, held=5
        let result = sah(&[42.0, 0.0, 0.5], &mut state, 48000.0);
        assert_eq!(result, 5.0); // no trigger → held value
    }

    #[test]
    fn delta_first_sample_uses_init() {
        // init prev=0 → first delta(1) = 1 - 0 = 1
        let mut state = [0.0];
        assert_eq!(delta(&[1.0], &mut state, 48000.0), 1.0);
        // init prev=3 → first delta(1) = 1 - 3 = -2
        let mut state = [3.0];
        assert_eq!(delta(&[1.0], &mut state, 48000.0), -2.0);
    }

    #[test]
    fn change_first_sample_uses_init() {
        // init prev=0 → change(1) = 1
        let mut state = [0.0];
        assert_eq!(change(&[1.0], &mut state, 48000.0), 1.0);
        // init prev=2 → change(1) = -1
        let mut state = [2.0];
        assert_eq!(change(&[1.0], &mut state, 48000.0), -1.0);
    }

    #[test]
    fn accum_init_sets_starting_sum() {
        let mut state = [10.0]; // pre-initialized
        assert_eq!(accum(&[5.0, 0.0], &mut state, 48000.0), 15.0);
        assert_eq!(state[0], 15.0);
    }
}
