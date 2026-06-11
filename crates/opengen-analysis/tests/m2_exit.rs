//! M2 exit criteria: deep assertions on the 5-file reference exit set + corpus coverage ratchets.
//!
//! # Testing rings (D17)
//!
//! **Deep assertions** (PRIMARY — Wakefield/official reference examples):
//! - `exit_crossover_complementary_response` — LR crossover shelf; lo/hi individual bands + sum
//! - `exit_freeverb_impulse_tail` — Freeverb reverb tank (comb + allpass chain)
//! - `exit_resonator_vendor_sign_bug_renders_silence` — Two-pole resonator: faithful
//!   emulation of the shipped patch's sign bug (exact silence for bw = 0.005)
//! - `exit_resonator_sign_fixed_peaks_at_drive_freq` — Same patch with obj-30
//!   sign-fixed in memory: impulse ringdown peaks within 10 Hz of 440
//!
//! **Stress + smoke** (skip-if-missing):
//! - `exit_dattorro_plate_stress` — dang-tools hardest single patch (Delay multi-tap + declarator lists)
//! - `exit_gsot_corpus_ratchet` — GSOT 197 files, pin observed pass count
//! - `exit_dang_tools_ratchet` — dang-tools 36 files, ratchet on load+compile+render
//! - `exit_fors_smoke_ratchet` — Fors .amxd, parse embedded + build gen patchers
//!
//! # Ratchet semantics (D17)
//!
//! Each corpus test pins the observed pass count at D17 implementation time.
//! The test fails if coverage ever DROPS. Failures print a per-file summary.
//! All corpus tests skip cleanly when the corpus is absent.
//!
//! # Running corpus tests
//!
//! Use `--release` for faster corpus runs: `cargo test -p opengen-analysis --release -- --test m2_exit`
//! Debug builds are slower but CI-friendly for the default 256-sample render length.

use std::path::{Path, PathBuf};

use rustfft::{FftPlanner, num_complex::Complex};

// ──────────────────────────────────────────────────────────────────────────────
// Shared helpers
// ──────────────────────────────────────────────────────────────────────────────

/// Return the repo `reference/` directory, skipping cleanly when absent.
fn reference_dir() -> Option<PathBuf> {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // manifest dir = opengen-analysis/crates/, go up 2 levels to repo root
    let root = p.parent()?.parent()?;
    let ref_path = root.join("reference");
    if ref_path.exists() { Some(ref_path) } else { None }
}

/// Return a directory from an environment variable, or a default.
/// Returns `None` if neither the env var is set nor the default path exists.
fn corpus_env(var: &str, default: &str) -> Option<PathBuf> {
    match std::env::var(var) {
        Ok(val) => {
            let p = PathBuf::from(val);
            if p.exists() { Some(p) } else { None }
        }
        Err(_) => {
            let p = PathBuf::from(default);
            if p.exists() { Some(p) } else { None }
        }
    }
}

/// Render a compiled graph with a unit impulse on the first channel.
fn impulse_render_graph(
    graph: &opengen_ir::Graph,
    sr: f64,
    n: usize,
) -> opengen_testkit::Render {
    let mut impulse = vec![0.0; n];
    if !impulse.is_empty() {
        impulse[0] = 1.0;
    }
    opengen_testkit::render_graph_with_inputs(graph, sr, &[&impulse], n)
}

/// Compute absolute magnitude (in dB) at a frequency from an impulse response.
/// Uses FFT of the full sample buffer.
fn db_at_freq(samples: &[f64], sr: f64, target_hz: f64) -> f64 {
    let nfft = samples.len().next_power_of_two();
    let mut buffer: Vec<Complex<f64>> = samples.iter()
        .take(nfft)
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    buffer.resize(nfft, Complex::new(0.0, 0.0));
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(nfft);
    fft.process(&mut buffer);
    let bin_spacing = sr / nfft as f64;
    let exact_bin = target_hz / bin_spacing;
    let bin_lo = exact_bin.floor() as usize;
    let bin_hi = (exact_bin.ceil() as usize).min(nfft / 2);
    if bin_lo == bin_hi || bin_hi > nfft / 2 {
        let mag = buffer[bin_lo.min(nfft / 2)].norm();
        return 20.0 * mag.log10();
    }
    let frac = exact_bin - bin_lo as f64;
    let mag_lo = buffer[bin_lo].norm();
    let mag_hi = buffer[bin_hi].norm();
    let mag = mag_lo + frac * (mag_hi - mag_lo);
    20.0 * mag.log10()
}

