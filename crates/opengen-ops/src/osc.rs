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

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "phasor",
            arity: 1,
            state: StateDecl::Slots(1),
            auto_state_update: false, // Self-managed state
            kernel: phasor,
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
}
