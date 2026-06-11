# M2 Finish Handoff — Resonator Debug + Docs Closeout + Release

Paste everything below the line into a fresh session at `~/src/opengen`.

---

Finish the opengen M2 milestone. State: 29/31 plan tasks complete, master @ `38d98dc` pushed, `cargo test --workspace` = 543 passed / 0 failed / **1 ignored**, `cargo doc --workspace --no-deps` warning-free. Three things remain, in order: (1) fix the one blocked exit test, (2) Task 29 docs closeout, (3) Task 30 release. Use the systematic-debugging skill for (1) and subagent-driven development (implementer → spec review → quality review per task) for (2)–(3), per `docs/plans/2026-06-10-m2-language-and-conformance.md` and `CLAUDE.md`.

**Read first:** `CLAUDE.md` (workflow contract, legal rules — `reference/` is gitignored, NEVER committed; never quote EULA-tagged files), then the plan's Task 29/30 sections (~lines 1400–1429), then `crates/opengen-analysis/tests/m2_exit.rs` (the blocked test and its `#[ignore]` note).

## 1. The blocker: `exit_resonator_peaks_at_drive_freq`

**Symptom:** `opengen_gendsp::load_gendsp("reference/gen/examples/gen_resonator.gendsp")` produces a graph that renders **exactly zero output** for all inputs (an impulse at sample 600 produces nothing). The test is `#[ignore]`d with a note forbidding release without sign-off. The originally-reported "peak at 24 kHz" was FFT noise on silence — do not chase Nyquist theories.

**The test's input contract is already CORRECT (verified by tracing patchlines — do not re-litigate):**
- `in 3 freq` flows through a voice latch (`?` cond `==`(param id, in2) / `history` / `slide 200 200`) **directly into `cos`** — no 2π/sr scaling exists in the patch, so in3 expects normalized radians: `ω = TAU·440/sr ≈ 0.05760`.
- `in 5 bw` flows through `* -1 → exp`, i.e. `r = e^(−bw)`: the test drives `in5 = 0.005` (r ≈ 0.995, sharp peak).
- Expected after fix: spectrum peak within 10 Hz of 440. Do **not** weaken this assertion.

**Patch structure (traced 2026-06-10; box ids from the gendsp JSON):**
- freq latch: `in3 → ?(obj-38) → history(obj-5) → slide(obj-3) → cos(obj-10)`
- bw latch: `in5 → ?(obj-47) → history(obj-26) → slide(obj-49)`; then `×0.5(obj-30) → exp(obj-46)` (one r form) AND `×-1(obj-45) → exp(obj-17) → ×-1(obj-18)` (−r² form)
- amp latch: `in4 → ?(obj-39) → history(obj-42) → slide(obj-35)`
- gate: `param id → [< voices](obj-15, arg is the param NAME voices) → slide(obj-32)`
- b: `cos × exp(obj-46) = obj-13 → ×2(obj-14) → min 1.9999(obj-41)`
- **multi-cord**: `min 1(obj-40)` has TWO cords on inlet 0 (obj-14 and obj-18) — gen~ sums same-inlet cords; `= min(b − r², 1)`
- a: `!- 1(obj-20) ← obj-40`; c: `-(obj-43) = obj-40 − obj-41`
- y: `+(obj-28)` has THREE cords on inlet 0: `(x·gate)·a (obj-22)`, `y1·b (obj-23)`, `y2·c (obj-24)`; `obj-28 → history y1(obj-7) → history y2(obj-25)`
- out: `×(obj-31) = slide(amp) × sum → out 1`

**Already RULED OUT via mini-fixtures through the LOAD path** (`parse_gendsp_bytes`, all rendered correctly):
`lt` with param-name arg (`lt(0, voices=1)=1`) · `eq(param id 0, in 0)=1` · `?`-latch + history feedback + slide convergence · `slide 200 200` with the corpus's `numinlets:1` (Max collapses arg-filled inlets — corpus boxes really say numinlets 1) · `cos(slide(latch(ω))) → 0.99834` · named `history y1` 1-sample delay `[0,1,2]` · feedback core `y = 0.5x + 0.9·y1` with multi-cord `[1.0, 0.9, 0.81]` · THREE cords one inlet `= 9` · two cords into arg-filled `min 1` (numinlets 1) `= 1.0`.

