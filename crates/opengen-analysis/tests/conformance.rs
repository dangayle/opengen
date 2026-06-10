//! Conformance runner: render all authored patches and compare against golden WAVs.
//!
//! Protocol:
//! - Render each `conformance/patches/*.genexpr` at 48 kHz for 4096 samples.
//! - For each output channel N: look up `conformance/golden/<stem>.ch<N>.wav`;
//!   if absent, fall back to `conformance/golden-self/<stem>.ch<N>.wav`.
//! - If neither exists, SKIP (eprintln).
//! - If golden exists, compare per-sample with a patch-dependent tolerance.
//!
//! ## Environment
//! - `OPENGEN_BLESS=1`: write rendered samples to `golden-self/<stem>.ch<N>.wav`
//!   (32-bit float WAV) instead of comparing. Use after first successful render
//!   to pin regression goldens.
//!
//! ## Tolerances
//! | Pattern in stem    | Tolerance | Rationale                |
//! |--------------------|-----------|--------------------------|
//! | `cycle`            | 5e-3      | Transcendental (sin via polyphase) |
//! | `phasor`           | 1e-6      | Deterministic ramp       |
//! | default            | 1e-6      | Deterministic arithmetic |
//!
//! ## Noise exclusion
//! Patches containing noise-like operators (e.g. `noise`) are excluded from
//! conformance because `opengen` uses a different PRNG than gen~.
//! No such patches exist in the current set.

use std::path::{Path, PathBuf};

/// Workspace root, resolved from CARGO_MANIFEST_DIR (opengen-analysis/ -> ../..).
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .to_path_buf()
}

/// Render a patch and return channels.
fn render_patch(src: &str) -> Vec<Vec<f64>> {
    let graph = opengen_genexpr::parse_and_lower(src)
        .expect("parse failed in conformance patch");
    let mut patch = opengen_compile::compile(
        &graph, &opengen_ops::Registry::core(), 48_000.0
    ).expect("compile failed in conformance patch");
    let outs = patch.output_count();
    let mut channels = vec![Vec::with_capacity(4096); outs];
    for _ in 0..4096 {
        let frame = patch.process(&[]);
        for (c, v) in channels.iter_mut().zip(frame) {
            c.push(v);
        }
    }
    channels
}

/// Determine tolerance based on patch stem.
fn tolerance_for(stem: &str) -> f64 {
    if stem.contains("cycle") {
        5e-3
    } else if stem.contains("phasor") {
        1e-6
    } else {
        1e-6
    }
}

/// Find golden file for a given stem and channel index.
fn find_golden(root: &Path, stem: &str, ch: usize) -> Option<PathBuf> {
    let name = format!("{stem}.ch{ch}.wav");
    // Primary: golden/
    let primary = root.join("conformance").join("golden").join(&name);
    if primary.exists() {
        return Some(primary);
    }
    // Fallback: golden-self/
    let fallback = root.join("conformance").join("golden-self").join(&name);
    if fallback.exists() {
        return Some(fallback);
    }
    None
}

/// Write golden-self file for a channel.
fn write_golden_self(root: &Path, stem: &str, ch: usize, samples: &[f64]) {
    let dir = root.join("conformance").join("golden-self");
    std::fs::create_dir_all(&dir).expect("create golden-self dir");
    let path = dir.join(format!("{stem}.ch{ch}.wav"));
    opengen_analysis::wav::write_wav(&path, samples, 48_000)
        .unwrap_or_else(|e| panic!("write golden-self {stem}.ch{ch}: {e}"));
    eprintln!("✓ Blessed golden-self file: {}", path.display());
}

// ─── Per-patch test functions ───────────────────────────────────────────────

fn run_single_patch(stem: &str, src: &str) {
    let root = workspace_root();
    let channels = render_patch(src);
    let tol = tolerance_for(stem);
    let bless = std::env::var("OPENGEN_BLESS").is_ok();

    for (ch_idx, samples) in channels.iter().enumerate() {
        if bless {
            write_golden_self(&root, stem, ch_idx, samples);
            continue;
        }

        match find_golden(&root, stem, ch_idx) {
            None => {
                eprintln!("⊘ SKIP  {stem}.ch{ch_idx}: no golden file found");
                eprintln!("  Run with OPENGEN_BLESS=1 to generate golden-self/");
            }
            Some(golden_path) => {
                let (golden, _sr) = opengen_analysis::wav::read_wav(&golden_path)
                    .expect("failed to read golden file");

                assert_eq!(samples.len(), golden.len(),
                    "{stem}.ch{ch_idx}: sample count mismatch: {} rendered vs {} golden",
                    samples.len(), golden.len());

                for (i, (&s, &g)) in samples.iter().zip(golden.iter()).enumerate() {
                    assert!((s - g).abs() <= tol,
                        "{stem}.ch{ch_idx}[{i}]: got {s} vs golden {g} (diff {}, tol {tol})",
                        (s - g).abs());
                }
                eprintln!("✓ PASS  {stem}.ch{ch_idx} (tol {tol})");
            }
        }
    }
}

// ─── Individual test cases ──────────────────────────────────────────────────
// Each patch is a separate #[test] so failures are isolated and labelled.

#[test]
fn conformance_cycle_440() {
    run_single_patch("cycle_440", include_str!("../../../conformance/patches/cycle_440.genexpr"));
}

#[test]
fn conformance_dcblock_step() {
    run_single_patch("dcblock_step", include_str!("../../../conformance/patches/dcblock_step.genexpr"));
}

#[test]
fn conformance_delay_echo() {
    run_single_patch("delay_echo", include_str!("../../../conformance/patches/delay_echo.genexpr"));
}

#[test]
fn conformance_history_counter() {
    run_single_patch("history_counter", include_str!("../../../conformance/patches/history_counter.genexpr"));
}

#[test]
fn conformance_phasor_incr_order() {
    run_single_patch("phasor_incr_order", include_str!("../../../conformance/patches/phasor_incr_order.genexpr"));
}

#[test]
fn conformance_range_inverted_bounds() {
    run_single_patch("range_inverted_bounds", include_str!("../../../conformance/patches/range_inverted_bounds.genexpr"));
}

#[test]
fn conformance_sah_latch() {
    run_single_patch("sah_latch", include_str!("../../../conformance/patches/sah_latch.genexpr"));
}

#[test]
fn conformance_slide_step() {
    run_single_patch("slide_step", include_str!("../../../conformance/patches/slide_step.genexpr"));
}

#[test]
fn conformance_triangle_duty() {
    run_single_patch("triangle_duty", include_str!("../../../conformance/patches/triangle_duty.genexpr"));
}