// ──────────────────────────────────────────────────────────────────────────────
// Deep assertion tests — PRIMARY (Wakefield/official)
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn exit_crossover_complementary_response() {
    let root = reference_dir().expect("reference/ directory not found — M2 exit tests require reference/examples");
    let path = root.join("gen/examples/crossover.gendsp");
    assert!(path.exists(), "crossover.gendsp not found at {}", path.display());

    let graph = opengen_gendsp::load_gendsp(&path, &opengen_gendsp::LoadOptions::default())
        .expect("load crossover.gendsp");

    // Crossover has 1 input (in1), 2 outputs (out1 = lo, out2 = hi)
    // Render both channels with impulse
    let nfft = 8192;
    let sr = 48_000.0;
    let out = impulse_render_graph(&graph, sr, nfft);

    assert!(out.ch(0).len() >= 2, "expected at least 2 output channels");
    assert!(out.ch(1).len() >= 2, "expected at least 2 output channels");

    // lo band: passband at 100 Hz should be strong, stopband at 20 kHz attenuated
    let lo_100 = db_at_freq(out.ch(0), sr, 100.0);
    let lo_20k = db_at_freq(out.ch(0), sr, 20_000.0);
    assert!(lo_100 > -1.0, "lo band at 100 Hz: got {lo_100} dB, expected > -1.0");
    assert!(lo_20k < -40.0, "lo band at 20 kHz: got {lo_20k} dB, expected < -40.0");

    // hi band: passband at 20 kHz should be strong, stopband at 100 Hz attenuated
    let hi_20k = db_at_freq(out.ch(1), sr, 20_000.0);
    let hi_100 = db_at_freq(out.ch(1), sr, 100.0);
    assert!(hi_20k > -3.0, "hi band at 20 kHz: got {hi_20k} dB, expected > -3.0");
    assert!(hi_100 < -20.0, "hi band at 100 Hz: got {hi_100} dB, expected < -20.0 (shelf filter attenuation)");

    // lo + hi: LR crossover should sum to approximately allpass (flat magnitude)
    let sum: Vec<f64> = out.ch(0).iter().zip(out.ch(1).iter()).map(|(a, b)| a + b).collect();

    // The sum should be near 0 dB across the frequency range of interest
    for f in &[100.0, 1000.0, 10_000.0] {
        let db = db_at_freq(&sum, sr, *f);
        assert!(
            db.abs() < 1.0,
            "crossover sum at {f} Hz: got {db} dB, expected |db| < 1.0 (allpass-flat)"
        );
    }
}

#[test]
fn exit_freeverb_impulse_tail() {
    let root = reference_dir().expect("reference/ directory not found");
    let dir = root.join("gen/examples");
    let path = dir.join("freeverb.gendsp");
    assert!(path.exists(), "freeverb.gendsp not found at {}", path.display());

    // freeverb.gendsp resolves freeverb_comb and freeverb_allpass abstractions from the same dir
    let opts = opengen_gendsp::LoadOptions {
        search_paths: vec![dir.clone()],
    };
    let graph = opengen_gendsp::load_gendsp(&path, &opts)
        .expect("load freeverb.gendsp");

    let sr = 48_000.0;
    let n = (2.0 * sr) as usize; // 2 seconds
    let out = impulse_render_graph(&graph, sr, n);

    // Freeverb should produce a stable, decaying tail
    opengen_analysis::assert_stable!(out.ch(0));

    // Split into early (0-0.5s) and late (1.5-2.0s) segments
    let half_sr = (0.5 * sr) as usize;
    let early_tail = &out.ch(0)[0..half_sr];
    let late_start = (1.5 * sr) as usize;
    let late_tail = &out.ch(0)[late_start..n];

    // Compute RMS of each segment
    fn rms(samples: &[f64]) -> f64 {
        if samples.is_empty() { return 0.0; }
        let sum_sq: f64 = samples.iter().map(|x| x * x).sum();
        (sum_sq / samples.len() as f64).sqrt()
    }

    let early_rms = rms(early_tail);
    let late_rms = rms(late_tail);

    assert!(
        early_rms > 0.0,
        "freeverb early tail RMS should be > 0, got {early_rms}"
    );
    assert!(
        late_rms > 0.0,
        "freeverb late tail RMS should be > 0, got {late_rms}"
    );
    assert!(
        early_rms > late_rms,
        "freeverb early RMS ({early_rms}) should exceed late RMS ({late_rms}) — reverb tail should decay"
    );
}

