//! Impulse/frequency response, FFT, golden-WAV comparison, plot helpers

pub use response::{impulse_response, freq_response, Response};
pub mod wav;

mod response;

/// Hidden helper for macro hygiene - allows `assert_render_matches!` to work cross-crate.
#[doc(hidden)]
pub fn __testkit_render(src: &str, sr: f64, n: usize) -> opengen_testkit::Render {
    opengen_testkit::render(src, sr, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_pole_lowpass_minus_3db_near_cutoff() {
        // y = mix(y[n-1], x, g) one-pole; g chosen for fc ≈ 1 kHz at 48 kHz:
        // g = 1 - exp(-2π·fc/sr)  (standard one-pole relation; constants validated
        // analytically at authoring time)
        let src = "h = history(mix(h, in1, 0.12278));
                   out1 = h;";
        let h = freq_response(src, 48_000.0, 8192);
        let db = h.db_at(1_000.0);
        assert!((db - (-3.01)).abs() < 0.1, "got {db} dB at 1 kHz");
    }
}
