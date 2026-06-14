//! Type conversion, DSP conversion, and miscellaneous operators.
//!
//! Determinism note: Bit-identical output on a given platform/std version. Cross-platform
//! bit-identity for transcendentals is tracked as an M3 emitter concern.

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Sign function: `out = sign(x)`.
///
/// # Definition
/// Returns 1.0 if x > 0, -1.0 if x < 0, x itself if x == 0 (per refpage: "zero returns itself").
/// IEEE-754 f64; preserves signed zero.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_sign.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = sign(0.0 - 3.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], -1.0);
/// let out2 = render("out1 = sign(0.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// ```
pub fn sign(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        x
    }
}

/// Fractional part: `out = fract(x)`.
///
/// # Definition
/// Returns `x - floor(x)`. Always non-negative. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_fract.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = fract(1.25);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.25);
/// let out2 = render("out1 = fract(0.0 - 0.25);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.75);
/// ```
pub fn fract(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    x - x.floor()
}

/// Truncate toward zero: `out = trunc(x)`.
///
/// # Definition
/// Removes fractional part, rounding toward zero. Delegates to `f64::trunc()`. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_trunc.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = trunc(0.0 - 1.7);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], -1.0);
/// ```
pub fn trunc(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].trunc()
}

/// Absolute difference: `out = absdiff(a, b)`.
///
/// # Definition
/// Returns `abs(a - b)`. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_absdiff.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = absdiff(2.0, 5.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 3.0);
/// ```
pub fn absdiff(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    (inputs[0] - inputs[1]).abs()
}

/// Hyperbolic sine: `out = sinh(x)`.
///
/// # Definition
/// Delegates to `f64::sinh(x)`. IEEE-754 f64 hyperbolic sine.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_sinh.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = sinh(0.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.5f64.sinh());
/// ```
pub fn sinh(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].sinh()
}

/// Hyperbolic cosine: `out = cosh(x)`.
///
/// # Definition
/// Delegates to `f64::cosh(x)`. IEEE-754 f64 hyperbolic cosine.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_cosh.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = cosh(0.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.5f64.cosh());
/// ```
pub fn cosh(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].cosh()
}

/// Hyperbolic tangent: `out = tanh(x)`.
///
/// # Definition
/// Delegates to `f64::tanh(x)`. IEEE-754 f64 hyperbolic tangent.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_tanh.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = tanh(0.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.5f64.tanh());
/// ```
pub fn tanh(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].tanh()
}

/// Inverse hyperbolic sine: `out = asinh(x)`.
///
/// # Definition
/// Delegates to `f64::asinh(x)`. IEEE-754 f64 inverse hyperbolic sine.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_asinh.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = asinh(0.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.5f64.asinh());
/// ```
pub fn asinh(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].asinh()
}

/// Inverse hyperbolic cosine: `out = acosh(x)`.
///
/// # Definition
/// Delegates to `f64::acosh(x)`. IEEE-754 f64 inverse hyperbolic cosine.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_acosh.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = acosh(1.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.5f64.acosh());
/// ```
pub fn acosh(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].acosh()
}

/// Inverse hyperbolic tangent: `out = atanh(x)`.
///
/// # Definition
/// Delegates to `f64::atanh(x)`. IEEE-754 f64 inverse hyperbolic tangent.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_atanh.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = atanh(0.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.5f64.atanh());
/// ```
pub fn atanh(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].atanh()
}

/// Convert radians to degrees: `out = degrees(x)`.
///
/// # Definition
/// Multiplies by 180/π. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_degrees.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = degrees(3.141592653589793);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 180.0);
/// ```
pub fn degrees(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].to_degrees()
}

/// Convert degrees to radians: `out = radians(x)`.
///
/// # Definition
/// Multiplies by π/180. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_radians.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = radians(180.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], std::f64::consts::PI);
/// ```
pub fn radians(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].to_radians()
}

