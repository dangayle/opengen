//! Oscillators: signal generators with internal phase state.

use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Ramp oscillator. Outputs a sawtooth wave 0..1 at the given frequency.
///
/// # Definition
/// `y[n] = wrap(y[n-1] + freq/sr, 0, 1)`, `y[0] = 0.0`. StateDecl::Slots(1), arity 1.
///
/// The kernel outputs the pre-increment phase value, ensuring `y[0] == 0.0` exactly:
/// `out = state; state = wrap(state + freq/sr, 0, 1)`.
///
/// # Vendor
/// `reference/rnbo/operators/phasor.js`
///
/// RNBO's implementation emits the pre-increment value and applies wrapping before
/// the increment step (via conditional bounds checks).
///
/// # Observed
/// This is an open conformance question vs real gen~, to be verified by the M2
/// conformance harness (open item).
///
/// ```
/// use opengen_testkit::render;
/// // Exact ramp at 1000 Hz / 48000 sr: samples are 0, 1000/48000, 2000/48000...
/// let out = render("out1 = phasor(1000);", 48000.0, 3);
/// assert_eq!(out.ch(0), &[0.0, 1000.0/48000.0, 2000.0/48000.0]);
/// ```
pub fn phasor(inputs: &[f64], state: &mut [f64], sr: f64) -> f64 {
    let freq = inputs[0];
    let phase = state[0];
    
    // Output the current phase (pre-increment)
    let output = phase;
    
    // Advance phase with wrapping
    let mut next_phase = phase + freq / sr;
    
    // Wrap to [0, 1) for ANY finite increment (handles |freq/sr| >= 1.0)
    // x - floor(x) maps any value to [0, 1); for in-range values floor(x)==0.0
    // so this is exact and doesn't perturb the existing doctest expectations.
    next_phase -= next_phase.floor();
    
    // Guard: floating-point edge case where result could be exactly 1.0
    // (e.g., tiny negative values like -1e-17 → -1e-17 + 1.0 may round to 1.0)
    if next_phase >= 1.0 {
        next_phase = 0.0;
    }
    
    state[0] = next_phase;
    output
}

/// Sine oscillator. Outputs a sine wave at the given frequency using f64::sin.
///
/// # Definition
/// `sin(2π · phase)` where phase advances like phasor. Slots(1), arity 1.
///
/// The phase state advances exactly as in phasor: `phase[n] = wrap(phase[n-1] + freq/sr, 0, 1)`.
///
/// # Divergence
/// gen~ uses an interpolated wavetable for cycle; we use `f64::sin` directly.
/// Rationale: exactness and determinism. Conformance tolerance will be handled
/// by the M2 harness.
///
/// # Documented
/// `reference/gen/refpages/dsp/gen_dsp_cycle.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// // First sample is exactly 0.0 (phase = 0 → sin(0) = 0)
/// let out = render("out1 = cycle(1000);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 0.0);
///
/// // Quarter period at freq=12000, sr=48000: quarter period = sr/(4*freq) = 1 sample
/// // At sample 1, phase = 12000/48000 = 0.25 → sin(2π * 0.25) = sin(π/2) = 1.0
/// let out2 = render("out1 = cycle(12000);", 48000.0, 2);
/// let val = out2.ch(0)[1];
/// assert!((val - 1.0).abs() <= f64::EPSILON, "Expected ~1.0, got {}", val);
/// ```
pub fn cycle(inputs: &[f64], state: &mut [f64], sr: f64) -> f64 {
    let freq = inputs[0];
    let phase = state[0];
    
    // Compute sine output
    let output = (2.0 * std::f64::consts::PI * phase).sin();
    
    // Advance phase with wrapping (same as phasor)
    let mut next_phase = phase + freq / sr;
    
    // Wrap to [0, 1) for ANY finite increment (handles |freq/sr| >= 1.0)
    // x - floor(x) maps any value to [0, 1); for in-range values floor(x)==0.0
    // so this is exact and doesn't perturb the existing doctest expectations.
    next_phase -= next_phase.floor();
    
    // Guard: floating-point edge case where result could be exactly 1.0
    // (e.g., tiny negative values like -1e-17 → -1e-17 + 1.0 may round to 1.0)
    if next_phase >= 1.0 {
        next_phase = 0.0;
    }
    
    state[0] = next_phase;
    output
}