**Unchased leads (start here):**
1. **Bindings are lost through the load path**: `compile_with_probes(&loaded_graph, …, &["y1","y2"])` → `"probe 'y1' not found in graph bindings"` even though the patch has `history y1`/`history y2` boxes. The flatten path likely drops `graph.bind` entries when copying nodes. Cosmetic for rendering but it BLOCKS probe-based debugging — fix it first (TDD: a loaded named-history fixture must be probeable), then probe y1/y2 and intermediate points in the full patch to find the dead stage.
2. **Two parallel builders exist** (architectural wart): `build.rs::build_graph` (used by fixture tests) vs `flatten.rs::build_graph_from_patcher` (used by `load_gendsp`). Both have multi-cord summation but can drift. `build_graph` currently FAILS on the resonator with `unknown operator 'fade' in box 'obj-33'` — it does not skip `maxclass == "comment"` boxes (flatten's builder does, ~line 284). Make build.rs skip comments (small fix + test), then **A/B the two builders' renders** on the resonator — if direct works and load doesn't, diff the graphs node-by-node.
3. Write a throwaway graph dump (nodes/kinds/args/edges/constants) of the loaded resonator and hand-verify against the structure table above. The composition kills the signal somewhere a mini-fixture doesn't reproduce — find which edge/constant differs from the patchline table.

When fixed: remove the `#[ignore]`, confirm the peak lands at 440 ± 10 Hz, run the corpus ratchets — counts may RISE (GSOT currently pinned 121/189, dang-tools 31/36, Fors 14/34 in `m2_exit.rs`) — re-pin to new observed values. Root-cause fixes go in the owning crate with their own regression tests and conventional commits. `cargo test --workspace` green + doc clean before every commit; push at milestones.

## 2. Task 29 — Documentation closeout (plan ~line 1400)

Per the plan, update:
- `docs/plans/2026-06-09-opengen-design.md` Open Items: remove resolved (testkit gaps, phase_at, inverted bounds, scalar ops, structured errors, genlib extraction); add corpus hierarchy (Wakefield primary / Fors secondary / dang-tools stress), `OPENGEN_DANG_TOOLS`/`OPENGEN_FORS` env vars, and the M3 backlog accumulated during execution: `selector`/`gate`/`elapsed`/multi-out ops · abstraction calls inside control flow · `wave` · multi-channel data · `require` · Delay member calls in regions · early returns in functions · lexer cursor-snapshot refactor (clone-lexer lookahead) · for-init comma expressions · 4 vendor genexpr parse failures (comma contexts + named-arg-in-call, recorded at Checkpoint C: 76/80) · peek/poke NaN + (−1,0)-index conformance items · `^^` semantics conformance cross-check (the plan's ladder was mis-transcribed; vendor PEG won — `||→^^→&&→|→^→&`) · phasor `# Observed` + `range_inverted_bounds` pending Max goldens (see `conformance/CHECKLIST.md`).
- `CLAUDE.md`: gendsp evidence protocol (corpus paths, D17 ratchet rule), genlib citation rule (`reference/genlib/gen_dsp/genlib_ops.h`, eula, facts only), `docs/research/gen_docs/` as citable in-repo research, the update-phase/deferred-port kernel contract, genbo machine-validation step (`tools/validate-with-genbo.sh`), and the gen~ multi-cord-summation semantic.
- `README.md`: add `opengen-gendsp` to the crate list.

Commit: `docs: M2 closeout — open items, corpus hierarchy, production-line updates`

## 3. Task 30 — Release (plan ~line 1409)

Only after the resonator test is green (or the user explicitly signs off otherwise):
```bash
cargo test --workspace && cargo doc --workspace --no-deps
git push origin master
git tag v0.2.0-m2 -m "M2: full GenExpr grammar, memory ops, .gendsp loader, conformance harness"
git push origin v0.2.0-m2
```
Final summary must list: `# Observed`/`# Divergence` notes added; ambiguities resolved (D-numbers + the `^^` correction); human-in-the-loop items awaiting Max renders (goldens per `conformance/CHECKLIST.md`); final ratchet numbers; M3 backlog.

## Hard constraints (violations are bugs)
`reference/`, Fors, dang-tools never committed; no verbatim EULA text. IEEE-754 f64 determinism contract. TDD: failing test → observe → fix → observe. Never weaken an exit assertion to pass. Workspace green + doc warning-free before every commit. Conventional commits.

Begin with lead #1 (binding propagation through flatten), announcing the systematic-debugging skill.
