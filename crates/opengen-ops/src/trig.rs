//! Trigonometric operators.
//!
//! All functions operate in radians and delegate to platform `f64` libm functions.
//! Determinism note: Bit-identical output on a given platform/std version. Cross-platform
//! bit-identity for transcendentals is tracked as an M3 emitter concern.

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Sine: `out = sin(x)`, x in radians.
///
/// # Definition
/// Delegates to `f64::sin(x)`. IEEE-754 f64 correctly-rounded sine.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_sin.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = sin(0.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.5f64.sin());
/// let out2 = render("out1 = sin(0.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// ```
pub fn sin(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].sin()
}

/// Cosine: `out = cos(x)`, x in radians.
///
/// # Definition
/// Delegates to `f64::cos(x)`. IEEE-754 f64 correctly-rounded cosine.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_cos.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = cos(0.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// ```
pub fn cos(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].cos()
}

/// Tangent: `out = tan(x)`, x in radians.
///
/// # Definition
/// Delegates to `f64::tan(x)`. IEEE-754 f64 correctly-rounded tangent.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_tan.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = tan(0.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.5f64.tan());
/// ```
pub fn tan(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].tan()
}

/// Arcsine: `out = asin(x)`, result in radians.
///
/// # Definition
/// Delegates to `f64::asin(x)`. Returns NaN for x outside [-1, 1] per IEEE-754.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_asin.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = asin(1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], std::f64::consts::FRAC_PI_2);
/// ```
pub fn asin(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].asin()
}

/// Arccosine: `out = acos(x)`, result in radians.
///
/// # Definition
/// Delegates to `f64::acos(x)`. Returns NaN for x outside [-1, 1] per IEEE-754.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_acos.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = acos(1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.0);
/// ```
pub fn acos(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].acos()
}

/// Arctangent: `out = atan(x)`, result in radians.
///
/// # Definition
/// Delegates to `f64::atan(x)`. IEEE-754 f64 correctly-rounded arctangent.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_atan.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = atan(1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], std::f64::consts::FRAC_PI_4);
/// ```
pub fn atan(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].atan()
}

/// Two-argument arctangent: `out = atan2(y, x)`, result in radians.
///
/// # Definition
/// Delegates to `y.atan2(x)`. Returns angle of point (x, y) from positive x-axis.
/// Inputs: (y, x) per refpage ordering. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_atan2.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = atan2(1.0, 1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], std::f64::consts::FRAC_PI_4);
/// ```
pub fn atan2(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].atan2(inputs[1])
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef { name: "sin", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: sin, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "cos", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: cos, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "tan", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: tan, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "asin", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: asin, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "acos", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: acos, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "atan", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: atan, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "atan2", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: atan2, cpp_kernel: None, emit_cpp_call: None },
    ]
}

#[cfg(test)]
mod tests {
    use opengen_testkit::render;

    #[test]
    fn asin_out_of_range_returns_nan() {
        let out = render("out1 = asin(2.0);", 48000.0, 1);
        assert!(out.ch(0)[0].is_nan());
    }

    #[test]
    fn acos_out_of_range_returns_nan() {
        let out = render("out1 = acos(2.0);", 48000.0, 1);
        assert!(out.ch(0)[0].is_nan());
    }
}
