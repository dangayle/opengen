//! Sample rate accessor.

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Sample rate value: `out = samplerate`.
///
/// # Definition
/// Returns the current sample rate as a constant `f64` value.
/// Arity 0 (takes no inputs). In GenExpr, `samplerate` is a builtin identifier
/// that resolves to the audio sample rate.
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_samplerate.maxref.xml`
///
/// ```
/// use opengen_ops::samplerate::samplerate_op;
/// assert_eq!(samplerate_op(&[], &mut [], 48000.0), 48000.0);
/// assert_eq!(samplerate_op(&[], &mut [], 96000.0), 96000.0);
/// ```
pub fn samplerate_op(_inputs: &[f64], _state: &mut [f64], sr: f64) -> f64 {
    sr
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef { name: "samplerate", arity: 0, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: samplerate_op, cpp_kernel: Some("return samplerate;"), emit_cpp_call: None },
    ]
}
