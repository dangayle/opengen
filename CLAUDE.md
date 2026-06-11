# OpenGen Agent Workflow

## Project

opengen is an independent, open implementation of the GenExpr DSP language (used by Cycling '74's gen~), built from scratch in Rust. The **core principle**: testing and testability of DSP is a first-class citizen. Every behavior must be assertable by a machine—no "use your ears" workflows. With Max/MSP gen~, there's no deterministic way to run and test DSP functionality: you write it, hope it compiles, then listen. LLMs work differently: they can write code and validate it programmatically. opengen makes DSP machine-testable through frequency-domain analysis, impulse response assertions, golden-WAV comparison, and deterministic execution (bit-identical output across platforms).

Read `docs/plans/2026-06-09-opengen-design.md` (the full design document) and `docs/plans/2026-06-09-m1-vertical-slice.md` (the executed M1 plan) for comprehensive context.

## One-Command Verification

- `cargo test --workspace` is the entire loop. Must be green before every commit.
- `cargo doc --workspace --no-deps` must build warning-free. Rustdoc IS the spec.

## Workspace Map

- `opengen-ir` — Typed dataflow graph IR: nodes, ports, operators, explicit state declarations
- `opengen-ops` — **THE SPEC**. One module per operator. Rustdoc = normative definition (math, provenance, boundaries). Doctests = executable spec. This is the language reference.
- `opengen-genexpr` — `.genexpr` lexer/parser → AST → lowering to IR
- `opengen-gendsp` — `.gendsp`/`.maxpat`/`.amxd` JSON loader: patcher model, box-text classifier, graph builders (direct + subpatcher/abstraction flattening)
- `opengen-compile` — IR → Rust closure graph; topo sort, cycle detection, probes, deterministic execution
- `opengen-testkit` — Doctest façade: `render(src, sr, n_samples)` for spec examples
- `opengen-analysis` — Impulse/frequency response, FFT, spectrum analysis, golden-WAV comparison, plotting (rustfft, hound, plotters)
- `opengen-cli` — `opengen run/plot/probe` commands (clap)

**Zero-external-deps rule**: `opengen-ir`, `opengen-ops`, `opengen-genexpr`, `opengen-gendsp`, `opengen-compile`, `opengen-testkit` have no external dependencies. Only `opengen-analysis` (rustfft/hound/plotters) and `opengen-cli` (clap) pull in crates.

## The Operator Production Line

Every operator follows this workflow (from the design doc):

1. **Research** — Provenance protocol: start with gen~ docs (`reference/gen/refpages/`), then forums/Discord for clarifications, then Max.app vendor references (`reference/rnbo/operators/`, PEG grammar, genlib), finally write a conformance probe patch if ambiguous.

