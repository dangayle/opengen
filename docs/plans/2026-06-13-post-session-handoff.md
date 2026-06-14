# Post-Session Handoff — 2026-06-13

Paste everything below the line into a fresh session at `~/src/opengen`.

---

State: **master** with **substantial uncommitted work** — 34 files changed, +20K/−750 lines. `cargo test --workspace` = **all green** (zero failures). `cargo doc --workspace --no-deps` = **zero warnings**. Working tree is dirty — first action in the new session is review-and-commit.

**Read first:** `CLAUDE.md` (workflow contract, evidence/provenance rules), then this document fully.

## What was accomplished this session

### 1. Bitwise operators removed from opengen

**Verified against four independent sources:** live Cycling '74 GenExpr docs, `gen_common_operators.json` reference, real gen~ example corpus (zero code usage), and in-Max gen~ codebox compiler (rejected both `bitand(a,b)` and `a & b`). GenExpr has NO bitwise operators — only logical `&&`, `||`, `^^`, `and`, `or`, `xor`, `not`.

**Removed:**
- 5 kernel functions + OpDefs: `bitand`, `bitor`, `bitxor`, `shl`, `shr`
- `BinOpKind` variants from `ast.rs`
- Parser precedence methods: `parse_bitor`, `parse_bitxor`, `parse_bitand`, `parse_shift`
- Lexer tokens: `Amp`, `Pipe`, `Caret`, `Shl`, `Shr` (single `&`/`|`/`^` now error with clear message; `<<`/`>>` lex as two separate `Lt`/`Gt` tokens)
- Test coverage updated to assert these expressions are rejected
- Language reference (`genexpr_language_reference.md`): removed bitwise operator table, updated precedence, removed integer-conversion example using `&`
- Generator (`gen_op_sweeps.py`): removed INFIX table + bitwise entries + GEN_UNSUPPORTED

**Kept:** `samplerate` op (was in same file, moved to `samplerate.rs`), logical operators (`&&`/`||`/`^^`).

### 2. Full operator conformance — 71 codebox-supported operators with real gen~ goldens

