# opengen — Design Document

**Date:** 2026-06-09
**Status:** Validated design (brainstorm complete)
**Repo:** new private GitHub repo, checked out at `~/src/opengen` (working title `opengen`; revisit name before going public — see Legal Posture)

## Vision

A white-room, open, extensible implementation of the gen~/GenExpr DSP language —
independent of Max/MSP. Goals:

1. **Reuse** the Max community's DSP work: run existing `.genexpr` and `.gendsp` code
2. **Testability first-class**: graph frequency/amplitude responses, assert DSP correctness
3. **Extensible language**: principled extensions beyond gen compatibility
4. **Multi-target export**: C++ first, others later
5. **Custom GUI/editor**: eventually, on top of the same core

Built for LLM agent plan/implement/test workflows: single language, single toolchain,
one-command verification, no compromises.

Testing audio/DSP is a fundamental goal of this project. With Max/MSP gen~, there's no deterministic way for an application to run and test any DSP functionality. The best you can do is write it, hope it compiles, then use your ears. This is fine if that is your workflow, but LLMs work much differently. They can write the code and do _some_ limited validation, but they currently cannot compile outside of the Max/MSP ecosystem, there's no way to step through the code, there's no way to validate that it sounds correct. With this project, we aim to make testing and testability of gen/gendsp/genexpr a first-class citizen.

## Core Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Implementation language | **Rust** (all-in-one workspace) | Type system catches agent mistakes pre-test; `cargo test` is the entire loop; WASM/CLI/native from one codebase. No Python sidecar — analysis is a first-class crate. |
| Spec format | **Executable specification**: rustdoc + doctests ARE the spec | Single source of truth; `cargo doc` renders the language reference; `cargo test` enforces it. Hybrid: happy paths + meaningful boundaries in doctests; sad paths/edge cases in `#[test]`/`tests/`. |
| Fidelity | **Spec-normative, gen~ as evidence** | Each operator gets a mathematical definition (IEEE-754 f64). gen~ conformance renders are cross-checks, not the definition. Divergences from Max documented, not copied. |
| Execution model | **Compile-to-Rust-closures from day one** (no interpreter) | Build it correctly first. Probes provide debuggability inside the compiled artifact. |
| Determinism | Bit-identical output everywhere | Spec'd evaluation order, IEEE-754 doubles, no fast-math, seeded PRNG for `noise`. Same patch + seed + inputs = same bits, all platforms, all backends. |

## Architecture

```
Frontends:   .genexpr parser   |   .gendsp JSON loader (M2)
                      ↓ lower to ↓
Core IR:     typed dataflow graph (spec-defined operator semantics)
                      ↓
Backends:    Rust closure compiler (run/test/probe)  |  C++ emitter (M3)
Support:     opengen-analysis  |  executable spec (opengen-ops)  |  conformance corpus
```

### Workspace layout

```
opengen/
  crates/
    opengen-ir/        # Typed dataflow IR, operator registry, type/shape checking
    opengen-ops/       # THE SPEC. One module per operator. Rustdoc = normative
                       #   definition (math, initial state, boundary semantics);
                       #   doctests = executable spec. cargo doc = language reference.
    opengen-genexpr/   # .genexpr lexer/parser → AST → lowering to IR
    opengen-gendsp/    # (M2) .gendsp JSON loader → IR (graphs + embedded codebox)
    opengen-compile/   # IR → Rust closure graph; probes; state mgmt; determinism
    opengen-emit-cpp/  # (M3) IR → dependency-free C++
    opengen-analysis/  # impulse/freq response, FFT, THD, spectrograms, SVG plots,
                       #   golden-WAV comparison, property-test helpers
    opengen-cli/       # opengen run/test/probe/plot/emit
  conformance/         # gen~ golden corpus: authored test patches + WAVs rendered
                       #   from real Max (evidence, not spec)
  reference/           # GITIGNORED. Extracted Max.app reference material (see below)
  tools/
    extract-max-refs.sh  # Declarative manifest: Max.app path → reference/, license tag
  CLAUDE.md            # Agent workflow: the spec→implement→test production line
```

