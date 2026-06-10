//! Range and mapping operators.
use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Clip (clamp) a value to a range: `out = clip(x, lo, hi)`.
///
/// # Definition
/// Returns lo if x < lo, hi if x > hi, otherwise x.
/// Boundary behavior: inclusive on both ends. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_clip.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = clip(1.5, 0.0, 1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = clip(0.0 - 0.5, 0.0, 1.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// // Boundary: exactly at high edge
/// let out3 = render("out1 = clip(1.0, 0.0, 1.0);", 48000.0, 1);
/// assert_eq!(out3.ch(0)[0], 1.0);
/// ```
pub fn clip(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    let lo = inputs[1];
    let hi = inputs[2];
    x.max(lo).min(hi)
}

/// Wrap a value to a range: `out = wrap(x, lo, hi)`.
///
/// # Definition
/// Wraps x into [lo, hi) — high bound is EXCLUSIVE.
/// Implements modulo-style wrapping: out = lo + (x - lo) % (hi - lo).
/// IEEE-754 f64. Works for negative inputs.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_wrap.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = wrap(1.25, 0.0, 1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.25);
/// // Boundary: exactly at high edge wraps to low
/// let out2 = render("out1 = wrap(1.0, 0.0, 1.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// // Negative input test
/// let out3 = render("out1 = wrap(0.0 - 0.25, 0.0, 1.0);", 48000.0, 1);
/// assert_eq!(out3.ch(0)[0], 0.75);
/// ```
pub fn wrap(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    let lo = inputs[1];
    let hi = inputs[2];
    let range = hi - lo;
    if range == 0.0 {
        return lo;
    }
    let offset = (x - lo) % range;
    // Handle negative modulo: ensure result is always in [lo, hi)
    if offset < 0.0 {
        lo + offset + range
    } else {
        lo + offset
    }
}

/// Fold a value to a range: `out = fold(x, lo, hi)`.
///
/// # Definition
/// Triangle-wave reflection: values beyond range fold back.
/// If x exceeds hi, reflects back down; if below lo, reflects back up.
/// Implements sawtooth-to-triangle conversion. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_fold.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = fold(1.25, 0.0, 1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.75);
/// let out2 = render("out1 = fold(0.0 - 0.25, 0.0, 1.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.25);
/// ```
pub fn fold(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    let lo = inputs[1];
    let hi = inputs[2];
    let range = hi - lo;
    if range == 0.0 {
        return lo;
    }
    
    // First wrap to [lo, hi + range)
    let mut v = x - lo;
    let double_range = 2.0 * range;
    v = v % double_range;
    if v < 0.0 {
        v += double_range;
    }
    
    // Now fold: if v > range, reflect back
    if v > range {
        lo + (double_range - v)
    } else {
        lo + v
    }
}

/// Linear scale (map) from input range to output range: `out = scale(x, inlo, inhi, outlo, outhi)`.
///
/// # Definition
/// Linear interpolation: outlo + (x - inlo) / (inhi - inlo) * (outhi - outlo).
/// No clamping — extrapolates beyond input range. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_scale.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = scale(0.5, 0.0, 1.0, 0.0, 10.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 5.0);
/// // Extrapolation test
/// let out2 = render("out1 = scale(2.0, 0.0, 1.0, 0.0, 10.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 20.0);
/// ```
pub fn scale(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let x = inputs[0];
    let inlo = inputs[1];
    let inhi = inputs[2];
    let outlo = inputs[3];
    let outhi = inputs[4];
    
    let in_range = inhi - inlo;
    if in_range == 0.0 {
        return outlo;
    }
    
    let normalized = (x - inlo) / in_range;
    outlo + normalized * (outhi - outlo)
}

/// Linear interpolation (mix): `out = mix(a, b, t)` = a + t * (b - a).
///
/// # Definition
/// Linear blend between a and b. t=0 returns a, t=1 returns b.
/// No clamping on t — extrapolates for t < 0 or t > 1. IEEE-754 f64.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_mix.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = mix(0.0, 10.0, 0.25);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 2.5);
/// let out2 = render("out1 = mix(0.0, 10.0, 0.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// let out3 = render("out1 = mix(0.0, 10.0, 1.0);", 48000.0, 1);
/// assert_eq!(out3.ch(0)[0], 10.0);
/// ```
pub fn mix(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let a = inputs[0];
    let b = inputs[1];
    let t = inputs[2];
    a + t * (b - a)
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef { name: "clip", arity: 3, state: StateDecl::None, kernel: clip },
        OpDef { name: "wrap", arity: 3, state: StateDecl::None, kernel: wrap },
        OpDef { name: "fold", arity: 3, state: StateDecl::None, kernel: fold },
        OpDef { name: "scale", arity: 5, state: StateDecl::None, kernel: scale },
        OpDef { name: "mix", arity: 3, state: StateDecl::None, kernel: mix },
    ]
}
