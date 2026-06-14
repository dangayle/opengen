//! `elapsed` operator — time in milliseconds since trigger was non-zero.
//!
//! # Definition
//!
//! `elapsed(trigger)` — when trigger > 0, reset counter and output 0.
//! When trigger ≤ 0, output = counter * (1000.0 / sr), increment counter.
//!
//! # Documented
//!
//! `reference/gen/refpages/common/gen_common_elapsed.maxref.xml`
//!
//! ```
//! use opengen_testkit::render;
//! let src = "out1 = elapsed(0);"; // trigger always 0, counter runs
//! let out = render(src, 1_000.0, 3);
//! // Sample 0: state 0→0, output 0.0, state becomes 1
//! // Sample 1: state 1→1.0 ms, state becomes 2
//! // Sample 2: state 2→2.0 ms
//! assert!((out.ch(0)[2] - 2.0).abs() < 1e-9);
//! ```

use crate::registry::OpDef;
use opengen_ir::StateDecl;

pub fn elapsed(inputs: &[f64], state: &mut [f64], sr: f64) -> f64 {
    if inputs[0] > 0.0 {
        state[0] = 0.0;
        return 0.0;
    }
    let ms_per_sample = 1000.0 / sr;
    let result = state[0] * ms_per_sample;
    state[0] += 1.0;
    result
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "elapsed",
            arity: 1,
            state: StateDecl::Slots(1),
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: elapsed,
        },
    ]
}
