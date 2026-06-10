# M1 Execution Handoff Prompt

Paste everything below the line into a fresh session at `~/src/opengen`.

---

Execute the M1 implementation plan for this repo end-to-end using subagent-driven development. Do not stop for design questions — all design decisions are already made and documented.

**Read first, in order:**
1. `docs/plans/2026-06-09-opengen-design.md` — the validated design. Pay particular attention to: Core Decisions table, Spec Provenance System, Testing (three rings), and Legal Posture (the `reference/` directory rules are hard constraints).
2. `docs/plans/2026-06-09-m1-vertical-slice.md` — the implementation plan. It is the authoritative task list: 14 tasks in 4 phases (A: foundation, B: thin vertical slice, C: state/oscillators/probes, D: analysis + exit criteria).

**Process:**
- Use the subagent-driven-development skill: fresh subagent per task, each receiving the task text from the plan verbatim plus pointers to both docs. Review each subagent's diff before accepting.
- Task order: Phases A and B are strictly sequential (Tasks 1–7 each depend on the previous). Within Phase C, Task 8's operator groups (math/compare/range modules) may run as parallel subagents after Task 7; Tasks 9 → 10 → 11 are sequential. Phase D is sequential (12 → 13 → 14).
- TDD is mandatory and the plan encodes it: write the failing test, run it, watch it fail, implement minimally, watch it pass, commit. Do not skip the "watch it fail" step.
- `cargo test --workspace` must be green before every commit. Conventional commit messages per the plan.
- At each CHECKPOINT (end of phases A, B, C), run the full workspace test suite plus `cargo doc --workspace --no-deps`, summarize state, then proceed.

**Hard constraints (from the design doc — violations are bugs):**
- `reference/` is gitignored and must never be committed. Never quote text from EULA-tagged reference files into code, comments, or docs — cite paths only (provenance tags). The MIT-licensed `reference/rnbo/core-mit` is the sole exception.
- Determinism: IEEE-754 f64 everywhere, no fast-math, spec'd evaluation order (topo order, ties by ascending NodeId), seeded PRNG for `noise`. Same patch + seed + inputs must produce bit-identical output.
- Operator rustdoc follows the provenance format: `# Definition` (the math, normative), then `# Documented` / `# Vendor` / `# Observed` / `# Divergence` headings with citations. Doctests are the spec — happy paths and meaningful boundaries in doctests, sad paths in `#[test]`.
- Core crates (`opengen-ir`, `opengen-ops`, `opengen-compile`, `opengen-genexpr`, `opengen-testkit`) take zero external dependencies. Only `opengen-analysis` (rustfft, hound, plotters) and `opengen-cli` (clap) may add deps.
- YAGNI: implement exactly what the plan's task scope says. M2 features (control flow, `.gendsp`, delay/buffer ops, conformance harness) are out of scope even if tempting.

**If a task is ambiguous or a subagent gets stuck:** consult the design doc; if still ambiguous, make the smallest spec-conformant choice, document it in the operator's rustdoc under the appropriate provenance heading, and note it in the final summary. Do not invent features to resolve ambiguity.

**Definition of done (the plan's final checkpoint):**
- All 14 tasks committed; `cargo test --workspace` green including doctests
- `cargo doc --workspace --no-deps` renders the operator spec readably
- M1 exit tests pass: one-pole lowpass −3 dB at ~1 kHz, 440 Hz `cycle` spectrum pure to −90 dB, probes record interior wires
- `git log` shows one commit per plan step, conventional messages
- Final summary lists: any `# Observed`/`# Divergence` notes added, any ambiguities resolved, and the open items carried to M2 (already listed at the bottom of the plan)

Begin with Task 1.
