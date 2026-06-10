//! Frequency-domain analysis for filter operators.

use opengen_analysis::*;

#[test]
fn dcblock_blocks_low_frequencies() {
    // dcblock is a one-pole highpass: H(z) = (1 - z^-1) / (1 - 0.9997*z^-1)
    // -3 dB cutoff at ~2.3 Hz (48 kHz sample rate).
    //
    // With 8192-point FFT (5.86 Hz/bin), sub-1 Hz measurements have limited resolution.
    let h = freq_response("out1 = dcblock(in1);", 48_000.0, 8192);
    // At 0.1 Hz: substantial attenuation
    assert!(
        h.db_at(0.1) < -15.0,
        "dcblock at 0.1 Hz: expected < -15 dB, got {} dB",
        h.db_at(0.1)
    );
    // At 10 Hz: minor attenuation (< 1 dB loss)
    assert!(
        h.db_at(10.0) > -1.0,
        "dcblock at 10 Hz: expected > -1 dB, got {} dB",
        h.db_at(10.0)
    );
    // At 20 kHz: basically pass-through (< 0.1 dB loss)
    assert!(
        h.db_at(20_000.0) > -0.1,
        "dcblock at 20 kHz: expected > -0.1 dB, got {} dB",
        h.db_at(20_000.0)
    );
}

#[test]
fn dcblock_high_resolution_shows_deep_attenuation() {
    // With 65536-point FFT (~0.73 Hz/bin), we can resolve sub-1 Hz behavior.
    let h = freq_response("out1 = dcblock(in1);", 48_000.0, 65536);
    // At 0.5 Hz: deeper resolution shows significant attenuation
    assert!(
        h.db_at(0.5) < -10.0,
        "dcblock at 0.5 Hz (65536 FFT): expected < -10 dB, got {} dB",
        h.db_at(0.5)
    );
}
