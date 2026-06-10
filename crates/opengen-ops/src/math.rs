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

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef { name: "add", arity: 2, state: StateDecl::None, auto_state_update: true, kernel: add },
        OpDef { name: "mul", arity: 2, state: StateDecl::None, auto_state_update: true, kernel: mul },
        OpDef { name: "sub", arity: 2, state: StateDecl::None, auto_state_update: true, kernel: sub },
        OpDef { name: "div", arity: 2, state: StateDecl::None, auto_state_update: true, kernel: div },
        OpDef { name: "mod", arity: 2, state: StateDecl::None, auto_state_update: true, kernel: mod_ },
        OpDef { name: "neg", arity: 1, state: StateDecl::None, auto_state_update: true, kernel: neg },
        OpDef { name: "abs", arity: 1, state: StateDecl::None, auto_state_update: true, kernel: abs },
        OpDef { name: "min", arity: 2, state: StateDecl::None, auto_state_update: true, kernel: min },
        OpDef { name: "max", arity: 2, state: StateDecl::None, auto_state_update: true, kernel: max },
        OpDef { name: "pow", arity: 2, state: StateDecl::None, auto_state_update: true, kernel: pow },
        OpDef { name: "sqrt", arity: 1, state: StateDecl::None, auto_state_update: true, kernel: sqrt },
        OpDef { name: "floor", arity: 1, state: StateDecl::None, auto_state_update: true, kernel: floor },
        OpDef { name: "ceil", arity: 1, state: StateDecl::None, auto_state_update: true, kernel: ceil },
    ]
}
