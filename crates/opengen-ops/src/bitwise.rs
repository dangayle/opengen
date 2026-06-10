//! Bitwise operators (opengen extension).
//!
//! The GenExpr language supports bitwise operations via `&`, `|`, `^`, `<<`, `>>`.
//! Since the dataflow graph operates on `f64` values, these operators:
//! 1. Truncate each input toward zero via `trunc()`.
//! 2. Convert to `i64` (saturating on overflow per Rust's `as i64` semantics).
//!    Note: `NaN as i64` yields 0 in Rust — deterministic, not undefined behavior.
//! 3. Perform the bitwise operation.
//! 4. Convert the result back to `f64`.
//!
//! Shift counts are masked to the range `0..63` (`& 63`), matching hardware behavior
//! and the GenExpr specification (per gen~ docs: shift count is masked to 6 bits).
//! Negative shift counts (e.g., `-1`) are converted via `as u64` which wraps around
//! in two's complement (e.g., `-1i64 as u64` = 18446744073709551615, masked to 63).

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Bitwise AND: `out = a & b`.
///
/// # Definition
/// Converts inputs to `i64` via truncation, performs `i64` bitwise AND, converts back to `f64`.
/// Equivalent to: `(trunc(a) as i64) & (trunc(b) as i64) as f64`.
///
/// # Documented
/// gen~ GenExpr guide (operator reference): `&` is the bitwise AND operator.
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = 5 & 3;", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = 6 & 2;", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 2.0);
/// ```
pub fn bitand(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    (inputs[0].trunc() as i64 & inputs[1].trunc() as i64) as f64
}

/// Bitwise OR: `out = a | b`.
///
/// # Definition
/// Converts inputs to `i64` via truncation, performs `i64` bitwise OR, converts back to `f64`.
/// Equivalent to: `(trunc(a) as i64) | (trunc(b) as i64) as f64`.
///
/// # Documented
/// gen~ GenExpr guide (operator reference): `|` is the bitwise OR operator.
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = 1 | 2;", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 3.0);
/// let out2 = render("out1 = 0 | 255;", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 255.0);
/// ```
pub fn bitor(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    (inputs[0].trunc() as i64 | inputs[1].trunc() as i64) as f64
}

/// Bitwise XOR: `out = a ^ b`.
///
/// # Definition
/// Converts inputs to `i64` via truncation, performs `i64` bitwise XOR, converts back to `f64`.
/// Equivalent to: `(trunc(a) as i64) ^ (trunc(b) as i64) as f64`.
///
/// # Documented
/// gen~ GenExpr guide (operator reference): `^` is the bitwise XOR operator.
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = 3 ^ 5;", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 6.0);
/// let out2 = render("out1 = 255 ^ 1;", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 254.0);
/// ```
pub fn bitxor(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    (inputs[0].trunc() as i64 ^ inputs[1].trunc() as i64) as f64
}

/// Bitwise left shift: `out = a << b`.
///
/// # Definition
/// Converts inputs to `i64` via truncation, shifts `a` left by `(b & 63)` bits.
/// Shift count is masked to 6 bits (0-63) per gen~ spec. A negative shift count
/// wraps through `i64 as u64` (e.g., `-1` → 18446744073709551615 → `& 63` = 63).
/// Converts result back to `f64`.
///
/// # Documented
/// gen~ GenExpr guide (operator reference): `<<` is the left shift operator.
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = 1 << 3;", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 8.0);
/// let out2 = render("out1 = 1 << 64;", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 1.0); // masked: 64 & 63 = 0
/// ```
pub fn shl(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let count = (inputs[1].trunc() as u64) & 63;
    (inputs[0].trunc() as i64).wrapping_shl(count as u32) as f64
}

/// Bitwise right shift: `out = a >> b`.
///
/// # Definition
/// Converts inputs to `i64` via truncation, shifts `a` right by `(b & 63)` bits
/// with sign extension (arithmetic shift). Shift count is masked to 6 bits (0-63).
/// A negative shift count wraps through `i64 as u64` (e.g., `-1` → 18446744073709551615
/// → `& 63` = 63). Converts result back to `f64`.
///
/// # Documented
/// gen~ GenExpr guide (operator reference): `>>` is the right shift operator.
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = 8 >> 3;", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// // Sign-extending (arithmetic) shift
/// let out2 = render("out1 = -8 >> 2;", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], -2.0);
/// ```
pub fn shr(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let count = (inputs[1].trunc() as u64) & 63;
    (inputs[0].trunc() as i64).wrapping_shr(count as u32) as f64
}

/// Sample rate value: `out = samplerate`.
///
/// # Definition
/// Returns the current sample rate as a constant `f64` value.
/// Arity 0 (takes no inputs). In GenExpr, `samplerate` is a builtin identifier
/// that resolves to the audio sample rate.
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_samplerate.maxref.xml`
///
/// ```
/// use opengen_ops::bitwise::samplerate_op;
/// assert_eq!(samplerate_op(&[], &mut [], 48000.0), 48000.0);
/// assert_eq!(samplerate_op(&[], &mut [], 96000.0), 96000.0);
/// ```
pub fn samplerate_op(_inputs: &[f64], _state: &mut [f64], sr: f64) -> f64 {
    sr
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef { name: "bitand", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: bitand },
        OpDef { name: "bitor", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: bitor },
        OpDef { name: "bitxor", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: bitxor },
        OpDef { name: "shl", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: shl },
        OpDef { name: "shr", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: shr },
        OpDef { name: "samplerate", arity: 0, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: samplerate_op },
    ]
}

#[cfg(test)]
mod tests {
    use crate::bitwise::*;

    #[test]
    fn bitand_negative_values() {
        // -1 & 3 = 3 (in two's complement, -1 is all 1s)
        assert_eq!(bitand(&[-1.0, 3.0], &mut [], 48000.0), 3.0);
    }

    #[test]
    fn bitor_result() {
        assert_eq!(bitor(&[4.0, 2.0], &mut [], 48000.0), 6.0);
    }

    #[test]
    fn bitxor_basic() {
        assert_eq!(bitxor(&[7.0, 5.0], &mut [], 48000.0), 2.0);
    }

    #[test]
    fn shl_large_shift_masked() {
        assert_eq!(shl(&[1.0, 66.0], &mut [], 48000.0), 4.0); // 66 & 63 = 2
    }

    #[test]
    fn shr_arithmetic() {
        // -8 >> 2 = -2 (sign-extending)
        assert_eq!(shr(&[-8.0, 2.0], &mut [], 48000.0), -2.0);
    }
}
