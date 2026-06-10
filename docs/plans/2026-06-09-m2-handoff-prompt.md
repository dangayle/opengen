# M2 Execution Handoff Prompt

Paste everything below the line into a fresh session at `~/src/opengen`.

---

Execute the M2 milestone for this repo end-to-end: first write the M2 implementation plan using the writing-plans skill, then execute it with subagent-driven development. Do not stop for design questions — design decisions are documented; where genuinely ambiguous, make the smallest spec-conformant choice, tag it with provenance, and note it in the final summary.

**Read first, in order:**
1. `CLAUDE.md` — the agent workflow contract. The core principle governs everything: testing/testability of DSP is first-class; every behavior must be machine-assertable (no "use your ears").
2. `docs/plans/2026-06-09-opengen-design.md` — the validated design. Pay particular attention to: the Vision testing paragraph, Core Decisions, Spec Provenance System, Testing (three rings), Milestones (M2 definition), Open Items (M1 carried these forward — they are M2 backlog), and Legal Posture (the `reference/` rules are hard constraints).
3. `docs/plans/2026-06-09-m1-vertical-slice.md` — the executed M1 plan. Use it as the FORMAT MODEL for the M2 plan you will write: phased tasks, verbatim TDD steps (failing test → observe → implement → observe → commit), checkpoints, exact commit messages.
4. Skim the M1 codebase: `crates/opengen-ops/src/osc.rs` (operator spec pattern incl. `# Observed`/`# Divergence`), `crates/opengen-compile/src/lib.rs` (Patch, state arena, `auto_state_update`, probes), `crates/opengen-genexpr/src/` (lexer/parser/lower), `crates/opengen-analysis/` (response, spectrum, WAV goldens), `crates/opengen-analysis/tests/m1_exit.rs`.

**Phase 0 — Write the plan with the writing-plans skill (before any implementation):**

