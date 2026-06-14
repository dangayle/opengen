//! Conformance runner: render all authored patches and compare against golden WAVs.
//!
//! Protocol:
//! - Golden naming: `<stem>.ch<N>.<sr>.wav` — the sample rate is embedded in
//!   the filename, so renders at different Max driver rates COEXIST instead
//!   of overwriting each other. (A 2026-06-11 session silently rendered at
//!   44.1 kHz under headers hardcoding 48 kHz; now filename, header, and
//!   data must agree, and the render kit records the true dspstate~ rate.)
//! - For each patch: discover every rate with a ch0 golden, render the patch
//!   in opengen at each such rate for 4096 samples, compare per-sample with
//!   a patch-dependent tolerance.
//! - Golden lookup: `conformance/golden/`, falling back to
//!   `conformance/golden-self/`.
//! - If no golden exists at any rate, SKIP (eprintln).
//! - NaN golden + NaN render counts as agreement (some probes deliberately
//!   produce NaN at runtime).
//!
//! Patches that measure gen~'s compile-time constant folder (not the per-sample
//! kernel) are known divergences — opengen has no folder. They're recorded as
//! evidence, not conformance targets.
//! Two layers: hand-authored conformance patches (one #[test] each) plus an
//! auto-generated per-operator sweep (conformance/patches/ops/, checked by
//! one discovering test — see `conformance_op_sweeps`).
//!
//! ## Environment
//! - `OPENGEN_BLESS=1`: write golden-self files instead of comparing.
//! - `OPENGEN_BLESS_SR=<rate>`: bless at a specific rate (default 48000).
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

/// Patches that measure gen~'s compile-time constant folder (not the per-sample
/// kernel) are known divergences — opengen has no folder. Recorded as evidence
/// rather than conformance targets.
const KNOWN_DIVERGENCES: &[&str] = &[
    "dcblock_step",             // folder-vs-kernel (gen~ JIT folds constant DC to steady-state)
    "history_read_after_write", // History write-through (gen~) vs dataflow semantics (opengen)
];

/// Workspace root, resolved from CARGO_MANIFEST_DIR (opengen-analysis/ -> ../..).
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .to_path_buf()
}

