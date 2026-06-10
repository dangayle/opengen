//! M1 exit criteria (design doc, Milestones).
use opengen_analysis::*;

#[test]
fn exit_one_pole_lowpass_response() {
    let src = "Param g(0.12278); h = history(mix(h, in1, g)); out1 = h;";
    let h = freq_response(src, 48_000.0, 8192);
    assert!((h.db_at(1_000.0) + 3.01).abs() < 0.1);
    assert!(h.db_at(100.0) > -0.2);          // passband flat
    assert!(h.db_at(20_000.0) < -20.0);      // stopband falling
}

#[test]
fn exit_phasor_driven_oscillator() {
    let src = "out1 = cycle(440);";
    let r = opengen_testkit::render(src, 48_000.0, 48_000);
    let h = spectrum(r.ch(0), 48_000.0);
    assert!((h.peak_hz() - 440.0).abs() < 1.0);   // fundamental where expected
    assert!(h.db_at(880.0) < -90.0);              // pure sine: no harmonics
}

#[test]
fn exit_probes_work_on_real_patch() {
    // One-pole lowpass with DC input: h converges toward 1.0
    let src = "h = history(mix(h, in1, 0.12278)); out1 = h;";
    let graph = opengen_genexpr::parse_and_lower(src).expect("parse");
    let mut patch = opengen_compile::compile_with_probes(
        &graph,
        &opengen_ops::Registry::core(),
        48_000.0,
        &["h"],
    ).expect("compile");
    
    // Feed DC input (1.0) for enough samples to converge
    let n_samples = 1000;
    for _ in 0..n_samples {
        patch.process(&[1.0]);
    }
    
    let h_trace = patch.probe("h").expect("probe 'h' exists");
    
    // Assert non-empty
    assert!(!h_trace.is_empty(), "probe should have recorded samples");
    
    // Assert monotonically non-decreasing (with small tolerance for fp noise)
    for i in 1..h_trace.len() {
        assert!(
            h_trace[i] >= h_trace[i-1] - 1e-10,
            "probe should be monotone non-decreasing at sample {}: {} -> {}",
            i, h_trace[i-1], h_trace[i]
        );
    }
    
    // Assert converges toward 1.0 (last sample very close)
    assert!(
        h_trace[h_trace.len() - 1] > 0.99,
        "probe should converge near 1.0, got {}",
        h_trace[h_trace.len() - 1]
    );
    
    // Assert differences are decreasing (exponential convergence)
    if h_trace.len() > 100 {
        let early_diff = h_trace[50] - h_trace[49];
        let late_diff = h_trace[h_trace.len() - 1] - h_trace[h_trace.len() - 2];
        assert!(
            late_diff < early_diff,
            "differences should decrease over time: early={}, late={}",
            early_diff, late_diff
        );
    }
}
