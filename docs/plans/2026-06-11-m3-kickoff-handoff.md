📌 Current branch: `master`
# M3 Kickoff Handoff — Conformance Closeout + M3 Plan

Paste everything below the line into a fresh session at `~/src/opengen`.

---

You are picking up opengen after the M2 release and its post-release conformance phase. State: master @ `6a7aee4` pushed, tag `v0.2.0-m2` published, `cargo test --workspace` = **547 passed / 0 failed / 0 ignored**, `cargo doc --workspace --no-deps` warning-free, conformance = **10/10** (9 PASS against committed Max goldens + 1 clean SKIP awaiting a render). Working tree clean.

**Read first:** `CLAUDE.md` (workflow contract, legal rules, the NEW Evidence & Conformance section — note the corrected genlib rule), then `conformance/CHECKLIST.md`, then this document fully. The design doc `docs/plans/2026-06-09-opengen-design.md` Open Items + M3 backlog is the authoritative work list.

## What happened since v0.2.0-m2 (context you must not re-derive)

1. **gen_resonator vendor sign bug**: root-caused (obj-30 `* 0.5` should be `* -0.5`), confirmed in real Max (silence as shipped / 440 Hz peak when fixed), reported to Cycling '74 via Discord. Exit tests pin both behaviors with an upstream-fix tripwire. Full analysis: `docs/research/gen_resonator_sign_bug.md`.
2. **Render kit rebuilt 4×** — it now WORKS; do not redesign it. v4 mechanics: GenExpr sources are EMBEDDED as codeboxes in dsp.gen subpatchers (gen~ cannot load `.genexpr` from box text — only `require()` for function libs); capture happens INSIDE each gen~ via `poke <buf> 0` (channel is ZERO-based!) at index `elapsed` — sample-aligned to patch t=0 by construction; float32 export via `writeraw <path> float32 4096 1` + Node-assembled WAV header (buffer~'s `write`/`writewave` are int16-only). Regenerate after any patch change: `python3 tools/gen_render_host.py` (reads `conformance/patches/`, currently 10 patches / 18 channels). Human flow: open patch → DSP ON ~1 s → OFF → click `writewavs`.
3. **17 authoritative gen~ goldens committed** (`conformance/golden/`, float32, alignment verified: `history_counter.ch0` = impulse at sample 0).
4. **Four ops corrected against the goldens** (TDD; conformance tests were the failing tests): `phasor` increment-then-output (first sample = f/sr); `cycle` cosine phase, lookup-then-increment (first sample = 1.0); `dcblock` lazy x1-init to first input (constant input → exact silence, Slots 2→3); `clip` = `max(min(x, hi), lo)` (inverted bounds pin to LO; NaN now yields hi). `wrap`/`fold` inverted-bounds semantics were CONFIRMED correct.
5. **Key epistemological finding** (now in CLAUDE.md): the in-Max gen~ JIT **demonstrably diverges from the genlib code-export runtime** (dcblock startup init, clip clamp order). genlib (`reference/genlib/gen_dsp/genlib_ops.h`, also mirrored in `../oopsy/source/gen_dsp/`) is strong but not authoritative; **conformance goldens from real Max win**. Record genlib disagreements under `# Divergence`.
6. **GenExpr dialect rules learned from Max** (both are opengen leniencies, documented in design-doc backlog): declarations must precede expression statements in gen~ codeboxes; self-referential `h = history(h + 1)` is rejected by gen~ (requires declared `History`, reads before write — gen~ History reads after an assignment see the NEW value, opengen always sees previous; authored patches read-first so both agree).
7. `impulse_response()` in opengen-analysis now injects its impulse after one discarded zero warm-up sample (so dcblock's first-sample init can't corrupt LTI measurements).

## Work queue, in order

### 1. Human-in-the-loop: render the `dcblock_impulse` probe (needs Max — ask the user)

`conformance/patches/dcblock_impulse.genexpr` distinguishes WHY gen~'s dcblock is silent for constant input:
- golden starts `[0, -1, -0.9997, …]` → **lazy x1-init** (current opengen implementation) confirmed
- golden starts `[1, …]` → **constant-folding** hypothesis wins → revert `dcblock` to the genlib form (x1=0 init), re-derive dcblock_step's explanation, update `# Observed`/`# Divergence` in `crates/opengen-ops/src/filter.rs`, and re-check the impulse_response warm-up rationale

Flow: user opens `conformance/render/render_host.maxpat` (already regenerated for 18 channels), DSP ON ~1 s → OFF → `writewavs` → `cargo test -p opengen-analysis --test conformance`. The test is already wired (`conformance_dcblock_impulse`, SKIPs until the golden exists). While the user has Max open, consider ALSO authoring + rendering probes for: clip/peek/poke **NaN** behavior, and the **History read-after-write** divergence (write a patch where a read follows the write and compare engines) — add patches to `conformance/patches/`, regenerate the host, extend conformance.rs, all BEFORE asking the user to render, so one Max session covers everything.

### 2. Small pre-M3 code tasks (TDD each; subagent-driven: implementer → spec-review → code-review)

- **build.rs comment-box skip**: `opengen-gendsp/src/build.rs::build_graph` fails on `maxclass == "comment"` boxes (`unknown operator 'fade'` on gen_resonator); `flatten.rs` skips them (~line 284). One-line fix + regression test. Longer term the two parallel builders should merge (architectural wart — this is how the binding bug happened); at minimum add the skip.
- **Subpatcher binding propagation**: flatten's named-history binding fix (2026-06-10) covers top-level patches only; bindings inside flattened subpatchers should get the `sub<N>/` prefix like params do (see `flatten.rs` ~line 802 and ~line 1132 copy loops). TDD: a fixture with a named history inside a subpatcher must be probeable as `sub0/<name>`.

### 3. Ratchet climbing (optional, bounded)

GSOT pinned 121/189 (`crates/opengen-analysis/tests/m2_exit.rs`). Each failure in the per-file summary is a candidate quick win; several map to M3 backlog items. Re-pin upward when fixes raise coverage (D17 rule). Corpora paths default under `reference/`; `OPENGEN_DANG_TOOLS`/`OPENGEN_FORS` override.

### 4. Write the M3 plan (use the writing-plans skill; this is the main deliverable)

Target: `docs/plans/<date>-m3-cpp-emitter.md`. The milestone is the **C++ emitter** — dependency-free C++ matching the Rust backend bit-for-bit (the shared determinism contract IS the test; golden WAVs identical across backends). Fold in the accumulated backlog from the design doc's "M3 backlog" section, including: `selector`/`gate`/`elapsed`/multi-out ops · abstraction calls inside control flow · `wave` · multi-channel data · `require` · Delay member calls in regions · early returns in functions · lexer cursor-snapshot refactor · for-init comma expressions · 4 vendor genexpr parse failures (76/80) · `^^` precedence conformance cross-check · declaration-ordering strict mode/lint · self-referential-history lint (same scope) · History read-after-write divergence probe + decision · peek/poke + clip NaN conformance items. Sequence the conformance-affecting items early (they may change goldens/specs that the C++ emitter must then match).

## Process contract (learned this session — follow it)

- **Subagent-driven development** (implementer → spec-reviewer → code-reviewer) for production-line work: op implementations, docs tasks, anything spec-shaped. Dispatch reviewers on the uncommitted diff BEFORE committing.
- **Orchestrator-direct** only for human-in-the-loop debugging (Max sessions, screenshot iteration) where context accumulates across user observations — and still run review passes before commit.
- **systematic-debugging skill** for any bug: root cause before fixes; the conformance suite is usually your failing test.
- Evidence protocol: check `reference/gen/refpages/` and the vendor corpus FIRST (this session: `elapsed`/`poke` refpages and the corpus's embedded-patcher JSON were decisive); the user's installed Max app bundle has the full msp refpages (`/Applications/Max.app/Contents/Resources/C74/docs/refpages/`) — that's where buffer~'s writeraw answer came from. Sibling repos: `../oopsy/source/gen_dsp/` (genlib mirror), `../max-sdk/`, `../min-api/`.

## Hard constraints (violations are bugs)
`reference/`, Fors, dang-tools never committed; no verbatim EULA text (cite paths + facts; `reference/rnbo/core-mit/` is the only quotable vendor code). IEEE-754 f64 determinism contract. TDD: failing test → observe → fix → observe. Never weaken an exit assertion. `cargo test --workspace` green + `cargo doc --workspace --no-deps` warning-free before every commit. Conventional commits; push at milestones.

Begin by asking the user whether they have Max available for the probe-render session (work item 1); if not, proceed to item 2 and prepare item 1's patches so the next Max session covers everything at once.
