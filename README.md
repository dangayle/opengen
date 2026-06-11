# opengen

**Testing audio/DSP is fundamental to this project.** With Max/MSP gen~, there's no deterministic way for an application to run and test DSP functionality—you write it, hope it compiles, then use your ears. LLMs need more: they can write code and perform programmatic validation, but they cannot compile outside the Max/MSP ecosystem, cannot step through code, and cannot verify that it sounds correct. **opengen makes testing and testability of DSP a first-class citizen.** Every behavior is machine-testable through frequency-domain analysis, impulse response assertions, and bit-identical deterministic execution.

## What It Is

opengen is an independent, open implementation compatible with the GenExpr language used by Cycling '74's gen~. Built from scratch in Rust, it provides a complete pipeline from `.genexpr` source to compiled DSP patches with first-class testing and analysis tools.

**Not affiliated with or endorsed by Cycling '74.** This is an independent implementation — original code only, with documented provenance for every behavioral claim — focused on testability, extensibility, and reproducibility.

## Quick Start

```bash
# Run all tests (88 passing, all must stay green)
cargo test --workspace

# Build and read the spec (rustdoc = language reference)
cargo doc --workspace --open

# Render a patch and get stats
opengen run patch.genexpr --sr 48000 --samples 48000

# Render to WAV
opengen run patch.genexpr --sr 48000 --samples 48000 --wav output.wav

# Plot frequency response to SVG
opengen plot patch.genexpr --sr 48000 --response freq.svg

# Probe internal signal values
opengen probe patch.genexpr --sr 48000 --tap signal_name --samples 100
```

## Testing in Action

Write a one-pole lowpass filter in `.genexpr`:

```genexpr
Param g(0.12278);           // Coefficient for ~1 kHz cutoff at 48 kHz
h = history(mix(h, in1, g));
out1 = h;
```

Assert its frequency response programmatically:

```rust
use opengen_analysis::freq_response;

let src = "Param g(0.12278); h = history(mix(h, in1, g)); out1 = h;";
let response = freq_response(src, 48_000.0, 8192);

assert!((response.db_at(1_000.0) + 3.01).abs() < 0.1);  // -3 dB at cutoff
assert!(response.db_at(100.0) > -0.2);                   // Passband flat
assert!(response.db_at(20_000.0) < -20.0);               // Stopband rolls off
```

No guessing. No listening. Machine-verifiable correctness.

## Status

**M1 complete** (vertical slice):
- Core IR, operator registry with 25+ operators
- `.genexpr` parser (expressions, assignment, params)
- Rust closure compiler with deterministic execution
- Stateful operators: `history`, `phasor`, `cycle`, `noise`
- Analysis toolkit: impulse/frequency response, spectrum, golden-WAV comparison
- CLI: `run`, `plot`, `probe`

**M2 complete** (language completion + gendsp frontend):
- Full GenExpr grammar: control flow (if/for/while), functions, multi-out, declarator lists
- Memory operators: `delay`, `data`, `buffer`, `peek`, `poke` with update-phase semantics
- `.gendsp`/`.maxpat`/`.amxd` loader (`opengen-gendsp`) — patcher model, box-text classifier, graph builders with subpatcher + abstraction flattening
- Full operator coverage across math, trig, logic, range, route, memory, filter, sample tiers (~90 operators in 11 modules)
- Conformance harness + corpus ratchets (GSOT 121/189, dang-tools 31/36, Fors 14/34)
- Verifiable proof point: machine-testing found a sign bug in a shipped gen~ example (`gen_resonator.gendsp` obj-30) — see `docs/research/gen_resonator_sign_bug.md`
- `tools/validate-with-genbo.sh` for CI-safe GenExpr syntax validation using Max's own parser

**M3 roadmap**: C++ emitter (dependency-free, bit-for-bit matching the Rust backend).

## Documentation

- **Design document**: `docs/plans/2026-06-09-opengen-design.md`
- **M1 plan**: `docs/plans/2026-06-09-m1-vertical-slice.md`
- **Agent workflow**: `CLAUDE.md`
- **Spec (rustdoc)**: `cargo doc --workspace --open` → `opengen_ops`

## License

MIT OR Apache-2.0