/// MIDI note to frequency: `out = mtof(note, tuning)`.
///
/// # Definition
/// Returns `tuning * 2^((note - 69) / 12)`. Inputs: (note, tuning). Default tuning: 440 Hz (A4).
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_mtof.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = mtof(69.0, 440.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 440.0);
/// let out2 = render("out1 = mtof(81.0, 440.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 880.0);
/// ```
pub fn mtof(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let note = inputs[0];
    let tuning = inputs[1];
    tuning * 2f64.powf((note - 69.0) / 12.0)
}

/// Frequency to MIDI note: `out = ftom(freq, tuning)`.
///
/// # Definition
/// Returns `69 + 12 * log2(freq / tuning)`. Inputs: (freq, tuning). Default tuning: 440 Hz (A4).
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_ftom.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = ftom(440.0, 440.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 69.0);
/// ```
pub fn ftom(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let freq = inputs[0];
    let tuning = inputs[1];
    69.0 + 12.0 * (freq / tuning).log2()
}

/// Decibels to amplitude: `out = dbtoa(db)`.
///
/// # Definition
/// Returns `10^(db / 20)`. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_dbtoa.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = dbtoa(0.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = dbtoa(0.0 - 6.0);", 48000.0, 1);
/// let expected = 0.5011872336272722;
/// assert!((out2.ch(0)[0] - expected).abs() < 1e-15);
/// ```
pub fn dbtoa(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    10f64.powf(inputs[0] / 20.0)
}

/// Amplitude to decibels: `out = atodb(a)`.
///
/// # Definition
/// Returns `20 * log10(a)`. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_atodb.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = atodb(1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.0);
/// ```
pub fn atodb(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    20.0 * inputs[0].log10()
}

/// Milliseconds to samples: `out = mstosamps(ms)`.
///
/// # Definition
/// Returns `ms * sr / 1000`. Sample-rate dependent. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_mstosamps.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = mstosamps(1000.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 48000.0);
/// ```
pub fn mstosamps(inputs: &[f64], _state: &mut [f64], sr: f64) -> f64 {
    inputs[0] * sr / 1000.0
}

/// Samples to milliseconds: `out = sampstoms(samps)`.
///
/// # Definition
/// Returns `samps * 1000 / sr`. Sample-rate dependent. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_sampstoms.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = sampstoms(48.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// ```
pub fn sampstoms(inputs: &[f64], _state: &mut [f64], sr: f64) -> f64 {
    inputs[0] * 1000.0 / sr
}

/// Reverse subtraction: `out = rsub(a, b)` = b - a.
///
/// # Definition
/// Subtracts first input from second. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_rsub.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = rsub(1.0, 5.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 4.0);
/// ```
pub fn rsub(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[1] - inputs[0]
}

/// Reverse division: `out = rdiv(a, b)` = b / a.
///
/// # Definition
/// Divides second input by first. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_rdiv.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = rdiv(2.0, 10.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 5.0);
/// ```
pub fn rdiv(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[1] / inputs[0]
}

/// Conditional switch: `out = switch(cond, iftrue, iffalse)`.
///
/// # Definition
/// Returns `iftrue` if `cond != 0`, otherwise `iffalse`. Both value inputs are evaluated
/// every sample (eager evaluation per dataflow semantics). IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_switch.maxref.xml`
///
/// # Definition
/// D5 design decision: ternary `?:` and logical `&&`/`||` lower to `switch`/`and`/`or`;
/// both branches/operands are always evaluated (dataflow semantics).
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = switch(1.0, 2.5, 3.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 2.5);
/// let out2 = render("out1 = switch(0.0, 2.5, 3.5);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 3.5);
/// ```
pub fn switch(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] != 0.0 {
        inputs[1]
    } else {
        inputs[2]
    }
}

