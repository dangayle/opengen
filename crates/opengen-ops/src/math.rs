//! Arithmetic operators.
use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Add two signals: `out = a + b`.
///
/// # Definition
/// IEEE-754 f64 addition. No saturation, no denormal handling.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_add.maxref.xml`
///
/// ```ignore
/// use opengen_testkit::render;
/// let out = render("out1 = 1.5 + 2.25;", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 3.75);
/// ```
pub fn add(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0] + inputs[1]
}

/// Multiply two signals: `out = a * b`.
///
/// # Definition
/// IEEE-754 f64 multiplication. No saturation, no denormal handling.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_mul.maxref.xml`
///
/// ```ignore
/// use opengen_testkit::render;
/// let out = render("out1 = 2.0 * 0.75;", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.5);
/// ```
pub fn mul(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0] * inputs[1]
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef { name: "add", arity: 2, state: StateDecl::None, kernel: add },
        OpDef { name: "mul", arity: 2, state: StateDecl::None, kernel: mul },
    ]
}
