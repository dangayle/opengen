//! Data memory operators: peek, poke — read/write named data buffers.
//!
//! These operators operate on named data regions declared via `Data` or `Buffer`
//! declarations. Each data region is an array of f64 values allocated in the
//! graph's state arena. The `data_ref` field on the IR node carries the data name;
//! the compiler resolves it to an arena range.
//!
//! # D8 Scope (M2)
//! - Single channel only
//! - No interpolation (peek truncates index)
//! - `boundmode ignore` (the default): OOB peek returns 0.0; OOB poke writes nothing
//! - Replace-write: poke overwrites the sample (no overdub)
//!
//! # Divergence
//! gen~ peek/poke support multi-channel buffers with interpolation attributes
//! (`@interp linear/cubic/spline`), index modes (`@index phase/lookup/wave`),
//! bound modes (`@boundmode wrap/fold/clip`), channel modes, and overdub modes.
//! These are M3+ backlog items. M2 implements the minimal single-channel,
//! no-interpolation, boundmode-ignore / replace-write subset.
//!
//! # D10 Divergence
//! In opengen, `buffer` is an alias of `data` — they parse to the same NodeKind::Data
//! and there is no external host providing `buffer~` objects. gen~ distinguishes
//! `Data` (opengen-style internal array) from `Buffer` (reference to a host
//! `buffer~` object); the latter has string-argument syntax (`Buffer b("mybuf")`)
//! that references an external buffer by name. Since opengen has no host-provided
//! `buffer~` abstraction, string arguments to `Buffer` or `Data` are rejected at
//! lowering time with a clear error instead of silently defaulting to size 512.

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Read a value from a data buffer: `out = peek(name, index)`.
///
/// # Definition
/// Returns `state[trunc(index)]` if `0 <= trunc(index) < state.len()`,
/// otherwise returns `0.0`. No interpolation — truncates the index toward zero
/// via Rust's `as i64` cast.
///
/// # NaN Index
/// `NaN as i64` converts to `0` (per Rust's saturating cast semantics). This means
/// `peek(…, NaN)` reads slot 0 (in-bounds).
///
/// # M2 Conformance
/// - Indices in `(-1, 0)` truncate toward zero to `0`, which is in-bounds (reads slot 0).
/// - `NaN` index converts to `0` (reads slot 0). These behaviors are
///   conformance-verified in `#[test]` and should be validated against gen~ output
///   in the M2 conformance harness.
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_peek.maxref.xml`: first argument names a
/// data/buffer; second argument (inlet) is the sample index. Default `@boundmode`
/// is `ignore` (OOB returns 0). Default `@interp` is `none`.
///
/// # M2 Limitation
/// gen~ supports multiple peek attributes (@interp, @index, @boundmode, @channelmode,
/// @channels) that are not implemented in M2. See module-level docs for backlog.
///
/// # Determinism
/// Within-sample read ordering follows topological order with ascending-NodeId ties
/// (the graph-level determinism contract). A poke at an earlier-ordered node is
/// visible to a later-ordered peek in the same sample.
///
/// ```
/// use opengen_testkit::render_with_inputs;
/// // Round-trip: in1 = 42.0 poked at index 1; OOB peek at index 9 returns 0
/// let out = render_with_inputs(
///     "Data d(4); poke(d, in1, 1); out1 = peek(d, 1); out2 = peek(d, 9);",
///     48000.0,
///     &[&[42.0]],
/// );
/// assert_eq!(out.ch(0)[0], 42.0);
/// assert_eq!(out.ch(1)[0], 0.0);
/// ```
pub fn peek(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    let idx = inputs[0] as i64;
    if idx >= 0 && (idx as usize) < state.len() {
        state[idx as usize]
    } else {
        0.0
    }
}

