//! `wave` operator — wavetable oscillator with linear interpolation.
//!
//! # Definition
//!
//! `wave(phase, data)` reads from a Data buffer at normalised phase 0.0–1.0
//! with linear interpolation between adjacent samples. The data buffer is
//! resolved at compile time via `data_ref` (same mechanism as `peek`/`poke`).
//!
//! # Documented
//!
//! `reference/gen/refpages/common/gen_common_wave.maxref.xml`
//!
//! ```
//! use opengen_testkit::render;
//! // wave phase=0 reads the first sample of the data buffer
//! let src = "Data d(4); out1 = wave(d, 0);";
//! let out = render(src, 48_000.0, 1);
//! // First sample of uninitialized buffer is 0.0
//! assert_eq!(out.ch(0)[0], 0.0);
//! ```

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Wavetable oscillator with linear interpolation.
/// `state` is the Data buffer; `inputs[0]` is phase (0.0–1.0).
/// The data name is extracted by the lowerer (same as peek/poke).
pub fn wave(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    let phase = inputs[0];
    let len = state.len() as f64;
    if len == 0.0 {
        return 0.0;
    }
    let idx_f = phase * len;
    let idx0 = (idx_f as usize) % state.len();
    let idx1 = (idx0 + 1) % state.len();
    let frac = idx_f - idx_f.floor();
    state[idx0] + frac * (state[idx1] - state[idx0])
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "wave",
            arity: 1,
            state: StateDecl::None,
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: wave,
        },
    ]
}
