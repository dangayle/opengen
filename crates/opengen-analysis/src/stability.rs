//! Stability assertions: finiteness, denormals, DC offset, RMS bounds.

/// Backing checker for the `assert_stable!` macro. Panics with a labeled message on violation.
/// `max_rms`/`max_dc` of `f64::INFINITY` disable that bound.
pub fn check_stable(samples: &[f64], max_rms: f64, max_dc: f64) {
    assert!(!samples.is_empty(), "assert_stable!: empty signal");
    for (i, &x) in samples.iter().enumerate() {
        assert!(x.is_finite(), "assert_stable!: non-finite sample {x} at index {i}");
        assert!(
            x == 0.0 || x.abs() >= f64::MIN_POSITIVE,
            "assert_stable!: denormal sample {x:e} at index {i}"
        );
    }
    let n = samples.len() as f64;
    let dc = samples.iter().sum::<f64>() / n;
    let rms = (samples.iter().map(|x| x * x).sum::<f64>() / n).sqrt();
    assert!(rms <= max_rms, "assert_stable!: RMS {rms} exceeds bound {max_rms}");
    assert!(dc.abs() <= max_dc, "assert_stable!: DC offset {dc} exceeds bound {max_dc}");
}

/// Assert a rendered signal is finite, denormal-free, and within optional RMS/DC bounds.
///
/// ```
/// opengen_analysis::assert_stable!(&[0.0, 0.5, -0.5]);
/// opengen_analysis::assert_stable!(&[0.1; 64], max_rms = 1.0, max_dc = 0.2);
/// ```
#[macro_export]
macro_rules! assert_stable {
    ($samples:expr) => {
        $crate::stability::check_stable($samples, f64::INFINITY, f64::INFINITY)
    };
    ($samples:expr, max_rms = $rms:expr) => {
        $crate::stability::check_stable($samples, $rms, f64::INFINITY)
    };
    ($samples:expr, max_dc = $dc:expr) => {
        $crate::stability::check_stable($samples, f64::INFINITY, $dc)
    };
    ($samples:expr, max_rms = $rms:expr, max_dc = $dc:expr) => {
        $crate::stability::check_stable($samples, $rms, $dc)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn stable_signal_passes() {
        let sine: Vec<f64> = (0..1000)
            .map(|i| (2.0 * std::f64::consts::PI * 440.0 * i as f64 / 48_000.0).sin())
            .collect();
        crate::assert_stable!(&sine);
        crate::assert_stable!(&sine, max_rms = 0.8, max_dc = 0.01);
    }

    #[test]
    #[should_panic(expected = "non-finite")]
    fn nan_fails() { crate::assert_stable!(&[0.0, f64::NAN]); }

    #[test]
    #[should_panic(expected = "denormal")]
    fn denormal_fails() { crate::assert_stable!(&[0.0, 1e-320]); }

    #[test]
    #[should_panic(expected = "RMS")]
    fn rms_bound_fails() { crate::assert_stable!(&[10.0; 100], max_rms = 1.0); }

    #[test]
    #[should_panic(expected = "DC")]
    fn dc_bound_fails() { crate::assert_stable!(&[0.5; 100], max_dc = 0.1); }
}
