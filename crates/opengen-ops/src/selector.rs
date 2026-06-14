//! `selector` operator — float-threshold signal selection.
//!
//! # Definition
//!
//! `selector3(index, in1, in2)` selects one of N signals based on a float
//! threshold: output = in1 when index < 1.0, output = in2 when index ≥ 1.0.
//!
//! # Documented
//!
//! `reference/gen/refpages/common/gen_common_selector.maxref.xml`
//!
//! # Vendor
//!
//! `reference/rnbo/operators/selector.js`
//!
//! Float threshold comparison (NOT integer-indexed):
//! - index < 0 → in1
//! - index < 1 → in1
//! - index ≥ 1 → in2
//! - index ≥ 2 → in3 (selector5 only)
//! - etc. Out-of-range: index ≥ N-1 → last input.
//!
//! # Extension
//!
//! opengen registers `selector3` and `selector5` as fixed-arity operators
//! because the IR currently requires a fixed `arity` per OpDef. Full
//! variable-arity `selector` is deferred to M4.
//!
//! ```
//! use opengen_testkit::render;
//! let out = render("out1 = selector3(in1, 10, 20);", 48_000.0, 1);
//! // in1 defaults to 0.0 → index < 1 → select in1_pos=1 (10)
//! assert_eq!(out.ch(0)[0], 10.0);
//! ```

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// `selector3(index, in1, in2)`: single-threshold selector.
pub fn selector3(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let index = inputs[0];
    if index < 1.0 {
        inputs[1]
    } else {
        inputs[2]
    }
}

/// `selector5(index, in1, in2, in3, in4)`: 4-signal selector.
pub fn selector5(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let index = inputs[0];
    if index < 1.0 {
        inputs[1]
    } else if index < 2.0 {
        inputs[2]
    } else if index < 3.0 {
        inputs[3]
    } else {
        inputs[4]
    }
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "selector3",
            arity: 3,
            state: StateDecl::None,
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: selector3,
            cpp_kernel: None,
            emit_cpp_call: None,
        },
        OpDef {
            name: "selector5",
            arity: 5,
            state: StateDecl::None,
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: selector5,
            cpp_kernel: None,
            emit_cpp_call: None,
        },
    ]
}
