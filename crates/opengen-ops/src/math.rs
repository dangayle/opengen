//! Arithmetic operators.
//!
//! Determinism note: Bit-identical output on a given platform/std version. Cross-platform
//! bit-identity for transcendentals is tracked as an M3 emitter concern.

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
/// ```
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
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = 2.0 * 0.75;", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.5);
/// ```
pub fn mul(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0] * inputs[1]
}

/// Subtract two signals: `out = a - b`.
///
/// # Definition
/// IEEE-754 f64 subtraction. No saturation, no denormal handling.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_sub.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = 5.0 - 2.0;", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 3.0);
/// ```
pub fn sub(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0] - inputs[1]
}

/// Divide two signals: `out = a / b`.
///
/// # Definition
/// IEEE-754 f64 division. Division by zero produces infinity per IEEE-754.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_div.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = 10.0 / 4.0;", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 2.5);
/// ```
pub fn div(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0] / inputs[1]
}

/// Modulo (remainder) operation: `out = a % b`.
///
/// # Definition
/// IEEE-754 f64 fmod (C semantics). Result has same sign as dividend.
/// Rust's `%` operator implements IEEE-754 fmod.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_mod.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = mod(5.5, 2.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.5);
/// // Negative dividend test
/// let out2 = render("out1 = mod(0.0 - 5.5, 2.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], -1.5);
/// ```
pub fn mod_(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0] % inputs[1]
}

/// Negate a signal: `out = -a`.
///
/// # Definition
/// IEEE-754 f64 negation. Sign bit flip.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_neg.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = neg(2.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], -2.5);
/// ```
pub fn neg(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    -inputs[0]
}

/// Absolute value: `out = abs(a)`.
///
/// # Definition
/// IEEE-754 f64 absolute value. Sign bit cleared.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_abs.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = abs(0.0 - 0.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.5);
/// ```
pub fn abs(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].abs()
}

/// Minimum of two signals: `out = min(a, b)`.
///
/// # Definition
/// IEEE-754 f64 minimum. NaN propagation: if either input is NaN, result is NaN.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_min.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = min(3.0, 1.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.5);
/// ```
pub fn min(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].min(inputs[1])
}

/// Maximum of two signals: `out = max(a, b)`.
///
/// # Definition
/// IEEE-754 f64 maximum. NaN propagation: if either input is NaN, result is NaN.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_max.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = max(1.5, 3.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 3.0);
/// ```
pub fn max(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].max(inputs[1])
}

/// Power: `out = pow(a, b)` = a^b.
///
/// # Definition
/// IEEE-754 f64 exponentiation via libm. NaN for negative base with non-integer exponent.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_pow.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = pow(2.0, 3.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 8.0);
/// ```
pub fn pow(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].powf(inputs[1])
}

/// Square root: `out = sqrt(a)`.
///
/// # Definition
/// IEEE-754 f64 square root. NaN for negative input.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_sqrt.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = sqrt(4.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 2.0);
/// ```
pub fn sqrt(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].sqrt()
}

/// Floor: `out = floor(a)` — largest integer ≤ a.
///
/// # Definition
/// IEEE-754 f64 floor operation.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_floor.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = floor(2.7);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 2.0);
/// let out2 = render("out1 = floor(0.0 - 2.3);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], -3.0);
/// ```
pub fn floor(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].floor()
}

/// Ceiling: `out = ceil(a)` — smallest integer ≥ a.
///
/// # Definition
/// IEEE-754 f64 ceiling operation.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_ceil.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = ceil(2.3);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 3.0);
/// let out2 = render("out1 = ceil(0.0 - 2.7);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], -2.0);
/// ```
pub fn ceil(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].ceil()
}

/// Exponential: `out = exp(x)` = e^x.
///
/// # Definition
/// Delegates to `f64::exp(x)`. IEEE-754 f64 exponential function.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_exp.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = exp(1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1f64.exp());
/// let out2 = render("out1 = exp(0.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 1.0);
/// ```
pub fn exp(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].exp()
}

/// Base-2 exponential: `out = exp2(x)` = 2^x.
///
/// # Definition
/// Delegates to `f64::exp2(x)`. IEEE-754 f64 base-2 exponential.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_exp2.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = exp2(3.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 8.0);
/// ```
pub fn exp2(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].exp2()
}

/// Natural logarithm: `out = ln(x)`.
///
/// # Definition
/// Delegates to `f64::ln(x)`. Returns -infinity for x=0, NaN for x<0 per IEEE-754.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_log.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = ln(1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.0);
/// ```
pub fn ln(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].ln()
}