Load and follow `/skill:writing-plans` properly — announce it, honor its boundaries (during planning, write ONLY to `docs/plans/`), and produce `docs/plans/<today's date>-m2-language-and-conformance.md`. That means:
- The skill's required plan header (Goal / Architecture / Tech Stack / REQUIRED SUB-SKILL line).
- Bite-sized steps (one 2–5 minute action each): write the failing test → run it, observe failure → minimal implementation → run, observe pass → commit. Exact file paths, complete code in the plan (not "add validation"), exact commands with expected output, exact conventional commit messages. Write for an engineer with zero context for this codebase: name the files to read, the APIs to call, the gotchas (e.g. `auto_state_update`, dev-dep cycles, `compile` reads `StateDecl` from IR nodes).
- It will exceed 8 tasks: split into phases with a checkpoint between each (M1's plan in `docs/plans/2026-06-09-m1-vertical-slice.md` is the format model — follow it).
- Before writing tasks, do the skill's research: read the exit-corpus `.gendsp` files and the PEG grammar to extract the REAL operator/grammar demand, and plan from that evidence, not guesswork.
- Commit the plan. Then take the skill's execution-handoff choice as pre-answered: **Subagent-Driven (this session)** — proceed directly into execution.

**Requirements brief for the plan** (scope inputs, not a task list — the plan decides ordering and decomposition, anchored to the design doc's M2 milestone and Open Items):
- **Testability quick wins** (design-doc Open Items; they serve everything after, so they likely come early): `render_with_inputs()` in testkit; `assert_stable!` (finiteness/denormal/DC/RMS bounds) in analysis; batch probe retrieval; `Response::phase_at` ±π wrap fix; a `# Definition` decision + tests for clip/wrap/fold inverted bounds (`hi < lo`).
- **Scalar math operators**: sin, cos, tan, asin, acos, atan, atan2, exp, log family, and trivial peers evidenced in the refpages — the Task-8 pattern from M1 (spec rustdoc + doctests; call syntax already parses).
- **Full GenExpr grammar**: control flow (`if`/`else`, `for`, `while`), user-defined functions, multi-statement bodies, comments, scientific notation, and whatever else the PEG grammar (`reference/rnbo/genexpr_js/genexpr.pegjs`, Vendor provenance — paraphrase, cite path, never quote) plus the corpus actually demand. Codebox control flow lowers to structured IR regions per the design doc. Structured error types with source locations replace the String errors.
- **Memory operators**: `delay` (taps, interpolation per refpage), `data`/`buffer`/`peek`/`poke`, `dcblock`, `slide`, plus whatever the exit corpus requires. State arena supports `Slots(n)`; if `data` needs runtime-sized state, extending `StateDecl` is a documented design decision.
- **`.gendsp` JSON loader**: new crate `crates/opengen-gendsp`, zero external deps (minimal JSON parser in-crate, or the plan justifies a dependency explicitly). `.gendsp` → IR for graph-style patches AND embedded codebox (reusing opengen-genexpr). Format is undocumented — derive structure from `reference/gen/examples/*.gendsp` (facts only, cite paths, `# Vendor`).
- **Conformance harness** (`conformance/`): authored ORIGINAL test patches (never copies of reference examples), Max-side batch-render script + human checklist (real gen~ renders are a human-in-the-loop deliverable), `assert_render_matches!`-based comparison at per-operator tolerances, skip-if-missing goldens so the suite stays green without Max. First question to encode: the phasor wrap/increment-order `# Observed` item in `crates/opengen-ops/src/osc.rs`.
- **M2 exit tests**: corpus `.gendsp` patches load, compile, and render finite, stable output, passing available conformance tolerances. Exit corpus: `~/src/dang-tools/patchers/dang.*.gendsp` if present on this machine; it currently is NOT — fall back to `reference/gen/examples/{crossover,freeverb,freeverb_comb,freeverb_allpass,gen_resonator}.gendsp` as the LOCAL corpus (read from `reference/` at runtime, never committed, cited by path only, tests skip cleanly when `reference/` is absent). Committed integration tests use authored patches only.

**Process (after the plan is committed):**
- Use the subagent-driven-development skill: fresh implementer subagent per task receiving the plan's task text verbatim plus pointers to the docs; spec-compliance review then code-quality review after each; fix-and-re-review loops until clean. Operator groups may be batched sequentially in one dispatch with one commit per group (parallel subagents race on shared files and the git worktree — don't).
- TDD is mandatory: failing test, watch it fail, implement minimally, watch it pass, commit. Doctests are the spec — happy paths and boundaries in doctests, sad paths in `#[test]`.
- `cargo test --workspace` green AND `cargo doc --workspace --no-deps` warning-free before every commit. Conventional commit messages. Checkpoint at each phase boundary: full suite + doc build + state summary.
- Push to `origin` (github.com/dangayle/opengen) at each checkpoint. Tag `v0.2.0-m2` at the end.

**Hard constraints (violations are bugs):**
- `reference/` is gitignored and must never be committed; never quote text from EULA-tagged reference files — cite paths only. `reference/rnbo/core-mit` is the sole MIT exception. Conformance patches are authored originals, never copied examples.
- Determinism: IEEE-754 f64, no fast-math, topo order with ascending-NodeId ties, seeded PRNG. Same patch + seed + inputs = bit-identical output. New control-flow lowering and `delay` interpolation must specify evaluation order explicitly.
- Operator rustdoc provenance format: `# Definition` (normative math), then `# Documented`/`# Vendor`/`# Observed`/`# Divergence` with citations to paths that exist on disk (verify before citing).
- Core crates (`opengen-ir`, `opengen-ops`, `opengen-compile`, `opengen-genexpr`, `opengen-testkit`, `opengen-gendsp`) take zero external dependencies. Only `opengen-analysis` (rustfft, hound, plotters) and `opengen-cli` (clap) carry deps.
- Testability first: every new language feature and operator ships with a machine-checkable assertion of its behavior. If a behavior cannot be asserted (e.g., needs a Max render), encode it as a skip-if-missing conformance test plus a written human checklist item — never as an untested claim.
- YAGNI: M3 (C++ emitter) and M4 (GUI) are out of scope. No SIMD, no buffer-at-a-time optimization beyond what exists, no new CLI subcommands unless an exit test needs one.

**Definition of done:**
- M2 plan committed; all its tasks committed with per-task review; `cargo test --workspace` green including doctests; `cargo doc` warning-free.
- Full GenExpr grammar parses the corpus codebox content; `.gendsp` loader loads the exit corpus; memory operators specified and tested; conformance harness merged with authored patches, Max render script + human checklist, and skip-if-missing goldens.
- M2 exit tests pass on this machine (reference corpus present).
- Design doc Open Items updated: resolved items removed, new findings recorded (including whatever the phasor question becomes).
- `CLAUDE.md` updated if the production line changed (e.g., gendsp evidence protocol).
- Pushed to origin with tag `v0.2.0-m2`. Final summary lists: `# Observed`/`# Divergence` notes added, ambiguities resolved, human-in-the-loop items awaiting Max renders, and open items carried to M3.

Begin with Phase 0.