/// Round to nearest multiple: `out = round(x, base)`.
///
/// # Definition
/// Rounds `x` to nearest multiple of `base`, halfway cases away from zero (per refpage).
/// If `base <= 0`, returns `x` unchanged (opengen decision; conformance TBD).
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_round.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = round(2.5, 1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 3.0);
/// let out2 = render("out1 = round(0.0 - 2.5, 1.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], -3.0);
/// let out3 = render("out1 = round(0.3, 0.25);", 48000.0, 1);
/// assert_eq!(out3.ch(0)[0], 0.25);
/// ```
pub fn round(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    let base = inputs[1];
    if base <= 0.0 {
        return x;
    }
    let ratio = x / base;
    let rounded = if ratio >= 0.0 {
        (ratio + 0.5).floor()
    } else {
        (ratio - 0.5).ceil()
    };
    rounded * base
}

/// Integer truncation: `out = int(x)`.
///
/// # Definition
/// Truncates toward zero. Refpage states "convert to integer" without specifying floor vs trunc;
/// implementing as truncation (toward zero) per common DSP convention. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_int.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = int(1.7);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = int(0.0 - 1.7);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], -1.0);
/// ```
pub fn int(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0].trunc()
}

/// Boolean conversion: `out = bool(x)`.
///
/// # Definition
/// Returns 1.0 if `x != 0`, otherwise 0.0. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_bool.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = bool(0.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = bool(0.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// ```
pub fn bool(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] != 0.0 { 1.0 } else { 0.0 }
}

/// Fix NaN: `out = fixnan(x)`.
///
/// # Definition
/// Returns 0.0 if `x` is NaN, otherwise `x`. IEEE-754 f64.
///
/// # Vendor
/// `reference/genlib/gen_dsp/genlib_ops.h`: `fixnan(v) = (v != v) ? 0.0 : v`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = fixnan(2.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 2.5);
/// ```
pub fn fixnan(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let v = inputs[0];
    if v.is_nan() { 0.0 } else { v }
}

/// Fix denormals: `out = fixdenorm(x)`.
///
/// # Definition
/// Returns 0.0 if `x` is a denormal (subnormal) number, otherwise `x`. IEEE-754 f64.
/// Denormal: `x != 0 && abs(x) < f64::MIN_POSITIVE`.
///
/// # Vendor
/// `reference/genlib/gen_dsp/genlib_ops.h`: `fixdenorm(v) = (v != 0 && fabs(v) < DBL_MIN) ? 0.0 : v`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = fixdenorm(1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// ```
pub fn fixdenorm(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let v = inputs[0];
    if v != 0.0 && v.abs() < f64::MIN_POSITIVE {
        0.0
    } else {
        v
    }
}

/// Triangle wave from phase and duty: `out = triangle(phase, duty)`.
///
/// # Definition
/// Generates unipolar triangle wave [0, 1] from phase [0, 1) and duty cycle [0, 1].
/// Phase is wrapped to [0, 1), duty is clamped to [0, 1].
/// Rising edge: `phase / duty` for `phase < duty` (or 0 if duty == 0).
/// Falling edge: `1 - (phase - duty) / (1 - duty)` after peak (or phase if duty == 1).
///
/// # Vendor
/// `reference/genlib/gen_dsp/genlib_ops.h`: paraphrased behavior
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = triangle(0.25, 0.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.5);
/// let out2 = render("out1 = triangle(0.75, 0.5);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.5);
/// let out3 = render("out1 = triangle(0.5, 0.5);", 48000.0, 1);
/// assert_eq!(out3.ch(0)[0], 1.0);
/// ```
pub fn triangle(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let phase_raw = inputs[0];
    let duty_raw = inputs[1];
    
    // Wrap phase to [0, 1)
    let phase = phase_raw - phase_raw.floor();
    
    // Clamp duty to [0, 1]
    let duty = duty_raw.clamp(0.0, 1.0);
    
    if phase < duty {
        if duty == 0.0 {
            0.0
        } else {
            phase / duty
        }
    } else if duty == 1.0 {
        phase
    } else {
        1.0 - (phase - duty) / (1.0 - duty)
    }
}

