//! Stateful operators: delay, feedback, memory.

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Single-sample delay. Read returns the PREVIOUS sample's written value.
///
/// # Definition
/// `y[n] = x[n-1]; y[0] = init` (default 0.0).
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_history.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// // counter via feedback: acc = history(acc + 1)
/// let out = render("h = history(h + 1); out1 = h;", 48000.0, 3);
/// assert_eq!(out.ch(0), &[0.0, 1.0, 2.0]);
/// ```
pub fn history(_inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    // Read previous sample's value from state[0].
    // The update function (run at end of sample) copies inputs[0] → state[0].
    // This implements y[n] = x[n-1] with y[0] = 0.0 (zero-initialized).
    state[0]
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "history",
            arity: 1,
            state: StateDecl::Slots(1),
            deferred_ports: &[0],
            update: Some(|i, s, _| s[0] = i[0]),
            init: Some(|args, s, _| if let Some(&v) = args.first() { s[0] = v }),
            kernel: history,
        },
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn history_delays_constant() {
        // Non-feedback case: delay a constant by one sample
        use opengen_testkit::render;
        let out = render("h = history(2.5); out1 = h;", 48000.0, 3);
        // First sample: state[0] = 0.0 (init), then StateUpdate writes 2.5
        // Second sample: state[0] = 2.5, then StateUpdate writes 2.5
        // Third sample: state[0] = 2.5
        assert_eq!(out.ch(0), &[0.0, 2.5, 2.5]);
    }
}
