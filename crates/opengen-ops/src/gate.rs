//! `gate` operator — sample-and-hold gate.
//!
//! # Definition
//!
//! `gate(trigger, input)` passes through the input when trigger > 0, and
//! holds the last passed value when trigger ≤ 0 (sample-and-hold).
//!
//! # Documented
//!
//! `reference/gen/refpages/common/gen_common_gate.maxref.xml`
//!
//! ```
//! use opengen_testkit::render;
//! // Render with phasor driving gate
//! let src = "t = phasor(1); g = gate(t > 0.5, t); out1 = g;";
//! let out = render(src, 48_000.0, 48_000);
//! // Gate holds the value when trigger ≤ 0.5
//! assert!(out.ch(0)[24_000] > 0.49); // near peak, held during closed phase
//! ```

use crate::registry::OpDef;
use opengen_ir::StateDecl;

pub fn gate(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] > 0.0 {
        state[0] = inputs[1];
    }
    state[0]
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "gate",
            arity: 2,
            state: StateDecl::Slots(1),
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: gate,
            cpp_kernel: None,
            emit_cpp_call: None,
        },
    ]
}