/// Logical AND: `out = and(a, b)`.
///
/// # Definition
/// Returns 1.0 if both `a != 0` and `b != 0`, otherwise 0.0.
/// Eager evaluation: both inputs always evaluated. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_and.maxref.xml`
///
/// # Definition
/// D5 design decision: `&&` lowers to `and`; both operands always evaluated (dataflow).
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = and(2.0, 3.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = and(2.0, 0.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// ```
pub fn and(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] != 0.0 && inputs[1] != 0.0 { 1.0 } else { 0.0 }
}

/// Logical OR: `out = or(a, b)`.
///
/// # Definition
/// Returns 1.0 if `a != 0` or `b != 0`, otherwise 0.0.
/// Eager evaluation: both inputs always evaluated. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_or.maxref.xml`
///
/// # Definition
/// D5 design decision: `||` lowers to `or`; both operands always evaluated (dataflow).
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = or(0.0, 3.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = or(0.0, 0.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// ```
pub fn or(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] != 0.0 || inputs[1] != 0.0 { 1.0 } else { 0.0 }
}

/// Logical NOT: `out = not(x)`.
///
/// # Definition
/// Returns 1.0 if `x == 0`, otherwise 0.0. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_not.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = not(0.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = not(2.5);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// ```
pub fn not(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] == 0.0 { 1.0 } else { 0.0 }
}

/// Logical XOR: `out = xor(a, b)`.
///
/// # Definition
/// Returns 1.0 if exactly one of `a` or `b` is nonzero, otherwise 0.0.
/// Implemented as `(a != 0) != (b != 0)`. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_xor.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = xor(1.0, 0.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = xor(1.0, 1.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// ```
pub fn xor(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if (inputs[0] != 0.0) != (inputs[1] != 0.0) { 1.0 } else { 0.0 }
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef { name: "sign", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: sign, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "fract", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: fract, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "trunc", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: trunc, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "absdiff", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: absdiff, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "sinh", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: sinh, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "cosh", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: cosh, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "tanh", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: tanh, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "asinh", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: asinh, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "acosh", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: acosh, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "atanh", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: atanh, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "degrees", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: degrees, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "radians", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: radians, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "mtof", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: mtof, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "ftom", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: ftom, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "dbtoa", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: dbtoa, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "atodb", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: atodb, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "mstosamps", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: mstosamps, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "sampstoms", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: sampstoms, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "rsub", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: rsub, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "rdiv", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: rdiv, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "switch", arity: 3, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: switch, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "round", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: round, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "int", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: int, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "bool", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: bool, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "fixnan", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: fixnan, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "fixdenorm", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: fixdenorm, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "triangle", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: triangle, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "and", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: and, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "or", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: or, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "not", arity: 1, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: not, cpp_kernel: None, emit_cpp_call: None },
        OpDef { name: "xor", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: xor, cpp_kernel: None, emit_cpp_call: None },
        // Alias: clamp -> clip
        OpDef { name: "clamp", arity: 3, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: crate::range::clip, cpp_kernel: None, emit_cpp_call: None },
    ]
}

#[cfg(test)]
mod tests {
    use opengen_testkit::render;
    use super::fixdenorm;

    #[test]
    fn fixnan_converts_nan_to_zero() {
        let out = render("out1 = fixnan(0.0 / 0.0);", 48000.0, 1);
        assert_eq!(out.ch(0)[0], 0.0);
    }

    #[test]
    fn fixdenorm_converts_denorm_to_zero() {
        // Use direct kernel test since genexpr doesn't parse scientific notation
        let denorm = 1e-320; // A denormal number
        let result = fixdenorm(&[denorm], &mut [], 48000.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn inverse_hyperbolic_domain_errors_propagate_nan() {
        assert!(render("out1 = acosh(0.5);", 48_000.0, 1).ch(0)[0].is_nan());
        assert!(render("out1 = atanh(2);", 48_000.0, 1).ch(0)[0].is_nan());
        assert!(render("out1 = atanh(1);", 48_000.0, 1).ch(0)[0].is_infinite());
        assert!(render("out1 = atodb(0 - 1);", 48_000.0, 1).ch(0)[0].is_nan());
        let ftom_out = render("out1 = ftom(0, 440);", 48_000.0, 1).ch(0)[0];
        assert!(ftom_out.is_infinite() || ftom_out.is_nan());
    }
}
