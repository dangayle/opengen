//! Impulse and frequency response analysis

use rustfft::{FftPlanner, num_complex::Complex};

pub struct Response {
    /// Complex frequency spectrum (nfft/2 + 1 bins for real input)
    spectrum: Vec<Complex<f64>>,
    /// Sample rate
    sr: f64,
    /// FFT size
    nfft: usize,
}

impl Response {
    /// Return the magnitude response in dB at the given frequency.
    ///
    /// # Valid Range
    /// `hz` must be in the range `[0.0, sr/2.0]` (DC to Nyquist frequency).
    ///
    /// # Panics
    /// Panics if `hz` is negative or above the Nyquist frequency.
    pub fn db_at(&self, hz: f64) -> f64 {
        assert!(
            hz >= 0.0 && hz <= self.sr / 2.0,
            "frequency {hz} Hz out of range [0, {}]",
            self.sr / 2.0
        );
        let bin_spacing = self.sr / self.nfft as f64;
        let exact_bin = hz / bin_spacing;
        
        // Linear interpolation between adjacent bins
        let bin_lo = exact_bin.floor() as usize;
        let bin_hi = (exact_bin.ceil() as usize).min(self.spectrum.len() - 1);
        
        if bin_lo == bin_hi || bin_hi >= self.spectrum.len() {
            // Exact bin or at Nyquist
            let mag = self.spectrum[bin_lo.min(self.spectrum.len() - 1)].norm();
            20.0 * mag.log10()
        } else {
            // Interpolate magnitude
            let frac = exact_bin - bin_lo as f64;
            let mag_lo = self.spectrum[bin_lo].norm();
            let mag_hi = self.spectrum[bin_hi].norm();
            let mag = mag_lo + frac * (mag_hi - mag_lo);
            20.0 * mag.log10()
        }
    }
    
    /// Return the phase response in radians at the given frequency.
    ///
    /// # Valid Range
    /// `hz` must be in the range `[0.0, sr/2.0]` (DC to Nyquist frequency).
    ///
    /// # Interpolation
    /// Phase is linearly interpolated between FFT bins. Phase wraps near ±π
    /// are not specially handled (this is an M1 limitation).
    ///
    /// # Panics
    /// Panics if `hz` is negative or above the Nyquist frequency.
    pub fn phase_at(&self, hz: f64) -> f64 {
        assert!(
            hz >= 0.0 && hz <= self.sr / 2.0,
            "frequency {hz} Hz out of range [0, {}]",
            self.sr / 2.0
        );
        let bin_spacing = self.sr / self.nfft as f64;
        let exact_bin = hz / bin_spacing;
        
        // Linear interpolation between adjacent bins
        let bin_lo = exact_bin.floor() as usize;
        let bin_hi = (exact_bin.ceil() as usize).min(self.spectrum.len() - 1);
        
        if bin_lo == bin_hi || bin_hi >= self.spectrum.len() {
            // Exact bin or at Nyquist
            self.spectrum[bin_lo.min(self.spectrum.len() - 1)].arg()
        } else {
            // Interpolate phase (unwrapped)
            let frac = exact_bin - bin_lo as f64;
            let phase_lo = self.spectrum[bin_lo].arg();
            let phase_hi = self.spectrum[bin_hi].arg();
            phase_lo + frac * (phase_hi - phase_lo)
        }
    }
}

/// Render a patch with a unit impulse input (1.0, then zeros) and return channel 0.
pub fn impulse_response(src: &str, sr: f64, n: usize) -> Vec<f64> {
    // Parse and compile the source
    let graph = opengen_genexpr::parse_and_lower(src).expect("parse");
    let mut patch = opengen_compile::compile(&graph, &opengen_ops::Registry::core(), sr)
        .expect("compile");
    
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        // Feed impulse: 1.0 at first sample, 0.0 thereafter
        let input = if i == 0 { 1.0 } else { 0.0 };
        let frame = patch.process(&[input]);
        // Use channel 0 of output as the impulse response
        result.push(frame.get(0).copied().unwrap_or(0.0));
    }
    
    result
}

/// Compute frequency response via FFT of the impulse response.
pub fn freq_response(src: &str, sr: f64, nfft: usize) -> Response {
    // Get impulse response
    let mut ir = impulse_response(src, sr, nfft);
    
    // Zero-pad if needed (should already be nfft length)
    ir.resize(nfft, 0.0);
    
    // Convert to complex
    let mut buffer: Vec<Complex<f64>> = ir.iter().map(|&x| Complex::new(x, 0.0)).collect();
    
    // Perform FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(nfft);
    fft.process(&mut buffer);
    
    // Keep only the positive frequencies (DC to Nyquist)
    // For real input, we only need bins 0..nfft/2+1
    let spectrum = buffer.into_iter().take(nfft / 2 + 1).collect();
    
    Response { spectrum, sr, nfft }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[should_panic(expected = "frequency -1 Hz out of range")]
    fn db_at_negative_hz_panics() {
        let src = "out1 = in1;";
        let h = freq_response(src, 48_000.0, 8192);
        h.db_at(-1.0);
    }
    
    #[test]
    #[should_panic(expected = "frequency 48000 Hz out of range")]
    fn db_at_above_nyquist_panics() {
        let src = "out1 = in1;";
        let h = freq_response(src, 48_000.0, 8192);
        h.db_at(48_000.0); // sr = 48000, Nyquist = 24000
    }
    
    #[test]
    #[should_panic(expected = "frequency -1 Hz out of range")]
    fn phase_at_negative_hz_panics() {
        let src = "out1 = in1;";
        let h = freq_response(src, 48_000.0, 8192);
        h.phase_at(-1.0);
    }
    
    #[test]
    #[should_panic(expected = "frequency 48000 Hz out of range")]
    fn phase_at_above_nyquist_panics() {
        let src = "out1 = in1;";
        let h = freq_response(src, 48_000.0, 8192);
        h.phase_at(48_000.0); // sr = 48000, Nyquist = 24000
    }
}
