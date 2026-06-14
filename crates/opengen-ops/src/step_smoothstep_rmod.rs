//! `step`, `smoothstep`, `rmod` — misc gen~ operators.
//!
//! # Documented
//!
//! `reference/gen/refpages/common/gen_common_step.maxref.xml`
//! `reference/gen/refpages/common/gen_common_smoothstep.maxref.xml`

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// `step(threshold, x)`: Heaviside step. Returns 1.0 when x ≥ threshold, 0.0 otherwise.
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = step(0.5, 1.0);", 48_000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// ```
pub fn step(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[1] >= inputs[0] { 1.0 } else { 0.0 }
}

/// `smoothstep(low, high, x)`: Hermite interpolation.
/// t = clamp((x - low) / (high - low), 0, 1); return t*t*(3 - 2*t).
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = smoothstep(0, 1, 0.5);", 48_000.0, 1);
/// assert!((out.ch(0)[0] - 0.5).abs() < 1e-9);
/// ```
pub fn smoothstep(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let t = ((inputs[2] - inputs[0]) / (inputs[1] - inputs[0])).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// `rmod(a, b)`: Reverse mod — computes b % a (swapped argument order).
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = rmod(3, 8);", 48_000.0, 1);
/// assert_eq!(out.ch(0)[0], 2.0); // 8 % 3 = 2
/// ```
pub fn rmod(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[1] % inputs[0]
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "step",
            arity: 2,
            state: StateDecl::None,
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: step,
            cpp_kernel: None,
            emit_cpp_call: None,
        },
        OpDef {
            name: "smoothstep",
            arity: 3,
            state: StateDecl::None,
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: smoothstep,
            cpp_kernel: None,
            emit_cpp_call: None,
        },
        OpDef {
            name: "rmod",
            arity: 2,
            state: StateDecl::None,
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: rmod,
            cpp_kernel: None,
            emit_cpp_call: None,
        },
    ]
}
