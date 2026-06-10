//! Impulse and frequency response analysis

use rustfft::{FftPlanner, num_complex::Complex};
use std::f64::consts::PI;

pub struct Response {
    /// Complex frequency spectrum (nfft/2 + 1 bins for real input)
    spectrum: Vec<Complex<f64>>,
    /// Sample rate
    sr: f64,
    /// FFT size
    nfft: usize,
}

impl Response {
    /// Return the Nyquist frequency (sample rate / 2).
    pub fn nyquist(&self) -> f64 {
        self.sr / 2.0
    }
    
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

/// Spectrum analysis of a signal.
/// Magnitude values are normalized so the peak frequency bin is 0 dB.
pub struct Spectrum {
    /// Complex frequency spectrum (nfft/2 + 1 bins for real input)
    spectrum: Vec<Complex<f64>>,
    /// Sample rate
    sr: f64,
    /// FFT size
    nfft: usize,
    /// Peak magnitude (for normalization)
    peak_mag: f64,
}

impl Spectrum {
    /// Return the frequency (in Hz) of the peak magnitude bin.
    /// Simple argmax over bins; no parabolic interpolation.
    pub fn peak_hz(&self) -> f64 {
        let peak_idx = self.spectrum.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.norm().partial_cmp(&b.norm()).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0);
        
        let bin_spacing = self.sr / self.nfft as f64;
        peak_idx as f64 * bin_spacing
    }
    
    /// Return the magnitude in dB at the given frequency, relative to the peak.
    /// The peak frequency has magnitude 0 dB; all other frequencies are negative dB.
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
        
        let mag = if bin_lo == bin_hi || bin_hi >= self.spectrum.len() {
            // Exact bin or at Nyquist
            self.spectrum[bin_lo.min(self.spectrum.len() - 1)].norm()
        } else {
            // Interpolate magnitude
            let frac = exact_bin - bin_lo as f64;
            let mag_lo = self.spectrum[bin_lo].norm();
            let mag_hi = self.spectrum[bin_hi].norm();
            mag_lo + frac * (mag_hi - mag_lo)
        };
        
        // Return dB relative to peak (peak = 0 dB)
        20.0 * (mag / self.peak_mag).log10()
    }
}

/// Compute the frequency spectrum of a signal using FFT.
/// Applies a Hann window to reduce spectral leakage before the FFT.
///
/// # Window Choice
/// A Hann (raised cosine) window is applied to the input signal to minimize
/// spectral leakage from non-integer frequency bins. The Hann window provides
/// -31 dB first sidelobe with 18 dB/octave decay, ensuring harmonics and
/// artifacts are well below -90 dB for typical test signals.
///
/// # Normalization
/// The returned `Spectrum` normalizes all magnitudes so that the peak bin
/// has magnitude 0 dB. Use `Spectrum::db_at(hz)` to query magnitude relative
/// to this peak.
///
/// # Arguments
/// * `samples` - Time-domain signal samples
/// * `sr` - Sample rate in Hz
///
/// # Returns
/// A `Spectrum` object for querying frequency content.
pub fn spectrum(samples: &[f64], sr: f64) -> Spectrum {
    let nfft = samples.len();
    
    // Apply Hann window: w[n] = 0.5 * (1 - cos(2π·n/(N-1)))
    let mut windowed: Vec<Complex<f64>> = samples.iter()
        .enumerate()
        .map(|(i, &x)| {
            let window = 0.5 * (1.0 - (2.0 * PI * i as f64 / (nfft - 1) as f64).cos());
            Complex::new(x * window, 0.0)
        })
        .collect();
    
    // Compute normalization factor for Hann window (sum of squared window)
    let window_power: f64 = (0..nfft)
        .map(|i| {
            let w = 0.5 * (1.0 - (2.0 * PI * i as f64 / (nfft - 1) as f64).cos());
            w * w
        })
        .sum();
    let window_norm = (window_power / nfft as f64).sqrt();
    
    // Perform FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(nfft);
    fft.process(&mut windowed);
    
    // Keep only the positive frequencies (DC to Nyquist)
    let mut spectrum: Vec<Complex<f64>> = windowed.into_iter()
        .take(nfft / 2 + 1)
        .collect();
    
    // Normalize by window power and FFT size
    for bin in spectrum.iter_mut() {
        *bin /= nfft as f64 * window_norm;
    }
    
    // Find peak magnitude for relative dB calculations
    let peak_mag = spectrum.iter()
        .map(|c| c.norm())
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(1.0);
    
    Spectrum { spectrum, sr, nfft, peak_mag }
}