2. **Spec** — Write rustdoc in `crates/opengen-ops/src/<module>.rs` using the provenance heading structure:
   ```rust
   /// Operator description.
   ///
   /// # Definition
   /// Mathematical definition: exact formula, IEEE-754 semantics, boundary behavior.
   ///
   /// # Documented
   /// Cite: `reference/gen/refpages/<category>/gen_<category>_<op>.maxref.xml`
   ///
   /// # Vendor (if applicable)
   /// Cite: `reference/rnbo/operators/<op>.js` or PEG grammar. Note: RNBO semantics
   /// occasionally diverge from gen~; verify against conformance WAVs.
   ///
   /// # Observed (if applicable)
   /// Established empirically via conformance harness (M2). Cite the test.
   ///
   /// # Divergence (if applicable)
   /// Deliberate departure from gen~. State rationale (e.g., not copying a bug).
   ///
   /// # Extension (if applicable)
   /// opengen-only behavior beyond gen~ compatibility.
   ///
   /// ```
   /// use opengen_testkit::render;
   /// let out = render("out1 = <example>;", 48000.0, 1);
   /// assert_eq!(out.ch(0)[0], <expected>);
   /// ```
   ```
   **Precedence on conflict**: `Clarified` (authority statement) > `Documented` > `Vendor` > `Observed`.

3. **Implement** — Kernel signature: `fn(inputs: &[f64], state: &mut [f64], sr: f64) -> f64`. Register in the module's `defs()` with `OpDef { name, arity, state: StateDecl, auto_state_update, kernel }`.

4. **Verify** — `cargo test -p opengen-ops` (doctests + unit tests) + analysis assertions in `crates/opengen-analysis/tests/` for complex behaviors.

5. **Review** — `cargo doc --workspace --open` to inspect rendered spec; check provenance accuracy.

## Testing Rings

### 1. Spec Doctests (opengen-ops)
Per-operator analytical assertions via `opengen_testkit::render`. 2–4 line examples in rustdoc.
- **Exact equality** where math is exact: `phasor`, `wrap`, arithmetic (`+`, `*`, …)
- **Tolerance/ulp** for transcendentals: `cycle` within 1 ulp of `f64::sin`

Example (from `crates/opengen-ops/src/math.rs`):
```rust
/// Add two signals: `out = a + b`.
///
/// # Definition
/// IEEE-754 f64 addition. No saturation, no denormal handling.
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = 1.5 + 2.25;", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 3.75);
/// ```
pub fn add(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[0] + inputs[1]
}
```

### 2. Analysis Tests (opengen-analysis)
Whole-patch frequency-domain properties in `crates/opengen-analysis/tests/`:
- **`impulse_response(src, sr, n)`** — Render with unit impulse input
- **`freq_response(src, sr, nfft)`** → `Response` with `db_at(hz)`, `phase_at(hz)`
- **`spectrum(samples, sr)`** → `Spectrum` with `peak_hz()`, `db_at(hz)` relative to peak
- **Probes** — `compile_with_probes(&graph, reg, sr, &["name"])` then `patch.probe("name")`
- **Golden WAVs** — `assert_render_matches!(src, golden_path, tol)` with `OPENGEN_BLESS=1` to write

Example (from `crates/opengen-analysis/tests/m1_exit.rs`):
```rust
#[test]
fn exit_one_pole_lowpass_response() {
    let src = "Param g(0.12278); h = history(mix(h, in1, g)); out1 = h;";
    let h = freq_response(src, 48_000.0, 8192);
    assert!((h.db_at(1_000.0) + 3.01).abs() < 0.1);
    assert!(h.db_at(100.0) > -0.2);          // passband flat
    assert!(h.db_at(20_000.0) < -20.0);      // stopband falling
}
```

Example with probes (from same file):
```rust
#[test]
fn exit_probes_work_on_real_patch() {
    let src = "h = history(mix(h, in1, 0.12278)); out1 = h;";
    let graph = opengen_genexpr::parse_and_lower(src).expect("parse");
    let mut patch = opengen_compile::compile_with_probes(
        &graph, &opengen_ops::Registry::core(), 48_000.0, &["h"]).unwrap();
    
    for _ in 0..1000 { patch.process(&[1.0]); }
    
    let h_trace = patch.probe("h").unwrap();
    assert!(h_trace[h_trace.len() - 1] > 0.99); // converges toward 1.0
}
```

### 3. Conformance (M2)
Authored test patches batch-rendered through real gen~ in Max (offline/occasional). WAVs checked in; CI compares at per-operator tolerances. Golden-file infra auto-blesses with `OPENGEN_BLESS=1`. Divergences documented under `# Divergence`.

## Determinism Contract

- **IEEE-754 f64 only**: no fast-math, no denormal flushing
- **Topological order with NodeId tie-break**: spec'd evaluation order
- **Seeded PRNG**: `noise` uses xoshiro256++ with fixed seed (0x0123456789ABCDEF)
- **Guarantee**: Same patch + seed + inputs = identical bits, all platforms, all backends

Any nondeterminism is a bug. Tests rely on exact equality for deterministic operations.

