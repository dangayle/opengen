//! Oscillators: signal generators with internal phase state.

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Ramp oscillator. Outputs a sawtooth wave 0..1 at the given frequency.
///
/// # Definition
/// `y[n] = wrap(y[n-1] + freq/sr, 0, 1)`, `y[0] = 0.0`. StateDecl::Slots(1), arity 1.
///
/// The kernel outputs the pre-increment phase value, ensuring y[0] == 0.0 exactly:
/// `out = state; state = wrap(state + freq/sr, 0, 1)`.
///
/// # Vendor
/// `reference/rnbo/operators/phasor.js`
///
/// RNBO's implementation emits the pre-increment value and applies wrapping before
/// the increment step (via conditional bounds checks).
///
/// # Observed
/// This is an open conformance question vs real gen~, to be verified by the M2
/// conformance harness (open item).
///
/// ```
/// use opengen_testkit::render;
/// // Exact ramp at 1000 Hz / 48000 sr: samples are 0, 1000/48000, 2000/48000...
/// let out = render("out1 = phasor(1000);", 48000.0, 3);
/// assert_eq!(out.ch(0), &[0.0, 1000.0/48000.0, 2000.0/48000.0]);
/// ```
pub fn phasor(inputs: &[f64], state: &mut [f64], sr: f64) -> f64 {
    let freq = inputs[0];
    let phase = state[0];
    
    // Output the current phase (pre-increment)
    let output = phase;
    
    // Advance phase with wrapping
    let mut next_phase = phase + freq / sr;
    
    // Wrap to [0, 1)
    if next_phase < 0.0 {
        next_phase = 1.0 + next_phase;
    }
    if next_phase >= 1.0 {
        next_phase = next_phase - 1.0;
    }
    
    state[0] = next_phase;
    output
}

/// Sine oscillator. Outputs a sine wave at the given frequency using f64::sin.
///
/// # Definition
/// `sin(2π · phase)` where phase advances like phasor. Slots(1), arity 1.
///
/// The phase state advances exactly as in phasor: `phase[n] = wrap(phase[n-1] + freq/sr, 0, 1)`.
///
/// # Divergence
/// gen~ uses an interpolated wavetable for cycle; we use `f64::sin` directly.
/// Rationale: exactness and determinism. Conformance tolerance will be handled
/// by the M2 harness.
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_cycle.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// // First sample is exactly 0.0 (phase = 0 → sin(0) = 0)
/// let out = render("out1 = cycle(1000);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.0);
///
/// // Quarter period at freq=12000, sr=48000: quarter period = sr/(4*freq) = 1 sample
/// // At sample 1, phase = 12000/48000 = 0.25 → sin(2π * 0.25) = sin(π/2) = 1.0
/// let out2 = render("out1 = cycle(12000);", 48000.0, 2);
/// let val = out2.ch(0)[1];
/// assert!((val - 1.0).abs() <= f64::EPSILON, "Expected ~1.0, got {}", val);
/// ```
pub fn cycle(inputs: &[f64], state: &mut [f64], sr: f64) -> f64 {
    let freq = inputs[0];
    let phase = state[0];
    
    // Compute sine output
    let output = (2.0 * std::f64::consts::PI * phase).sin();
    
    // Advance phase with wrapping (same as phasor)
    let mut next_phase = phase + freq / sr;
    
    // Wrap to [0, 1)
    if next_phase < 0.0 {
        next_phase = 1.0 + next_phase;
    }
    if next_phase >= 1.0 {
        next_phase = next_phase - 1.0;
    }
    
    state[0] = next_phase;
    output
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "phasor",
            arity: 1,
            state: StateDecl::Slots(1),
            auto_state_update: false, // Self-managed state
            kernel: phasor,
        },
        OpDef {
            name: "cycle",
            arity: 1,
            state: StateDecl::Slots(1),
            auto_state_update: false, // Self-managed state
            kernel: cycle,
        },
    ]
}

#[cfg(test)]
mod tests {
    use opengen_testkit::render;

    #[test]
    fn phasor_exact_ramp() {
        // Exact equality test: 1000 Hz at 48000 sr
        let out = render("out1 = phasor(1000);", 48000.0, 3);
        assert_eq!(out.ch(0), &[0.0, 1000.0/48000.0, 2000.0/48000.0]);
    }

    #[test]
    fn cycle_first_sample_zero() {
        let out = render("out1 = cycle(440);", 48000.0, 1);
        assert_eq!(out.ch(0)[0], 0.0);
    }

    #[test]
    fn cycle_quarter_period_near_one() {
        // At sr=48000, freq=12000 → period = 4 samples
        // Quarter period at sample 1: phase = 0.25 → sin(π/2) ≈ 1.0
        let out = render("out1 = cycle(12000);", 48000.0, 2);
        let val = out.ch(0)[1];
        assert!((val - 1.0).abs() <= f64::EPSILON, "Expected ~1.0, got {}", val);
    }
}