// ── gen_resonator: vendor sign bug (root-caused 2026-06-10) ──────────────────
//
// The shipped example computes its two pole-radius branches INCONSISTENTLY:
//   c-branch (obj-45 `* -1` → obj-17 `exp` → obj-18 `* -1`): c = −e^(−bw) = −r²
//   b-branch (obj-30 `* 0.5` → obj-46 `exp`):                 b = 2·cosω·e^(+bw/2)
// A canonical two-pole needs b = 2·cosω·e^(−bw/2). Consequences (poles of
// y = a·x + b·y1 + c·y2 satisfy z² − bz − c = 0, cosθ = b/2R, R² = −c):
//   • as shipped: cosθ = cosω·e^(+bw) — detuned flat; and for
//     bw > ~(2−2cosω)/(1+cosω) the input coefficient a = 1 − min(b − r², 1)
//     clamps to exactly 0, so the filter receives NO input and outputs silence.
//   • sign-fixed (obj-30 = `* -0.5`): cosθ = cosω exactly — tuned to ω for
//     ANY bw — clearly the design intent.
// The host example (gen~.resonator_bank_v2.maxpat codebox) only drives
// bw ∈ [6.5e-5, 1.3e-3], below the silence threshold, which is why the bug
// ships unnoticed. Full analysis: docs/research/gen_resonator_sign_bug.md.
// Conformance probe (real Max should also go silent): conformance/CHECKLIST.md.

const RESONATOR_VENDOR_BUG_TEXT: &str = r#""text" : "* 0.5""#;
const RESONATOR_FIXED_TEXT: &str = r#""text" : "* -0.5""#;

fn resonator_bytes() -> Vec<u8> {
    let root = reference_dir().expect("reference/ directory not found");
    let path = root.join("gen/examples/gen_resonator.gendsp");
    assert!(path.exists(), "gen_resonator.gendsp not found at {}", path.display());
    std::fs::read(&path).expect("read gen_resonator.gendsp")
}

/// Drive signals for the resonator: (in1 excitation, in2 voice, in3 ω, in4 amp, in5 bw).
/// `in 3 freq` flows through the voice latch (?/history/slide) DIRECTLY into
/// `cos` — no 2π/sr scaling inside the patch — so in3 expects NORMALIZED
/// RADIAN frequency ω = 2π·f/sr, not Hz. `in 5 bw` sets r² = e^(−bw).
fn resonator_render(graph: &opengen_ir::Graph, in1: &[f64], sr: f64) -> Vec<f64> {
    let n = in1.len();
    let omega = std::f64::consts::TAU * 440.0 / sr;
    let in2 = vec![0.0; n];        // voice index 0 (matches param id 0)
    let in3 = vec![omega; n];      // ω for 440 Hz
    let in4 = vec![1.0; n];        // amp
    let in5 = vec![0.005; n];      // r² = e^-0.005 — sharp peak, ~38 Hz wide
    let out = opengen_testkit::render_graph_with_inputs(
        graph, sr, &[in1, &in2, &in3, &in4, &in5], n,
    );
    out.ch(0).to_vec()
}