## Evidence & Conformance (M2 additions)

### 1. Gendsp evidence protocol

Conformance uses three corpora (see `crates/opengen-analysis/tests/m2_exit.rs`):
- **Wakefield GSOT** (official gen~ examples, primary) — deep assertions: parse, compile, render, analysis.
- **Fors .amxd** (secondary) — parse embedded `dsp.gen` patchers + build smoke.
- **dang-tools** (stress) — complex routing, multi-tap Delay, declarator lists.

Corpus paths default to `reference/` (gitignored). Override with `OPENGEN_DANG_TOOLS` / `OPENGEN_FORS` env vars.

**Ratchet rule (D17):** each corpus test pins the observed pass count at commit time. The test fails only if coverage DROPS. Current pins: GSOT 121/189, dang-tools 31/36, Fors 14/34. Re-pin upward when fixes raise coverage.

### 2. genlib citation rule

`reference/genlib/gen_dsp/genlib_ops.h` is EULA-tagged (`eula-reference`). Cite path + facts only, never quote verbatim. It is top-tier evidence — actual gen~ C++ semantics — ranking alongside `# Documented` in the provenance hierarchy.

### 3. `docs/research/gen_docs/` citation rule

The `docs/research/gen_docs/` directory contains our own research prose (EBNF, language reference, Zod schemas). It is **in-repo, citable** — safe to quote. These documents were derived from Max reference materials but are original expression.

### 4. Update-phase / deferred-port kernel contract

Compiled execution follows a two-phase model (`crates/opengen-compile/src/lib.rs`, `crates/opengen-ops/src/registry.rs`):
1. **Compute phase** — all kernels (`Kernel` type) execute in topological order (NodeId tie-break). Kernels **MUST NOT** read input values arriving on `deferred_ports` (the "write" feedback ports of `history` port 0, `delay` port 0).
2. **Update phase** — all deferred `UpdateFn` callbacks run after every Compute step completes, in ascending NodeId order. These handle end-of-sample state writes (history push, delay shift).

This guarantees freedom from read-after-write hazards: all reads happen before any deferred write updates state.

### 5. genbo machine-validation step

Authored conformance patches must pass `tools/validate-with-genbo.sh` before golden rendering. This validates GenExpr syntax against Max's own `genbo.js` parser (no GUI needed). The script exits 0 when Max is absent, so it is CI-safe.

### 6. gen~ patcher semantics: inlet summation

Multiple patch cords into the same inlet are **summed** (verified M2 in both graph builders at `crates/opengen-gendsp/src/build.rs:331-352`). This matches gen~'s implicit signal summation.

## Legal Rules (Hard Constraints)

- `reference/` is gitignored, **NEVER commit it**
- **NEVER quote text verbatim** from EULA-tagged reference files (`reference/rnbo/operators/`, `reference/rnbo/genexpr_js/`, gen docs)
- Cite paths only: `# Vendor: reference/rnbo/operators/phasor.js`
- Exception: `reference/rnbo/core-mit/` is MIT-licensed, safe to quote/vendor
- Provenance tags cite **facts** (behavior, constants, formula), not expression
- Nominative description only: "an independent implementation compatible with the GenExpr language used by Cycling '74's gen~"

## TDD Discipline

1. **Failing test first** — Write the doctest or `#[test]` that specifies the behavior
2. **Watch it fail** — Run `cargo test -p <crate>` and confirm the expected failure
3. **Implement minimally** — Write just enough code to make the test pass
4. **Watch it pass** — Rerun and confirm green
5. **Refactor if needed** — Keep tests green

**Doctest scope**: happy paths + meaningful boundaries (exact values, edge cases like wrap at 1.0).
**Sad paths in `#[test]`**: error handling, cycle rejection, out-of-range, NaN propagation.

Example: `phasor` doctest shows exact ramp at 1000 Hz; unit tests in `#[cfg(test)] mod tests` cover high-freq wrap and negative frequencies.