`#[doc = include_str!("op.md")]` allows longer operator essays as standalone markdown
that are still doctested.

### IR and compilation model

- **IR**: typed dataflow graph, SSA-ish. All signals `f64` (gen's single type).
  Stateful operators (`history`, `delay`, `phasor`, `noise`, `data`) declare state
  explicitly: size, initial value, update semantics. Codebox control flow lowers to
  structured regions.
- **Compilation**: topological sort; feedback legal only through `history`/`delay`
  (cycle without one = compile error with good message). Output: `Patch` object with
  flat `Vec<f64>` state arena and per-sample `process(inputs) -> outputs` composed
  from monomorphized operator kernels. Buffer-at-a-time wraps the per-sample core.
- **Probes**: any IR edge taggable; compiler records tapped values per sample into
  ring buffers. CLI: `opengen probe patch.genexpr --tap "lpf.out" --samples 100`.
  Tests can assert on interior wires. Debugging is a compile feature.
- **Parameters**: `Param` nodes settable between buffers without recompile.
  Smoothing is the patch author's job (matches gen~).

## Spec Provenance System

Every behavioral claim in an operator's rustdoc carries a provenance tag under
standardized headings:

- **`# Documented`** — official gen~ docs/refpages. Cites source.
- **`# Clarified`** — stated by an authority (Graham Wakefield on C74 forums/Discord).
  Cites link, date, quote. Normative.
- **`# Vendor`** — informed by Max.app reference artifacts (RNBO operator source,
  PEG grammar, genlib export). Cites path + Max version + license status.
  NOTE: RNBO semantics occasionally diverge from gen~ — Vendor(RNBO) evidence is
  cross-checked against gen~ conformance WAVs.
- **`# Observed`** — established empirically via conformance harness. Cites the
  conformance test. Normative but inferred; superseded by Clarified.
- **`# Divergence`** — deliberate departure from gen~ (e.g., not copying a denormal
  bug). Cites rationale.
- **`# Extension`** — opengen-only behavior beyond the language.

Precedence on conflict: `Clarified` > `Documented` > `Vendor` > `Observed`.

Agent research protocol for ambiguous behavior: gen_docs → forums/Discord archive →
Max.app vendor references → write a conformance probe patch. The tag records which
path resolved it.

## Testing: Three Rings (all under `cargo test`)

1. **Spec doctests** (`opengen-ops`): per-operator analytical assertions via a small
   testing façade (2–4 line examples). Exact where math is exact (`phasor`, `wrap`,
   arithmetic); tolerance-based for transcendentals (`cycle` within 1 ulp of `f64::sin`).
2. **Analysis tests** (`opengen-analysis`): whole-patch frequency-domain properties.
   `impulse_response`, `freq_response` (`db_at`, `phase_at`, −3 dB helpers),
   `sine_sweep_thd`, `dc_offset`, `peak`, `rms`, aliasing detection (energy above
   nominal Nyquist of oversampled render). `proptest` properties (stability/finiteness
   across parameter ranges, no denormal stalls). Plot-to-SVG committed alongside tests
   so filter curves are reviewable in PR diffs. Reference values validated once
   against scipy at authoring time, hard-coded as constants with citation comments —
   no Python in the loop afterward.
3. **Conformance** (`conformance/`): authored test patches batch-rendered through real
   gen~ (Max project + script on the dev machine, offline/occasional). WAVs checked in;
   CI compares at per-operator tolerances. Divergences documented under `# Divergence`.
   Golden-file infra: `assert_render_matches!(patch, golden, tol)` with auto-bless mode.

## Max.app Reference Extraction

`tools/extract-max-refs.sh` with a declarative manifest (source path → dest, license
tag: `mit | eula-reference`). MIT items optionally vendorable; everything else
strictly gitignored, never committed, never quoted verbatim.

**Tier 1 — Operator semantics:**
- `C74/packages/RNBO/source/operators/` — 197 operator implementations in C74's
  internal TS-like DSL (phasor accumulator order, reset semantics, cycle, noise,
  delay, history, dcblock, slide, biquad...) + `characterconstants.json`.
  License: Max EULA → eula-reference. Provenance: `Vendor(RNBO)`.
- `C74/packages/RNBO/server/node_modules/@rnbo/genexpr_js/` — official PEG grammar
  (`genexpr.pegjs`, 832 lines), `operators.json` (155 signatures: types, defaults,
  constructor overloads), `operator_exprs.json` (108 ops defined as GenExpr),
  ~50 example `.genexpr` files. Proprietary (all rights reserved) → eula-reference.
- **genlib** (`genlib.cpp`/`genlib_ops.h`) — embedded in gen~ binary, written at code
  export. Routes: run one gen~ code export and stash output in `reference/`; and/or
  Cycling '74's public `gen-plugin-export` GitHub repo (ships the gen_dsp folder).
  This is actual gen~ (not RNBO) C++ semantics — top-tier evidence.

**Tier 2 — Documentation (`# Documented` source):**
- `C74/packages/Gen/docs/refpages/` — 189 per-operator maxref.xml (105 common,
  57 dsp, 27 jit): digests, constructor overloads, inlet types/defaults, attributes.
- `C74/docs/userguide/content/gen/*.json` — GenExpr guide chapters.

**Tier 3 — Test corpus:**
- `Resources/Examples/gen/` — ~60 official example patches + `.gendsp` (freeverb,
  biquad, band-limited saw...) → conformance corpus seeds.
- `C74/help/msp/gen~*.maxhelp`, `@rnbo/genexpr_js/genexprs/` — parser corpus.

**Tier 4 — Architecture references (design reference only):**
- `C74/packages/RNBO/source/rnbo/` C++ core — **MIT**, vendorable.
- `@rnbo/cpp-target`, `@rnbo/xam` (codegen pipeline) — proprietary, design reference
  for our emitter.

## Milestones

**M1 — Vertical slice (codebox core).** Workspace skeleton + `opengen-ir` +
`.genexpr` parser (expressions/assignment/params) + Rust closure backend + analysis
crate v1 + CLI (`run`, `plot`, `probe`) + extraction script. Operators:
arithmetic/comparison, `abs/min/max/clip/wrap/fold`, `history`, `phasor`, `cycle`,
`noise`, `mix`, `scale`. **Exit:** compile a one-pole lowpass + phasor-driven
oscillator from real `.genexpr`; frequency-response assertions pass; probes work.

**M2 — Language completion + `.gendsp` frontend.** Full GenExpr grammar (control
flow, functions, multi-out), `delay`/`data`/`buffer`/`peek`/`poke`, remaining
operator tiers, conformance harness + corpus. **Exit:** patches from dang-tools
`patchers/` (e.g., `dang.*.gendsp`) load and render within tolerance.

**M3 — C++ emitter.** Dependency-free C++ matching the Rust backend bit-for-bit
(shared determinism contract is the test). **Exit:** golden WAVs identical across
backends.

**M4+ — GUI/editor, other targets, extensions.** Deliberately unspecced (YAGNI).

**Agent production line** (codified in repo CLAUDE.md): after M1 skeleton, each
operator is an independent task: *research* (provenance protocol) → *spec* (rustdoc +
doctests) → *implement* (kernel in `opengen-ops`) → *verify* (`cargo test` + analysis
assertions) → *review*. Parallelizable across agents (operators only share registry
registration).

## Risks

- ~~No official EBNF~~ — resolved: official PEG grammar found in Max.app.
- `.gendsp` JSON undocumented — mitigation: generate minimal diff pairs from owned Max.
- RNBO vs gen~ semantic divergence — mitigation: conformance WAVs from real gen~ are
  the tiebreaker; provenance tags keep sources honest.
- Solo non-clean-room — mitigation: provenance discipline (facts, not expression);
  see Legal Posture.

## Legal Posture (not legal advice)

**Not protected (functional):** the language itself — syntax, grammar rules, operator
names, semantics (*SAS v. WPL*; *Oracle v. Google* fair-use holding); the
`.gendsp`/`.genexpr` file formats (interoperability); operator behavior observed via
black-box study on a licensed Max.

**Protected (expression):** Cycling '74's source and prose — `genexpr.pegjs`, RNBO
`operators/*.js`, refpage text, binaries. Exception: `source/rnbo/` core is MIT.
Trademarks: "Max," "gen~," "RNBO," likely "Gen."

**Rules if/when public:**
1. Repo contains original code only, MIT/Apache-licensed.
2. `reference/` extracts never committed, never quoted verbatim; docs paraphrased.
3. Provenance tags cite facts with sources — documented evidence we took behavior,
   not expression.
4. Non-confusing name (reconsider leading with "gen"); nominative description only:
   "an independent implementation compatible with the GenExpr language used by
   Cycling '74's gen~; not affiliated with or endorsed by Cycling '74."
5. Known gray zones: reading proprietary reference files weakens (doesn't destroy)
   clean-room defense — merger doctrine helps for grammar; Max EULA
   reverse-engineering clauses are contract risk, separate from copyright; patents
   unlikely but not excludable.

## Open Items

- **Final project name** before any public release.

### Corpus hierarchy

Conformance exit evidence (`crates/opengen-analysis/tests/m2_exit.rs`) uses three corpora:
- **Wakefield GSOT** (official gen~ examples, 189 `.gendsp` files) — primary corpus, deep assertions (parse + compile + render + analysis).
- **Fors .amxd** devices (34 files with embedded `dsp.gen` patchers) — secondary, parse + build smoke.
- **dang-tools** (36 `.gendsp` patchers) — stress: Delay multi-tap, declarator lists, complex routing.

Default paths live under `reference/` (gitignored). Override with `OPENGEN_DANG_TOOLS` / `OPENGEN_FORS` environment variables.

**Ratchet rule (D17):** each corpus test pins the observed pass count at commit time. The test fails only if coverage DROPS. Current pins: GSOT 121/189, dang-tools 31/36, Fors 14/34. When fixes raise coverage, re-pin upward.

### Human-in-the-loop (awaiting Max renders)

These items need real Max gen~ renders to produce golden WAVs (per `conformance/CHECKLIST.md`):
- **phasor** `# Observed` increment-order + range_inverted_bounds golden WAVs.
- **gen_resonator vendor sign bug:** shipped example `gen_resonator.gendsp` obj-30 computes `e^(+bw/2)` where the canonical two-pole needs `e^(−bw/2)`. Root-caused 2026-06-10 against opengen's deterministic engine. Full analysis + draft Cycling '74 report in [`docs/research/gen_resonator_sign_bug.md`](../research/gen_resonator_sign_bug.md). Pending: verify silence in real Max (load in gen~ host, drive with noise on in1, bw = 0.005, confirm silence) and submit the report upstream. See `conformance/CHECKLIST.md`.

### M3 backlog (accumulated during M2 execution)

- `selector`/`gate`/`elapsed`/multi-out operators
- abstraction calls inside control flow
- `wave` operator
- multi-channel data
- `require`
- Delay member calls inside regions
- early returns in functions
- lexer cursor-snapshot refactor (replace clone-lexer lookahead)
- for-init comma expressions
- 4 remaining vendor genexpr parse failures (comma contexts + named-arg-in-call; recorded at Checkpoint C: 76/80)
- peek/poke NaN + (−1,0)-index conformance items
- `^^` precedence conformance cross-check (the plan's ladder was mis-transcribed; vendor PEG won: `||` → `^^` → `&&` → `|` → `^` → `&`)
- declaration-ordering strictness: real gen~ rejects declarations after expression statements ("declarations must come before expressions", observed Max 9 2026-06-10; matches `docs/research/gen_docs/genexpr_ebnf.md` program order). opengen's parser is lenient — add a strict mode or lint warning so authored patches stay gen~-loadable
- C++ emitter (the M3 milestone itself)