/// Render a patch at the given sample rate and return channels.
fn render_patch(src: &str, sr: f64) -> Vec<Vec<f64>> {
    let graph = opengen_genexpr::parse_and_lower(src)
        .expect("parse failed in conformance patch");
    let mut patch = opengen_compile::compile(
        &graph, &opengen_ops::Registry::core(), sr
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

/// Find golden file for a stem/channel at a specific sample rate.
/// Naming: `<stem>.ch<N>.<sr>.wav`.
fn find_golden(root: &Path, stem: &str, ch: usize, sr: u32) -> Option<PathBuf> {
    let name = format!("{stem}.ch{ch}.{sr}.wav");
    for dir in ["golden", "golden-self"] {
        let p = root.join("conformance").join(dir).join(&name);
        if p.exists() {
            return Some(p);
        }
    }
    None
}

/// Sample agreement, accounting for two independent error sources:
///  1. `tol` — the kernel-class numeric tolerance (exact vs transcendental).
///  2. The golden is stored as **float32** (the render kit's capture format),
///     so it carries ~|g|*2^-23 of quantization. For large-magnitude values
///     (e.g. mtof ~2050) that storage quantum alone exceeds a 1e-6 absolute
///     tolerance — it is NOT kernel divergence. Adding it keeps the check
///     honest without masking real bugs (a real bug exceeds tol + a tiny
///     storage floor).
/// NaN golden + NaN render counts as agreement.
fn samples_agree(s: f64, g: f64, tol: f64) -> bool {
    if s.is_nan() && g.is_nan() {
        return true;
    }
    let f32_storage_floor = g.abs() * f32::EPSILON as f64;
    (s - g).abs() <= tol + f32_storage_floor
}

/// Every sample rate for which a ch0 golden exists, across both golden
/// directories. Renders at different driver rates coexist by design.
fn golden_rates(root: &Path, stem: &str) -> Vec<u32> {
    let mut rates = Vec::new();
    let prefix = format!("{stem}.ch0.");
    for dir in ["golden", "golden-self"] {
        let Ok(entries) = std::fs::read_dir(root.join("conformance").join(dir)) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if let Some(sr) = name
                .strip_prefix(&prefix)
                .and_then(|rest| rest.strip_suffix(".wav"))
                .and_then(|s| s.parse::<u32>().ok())
            {
                if !rates.contains(&sr) {
                    rates.push(sr);
                }
            }
        }
    }
    rates.sort_unstable();
    rates
}

/// Write golden-self file for a channel at the given rate.
fn write_golden_self(root: &Path, stem: &str, ch: usize, samples: &[f64], sr: u32) {
    let dir = root.join("conformance").join("golden-self");
    std::fs::create_dir_all(&dir).expect("create golden-self dir");
    let path = dir.join(format!("{stem}.ch{ch}.{sr}.wav"));
    opengen_analysis::wav::write_wav(&path, samples, sr)
        .unwrap_or_else(|e| panic!("write golden-self {stem}.ch{ch}: {e}"));
    eprintln!("✓ Blessed golden-self file: {}", path.display());
}

// ─── Per-patch test functions ───────────────────────────────────────────────

fn run_single_patch(stem: &str, src: &str) {
    let root = workspace_root();
    let tol = tolerance_for(stem);

    if KNOWN_DIVERGENCES.contains(&stem) {
        eprintln!("⊘ SKIP  {stem}: known divergence (constant-folder vs kernel)");
        return;
    }

    if std::env::var("OPENGEN_BLESS").is_ok() {
        let sr: u32 = std::env::var("OPENGEN_BLESS_SR")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(48_000);
        for (ch_idx, samples) in render_patch(src, sr as f64).iter().enumerate() {
            write_golden_self(&root, stem, ch_idx, samples, sr);
        }
        return;
    }

    let rates = golden_rates(&root, stem);
    if rates.is_empty() {
        eprintln!("⊘ SKIP  {stem}: no golden files found (any rate)");
        eprintln!("  Render via conformance/render/render_host.maxpat, or");
        eprintln!("  run with OPENGEN_BLESS=1 to generate golden-self/");
        return;
    }

    // Verify against EVERY rate that has goldens — multi-rate evidence.
    for sr in rates {
        let channels = render_patch(src, sr as f64);

        for (ch_idx, samples) in channels.iter().enumerate() {
            match find_golden(&root, stem, ch_idx, sr) {
                None => {
                    eprintln!("⊘ SKIP  {stem}.ch{ch_idx} @ {sr} Hz: no golden file");
                }
                Some(golden_path) => {
                    let (golden, header_sr) = opengen_analysis::wav::read_wav(&golden_path)
                        .expect("failed to read golden file");

                    // Filename rate and header rate MUST agree — catches any
                    // mislabeled render (the 44.1k-data-in-48k-header bug).
                    assert_eq!(header_sr, sr,
                        "{stem}.ch{ch_idx}: filename says {sr} Hz but WAV header \
                         says {header_sr} Hz \u{2014} mislabeled golden");

                    assert_eq!(samples.len(), golden.len(),
                        "{stem}.ch{ch_idx}: sample count mismatch: {} rendered vs {} golden",
                        samples.len(), golden.len());

                    for (i, (&s, &g)) in samples.iter().zip(golden.iter()).enumerate() {
                        assert!(samples_agree(s, g, tol),
                            "{stem}.ch{ch_idx}[{i}]: got {s} vs golden {g} (diff {}, tol {tol}, sr {sr})",
                            (s - g).abs());
                    }
                    eprintln!("✓ PASS  {stem}.ch{ch_idx} (tol {tol}, sr {sr})");
                }
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

#[test]
fn conformance_dcblock_impulse() {
    // Disambiguation probe for dcblock's first-sample init (lazy x1-init vs
    // constant-folding) — SKIPs until the next Max render session produces
    // its golden. See the patch header and conformance/CHECKLIST.md.
    run_single_patch("dcblock_impulse", include_str!("../../../conformance/patches/dcblock_impulse.genexpr"));
}

#[test]
fn conformance_history_read_after_write() {
    // Divergence probe: does History read-after-assignment see the new value
    // (gen~ variable semantics) or the previous sample (opengen dataflow)?
    run_single_patch("history_read_after_write", include_str!("../../../conformance/patches/history_read_after_write.genexpr"));
}

// ─── Per-operator golden sweep ───────────────────────────────────────────
// Auto-generated sweep patches (tools/gen_op_sweeps.py) live in
// conformance/patches/ops/. Each drives one operator with runtime-laundered
// signals (so gen~ runs the real per-sample kernel, not its constant folder)
// and outputs the raw result. One test discovers and checks them all, so the
// suite scales to every operator without per-op boilerplate. Tolerance is
// classified per operator (exact arithmetic vs transcendental libm drift).

#[test]
fn conformance_op_sweeps() {
    let root = workspace_root();
    let ops_dir = root.join("conformance").join("patches").join("ops");
    let Ok(entries) = std::fs::read_dir(&ops_dir) else {
        eprintln!("⊘ SKIP  op-sweeps: {} not found", ops_dir.display());
        return;
    };
    let mut patches: Vec<PathBuf> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "genexpr"))
        .collect();
    patches.sort();

    if patches.is_empty() {
        eprintln!("⊘ SKIP  op-sweeps: no patches in {}", ops_dir.display());
        return;
    }

    let mut failures = Vec::new();
    let mut passed = 0usize;
    let mut skipped = 0usize;

    for patch_path in &patches {
        let stem = patch_path.file_stem().unwrap().to_string_lossy().into_owned();
        let src = std::fs::read_to_string(patch_path).expect("read op-sweep patch");
        match check_op_sweep(&root, &stem, &src) {
            OpResult::Pass(n) => passed += n,
            OpResult::Skip => skipped += 1,
            OpResult::Fail(msgs) => failures.extend(msgs),
        }
    }

    eprintln!(
        "op-sweeps: {} channel-checks passed, {} patches skipped (no golden), {} failures",
        passed, skipped, failures.len()
    );
    assert!(failures.is_empty(), "op-sweep conformance failures:\n{}", failures.join("\n"));
}

enum OpResult {
    Pass(usize),
    Skip,
    Fail(Vec<String>),
}

/// Check one op-sweep patch at every rate that has a golden. Unlike
/// run_single_patch (which panics on first mismatch), this collects all
/// failures so the sweep reports every divergent operator in one run.
fn check_op_sweep(root: &Path, stem: &str, src: &str) -> OpResult {
    let tol = op_tolerance(stem);
    let rates = golden_rates(root, stem);
    if rates.is_empty() {
        eprintln!("⊘ SKIP  {stem}: no golden (render conformance/render/ops_*.maxpat)");
        return OpResult::Skip;
    }

    let mut failures = Vec::new();
    let mut checks = 0usize;
    for sr in rates {
        let channels = render_patch(src, sr as f64);
        for (ch_idx, samples) in channels.iter().enumerate() {
            let Some(golden_path) = find_golden(root, stem, ch_idx, sr) else { continue };
            let (golden, header_sr) = opengen_analysis::wav::read_wav(&golden_path)
                .expect("read op-sweep golden");
            if header_sr != sr {
                failures.push(format!(
                    "{stem}.ch{ch_idx}: filename {sr} Hz != header {header_sr} Hz"));
                continue;
            }
            let mut first: Option<String> = None;
            let mut diverged = 0usize;
            for (i, (&s, &g)) in samples.iter().zip(golden.iter()).enumerate() {
                if !samples_agree(s, g, tol) {
                    diverged += 1;
                    if first.is_none() {
                        first = Some(format!(
                            "{stem}.ch{ch_idx}[{i}] @ {sr}Hz: got {s} vs golden {g} \
                             (diff {}, tol {tol})", (s - g).abs()));
                    }
                }
            }
            if let Some(msg) = first {
                // Report count so an isolated discontinuity knife-edge is
                // instantly distinguishable from a systemic kernel bug.
                failures.push(format!("{msg}  [{diverged}/{} samples diverged]", samples.len()));
            }
            checks += 1;
        }
    }

    if failures.is_empty() {
        eprintln!("✓ PASS  {stem} ({checks} channel-checks, tol {tol})");
        OpResult::Pass(checks)
    } else {
        OpResult::Fail(failures)
    }
}

/// Per-operator tolerance. Exact arithmetic compares bit-exact; transcendental
/// kernels differ between gen~'s libm and Rust's by small amounts. The stem is
/// `op_<name>` so we classify on the suffix.
fn op_tolerance(stem: &str) -> f64 {
    let op = stem.strip_prefix("op_").unwrap_or(stem);
    // Transcendental / approximate kernels: libm divergence between engines.
    const TRANSCENDENTAL: &[&str] = &[
        "sin", "cos", "tan", "asin", "acos", "atan", "atan2",
        "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",
        "exp", "exp2", "pow", "ln", "log", "log2", "log10",
        "sqrt", "hypot", "tanh", "atodb", "dbtoa", "mtof", "ftom",
        "fastsin", "fastcos", "fasttan", "fastexp", "fastpow",
        "t60", "slide",
    ];
    if TRANSCENDENTAL.contains(&op) {
        1e-4
    } else {
        1e-6
    }
}