/// Natural logarithm (alias): `out = log(x)`.
///
/// # Definition
/// Alias of `ln`. The refpage states "natural logarithm". Delegates to `f64::ln(x)`.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_log.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = log(1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.0);
/// ```
pub fn log(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].ln()
}

/// Base-2 logarithm: `out = log2(x)`.
///
/// # Definition
/// Delegates to `f64::log2(x)`. IEEE-754 f64 base-2 logarithm.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_log2.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = log2(8.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 3.0);
/// ```
pub fn log2(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].log2()
}

/// Base-10 logarithm: `out = log10(x)`.
///
/// # Definition
/// Delegates to `f64::log10(x)`. IEEE-754 f64 base-10 logarithm.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_log10.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = log10(1000.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 3.0);
/// ```
pub fn log10(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].log10()
}

/// Hypotenuse: `out = hypot(a, b)` = sqrt(a² + b²).
///
/// # Definition
/// Delegates to `f64::hypot(b)`. Computes Euclidean distance without overflow/underflow.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_hypot.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = hypot(3.0, 4.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 5.0);
/// ```
pub fn hypot(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].hypot(inputs[1])
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef { name: "add", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: add, cpp_kernel: Some("return {a0} + {a1};"), emit_cpp_call: None },
        OpDef { name: "mul", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: mul, cpp_kernel: Some("return {a0} * {a1};"), emit_cpp_call: None },
        OpDef { name: "sub", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: sub, cpp_kernel: Some("return {a0} - {a1};"), emit_cpp_call: None },
        OpDef { name: "div", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: div, cpp_kernel: Some("return {a0} / {a1};"), emit_cpp_call: None },
        OpDef { name: "mod", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: mod_, cpp_kernel: Some("return std::fmod({a0}, {a1});"), emit_cpp_call: None },
        OpDef { name: "neg", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: neg, cpp_kernel: Some("return -{a0};"), emit_cpp_call: None },
        OpDef { name: "abs", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: abs, cpp_kernel: Some("return std::abs({a0});"), emit_cpp_call: None },
        OpDef { name: "min", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: min, cpp_kernel: Some("return std::min({a0}, {a1});"), emit_cpp_call: None },
        OpDef { name: "max", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: max, cpp_kernel: Some("return std::max({a0}, {a1});"), emit_cpp_call: None },
        OpDef { name: "pow", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: pow, cpp_kernel: Some("return std::pow({a0}, {a1});"), emit_cpp_call: None },
        OpDef { name: "sqrt", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: sqrt, cpp_kernel: Some("return std::sqrt({a0});"), emit_cpp_call: None },
        OpDef { name: "floor", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: floor, cpp_kernel: Some("return std::floor({a0});"), emit_cpp_call: None },
        OpDef { name: "ceil", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: ceil, cpp_kernel: Some("return std::ceil({a0});"), emit_cpp_call: None },
        OpDef { name: "exp", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: exp, cpp_kernel: Some("return std::exp({a0});"), emit_cpp_call: None },
        OpDef { name: "exp2", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: exp2, cpp_kernel: Some("return std::exp2({a0});"), emit_cpp_call: None },
        OpDef { name: "ln", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: ln, cpp_kernel: Some("return std::log({a0});"), emit_cpp_call: None },
        OpDef { name: "log", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: log, cpp_kernel: Some("return std::log({a0});"), emit_cpp_call: None },
        OpDef { name: "log2", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: log2, cpp_kernel: Some("return std::log2({a0});"), emit_cpp_call: None },
        OpDef { name: "log10", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: log10, cpp_kernel: Some("return std::log10({a0});"), emit_cpp_call: None },
        OpDef { name: "hypot", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: hypot, cpp_kernel: Some("return std::hypot({a0}, {a1});"), emit_cpp_call: None },
    ]
}

#[cfg(test)]
mod tests {
    use opengen_testkit::render;

    #[test]
    fn ln_negative_returns_nan() {
        let out = render("out1 = ln(0.0 - 1.0);", 48000.0, 1);
        assert!(out.ch(0)[0].is_nan());
    }

    #[test]
    fn ln_zero_returns_neg_inf() {
        let out = render("out1 = ln(0.0);", 48000.0, 1);
        assert!(out.ch(0)[0].is_infinite());
        assert!(out.ch(0)[0].is_sign_negative());
    }

    #[test]
    fn log_family_negative_propagates_nan() {
        assert!(render("out1 = log2(0 - 1);", 48_000.0, 1).ch(0)[0].is_nan());
        assert!(render("out1 = log10(0 - 1);", 48_000.0, 1).ch(0)[0].is_nan());
    }
}