/// Write a value into a data buffer: `poke(name, value, index)`.
///
/// # Definition
/// If `0 <= trunc(index) < state.len()`, sets `state[trunc(index)] = value`.
/// Otherwise, no write occurs (`boundmode ignore`). Always returns 0.0 (sink).
/// Truncation is toward zero via Rust's `as i64` cast.
///
/// # NaN Index
/// `NaN as i64` converts to `0` (per Rust's saturating cast semantics). This means
/// `poke(…, val, NaN)` writes slot 0 (in-bounds).
///
/// # M2 Conformance
/// - Indices in `(-1, 0)` truncate toward zero to `0`, which is in-bounds (writes slot 0).
/// - `NaN` index converts to `0` (writes slot 0). These behaviors are
///   conformance-verified in `#[test]` and should be validated against gen~ output
///   in the M2 conformance harness.
///
/// Replace-write: the new value overwrites the old value (no overdub/accum).
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_poke.maxref.xml`: first argument names a
/// data/buffer; first inlet (signal) is the value; second inlet is the position.
/// Default `@boundmode` is `ignore` (OOB writes nothing). Default `@overdubmode`
/// is `accum` in gen~; M2 uses replace-write (divergence documented below).
///
/// # Divergence
/// gen~ `poke` supports overdub modes (`@overdubmode accum` by default, `@overdubmode mix`)
/// and a separate overdub signal inlet. M2 uses replace-write (the tagged D8 scope)
/// — the new value replaces the old value unconditionally. Overdub is M3+.
///
/// # Determinism
/// Within-sample write ordering follows topological order with ascending-NodeId ties
/// (the graph-level determinism contract). A poke at an earlier-ordered node is
/// visible to a later-ordered peek in the same sample. Write-write ordering: if two
/// pokes target the same index, the poke with the higher NodeId (later in execution
/// order) wins.
///
/// ```
/// use opengen_testkit::render;
/// // Basic write: poke(d, 42.0, 1) writes 42 at index 1
/// let out = render("Data d(4); poke(d, 42.0, 1); out1 = peek(d, 1);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 42.0);
///
/// // OOB poke writes nothing
/// let out2 = render("Data d(4); poke(d, 99.0, 9); out1 = peek(d, 0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
///
/// // Multiple pokes to same index: last one wins (replace-write)
/// let out3 = render("Data d(4); poke(d, 1.0, 0); poke(d, 2.0, 0); out1 = peek(d, 0);", 48000.0, 1);
/// assert_eq!(out3.ch(0)[0], 2.0);
/// ```
pub fn poke(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    let value = inputs[0];
    let idx = inputs[1] as i64;
    if idx >= 0 && (idx as usize) < state.len() {
        state[idx as usize] = value;
    }
    0.0
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "peek",
            arity: 1,
            state: StateDecl::None,
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: peek,
        },
        OpDef {
            name: "poke",
            arity: 2,
            state: StateDecl::None,
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: poke,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_oob_negative_index() {
        let result = peek(&[-1.0], &mut [10.0, 20.0, 30.0], 48000.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn peek_oob_beyond_length() {
        let result = peek(&[5.0], &mut [10.0, 20.0, 30.0], 48000.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn peek_truncates_float_index() {
        let mut state = [10.0, 20.0, 30.0];
        assert_eq!(peek(&[0.9], &mut state, 48000.0), 10.0); // truncates to 0
        assert_eq!(peek(&[1.2], &mut state, 48000.0), 20.0); // truncates to 1
        assert_eq!(peek(&[1.99], &mut state, 48000.0), 20.0); // truncates to 1
    }

    #[test]
    fn poke_oob_negative_index_writes_nothing() {
        let mut state = [0.0; 4];
        poke(&[42.0, -1.0], &mut state, 48000.0);
        assert_eq!(state[0], 0.0);
    }

    #[test]
    fn poke_oob_beyond_length_writes_nothing() {
        let mut state = [0.0; 4];
        poke(&[42.0, 4.0], &mut state, 48000.0);
        assert_eq!(state[3], 0.0); // last element unchanged
    }

    #[test]
    fn poke_replace_write() {
        let mut state = [0.0, 0.0, 0.0];
        poke(&[10.0, 1.0], &mut state, 48000.0);
        assert_eq!(state[1], 10.0);
        // Write again to same index — replaces
        poke(&[20.0, 1.0], &mut state, 48000.0);
        assert_eq!(state[1], 20.0);
    }

    #[test]
    fn poke_returns_zero() {
        let mut state = [0.0; 4];
        let result = poke(&[42.0, 0.0], &mut state, 48000.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn peek_oob_on_negative_float_index() {
        // Large negative floats — truncated to i64, -1.5 → -1 (OOB), -0.1 → 0 (in bounds)
        assert_eq!(peek(&[-1.5], &mut [10.0], 48000.0), 0.0);
        assert_eq!(peek(&[-2.0], &mut [10.0], 48000.0), 0.0);
    }

    #[test]
    fn peek_negative_fraction_truncates_in_bounds() {
        // -0.5 as i64 → 0 (truncation toward zero), which is in-bounds for a 4-slot buffer
        let state = [42.0, 0.0, 0.0, 0.0];
        assert_eq!(peek(&[-0.5], &mut state.clone(), 48000.0), 42.0);
    }

    #[test]
    fn poke_negative_fraction_truncates_in_bounds() {
        // -0.5 as i64 → 0 (truncation toward zero), writes slot 0
        let mut state = [0.0; 4];
        poke(&[99.0, -0.5], &mut state, 48000.0);
        assert_eq!(state[0], 99.0);
        assert_eq!(state[1], 0.0);
    }

    #[test]
    fn peek_nan_index_reads_slot_zero() {
        // NaN as i64 → 0, reads slot 0
        let state = [42.0, 0.0];
        assert_eq!(peek(&[f64::NAN], &mut state.clone(), 48000.0), 42.0);
    }

    #[test]
    fn poke_nan_index_writes_slot_zero() {
        // NaN as i64 → 0, writes slot 0
        let mut state = [0.0; 4];
        poke(&[99.0, f64::NAN], &mut state, 48000.0);
        assert_eq!(state[0], 99.0);
        assert_eq!(state[1], 0.0);
    }

    #[test]
    fn poke_truncates_float_index() {
        let mut state = [0.0; 4];
        // 0.9 as i64 → 0
        poke(&[99.0, 0.9], &mut state, 48000.0);
        assert_eq!(state[0], 99.0);
        assert_eq!(state[1], 0.0);
    }

    #[test]
    fn poke_oob_beyond_length_entire_state_unchanged() {
        let mut state = [1.0, 2.0, 3.0, 4.0];
        let original = state;
        poke(&[42.0, 4.0], &mut state, 48000.0);
        assert_eq!(state, original, "entire state should be unchanged after OOB poke");
    }
}