/// Faithful emulation of the SHIPPED patch: with bw = 0.005 (above the
/// silence threshold ≈ 0.00166 at ω for 440 Hz) the input coefficient is
/// exactly 0 and the output is exactly silent. This pins our engine to the
/// vendor patch's real behavior, sign bug included.
#[test]
fn exit_resonator_vendor_sign_bug_renders_silence() {
    let bytes = resonator_bytes();

    // Tripwire: if Cycling '74 ever fixes obj-30 upstream, this fails and the
    // 440 Hz assertion below should be re-aimed at the unmodified patch.
    let txt = String::from_utf8_lossy(&bytes);
    assert_eq!(
        txt.matches(RESONATOR_VENDOR_BUG_TEXT).count(), 1,
        "obj-30 no longer says `* 0.5` — upstream patch changed; re-evaluate \
         exit_resonator_sign_fixed_peaks_at_drive_freq against the vendor file"
    );

    let graph = opengen_gendsp::parse_gendsp_bytes(
        &bytes, None, &opengen_gendsp::LoadOptions::default(),
    ).expect("load gen_resonator.gendsp");

    let sr = 48_000.0;
    let n = sr as usize; // 1 second
    // White noise excitation — drives the filter the whole render.
    let mut in1 = vec![0.0; n];
    let mut rng: u64 = 0x0123456789ABCDEF;
    for sample in in1.iter_mut() {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *sample = ((rng >> 11) as f64) / (1u64 << 53) as f64 * 2.0 - 1.0;
    }
    let out = resonator_render(&graph, &in1, sr);
    assert!(
        out.iter().all(|&x| x == 0.0),
        "shipped gen_resonator should render exact silence for bw = 0.005 \
         (a ≡ 0); got nonzero output — did the loader or an op change?"
    );
}