/// Uniform random noise generator using xoshiro256++ PRNG.
///
/// # Definition
/// Outputs uniform random values in [-1, 1) using the xoshiro256++ algorithm.
/// State: 4 × u64 stored as f64 via to_bits/from_bits (deterministic round-trip).
/// StateDecl::Slots(4), arity 0.
///
/// **Seeding**: State arena is zero-initialized. Since all-zero state produces
/// degenerate output (all zeros), the kernel lazily initializes on first call:
/// if all four state slots are zero-bits, seeds via splitmix64 from a fixed
/// constant seed (0x0123456789ABCDEF).
///
/// **Mapping**: xoshiro256++ produces u64 values. We map to uniform [0, 1) via
/// the standard method: `(x >> 11) as f64 * 2^-53`, then to [-1, 1) via
/// `2.0 * u - 1.0`.
///
/// # Algorithm
/// xoshiro256++ public domain implementation from <https://prng.di.unimi.it/xoshiro256plusplus.c>
///
/// ```
/// use opengen_testkit::render;
///
/// // Determinism: two renders produce identical output
/// let out1 = render("out1 = noise();", 48000.0, 64);
/// let out2 = render("out1 = noise();", 48000.0, 64);
/// assert_eq!(out1.ch(0), out2.ch(0));
///
/// // All values within [-1, 1)
/// for &val in out1.ch(0) {
///     assert!(val >= -1.0 && val < 1.0, "Out of range: {}", val);
/// }
/// ```
pub fn noise(_inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    // Check if state is uninitialized (all zero bits)
    if state[0].to_bits() == 0
        && state[1].to_bits() == 0
        && state[2].to_bits() == 0
        && state[3].to_bits() == 0
    {
        // Lazy initialization via splitmix64
        const SEED: u64 = 0x0123456789ABCDEF;
        let mut sm_state = SEED;
        for i in 0..4 {
            sm_state = sm_state.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = sm_state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z = z ^ (z >> 31);
            state[i] = f64::from_bits(z);
        }
    }

    // xoshiro256++ algorithm
    let s0 = state[0].to_bits();
    let s1 = state[1].to_bits();
    let s2 = state[2].to_bits();
    let s3 = state[3].to_bits();

    // result = rotl(s0 + s3, 23) + s0
    let result = s0
        .wrapping_add(s3)
        .rotate_left(23)
        .wrapping_add(s0);

    // t = s1 << 17
    let t = s1 << 17;

    // State update
    let new_s2 = s2 ^ s0;
    let new_s3 = s3 ^ s1;
    let new_s1 = s1 ^ new_s2;
    let new_s0 = s0 ^ new_s3;

    let final_s2 = new_s2 ^ t;
    let final_s3 = new_s3.rotate_left(45);

    state[0] = f64::from_bits(new_s0);
    state[1] = f64::from_bits(new_s1);
    state[2] = f64::from_bits(final_s2);
    state[3] = f64::from_bits(final_s3);

    // Map to [0, 1) using standard method: (x >> 11) * 2^-53
    let uniform_0_1 = (result >> 11) as f64 * (1.0 / (1u64 << 53) as f64);

    // Map to [-1, 1)
    2.0 * uniform_0_1 - 1.0
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "phasor",
            arity: 1,
            state: StateDecl::Slots(1),
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: phasor,
        },
        OpDef {
            name: "cycle",
            arity: 1,
            state: StateDecl::Slots(1),
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: cycle,
        },
        OpDef {
            name: "noise",
            arity: 0,
            state: StateDecl::Slots(4),
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: noise,
        },
    ]
}

#[cfg(test)]
mod tests {
    use opengen_testkit::render;

    #[test]
    fn phasor_exact_ramp() {
        // Exact equality test: 1000 Hz at 48000 sr
        let out = render("out1 = phasor(1000);", 48000.0, 3);
        assert_eq!(out.ch(0), &[0.0, 1000.0/48000.0, 2000.0/48000.0]);
    }

    #[test]
    fn cycle_first_sample_zero() {
        let out = render("out1 = cycle(440);", 48000.0, 1);
        assert_eq!(out.ch(0)[0], 0.0);
    }

    #[test]
    fn cycle_quarter_period_near_one() {
        // At sr=48000, freq=12000 → period = 4 samples
        // Quarter period at sample 1: phase = 0.25 → sin(π/2) ≈ 1.0
        let out = render("out1 = cycle(12000);", 48000.0, 2);
        let val = out.ch(0)[1];
        assert!((val - 1.0).abs() <= f64::EPSILON, "Expected ~1.0, got {}", val);
    }

    #[test]
    fn noise_determinism() {
        // Two renders with same seed produce identical output
        let out1 = render("out1 = noise();", 48000.0, 64);
        let out2 = render("out1 = noise();", 48000.0, 64);
        assert_eq!(out1.ch(0), out2.ch(0));
    }

    #[test]
    fn noise_range() {
        // All values must be in [-1, 1)
        let out = render("out1 = noise();", 48000.0, 1000);
        for &val in out.ch(0) {
            assert!(val >= -1.0 && val < 1.0, "Out of range: {}", val);
        }
    }

    #[test]
    fn phasor_high_freq_wrap() {
        // Test freq/sr = 2.5: freq=120000, sr=48000
        // Expected: y[0]=0.0, y[1]=wrap(2.5)=0.5, y[2]=wrap(0.5+2.5)=wrap(3.0)=0.0
        let out = render("out1 = phasor(120000);", 48000.0, 3);
        let samples = out.ch(0);
        
        // All samples must be in [0, 1)
        for (i, &val) in samples.iter().enumerate() {
            assert!(val >= 0.0 && val < 1.0, "Sample {} out of range [0,1): {}", i, val);
        }
        
        // Check exact values
        assert_eq!(samples[0], 0.0);
        assert_eq!(samples[1], 0.5);
        assert_eq!(samples[2], 0.0);
    }

    #[test]
    fn phasor_negative_freq_wrap() {
        // Test negative freq/sr = -1.25: freq=-60000, sr=48000
        // With port-level cycle breaking, phasor sees -60000 from sample 0 (no stale freq artifact)
        let out = render("out1 = phasor(0 - 60000);", 48000.0, 4);
        let samples = out.ch(0);
        
        // All samples must be in [0, 1)
        for (i, &val) in samples.iter().enumerate() {
            assert!(val >= 0.0 && val < 1.0, "Sample {} out of range [0,1): {}", i, val);
        }
        
        // Check exact values: phase starts at 0, each sample wraps -1.25 increment
        assert_eq!(samples[0], 0.0);  // phase=0 at start
        assert_eq!(samples[1], 0.75); // phase=0.75 after wrapping -1.25
        assert_eq!(samples[2], 0.5);  // phase=0.5 after wrapping -0.5
        assert_eq!(samples[3], 0.25); // phase=0.25 after wrapping 0.25
    }
}