**Generator built:** `tools/gen_op_sweeps.py` — registry-driven, emits one `.genexpr` sweep per operator with runtime-laundered inputs (History counter defeats gen~'s constant folder). Two modes: dense sweep for continuous ops, discrete points for discontinuous ops (avoids the `mod` knife-edge discovered earlier).

**Render kit hardened:**
- WAV headers now record the **true** DSP rate from `[dspstate~]` (no more 44.1k data in 48k headers)
- Golden filename includes rate: `<stem>.ch<N>.<sr>.wav` — renders at different rates coexist
- Single source-of-truth manifest: `conformance/render/patches.json` (generated alongside the host)
- Runner refuses `writewavs` if DSP was never toggled
- NaN and inf survive the float32 capture path faithfully

**Comparator improvements:**
- Float32 storage floor: `samples_agree()` adds `|g| * f32::EPSILON` to tolerance — large values like `mtof` (~2050) whose golden precision is limited by f32 storage no longer trigger false failures
- Discontinuity-count reporting: failures show `[X/4096 samples diverged]` — instantly distinguishes an isolated knife-edge from a systemic kernel bug
- NaN==NaN counts as agreement

**71/71 operators pass** (120 channel-checks, 0 failures). The 5 bitwise operators were removed (not gen~), not skipped.

### 3. dcblock kernel fixed — classic genlib form

Changed from "lazy x1-init to first input" (`y[0]` = 0) to genlib form: x1=0, y1=0 init (`y[0]` = 1.0 for an impulse). Confirmed against `dcblock_impulse` golden: `[1.0, -0.0003, -0.0002999, ...]`.

State slots reduced from 3→2 (removed first-sample flag).

`dcblock_step` (constant DC input) is a **known divergence**: gen~'s JIT constant-folds `dcblock(1.0)` to the steady-state (all zeros) at compile time. opengen has no constant folder — it runs the kernel and produces the classic step response. Documented as `# Divergence` in the dcblock rustdoc and exempted from conformance comparison via `KNOWN_DIVERGENCES`.

### 4. History read-after-write documented as divergence

gen~ History is write-through: reads after an assignment in the same sample see the NEW value (`[1, 2, 3, ...]`). opengen History is dataflow: all reads see the previous sample's value (`[0, 1, 2, ...]`). Confirmed via goldens at both 44.1k and 48k.

This is a deliberate architectural choice (dataflow semantics are cleaner for compilation). Documented as `# Divergence`, exempted from conformance via `KNOWN_DIVERGENCES`. The divergence probe and golden are committed for evidence.

### 5. Constant-folder-vs-kernel divergence (the session's key finding)

gen~'s JIT compiler has a compile-time constant folder whose edge-case behavior **diverges from its own per-sample DSP kernels**. This was known from M2 (`dcblock`, `clip`) but systematically mapped this session:

| Expression | gen~ folder | gen~ runtime | opengen (runtime) |
|---|---|---|---|
| `sqrt(-1)` (constant) | 0 | — | NaN |
| `sqrt(-(1+h*0))` (runtime) | — | NaN | NaN ✓ |
| `1/0` (constant) | 0 | — | NaN |
| `1/(h*0)` (runtime) | — | 0 (safediv) | inf (raw IEEE) |
| `log(0)` (runtime) | — | −inf | −inf ✓ |
| `dcblock(1.0)` (constant) | all zeros | — | step response |

**Rule adopted:** all conformance patches launder inputs through `h*0` (runtime constant) or a sweep ramp to force the runtime path. Constant-only patches (like `dcblock_step`) are documented as folder-vs-kernel divergences.

## What was NOT done (carried from original M3 handoff)

1. **build.rs comment-box skip** — `opengen-gendsp/src/build.rs` still fails on `maxclass == "comment"` boxes. One-line fix + regression test.
2. **Subpatcher binding propagation** — flatten's named-history binding fix only covers top-level patches. Bindings inside flattened subpatchers need the `sub<N>/` prefix.
3. **Ratchet climbing** — GSOT pinned at 121/189. No coverage improvement this session.
4. **Write the M3 plan** (`docs/plans/<date>-m3-cpp-emitter.md`) — the main deliverable. Not started.

## Files changed (uncommitted)

```
 M conformance/CHECKLIST.md
 R golden files renamed (*.ch<N>.wav → *.ch<N>.<sr>.wav)
 ?? 71 golden files added (op_*.ch0.<sr>.wav)
 ?? 71 op-sweep patches (conformance/patches/ops/op_*.genexpr)
 M crates/opengen-ops/src/filter.rs (dcblock kernel + state)
 D crates/opengen-ops/src/bitwise.rs → A crates/opengen-ops/src/samplerate.rs
 M crates/opengen-ops/src/lib.rs (module rename)
 M crates/opengen-ops/src/registry.rs (module rename)
 M crates/opengen-genexpr/src/ast.rs (remove 5 BinOpKind variants)
 M crates/opengen-genexpr/src/lexer.rs (remove 5 tokens, error on bitwise chars)
 M crates/opengen-genexpr/src/parser.rs (rewire precedence chain)
 M crates/opengen-genexpr/tests/parser_v2.rs (update bitwise tests)
 M crates/opengen-genexpr/tests/lower_v2.rs (update bitwise tests)
 M crates/opengen-genexpr/tests/lexer_v2.rs (update bitwise test)
 M crates/opengen-analysis/tests/conformance.rs (KNOWN_DIVERGENCES, f32-aware comparator, op-sweep test)
 A tools/gen_op_sweeps.py
 M tools/gen_render_host.py (ops/ dir scan, dspstate~ SR reporting, f-string counts, patches.json)
 M tools/validate_gendsp.js (ops/ dir scan)
 M conformance/render/render_runner.js (SR-aware headers, patches.json)
 M conformance/render/render_host.maxpat (regenerated)
 A conformance/render/patches.json
 M docs/research/gen_docs/genexpr_language_reference.md (remove bitwise)
 A docs/research/gen_docs/CLAUDE.md was touched
```

## Work queue for next session

### 0. Review and commit

This is a lot of uncommitted work. Review the diff, commit in logical chunks (bitwise removal first, then operator conformance, then render-kit hardening, then dcblock fix).

### 1. Small pre-M3 fixes (from original handoff, item 2)

- **comment-box skip**: one-line fix in `opengen-gendsp/src/build.rs` + regression test
- **subpatcher binding propagation**: TDD fix in `flatten.rs`

### 2. Write the M3 plan

Target: `docs/plans/<date>-m3-cpp-emitter.md`. Use the writing-plans skill. Fold in:
- C++ emitter architecture (bit-identical to Rust backend)
- Accumulated M3 backlog from the design doc (selector, gate, elapsed, wave, multi-channel data, require, …)
- The 4 vendor genexpr parse failures (76/80)
- The FMA/determinism constraint discovered this session (C++ and Rust backends must fold identically — or neither folds)
- The discontinuous-operator conformance methodology (sweep vs discrete points)
- `^^` precedence conformance cross-check
- Declaration-ordering strict mode / self-referential-history lint
- History read-after-write divergence decision

### 3. Ratchet climbing (optional, bounded)

Re-pin GSOT upward if any session fixes improved coverage.

## Rules (unchanged)

- `cargo test --workspace` green + `cargo doc --workspace --no-deps` warning-free before every commit
- TDD: failing test → observe → fix → observe
- Never weaken an exit assertion
- Conventional commits; push at milestones
- `reference/` never committed, no verbatim EULA text
- IEEE-754 f64 determinism contract