/// The sign-FIXED resonator (obj-30 `* 0.5` → `* -0.5`, patched in memory —
/// reference/ is never modified) peaks within 10 Hz of the 440 Hz drive.
/// Excitation is a unit impulse after slide settling: deterministic, unlike a
/// single-realization noise periodogram whose peak-bin variance is comparable
/// to the 38 Hz resonance width.
#[test]
fn exit_resonator_sign_fixed_peaks_at_drive_freq() {
    let bytes = resonator_bytes();
    let fixed = String::from_utf8(bytes).expect("utf8")
        .replacen(RESONATOR_VENDOR_BUG_TEXT, RESONATOR_FIXED_TEXT, 1);
    assert!(fixed.contains(RESONATOR_FIXED_TEXT), "in-memory sign fix applied");

    let graph = opengen_gendsp::parse_gendsp_bytes(
        fixed.as_bytes(), None, &opengen_gendsp::LoadOptions::default(),
    ).expect("load sign-fixed gen_resonator");

    let sr = 48_000.0;
    let settle = 5_000usize; // slides (200/200) fully converged
    let fft_len = 32_768usize;
    let n = settle + fft_len;
    let mut in1 = vec![0.0; n];
    in1[settle] = 1.0; // unit impulse once params are settled
    let out = resonator_render(&graph, &in1, sr);
    opengen_analysis::assert_stable!(&out);

    // FFT of the deterministic ringdown.
    let mut buffer: Vec<Complex<f64>> = out[settle..]
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_len);
    fft.process(&mut buffer);

    let bin_spacing = sr / fft_len as f64;
    let peak_bin = (1..(fft_len / 2)) // skip DC and Nyquist bins
        .max_by(|&a, &b| {
            buffer[a].norm().partial_cmp(&buffer[b].norm()).unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(0);
    let peak_hz = peak_bin as f64 * bin_spacing;

    assert!(
        (peak_hz - 440.0).abs() < 10.0,
        "resonator peak at {peak_hz:.1} Hz, expected within 10 Hz of 440"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Stress — dattorro_plate
// ──────────────────────────────────────────────────────────────────────────────

#[test]
fn exit_dattorro_plate_stress() {
    let dang_tools = match corpus_env(
        "OPENGEN_DANG_TOOLS",
        "/Users/dangayle/Music/Ableton/User Library/Presets/M4L/dang-tools/patchers",
    ) {
        Some(d) => d,
        None => {
            eprintln!("exit_dattorro_plate_stress: SKIP — dang-tools patchers not found");
            return;
        }
    };

    let path = dang_tools.join("dattorro_plate.gendsp");
    if !path.exists() {
        eprintln!("exit_dattorro_plate_stress: SKIP — dattorro_plate.gendsp not found");
        return;
    }

    let opts = opengen_gendsp::LoadOptions {
        search_paths: vec![dang_tools.clone()],
    };
    let graph = opengen_gendsp::load_gendsp(&path, &opts)
        .expect("load dattorro_plate.gendsp");

    let sr = 48_000.0;
    let n = (2.0 * sr) as usize; // 2 seconds
    let out = impulse_render_graph(&graph, sr, n);

    // Output must be finite and denormal-free
    opengen_analysis::assert_stable!(out.ch(0));

    // Tail RMS should decay from early to late
    fn rms(samples: &[f64]) -> f64 {
        if samples.is_empty() { return 0.0; }
        let sum_sq: f64 = samples.iter().map(|x| x * x).sum();
        (sum_sq / samples.len() as f64).sqrt()
    }

    let half_sr = (0.5 * sr) as usize;
    let late_start = (1.5 * sr) as usize;
    let early_rms = rms(&out.ch(0)[0..half_sr]);
    let late_rms = rms(&out.ch(0)[late_start..n]);

    assert!(
        early_rms > late_rms,
        "dattorro plate: early RMS ({early_rms}) should exceed late RMS ({late_rms}) — tail must decay"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Corpus ratchets — GSOT (189 files)
// ──────────────────────────────────────────────────────────────────────────────

/// PINNED at 2026-06-10 for GSOT corpus (189 `.gendsp` files).
/// Cargo test: `cargo test -p opengen-analysis --release exit_gsot_corpus_ratchet`
///
/// Top remaining failure reasons (M3 backlog):
/// 1. Unknown operators (`+=`, `interp`, `t60`, `min`, `max` with >2 args) — ~15 files
/// 2. Multiple `gen`-started commands before first `out` (codebox parse order) — ~10 files
/// 3. Codebox identifier resolution (undefined `rt`, unbound locals across branches) — ~8 files
///
/// Use `OPENGEN_DANG_TOOLS`/`OPENGEN_FORS` env vars to override default paths.
/// All corpus tests skip cleanly when the corpus is absent.
const PINNED_GSOT: usize = 121;

#[test]
fn exit_gsot_corpus_ratchet() {
    let root = match reference_dir() {
        Some(r) => r.join("packages/gsot"),
        None => {
            eprintln!("exit_gsot_corpus_ratchet: SKIP — GSOT corpus absent (reference/ not available)");
            return;
        }
    };

    let gsot_dir = root.join("patchers");
    if !gsot_dir.exists() {
        eprintln!("exit_gsot_corpus_ratchet: SKIP — GSOT patchers dir not found at {}", gsot_dir.display());
        return;
    }

    let search_paths = vec![gsot_dir.clone()];
    let opts = opengen_gendsp::LoadOptions { search_paths };

    let mut gendsp_files: Vec<PathBuf> = std::fs::read_dir(&gsot_dir)
        .expect("read GSOT dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("gendsp"))
        .collect();
    gendsp_files.sort();

    let total = gendsp_files.len();
    let mut ok = 0usize;
    let mut failures: Vec<(String, String)> = Vec::new();

    for path in &gendsp_files {
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        match opengen_gendsp::load_gendsp(path, &opts) {
            Err(e) => {
                failures.push((name, format!("load: {e}")));
            }
            Ok(graph) => {
                // Compile and render 256 samples of silence
                let _out = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    opengen_testkit::render_graph_with_inputs(&graph, 48_000.0, &[] as &[&[f64]], 256)
                })) {
                    Ok(r) => r,
                    Err(e) => {
                        let msg = if let Some(s) = e.downcast_ref::<&str>() {
                            s.to_string()
                        } else if let Some(s) = e.downcast_ref::<String>() {
                            s.clone()
                        } else {
                            "unknown panic".to_string()
                        };
                        failures.push((name, format!("render: {msg}")));
                        continue;
                    }
                };
                ok += 1;
            }
        }
    }

    // Print failures
    for (name, reason) in &failures {
        eprintln!("  FAIL  {name}: {reason}");
    }
    eprintln!(
        "GSOT corpus: {ok}/{total} passed, {} failed",
        failures.len()
    );

    assert!(
        ok >= PINNED_GSOT,
        "GSOT corpus coverage dropped: {ok} passed, pinned at {PINNED_GSOT}"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Corpus ratchets — dang-tools (36 files)
// ──────────────────────────────────────────────────────────────────────────────

/// PINNED at 2026-06-10 for dang-tools corpus (36 `.gendsp` files).
/// Top remaining failure reasons (M3 backlog):
/// 1. Unknown functions: `selector`, `mix` with non-numeric args — ~3 files
/// 2. Abstraction calls inside control flow (unsupported in M2) — ~1 file
/// 3. Undefined identifier `rt` in codebox expressions — ~1 file
const PINNED_DANG_TOOLS: usize = 31;

#[test]
fn exit_dang_tools_ratchet() {
    let dang_dir = match corpus_env(
        "OPENGEN_DANG_TOOLS",
        "/Users/dangayle/Music/Ableton/User Library/Presets/M4L/dang-tools/patchers",
    ) {
        Some(d) => d,
        None => {
            eprintln!("exit_dang_tools_ratchet: SKIP — dang-tools patchers not found");
            return;
        }
    };

    let opts = opengen_gendsp::LoadOptions {
        search_paths: vec![dang_dir.clone()],
    };

    let mut gendsp_files: Vec<PathBuf> = std::fs::read_dir(&dang_dir)
        .expect("read dang-tools dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("gendsp"))
        .collect();
    gendsp_files.sort();

    let total = gendsp_files.len();
    let mut ok = 0usize;
    let mut failures: Vec<(String, String)> = Vec::new();

    for path in &gendsp_files {
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        match opengen_gendsp::load_gendsp(path, &opts) {
            Err(e) => {
                failures.push((name, format!("load: {e}")));
            }
            Ok(graph) => {
                let _out = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    opengen_testkit::render_graph_with_inputs(&graph, 48_000.0, &[] as &[&[f64]], 256)
                })) {
                    Ok(r) => r,
                    Err(e) => {
                        let msg = if let Some(s) = e.downcast_ref::<&str>() {
                            s.to_string()
                        } else if let Some(s) = e.downcast_ref::<String>() {
                            s.clone()
                        } else {
                            "unknown panic".to_string()
                        };
                        failures.push((name, format!("render: {msg}")));
                        continue;
                    }
                };
                ok += 1;
            }
        }
    }

    for (name, reason) in &failures {
        eprintln!("  FAIL  {name}: {reason}");
    }
    eprintln!(
        "dang-tools corpus: {ok}/{total} passed, {} failed",
        failures.len()
    );

    assert!(
        ok >= PINNED_DANG_TOOLS,
        "dang-tools corpus coverage dropped: {ok} passed, pinned at {PINNED_DANG_TOOLS}"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Corpus ratchets — Fors (embedded gen patchers in .amxd)
// ──────────────────────────────────────────────────────────────────────────────

/// PINNED at 2026-06-10 for Fors corpus (34 `.amxd` files with embedded dsp.gen patchers).
/// Top remaining failure reasons (M3 backlog):
/// 1. No dsp.gen sub-patchers in `.amxd` (genuinely non-gen devices: Slate, Opal, Opal-Ctl, Dust, etc.) — ~10 files
/// 2. Unknown operator `gen` (gen~ abstraction references like `gen lp`, `gen hp`, `gen env`) — ~6 boxes
/// 3. Unknown operators: `wave`, `ap`, `gate`, `gen` (various missing operators) — ~5 files
const PINNED_FORS: usize = 14;

#[test]
fn exit_fors_smoke_ratchet() {
    let fors_dir = match corpus_env(
        "OPENGEN_FORS",
        "/Users/dangayle/Music/Ableton/User Library/Presets/M4L/Fors",
    ) {
        Some(d) => d,
        None => {
            eprintln!("exit_fors_smoke_ratchet: SKIP — Fors directory not found");
            return;
        }
    };

    let _opts = opengen_gendsp::LoadOptions {
        search_paths: vec![fors_dir.clone()],
    };

    let mut amxd_files: Vec<PathBuf> = Vec::new();
    collect_files(&fors_dir, &mut amxd_files, "amxd");
    amxd_files.sort();

    let total = amxd_files.len();
    let mut ok = 0usize;
    let mut failures: Vec<(String, String)> = Vec::new();

    for path in &amxd_files {
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                failures.push((name, format!("read: {e}")));
                continue;
            }
        };

        // Parse embedded JSON (skips binary header)
        let j = match opengen_gendsp::json::parse_embedded(&bytes) {
            Ok(j) => j,
            Err(e) => {
                failures.push((name, format!("json: {e}")));
                continue;
            }
        };

        // Walk to find all dsp.gen sub-patchers
        let gen_patchers = extract_dsp_gen_patchers(&j);
        if gen_patchers.is_empty() {
            failures.push((name, "no dsp.gen sub-patchers found".to_string()));
            continue;
        }

        let mut patcher_ok = 0u32;
        for gen_json in &gen_patchers {
            let patcher = match opengen_gendsp::model::Patcher::from_json(gen_json) {
                Ok(p) => p,
                Err(e) => {
                    failures.push((name.clone(), format!("model: {e}")));
                    continue;
                }
            };

            match opengen_gendsp::build::build_graph(
                &patcher,
                &opengen_ops::Registry::core(),
            ) {
                Ok(graph) => {
                    match opengen_compile::compile(
                        &graph,
                        &opengen_ops::Registry::core(),
                        48_000.0,
                    ) {
                        Ok(_) => patcher_ok += 1,
                        Err(e) => {
                            failures.push((name.clone(), format!("compile: {e}")));
                        }
                    }
                }
                Err(e) => {
                    failures.push((name.clone(), format!("build: {e}")));
                }
            }
        }
        if patcher_ok > 0 {
            ok += 1;
        }
    }

    for (name, reason) in &failures {
        eprintln!("  FAIL  {name}: {reason}");
    }
    eprintln!(
        "Fors corpus: {ok}/{total} files with at least one buildable gen patcher, {} failed",
        failures.len()
    );

    assert!(
        ok >= PINNED_FORS,
        "Fors corpus coverage dropped: {ok} passed, pinned at {PINNED_FORS}"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Helpers for Fors
// ──────────────────────────────────────────────────────────────────────────────

/// Recursively collect files with a given extension.
fn collect_files(dir: &Path, files: &mut Vec<PathBuf>, ext: &str) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, files, ext);
            } else if path.extension().and_then(|e| e.to_str()) == Some(ext) {
                files.push(path);
            }
        }
    }
}

/// Walk a parsed JSON value to find all sub-patchers with classnamespace == "dsp.gen".
fn extract_dsp_gen_patchers(j: &opengen_gendsp::json::Json) -> Vec<opengen_gendsp::json::Json> {
    let mut result = Vec::new();
    extract_dsp_gen_patchers_recursive(j, &mut result);
    result
}

fn extract_dsp_gen_patchers_recursive(j: &opengen_gendsp::json::Json, result: &mut Vec<opengen_gendsp::json::Json>) {
    match j {
        opengen_gendsp::json::Json::Obj(pairs) => {
            let is_dsp_gen = pairs.iter().any(|(k, v)| {
                k == "classnamespace" && v.as_str() == Some("dsp.gen")
            });
            if is_dsp_gen {
                result.push(j.clone());
            }
            // Continue searching inside embedded patchers (gnm_patcher, patcher, etc.)
            for (_, v) in pairs {
                extract_dsp_gen_patchers_recursive(v, result);
            }
        }
        opengen_gendsp::json::Json::Arr(items) => {
            for item in items {
                extract_dsp_gen_patchers_recursive(item, result);
            }
        }
        _ => {}
    }
}
