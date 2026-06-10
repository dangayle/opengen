# M2 — Language Completion + `.gendsp` Frontend + Conformance Implementation Plan

> **REQUIRED SUB-SKILL:** Use the subagent-driven-development skill to implement this plan task-by-task (pre-answered in the M2 handoff).

**Goal:** Full GenExpr grammar (control flow, functions, comments, sci-notation), the scalar math + memory operator tiers (`delay`, `data`/`peek`/`poke`, `dcblock`, `slide`), a zero-dependency `.gendsp` JSON loader that loads the reference example corpus, and a gen~ conformance harness — proven by M2 exit tests that load, compile, and render the reference corpus with finite, stable, frequency-asserted output.

**Architecture:** Builds on the M1 pipeline (`.genexpr` → `opengen-ir::Graph` → `opengen-compile::Patch`). M2 adds: port-level cycle-breaking in compile (deferred *write ports* instead of blanket stateful-node deferral); an `update`-phase kernel hook replacing `auto_state_update`; a structured-region IR (`NodeKind::Region(ProcRegion)`) for codebox control flow; named data regions (`NodeKind::Data` + `data_ref` on ops) for `peek`/`poke`; and a new `crates/opengen-gendsp` crate that flattens graph-style patchers (boxes/lines/subpatchers/send-receive/arg-expressions) into one `Graph`.

**Tech Stack:** Rust (edition 2024). Zero external deps in `opengen-ir`, `opengen-ops`, `opengen-genexpr`, `opengen-compile`, `opengen-testkit`, **`opengen-gendsp`** (in-crate minimal JSON parser). `rustfft`/`hound`/`plotters` stay confined to `opengen-analysis`, `clap` to `opengen-cli`.

**Reference design:** `docs/plans/2026-06-09-opengen-design.md` (read first: Spec Provenance System, Testing rings, M2 milestone, Open Items, Legal Posture). Format model: `docs/plans/2026-06-09-m1-vertical-slice.md`.

---

## Research Evidence (gathered 2026-06-10; plan from evidence, not guesswork)

### Corpus hierarchy (user decision, 2026-06-10)

**PRIMARY — Graham Wakefield's code** (the gen~ author; canonical idiomatic gen~, predominantly **graph-style**, not codeboxes):
1. **GeneratingSoundAndOrganizingTime** (`~/Documents/Max 9/Packages/GeneratingSoundAndOrganizingTime`, Wakefield & Taylor): **197 graph-style `.gendsp`**, only 6 codeboxes. High-frequency box ops beyond M1: `switch` (146), `fixnan` (114), `latch` (87), `or` (42), `neg` (37), `!-`/`!/`, `int` (17), `exp2`, `triangle` (12), `accum` (11), `bool` (10), `change` (8), `round`, `interp`, `gate`, plus `go.*` abstractions (loader needs a **search-path** list) and bare numeric constant boxes (`0`, `1`). No license declared → reference-only tier (extract to `reference/`, never commit, cite paths).
2. **`reference/gen/examples/`** official examples (partly Wakefield — freeverb credits him), incl. the deep-assertion exit set `{crossover,freeverb,freeverb_comb,freeverb_allpass,gen_resonator}.gendsp`.
3. **oopsy examples** (`~/Documents/Max 9/Packages/oopsy/examples/*.maxpat`, Wakefield): gen~ embedded in maxpat — same JSON shape; corroborating corpus, not an exit gate.

**SECONDARY — Fors** (`/Users/dangayle/Music/Ableton/User Library/Presets/M4L/Fors`, commercial M4L devices by Fors/Trent Gill+: 30/30 devices use gen~; **189 embedded `dsp.gen` patchers, 85 codeboxes** inside `.amxd` containers — binary header + maxpat JSON). Gen-level op demand adds: **`sah` (282)**, **`delta` (172)**, **`r`/`s` send-receive aliases (215/89)**, **`expr` boxes (66)**, `mstosamps`, `wave` (24, buffer wavetable reader — M3 backlog), `poke` (21), **`setparam` (20)**, `gate` (11), `dcblock` (6), numeric-constant boxes. Codebox demand: History (53), function defs (53), if (19), ternary (8), for (7), Buffer (6), Delay (3). Proprietary → read-in-place smoke corpus, never extracted/committed.

**STRESS — dang-tools** (`/Users/dangayle/Music/Ableton/User Library/Presets/M4L/dang-tools/patchers`, the user's own library; explicitly NOT primary — it uses uncommon, codebox-heavy patterns; useful precisely because it stresses the grammar): 36 `.gendsp`, 34 codeboxes, ~134 KB GenExpr. Demand (files-using counts): comments (34), `History` decls (32, **comma-separated declarator lists** with init args: `History a(0), b(0);`), ternary (26), `samplerate` (24), user function defs (14), `if`/`else` (11/10), `&&` (10), compound assignment (8), `||` (6), sci-notation (5), `return` (4), multi-assign (3), `for` (3), `Data` + `peek`/`poke` (2 files, 160/270 call sites — voice allocators drive `poke` **inside `if`/`for` bodies**), `Delay` decls + `.read()`/`.write()` member calls (2 files; `dattorro_plate.gendsp`: 24 reads / 17 writes over 16 delay objects → **multi-tap reads required**; verified: neither Delay-decl codebox uses control flow), **`.gendsp` files called as functions from codeboxes** (`dang_01iv_env(...)` etc.). Operator calls: `clamp` (79), `int` (54), `exp`, `max`, `pow`, `noise`, `sin`, `cos`, `mix`, `wrap`, `sign`, `log`/`ln`, `sqrt`, `tanh`, `tan`, `floor`, `fract`, `xor`, `triangle`. Box-level: `setparam <name>` (7), `mc_channel` (1). Read via `OPENGEN_DANG_TOOLS` env var (default that path), skip-if-missing.

**User-authored gen docs** (`dang-tools/docs/gen_docs/`, ~100 KB — Task 0 copies them into `docs/research/gen_docs/` so they are citable in-repo, **EXCLUDING `genexpr_grammar.pegjs`**, which is a direct copy of Cycling '74's proprietary grammar and must never be committed; the derived `genexpr_ebnf.md` IS user-authored and fair to vendor — user ruling 2026-06-10): GenExpr language reference (34 KB), GenExpr EBNF (derived), gendsp EBNF + Zod schema, and `validate_gendsp.js` — a validator that drives **Max's own genbo parser** offline via Max's bundled Node. Key facts adopted: codebox file structure order = compiler commands (`require`) → function declarations → typed declarations → body statements; `Param name(default, min=lo, max=hi)` named attribute args; declarator lists; **abstraction-as-function**: a `.gendsp` filename on the search path is callable from GenExpr — positional args map to `in N`, named args (`cutoff=1000`) drive `Param`s by name, multi-return destructures `out N`, and **state is per call site**. (Note to verify during execution: the docs state `Data name(channels, size)` — check argument order against the official refpage before implementing.)

**Deep-assertion exit set — reference examples** (graph-style, zero codeboxes; `patcher.boxes[].box` + `patcher.lines[].patchline`). Box text demand:

| Box text pattern | Meaning | Needed support |
|---|---|---|
| `+`, `-`, `*`, `/`, `* 0.5`, `+ 1`, `- 1.` | binary math, constant args fill trailing inlets | loader arg-filling; `1.` literal |
| `!- 1`, `!/ samplerate` | reverse sub/div with arg | `rsub`, `rdiv` kernels; arg expressions |
| `* twopi/samplerate`, `+ spread`, `< voices` | args are *expressions* over builtin constants and param names | expr-arg parsing via opengen-genexpr |
| `history`, `history y1` | optionally named single-sample delay | named history binding |
| `delay 2000`, `delay 44100` | delay line, arg = max size (samples), 2 inlets (signal, tap time) | `delay` op, node-level state size |
| `slide 200 200` | slide up/down args | `slide` op |
| `cos`, `sin`, `exp`, `min 1`, `min 1.9999`, `==`, `?` | scalar ops, switch | trig/exp kernels, `switch` |
| `param freq 600 @min 1 @max samplerate/2`, `param id 0` | param with default + attrs | loader param parsing (@min/@max parsed, ignored at runtime in M2) |
| `in 1 @comment input`, `in 5 bw`, `out 2 @comment hi` | I/O with comment/name | loader in/out parsing |
| `f 556` | constant box | loader constant |
| `send fb` / `receive fb` (1 send, 8 receives) | wireless bus | loader bus resolution |
| `gen @file freeverb_comb` (one instance embeds a `patcher`), bare `freeverb_comb` | subpatcher / abstraction | flattening + sibling-file resolution |

**Grammar demand** — PEG grammar `reference/rnbo/genexpr_js/genexpr.pegjs` (Vendor; paraphrased facts only) has rules for: line/block comments, function declarations, if/else, while, do-while, numeric for, break/continue/return, ternary conditional, assignment operators, multiple assignment, member call expressions, exponent-part numeric literals, logical (`&& || ^^ !`) and bitwise (`& | ^ << >>`) operators. The 80 vendor `.genexpr` examples (`reference/rnbo/genexpr_js/genexprs/`) use: `History` decls (53 files), `Param` (31), line comments (27), `samplerate` (26), function defs (24), `Delay` decls (23), multi-assign (19), `return` (19), ternary (17), `if` (14), `+=` (13), `else` (9), bitwise (6), `Buffer` (6), `Data` (5), `&&` (5), `!` (5), `||` (3), block comments (3), `twopi` (3), `for` (3), `while` (1).

**genlib found locally** — `oopsy/source/gen_dsp/{genlib.h,genlib_ops.h,genlib_exportfunctions.h}` (same Packages dir). NOTE: oopsy's repo LICENSE is MIT (Wakefield), but the `gen_dsp` headers carry the **"Cycling '74 License for Max-Generated Code for Export"** (non-commercial restriction) → **eula-reference tier**: extract to `reference/genlib/`, never commit, never quote; paraphrase facts and cite paths. This resolves the design-doc Open Item "genlib export … deferred from Task 2" and supplies top-tier gen~ semantics evidence used by D4/D7 below (cited as `# Vendor` with `reference/genlib/gen_dsp/genlib_ops.h`). Other packages surveyed (gen-filters, smFilterPack, Gen CV Tools, OWL, MOD Duo): no licenses declared; not needed for M2.

**Refpages verified on disk** for every operator cited below (`reference/gen/refpages/{common,dsp}/gen_*_<op>.maxref.xml`). `operators.json` (Vendor) confirms signatures/defaults cited per task.

**M1 defect found during research (fixed by Task 6):** `opengen-compile` defers *all* edges into stateful nodes for cycle-breaking. Consequence: `phasor`'s freq input can be read one sample stale (see the workaround comment in `crates/opengen-ops/src/osc.rs::phasor_negative_freq_wrap`), and a feedforward `delay` would read stale inputs. M2 moves to **port-level deferral**.

---

## Design Decisions (made now; tagged in code with provenance)

- **D1 — Port-level cycle breaking.** `OpDef` gains `deferred_ports: &'static [u16]` — input ports whose incoming edges are non-blocking for topo sort (the "write" ports of `history` (port 0) and `delay` (port 0)). All other edges block. Fixes the M1 phasor staleness artifact (tests updated). Kernels must not read input values arriving on deferred ports; those are update-phase-only.
- **D2 — Update phase.** `OpDef.auto_state_update: bool` is replaced by `update: Option<UpdateFn>` where `pub type UpdateFn = fn(&[f64], &mut [f64], f64);`. After all Compute steps, Update steps run **in ascending NodeId order** (determinism contract extension). `history`: copy `inputs[0] → state[0]`. `delay`: ring-write + cursor advance.
- **D3 — State init.** `OpDef` gains `init: Option<fn(args: &[f64], state: &mut [f64], sr: f64)>`, called once at compile with the IR node's `args`. Used by `history(init)` and `slide @init` later; arena default stays zero.
- **D4 — `clip`/`wrap`/`fold` inverted bounds (`hi < lo`).** Settled from genlib facts (`# Vendor`, cite `reference/genlib/gen_dsp/genlib_ops.h`): **`clip`/`clamp` do NOT swap** — they are literally `min(max(x, lo), hi)`, so inverted bounds pin the output to the second bound (e.g. `clip(x, 1, 0) == 0` for all x). **`wrap` and `fold` DO normalize** by swapping (`lo' = min`, `hi' = max`); equal bounds return `lo`; `wrap` additionally returns `lo` when the (normalized) range is `<= 1e-9` (genlib guard — adopt it verbatim as a fact, cited). An authored conformance patch (Task 26) upgrades these to `# Observed` once Max renders land.
- **D5 — Eager evaluation.** Ternary `c ? a : b` lowers to the `switch` op; `&&`/`||` lower to `and`/`or` kernels. Both branches/operands are always evaluated (dataflow semantics; expressions are side-effect-free in graph position). Documented under `# Definition` on `switch`/`and`/`or`.
- **D6 — Structured regions.** Codebox control flow lowers to a single `NodeKind::Region(ProcRegion)` node embedded in the dataflow graph (per the design doc). Region locals are zero-initialized per sample; `History` inside a region is region state; stateful op calls inside regions get region-state sub-ranges; **`peek`/`poke`/`Data` references ARE supported inside regions** (the dang-tools voice allocators demand `poke` inside `if`/`for` — region call sites carry resolved absolute arena ranges). `Delay` member calls (`.read`/`.write`) inside control-flow codeboxes are rejected with a clear error (M3 backlog — corpus-verified safe: no dang-tools codebox mixes the two). Probes are graph-level only (region interiors not probeable in M2 — documented limitation).
- **D7 — `delay` is a named buffer with separate tap readers (multi-tap REQUIRED — dattorro_plate taps tank delays repeatedly).** IR shape: one writer node (`NodeKind::Op "delay_write"`, deferred port 0, update-phase ring write) owning the buffer via a named state region (same mechanism as D8 `data`), plus one `"delay_read"` tap node per `.read(t)` / per graph-box tap, carrying `data_ref` to the buffer. genlib facts adopted (`# Vendor`, `reference/genlib/gen_dsp/genlib_ops.h`, struct Delay): reads are scheduled before the write (writes happen in the update phase → automatic), minimum effective delay for read-before-write is **1 sample**, tap time clamped to `[1, maxdelay]`; `@interp linear` (default): split `t` into integer `i = floor(t)` and `frac`, output `s(i) + frac·(s(i+1) − s(i))` where `s(k)` = sample written `k` samples ago; `@interp none`: nearest sample via `floor` after a half-sample offset (genlib read_step). Other interp modes error in M2. Buffer size = declared size (genlib rounds to next power of two internally — allocation detail, not semantics; noted, not copied).
- **D8 — `data`/`peek`/`poke`.** Single channel in M2. `poke` replaces (overdub backlog). Out-of-range `peek` reads 0; out-of-range `poke` writes nothing (`boundmode ignore`, the documented default). Shared state: `NodeKind::Data { name, size }` allocates the arena range; `peek`/`poke` ops carry `data_ref: Option<String>` and are compiled with the data node's state range. Within-sample read/write ordering = topo order with ascending-NodeId ties (the existing determinism contract; documented on both ops).
- **D9 — `.gendsp` loader semantics.** `param @min/@max` parsed but ignored at runtime (M2 params are compile-time defaults). `send`/`receive` resolve per-patcher scope. Abstraction resolution: embedded `patcher` first, else `<name>.gendsp` sibling file of the loaded patch. Subpatcher param/binding names are prefixed (`sub<N>/name`) to avoid collisions.
- **D10 — `buffer`.** Alias of `data` (`# Divergence`: opengen has no external host providing `buffer~`).
- **D11 — User functions inline at AST level.** Fresh renamed locals per call site; `return a, b;` + multi-assign supported; recursion → structured error.
- **D12 — `StateDecl` unchanged.** `data`/`delay` sizes are literals known at lowering, so `StateDecl::Slots(n)` on the IR *node* (which compile already reads) suffices. No runtime-sized state needed in M2 — recorded as the documented design decision the handoff asked for.
- **D13 — `setparam`.** A `setparam <name>` box drives the named param from a signal. Loader transform: consumers of param `<name>` are rewired to the `setparam` box's input source (the `Param` node remains for its default but is shadowed). If this rewiring creates an illegal cycle, the normal compile error surfaces. Documented as loader semantics (`# Vendor`: observed box shape `setparam release`, 1-in/1-out, `dang-tools/patchers/perfourmer_voice.gendsp`).
- **D14 — `mc_channel`.** Stubbed as constant `1.0` (`# Divergence`: opengen has no mc context; single-instance rendering).
- **D15 — Binding shadowing.** In identifier position, local/param/declared names shadow operator names (dang-tools declares `Param mix(0.33)` alongside uses of the `mix` op in call position). Call position resolves user functions first, then abstraction files (D16), then registry ops; identifier position resolves bindings only.
- **D16 — Abstraction-as-function.** A call to an unknown name resolves against the loader search path as `<name>.gendsp`; the abstraction's lowered graph is **inlined per call site** (giving the documented per-call-site state for free, identical to function inlining). Positional args feed `in N`; named args (`cutoff=1000`) override the named `Param` defaults; multi-return destructuring maps `out N`. Source: user gen docs (`docs/research/gen_docs/genexpr_language_reference.md` §12/§15); demanded by dang-tools codeboxes. M2 scope: supported in straight-line codebox/graph contexts; inside control-flow regions → clear error (M3 backlog). `require "file"` compiler commands: parsed, clear “unsupported in M2” error (zero corpus demand).
- **D17 — Corpus exit gates are ratchets.** Deep assertions run on the 5-file reference exit set. The three big corpora (GSOT 197, dang-tools 36, Fors-extracted gen patchers) run as load+compile+render-stable **coverage ratchets**: the test pins the observed pass count at implementation time and fails if coverage ever drops; failures print a per-file summary. All corpus tests skip cleanly when the corpus is absent.

**Conventions for every task** (same as M1):
- TDD: failing test → observe failure → minimal implementation → observe pass → commit. Doctests in `opengen-ops` are spec; sad paths in `#[test]`.
- `cargo test --workspace` green **and** `cargo doc --workspace --no-deps` warning-free before every commit.
- Conventional commits. Operator rustdoc: `# Definition`, then `# Documented`/`# Vendor`/`# Observed`/`# Divergence` with citations to paths verified on disk. **Never quote** EULA-tagged reference text; cite paths only.

---

## Phase A — Testability quick wins + scheduling foundations (Tasks 0–6)

### Task 0: Reference extraction — local packages + genlib

**TDD scenario:** Script/manifest change — verify by running; no unit tests.

**Files:**
- Modify: `tools/extract-max-refs.sh` (support absolute source roots)
- Modify: `tools/max-refs.manifest`

**Step 1:** Extend the script so a manifest source starting with `/` or `~` is used as-is (instead of being joined to `MAX_RES`). Keep `rsync -a --exclude node_modules`, expand `~` via `$HOME`.

**Step 2:** Append to `tools/max-refs.manifest` (tab-separated, same columns):

```text
eula	~/Documents/Max 9/Packages/GeneratingSoundAndOrganizingTime	packages/gsot
eula	~/Documents/Max 9/Packages/oopsy/source/gen_dsp	genlib/gen_dsp
```

(GSOT: no license declared → reference-only. oopsy `gen_dsp`: headers carry the Cycling '74 export license (non-commercial) despite oopsy's MIT repo license → eula tier. dang-tools is NOT extracted — it is the user's own corpus, read in place via `OPENGEN_DANG_TOOLS`.)

**Step 3:** Run `./tools/extract-max-refs.sh`. Expected: `reference/packages/gsot/` (~197 gendsp) and `reference/genlib/gen_dsp/{genlib.h,genlib_ops.h,genlib_exportfunctions.h}` exist; `git status` shows only `tools/` changes (`reference/` stays ignored). Fors and dang-tools are deliberately NOT extracted — tests read them in place, skip-if-missing.

**Step 4:** Copy the user-authored gen docs into the repo — **excluding `genexpr_grammar.pegjs`** (it is a direct copy of Cycling '74's proprietary grammar; the official copy already lives gitignored at `reference/rnbo/genexpr_js/genexpr.pegjs` and is cited by path only). The derived EBNF is user-authored and committable:

```bash
mkdir -p docs/research
cp -R "$HOME/Music/Ableton/User Library/Presets/M4L/dang-tools/docs/gen_docs" docs/research/gen_docs
rm docs/research/gen_docs/genexpr_grammar.pegjs
grep -ril "pegjs" docs/research/gen_docs | xargs -I{} echo "note: {} references the pegjs — fine, references are not copies"
```

Sanity check before committing: `! test -f docs/research/gen_docs/genexpr_grammar.pegjs`.

**Step 5: Commit**

```bash
git add tools docs/research
git commit -m "chore: extract GSOT + genlib refs; vendor user gen_docs research into docs/research"
```

> Provenance note for all later tasks: genlib citations use `reference/genlib/gen_dsp/genlib_ops.h`; GSOT citations use `reference/packages/gsot/...`. This closes the design-doc Open Item “genlib export … deferred from Task 2”.

---

### Task 1: `render_with_inputs()` in testkit

**TDD scenario:** New feature — full TDD.

**Files:**
- Modify: `crates/opengen-testkit/src/lib.rs`

**Step 1: Write the failing test** (append `#[cfg(test)] mod tests` to `crates/opengen-testkit/src/lib.rs`):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_with_inputs_feeds_channels() {
        // out1 = in1 + in2, driven by two 3-sample channels
        let out = render_with_inputs(
            "out1 = in1 + in2;",
            48_000.0,
            &[&[1.0, 2.0, 3.0], &[10.0, 20.0, 30.0]],
        );
        assert_eq!(out.ch(0), &[11.0, 22.0, 33.0]);
    }

    #[test]
    fn render_with_inputs_short_channel_pads_zero() {
        let out = render_with_inputs("out1 = in1;", 48_000.0, &[&[1.0]]);
        assert_eq!(out.ch(0), &[1.0]);
        let out = render_with_inputs_n("out1 = in1;", 48_000.0, &[&[1.0]], 3);
        assert_eq!(out.ch(0), &[1.0, 0.0, 0.0]);
    }
}
```

**Step 2: Run** `cargo test -p opengen-testkit` — Expected: FAIL (`render_with_inputs` not found).

**Step 3: Implement** (in the same file, after `render`):

```rust
/// Compile `src` and render with per-channel input samples.
/// `n` = longest input channel. Short channels are zero-padded.
/// Panics on compile error — tests want loud failures.
pub fn render_with_inputs(src: &str, sr: f64, inputs: &[&[f64]]) -> Render {
    let n = inputs.iter().map(|c| c.len()).max().unwrap_or(0);
    render_with_inputs_n(src, sr, inputs, n)
}

/// Like `render_with_inputs` but renders exactly `n` samples (zero-padding all inputs).
pub fn render_with_inputs_n(src: &str, sr: f64, inputs: &[&[f64]], n: usize) -> Render {
    let graph = opengen_genexpr::parse_and_lower(src).expect("parse");
    render_graph_with_inputs(&graph, sr, inputs, n)
}

/// Render an already-lowered Graph with inputs (used by opengen-analysis and the
/// .gendsp exit tests, which produce Graphs without genexpr source).
pub fn render_graph_with_inputs(
    graph: &opengen_ir::Graph,
    sr: f64,
    inputs: &[&[f64]],
    n: usize,
) -> Render {
    let mut patch = compile(graph, &opengen_ops::Registry::core(), sr).expect("compile");
    let outs = patch.output_count();
    let mut channels = vec![Vec::with_capacity(n); outs];
    let mut frame_in = vec![0.0; inputs.len()];
    for i in 0..n {
        for (k, ch) in inputs.iter().enumerate() {
            frame_in[k] = ch.get(i).copied().unwrap_or(0.0);
        }
        let frame = patch.process(&frame_in);
        for (c, v) in channels.iter_mut().zip(frame) {
            c.push(v);
        }
    }
    Render { channels }
}
```

Add `opengen-ir.workspace = true` to `crates/opengen-testkit/Cargo.toml` `[dependencies]` (path-only, zero external deps preserved).

**Step 4: Run** `cargo test -p opengen-testkit` — Expected: PASS.

**Step 5: Verify workspace + docs:** `cargo test --workspace && cargo doc --workspace --no-deps` — green, warning-free.

**Step 6: Commit**

```bash
git add crates/opengen-testkit
git commit -m "feat(testkit): render_with_inputs + render_graph_with_inputs"
```

---

### Task 2: Batch probe retrieval

**TDD scenario:** New feature — full TDD.

**Files:**
- Modify: `crates/opengen-compile/src/lib.rs` (add `probe_names()`)
- Modify: `crates/opengen-testkit/src/lib.rs` (add `render_with_probes`)
- Test: `crates/opengen-compile/tests/probes.rs`

**Step 1: Failing test** (append to `crates/opengen-compile/tests/probes.rs`):

```rust
#[test]
fn batch_probe_retrieval() {
    let graph = opengen_genexpr::parse_and_lower(
        "a = history(a + 1); b = a * 2; out1 = b;").unwrap();
    let mut patch = opengen_compile::compile_with_probes(
        &graph, &opengen_ops::Registry::core(), 48_000.0, &["a", "b"]).unwrap();
    for _ in 0..3 { patch.process(&[]); }
    let mut names = patch.probe_names();
    names.sort();
    assert_eq!(names, vec!["a", "b"]);
    assert_eq!(patch.probe("a").unwrap(), &[0.0, 1.0, 2.0]);
    assert_eq!(patch.probe("b").unwrap(), &[0.0, 2.0, 4.0]);
}
```

And in testkit's test module:

```rust
#[test]
fn render_with_probes_returns_all_traces() {
    let (out, probes) = render_with_probes(
        "h = history(h + 1); out1 = h;", 48_000.0, 3, &["h"]);
    assert_eq!(out.ch(0), &[0.0, 1.0, 2.0]);
    assert_eq!(probes["h"], vec![0.0, 1.0, 2.0]);
}
```

**Step 2: Run** `cargo test -p opengen-compile -p opengen-testkit` — FAIL.

**Step 3: Implement.** In `Patch`:

```rust
/// Names of all probes registered at compile time.
pub fn probe_names(&self) -> Vec<&str> {
    self.probes.keys().map(|s| s.as_str()).collect()
}
```

In testkit:

```rust
/// Render `n` samples (no inputs) recording the named probes.
/// Returns the rendered output plus a map of every probe trace.
pub fn render_with_probes(
    src: &str,
    sr: f64,
    n: usize,
    probes: &[&str],
) -> (Render, std::collections::HashMap<String, Vec<f64>>) {
    let graph = opengen_genexpr::parse_and_lower(src).expect("parse");
    let mut patch = opengen_compile::compile_with_probes(
        &graph, &opengen_ops::Registry::core(), sr, probes).expect("compile");
    let outs = patch.output_count();
    let mut channels = vec![Vec::with_capacity(n); outs];
    for _ in 0..n {
        let frame = patch.process(&[]);
        for (c, v) in channels.iter_mut().zip(frame) { c.push(v); }
    }
    let map = patch.probe_names().iter()
        .map(|&name| (name.to_string(), patch.probe(name).unwrap().to_vec()))
        .collect();
    (Render { channels }, map)
}
```

**Step 4: Run** — PASS. **Step 5:** `cargo test --workspace && cargo doc --workspace --no-deps`.

**Step 6: Commit** `git commit -am "feat(testkit,compile): batch probe retrieval (probe_names + render_with_probes)"`

---

### Task 3: `assert_stable!` in analysis

**TDD scenario:** New feature — full TDD.

**Files:**
- Create: `crates/opengen-analysis/src/stability.rs`
- Modify: `crates/opengen-analysis/src/lib.rs` (`pub mod stability;` + re-export)

**Step 1: Failing tests** (in `stability.rs`, `#[cfg(test)] mod tests`):

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn stable_signal_passes() {
        let sine: Vec<f64> = (0..1000)
            .map(|i| (2.0 * std::f64::consts::PI * 440.0 * i as f64 / 48_000.0).sin())
            .collect();
        crate::assert_stable!(&sine);
        crate::assert_stable!(&sine, max_rms = 0.8, max_dc = 0.01);
    }

    #[test]
    #[should_panic(expected = "non-finite")]
    fn nan_fails() { crate::assert_stable!(&[0.0, f64::NAN]); }

    #[test]
    #[should_panic(expected = "denormal")]
    fn denormal_fails() { crate::assert_stable!(&[0.0, 1e-320]); }

    #[test]
    #[should_panic(expected = "RMS")]
    fn rms_bound_fails() { crate::assert_stable!(&[10.0; 100], max_rms = 1.0); }

    #[test]
    #[should_panic(expected = "DC")]
    fn dc_bound_fails() { crate::assert_stable!(&[0.5; 100], max_dc = 0.1); }
}
```

**Step 2: Run** `cargo test -p opengen-analysis stability` — FAIL.

**Step 3: Implement:**

```rust
//! Stability assertions: finiteness, denormals, DC offset, RMS bounds.

/// Backing checker for [`assert_stable!`]. Panics with a labeled message on violation.
/// `max_rms`/`max_dc` of `f64::INFINITY` disable that bound.
pub fn check_stable(samples: &[f64], max_rms: f64, max_dc: f64) {
    assert!(!samples.is_empty(), "assert_stable!: empty signal");
    for (i, &x) in samples.iter().enumerate() {
        assert!(x.is_finite(), "assert_stable!: non-finite sample {x} at index {i}");
        assert!(
            x == 0.0 || x.abs() >= f64::MIN_POSITIVE,
            "assert_stable!: denormal sample {x:e} at index {i}"
        );
    }
    let n = samples.len() as f64;
    let dc = samples.iter().sum::<f64>() / n;
    let rms = (samples.iter().map(|x| x * x).sum::<f64>() / n).sqrt();
    assert!(rms <= max_rms, "assert_stable!: RMS {rms} exceeds bound {max_rms}");
    assert!(dc.abs() <= max_dc, "assert_stable!: DC offset {dc} exceeds bound {max_dc}");
}

/// Assert a rendered signal is finite, denormal-free, and within optional RMS/DC bounds.
///
/// ```
/// opengen_analysis::assert_stable!(&[0.0, 0.5, -0.5]);
/// opengen_analysis::assert_stable!(&[0.1; 64], max_rms = 1.0, max_dc = 0.2);
/// ```
#[macro_export]
macro_rules! assert_stable {
    ($samples:expr) => {
        $crate::stability::check_stable($samples, f64::INFINITY, f64::INFINITY)
    };
    ($samples:expr, max_rms = $rms:expr) => {
        $crate::stability::check_stable($samples, $rms, f64::INFINITY)
    };
    ($samples:expr, max_dc = $dc:expr) => {
        $crate::stability::check_stable($samples, f64::INFINITY, $dc)
    };
    ($samples:expr, max_rms = $rms:expr, max_dc = $dc:expr) => {
        $crate::stability::check_stable($samples, $rms, $dc)
    };
}
```

**Step 4: Run** — PASS. **Step 5:** workspace + doc check. **Step 6: Commit** `git commit -am "feat(analysis): assert_stable! — finiteness/denormal/RMS/DC assertions"`

---

### Task 4: `Response::phase_at` ±π wrap fix

**TDD scenario:** Bug fix on tested code — write the failing test first, run existing response tests before changing anything.

**Files:**
- Modify: `crates/opengen-analysis/src/response.rs`

**Step 1: Failing test.** A pure k-sample delay has linear phase `−2π·k·f/sr`, which wraps through ±π many times; querying between two bins that straddle a wrap exposes the bug. `history` is a 1-sample delay; chain 8 of them so phase reaches −π well inside the band:

```rust
#[test]
fn phase_at_interpolates_across_pi_wrap() {
    // 8-sample delay: phase(f) = -2π·8·f/sr (mod 2π). With nfft = 64 the bin
    // spacing is 750 Hz at 48 kHz and the wrap at phase = -π falls at f = 1500 Hz,
    // i.e. exactly between bins — interpolation must unwrap, not average +π with -π.
    let src = "a = history(in1); b = history(a); c = history(b); d = history(c);
               e = history(d); f = history(e); g = history(f); h = history(g);
               out1 = h;";
    let r = freq_response(src, 48_000.0, 64);
    // Query midway between bin 2 (1500 Hz exact wrap region) neighbors:
    let f_hz = 1_312.5; // between bins 1 (750 Hz) and 2 (1500 Hz)
    let expected = -2.0 * PI * 8.0 * f_hz / 48_000.0; // = -1.375π → wraps to +0.625π
    let got = r.phase_at(f_hz);
    // Compare on the wrapped circle: difference modulo 2π should be ~0
    let mut diff = (got - expected) % (2.0 * PI);
    if diff > PI { diff -= 2.0 * PI; }
    if diff < -PI { diff += 2.0 * PI; }
    assert!(diff.abs() < 1e-6, "phase {got} vs expected {expected} (diff {diff})");
}
```

**Step 2: Run** `cargo test -p opengen-analysis phase_at_interpolates` — FAIL (interpolation averages across the ±π jump).

**Step 3: Fix.** In `phase_at`, unwrap the neighbor before interpolating:

```rust
// Interpolate phase, unwrapping the bin-to-bin jump onto the same branch:
let frac = exact_bin - bin_lo as f64;
let phase_lo = self.spectrum[bin_lo].arg();
let mut phase_hi = self.spectrum[bin_hi].arg();
// Shift phase_hi by multiples of 2π so |phase_hi - phase_lo| <= π (nearest branch).
while phase_hi - phase_lo > std::f64::consts::PI { phase_hi -= 2.0 * std::f64::consts::PI; }
while phase_hi - phase_lo < -std::f64::consts::PI { phase_hi += 2.0 * std::f64::consts::PI; }
phase_lo + frac * (phase_hi - phase_lo)
```

Update the rustdoc: remove the "M1 limitation" note; document that the returned value may exceed ±π by up to the unwrap step (callers compare on the circle).

**Step 4: Run** `cargo test -p opengen-analysis` — PASS (including the four existing panic tests).

**Step 5: Commit** `git commit -am "fix(analysis): phase_at unwraps ±π before interpolating between bins"`

---

### Task 5: `clip`/`wrap`/`fold` inverted bounds — genlib-evidenced `# Definition` + tests

**TDD scenario:** Modifying tested code — run `cargo test -p opengen-ops range` first to observe current green, then add failing tests. Semantics per D4 (genlib facts; cite `reference/genlib/gen_dsp/genlib_ops.h` — verify the file exists from Task 0 before citing).

**Files:**
- Modify: `crates/opengen-ops/src/range.rs`

**Step 1: Failing tests** (in `range.rs` `#[cfg(test)] mod tests`):

```rust
#[test]
fn clip_does_not_swap_inverted_bounds() {
    use opengen_testkit::render;
    // D4 / genlib: clip is literally min(max(x, lo), hi) — inverted bounds pin to hi.
    assert_eq!(render("out1 = clip(0.5, 1, 0);", 48_000.0, 1).ch(0)[0], 0.0);
    assert_eq!(render("out1 = clip(-3.0, 1, 0);", 48_000.0, 1).ch(0)[0], 0.0);
    assert_eq!(render("out1 = clip(0.5, 0.25, 0.25);", 48_000.0, 1).ch(0)[0], 0.25);
    // Regression: normal order unchanged
    assert_eq!(render("out1 = clip(0.5, 0, 1);", 48_000.0, 1).ch(0)[0], 0.5);
}

#[test]
fn wrap_fold_swap_inverted_bounds() {
    use opengen_testkit::render;
    // D4 / genlib: wrap and fold normalize bounds by swapping.
    assert_eq!(render("out1 = wrap(1.25, 1, 0);", 48_000.0, 1).ch(0)[0], 0.25);
    assert_eq!(render("out1 = fold(1.25, 1, 0);", 48_000.0, 1).ch(0)[0], 0.75);
}

#[test]
fn wrap_fold_degenerate_bounds_return_lo() {
    use opengen_testkit::render;
    assert_eq!(render("out1 = wrap(0.5, 0.25, 0.25);", 48_000.0, 1).ch(0)[0], 0.25);
    assert_eq!(render("out1 = fold(0.5, 0.25, 0.25);", 48_000.0, 1).ch(0)[0], 0.25);
    // genlib guard: wrap with normalized range <= 1e-9 returns lo
    assert_eq!(render("out1 = wrap(0.5, 0.0, 0.0000000001);", 48_000.0, 1).ch(0)[0], 0.0);
}
```

**Step 2: Run** `cargo test -p opengen-ops -- clip_does wrap_fold` — observe which fail (current kernels assume `lo <= hi`; `clip` may already be a min/max composition and pass — if so, keep it and add the doc only).

**Step 3: Implement.**
- `clip`: ensure the body is exactly `inputs[0].max(lo).min(hi)` with `lo = inputs[1], hi = inputs[2]` — NO swap.
- `wrap`/`fold`: prepend bound normalization:

```rust
let (lo, hi) = if inputs[1] <= inputs[2] { (inputs[1], inputs[2]) } else { (inputs[2], inputs[1]) };
if lo == hi { return lo; }
// wrap only (genlib guard):
if hi - lo <= 1e-9 { return lo; }
```

Rustdoc: extend each `# Definition` with the exact rule; add `# Vendor` citing `reference/genlib/gen_dsp/genlib_ops.h` (facts: clamp = min/max composition without swap; wrap/fold swap; wrap tiny-range guard 1e-9; equal bounds → lo). Add `# Observed`: "Authored conformance patch `conformance/patches/range_inverted_bounds.genexpr` (Task 25) cross-checks against real gen~ renders." This resolves the design-doc Open Item.

**Step 4: Run** `cargo test -p opengen-ops` — PASS (existing boundary doctests unchanged).

**Step 5: Commit** `git commit -am "feat(ops): clip/wrap/fold inverted-bounds semantics per genlib evidence"`

---

### Task 6: Port-level cycle breaking + update-phase refactor (D1, D2, D3)

**TDD scenario:** Modifying tested code — run the full compile + ops suites first; this task deliberately changes two documented M1 behaviors (listed below).

**Files:**
- Modify: `crates/opengen-ops/src/registry.rs` (OpDef shape)
- Modify: `crates/opengen-ops/src/{math,compare,range,state,osc}.rs` (defs() mechanical update)
- Modify: `crates/opengen-compile/src/lib.rs` (topo sort + Update steps + init)

**Behavior changes (intentional, update these tests):**
1. `opengen-compile::tests::stateful_node_breaks_feedback_cycle` used `add` with a forged `Slots(1)` — under port-level deferral a self-loop on `add` (no deferred ports) is again a compile error. Rewrite the test using `history`.
2. `opengen-ops::osc::tests::phasor_negative_freq_wrap` no longer sees the stale-freq artifact: with the freq edge blocking, `phasor(0 - 60000)` sees −60000 from sample 0. New expected output: `[0.0, 0.75, 0.5, 0.25]`. Remove the artifact comment.

**Step 1: New OpDef shape** (registry.rs):

```rust
use opengen_ir::StateDecl;

/// A pure-Rust per-sample kernel: (inputs, state slots, samplerate) -> output.
/// Kernels MUST NOT read input values arriving on `deferred_ports` (update-phase only).
pub type Kernel = fn(&[f64], &mut [f64], f64) -> f64;
/// End-of-sample state update: (inputs, state slots, samplerate).
/// Runs after ALL Compute steps, in ascending NodeId order (determinism contract).
pub type UpdateFn = fn(&[f64], &mut [f64], f64);
/// One-time state initializer at compile: (IR node args, state slots, samplerate).
pub type InitFn = fn(&[f64], &mut [f64], f64);

pub struct OpDef {
    pub name: &'static str,
    pub arity: u16,
    pub state: StateDecl,
    /// Input ports whose incoming edges do NOT block topological scheduling
    /// (the "write" ports of feedback-capable ops: history port 0, delay port 0).
    pub deferred_ports: &'static [u16],
    /// End-of-sample state writer; None = stateless or kernel-managed state.
    pub update: Option<UpdateFn>,
    /// One-time state initializer from IR node args; None = zero-init.
    pub init: Option<InitFn>,
    pub kernel: Kernel,
}
```

**Step 2: Mechanical defs() update.** Every existing stateless def becomes:
`deferred_ports: &[], update: None, init: None`. Specific ops:
- `history` (state.rs): `deferred_ports: &[0], update: Some(|i, s, _| s[0] = i[0]), init: Some(|args, s, _| if let Some(&v) = args.first() { s[0] = v })` — the init arm enables `history(init)` via `History h(init);` in Task 14. Kernel unchanged (`state[0]` read).
- `phasor`, `cycle`, `noise` (osc.rs): `deferred_ports: &[], update: None, init: None` (kernel-managed state; freq edges now BLOCK — this is the fix).

Update the `registry_resolves_add` unit test (`auto_state_update` field is gone; assert `op.update.is_none()` and `op.deferred_ports.is_empty()`).

**Step 3: Compile changes** (`crates/opengen-compile/src/lib.rs`):

a. Replace `StepKind::StateUpdate { input_slot, state_range }` with:

```rust
/// End-of-sample update: gather input slots, call the op's UpdateFn.
Update { update: opengen_ops::UpdateFn, inputs: Vec<usize>, state_range: std::ops::Range<usize> },
```

and in `process()`:

```rust
StepKind::Update { update, inputs: input_slots, state_range } => {
    let input_vals: Vec<f64> = input_slots.iter().map(|&i| self.values[i]).collect();
    update(&input_vals, &mut self.state[state_range.clone()], self.sr);
}
```

b. In the dependency-graph build, replace the `if node.state() == StateDecl::None` blanket with a per-port check:

```rust
let deferred: &[u16] = match &node.kind {
    NodeKind::Op { name, .. } => reg.get(name)
        .ok_or_else(|| CompileError(format!("unknown operator: {}", name)))?
        .deferred_ports,
    _ => &[],
};
for port_idx in 0..arity {
    if let Some(src) = g.input_of(Port { node: id, index: port_idx }) {
        if !deferred.contains(&port_idx) {
            *in_degree.get_mut(&id).unwrap() += 1;
            dependencies.entry(src.node).or_default().push(id);
        }
        let _ = src;
    } else if matches!(node.kind, NodeKind::Op { .. }) {
        return Err(CompileError(format!("missing input {} for op node {:?}", port_idx, id)));
    }
}
```

c. In the step-build loop, for op nodes: emit the Compute step always; if `op_def.update` is `Some(f)`, push `StepKind::Update { update: f, inputs: input_slots.clone(), state_range }` into `stateful_updates`. **Sort `stateful_updates` by NodeId before appending** (they are produced in topo order; the contract is NodeId order — collect `(NodeId, Step)` pairs and sort).

d. After allocating the state arena, run inits:

```rust
for (id, node) in g.nodes() {
    if let NodeKind::Op { name, args, .. } = &node.kind {
        if let Some(range) = state_ranges.get(&id) {
            if let Some(init) = reg.get(name).and_then(|d| d.init) {
                init(args, &mut state[range.clone()], sr);
            }
        }
    }
}
```

(`state` must be `let mut state = ...` now.)

**Step 4: Update the two behavior-change tests** (listed above) and run:

```
cargo test --workspace
```

Expected: PASS everywhere. The M1 exit tests (`crates/opengen-analysis/tests/m1_exit.rs`) must pass UNCHANGED — `history` feedback semantics are preserved by the update-phase design.

**Step 5: Docs.** Update `Patch::process` rustdoc: "Execution: all Compute steps in topo order (NodeId ties ascending), then all Update steps in ascending NodeId order." Run `cargo doc --workspace --no-deps`.

**Step 6: Commit**

```bash
git add -A
git commit -m "feat(compile,ops): port-level cycle breaking, update-phase state writes, state init hook"
```

**CHECKPOINT A:** full suite green; phasor staleness artifact gone; foundations ready for delay. Push: `git push origin master`.

---

## Phase B — Scalar math operators (Tasks 7–9)

Pattern: M1 Task 8. For each operator: rustdoc spec (`# Definition` + provenance citation) → doctest → kernel → register in the module's `defs()` → `cargo test -p opengen-ops`. Call syntax already parses (`name(arg, ...)`). Doctests use the **std-reference pattern** for transcendentals — exact equality against the corresponding `f64` std function at a sample argument (we *define* these ops as the platform's correctly-rounded-ish libm; determinism note: identical bits on a given platform/std version; cross-platform bit-identity for transcendentals is tracked as an M3 emitter concern, note it in each module header).

### Task 7: Trigonometry — `crates/opengen-ops/src/trig.rs` (new module)

Operators (all arity per Vendor `operators.json`; refpages in `reference/gen/refpages/common/`):

| op | arity | Definition | doctest assertion |
|---|---|---|---|
| `sin` | 1 | `f64::sin(x)`, radians | `render("out1 = sin(0.5);",…) == 0.5f64.sin()`; `sin(0) == 0.0` |
| `cos` | 1 | `f64::cos(x)` | `cos(0) == 1.0` |
| `tan` | 1 | `f64::tan(x)` | `tan(0.5) == 0.5f64.tan()` |
| `asin` | 1 | `f64::asin(x)`; NaN outside [-1,1] (IEEE) | `asin(1) == std::f64::consts::FRAC_PI_2` |
| `acos` | 1 | `f64::acos(x)` | `acos(1) == 0.0` |
| `atan` | 1 | `f64::atan(x)` | `atan(1) == std::f64::consts::FRAC_PI_4` |
| `atan2` | 2 | `y.atan2(x)`; inputs (y, x) per refpage | `atan2(1, 1) == std::f64::consts::FRAC_PI_4` |

Cite each: `# Documented reference/gen/refpages/common/gen_common_<op>.maxref.xml`. NaN-propagation sad-path `#[test]`: `asin(2.0)` output `is_nan()`.

Steps: write all doctests (failing: module/ops unknown) → `cargo test -p opengen-ops` FAIL → implement kernels + `defs()` + register in `Registry::core()` (`for def in crate::trig::defs() { ops.insert(def.name, def); }`) and `pub mod trig;` in lib.rs → PASS → workspace + doc check.

**Commit:** `git commit -am "feat(ops): trig operators (sin cos tan asin acos atan atan2)"`

### Task 8: Exponentials/logarithms — extend `crates/opengen-ops/src/math.rs`

| op | arity | Definition | doctest |
|---|---|---|---|
| `exp` | 1 | `x.exp()` | `exp(1) == 1f64.exp()`; `exp(0) == 1.0` |
| `exp2` | 1 | `x.exp2()` | `exp2(3) == 8.0` |
| `ln` | 1 | `x.ln()`; `ln(0) = -inf`, `ln(x<0) = NaN` (IEEE) | `ln(1) == 0.0` |
| `log` | 1 | natural log, alias of `ln` (refpage: "The natural logarithm") | `log(1) == 0.0` |
| `log2` | 1 | `x.log2()` | `log2(8) == 3.0` |
| `log10` | 1 | `x.log10()` | `log10(1000) == 3.0` |
| `hypot` | 2 | `a.hypot(b)` | `hypot(3, 4) == 5.0` |

Sad-path `#[test]`: `ln(-1)` is NaN; `ln(0)` is `-inf` (and therefore caught by `assert_stable!` — cross-link in rustdoc).

**Commit:** `git commit -am "feat(ops): exp/log family (exp exp2 ln log log2 log10 hypot)"`

### Task 9: Conversions + misc — new module `crates/opengen-ops/src/convert.rs`

| op | arity | Definition | doctest |
|---|---|---|---|
| `sign` | 1 | x>0→1, x<0→−1, else x itself (refpage: "zero returns itself") | `sign(-3.5) == -1.0`; `sign(0) == 0.0` |
| `fract` | 1 | `x - x.floor()` | `fract(1.25) == 0.25`; `fract(-0.25) == 0.75` |
| `trunc` | 1 | `x.trunc()` (toward zero, per refpage) | `trunc(-1.7) == -1.0` |
| `absdiff` | 2 | `(a - b).abs()` | `absdiff(2, 5) == 3.0` |
| `sinh`/`cosh`/`tanh` | 1 | std fns | std-reference pattern |
| `asinh`/`acosh`/`atanh` | 1 | std fns | std-reference pattern |
| `degrees` | 1 | `x * 180/π` | `degrees(std::f64::consts::PI) == 180.0` |
| `radians` | 1 | `x * π/180` | `radians(180) == std::f64::consts::PI` |
| `mtof` | 2 | `tuning * 2^((note-69)/12)`; inputs (note, tuning=440) | `mtof(69, 440) == 440.0`; `mtof(81, 440) == 880.0` |
| `ftom` | 2 | `69 + 12*log2(freq/tuning)` | `ftom(440, 440) == 69.0` |
| `dbtoa` | 1 | `10^(db/20)` | `dbtoa(0) == 1.0`; `(dbtoa(-6.0) - 0.5011872336272722).abs() < 1e-15` |
| `atodb` | 1 | `20*log10(a)` | `atodb(1) == 0.0` |
| `mstosamps` | 1 | `ms * sr / 1000` (sr-dependent) | at sr 48 000: `mstosamps(1000) == 48000.0` |
| `sampstoms` | 1 | `samps * 1000 / sr` | at sr 48 000: `sampstoms(48) == 1.0` |
| `rsub` | 2 | `b - a` (Vendor: reverse subtraction) | `render("out1 = rsub(1, 5);") == 4.0` |
| `rdiv` | 2 | `b / a` | `rdiv(2, 10) == 5.0` |
| `switch` | 3 | inputs (cond, iftrue, iffalse): `if cond != 0 { iftrue } else { iffalse }`; D5 eager-eval note | `switch(1, 2.5, 3.5) == 2.5`; `switch(0, 2.5, 3.5) == 3.5` |
| `round` | 2 | nearest multiple of `base` (default handled by caller), halfway away from zero (refpage); `base <= 0` → x unchanged (`# Definition` decision, conformance TBD) | `round(2.5, 1) == 3.0`; `round(-2.5, 1) == -3.0`; `round(0.3, 0.25) == 0.25` |
| `clamp` | 3 | alias of `clip` — register the existing `range::clip` kernel under name `"clamp"` (refpage: inclusive bounds) | `clamp(1.5, 0, 1) == 1.0` |
| `int` | 1 | integer truncation toward zero — **verify against `reference/gen/refpages/common/gen_common_int.maxref.xml` digest before committing**; if the refpage says floor, follow the refpage and cite it | `int(1.7) == 1.0`; `int(-1.7) == -1.0` (adjust if refpage says floor) |
| `bool` | 1 | `if x != 0 { 1.0 } else { 0.0 }` | `bool(0.5) == 1.0`; `bool(0) == 0.0` |
| `fixnan` | 1 | NaN → 0.0, else passthrough (genlib `# Vendor` cite) | `fixnan(0/0)` via `#[test]` (NaN literal needs std); doctest `fixnan(2.5) == 2.5` |
| `fixdenorm` | 1 | denormals → 0.0, else passthrough (genlib `# Vendor`) | `fixdenorm(1e-320) == 0.0` (`#[test]`), `fixdenorm(1.0) == 1.0` |
| `triangle` | 2 | unipolar triangle from phase + duty: wrap phase to [0,1), clamp duty `p` to [0,1]; rising `phase/p` for `phase < p`, falling `1 - (phase-p)/(1-p)` after; degenerate `p==0`→falling-only, `p==1`→phase (genlib `# Vendor`, paraphrased) | `triangle(0.25, 0.5) == 0.5`; `triangle(0.75, 0.5) == 0.5`; `triangle(0.5, 0.5) == 1.0` |
| `and` | 2 | logical: `(a != 0 && b != 0) as f64` — eager, value-level (D5) | `and(2, 3) == 1.0`; `and(2, 0) == 0.0` |
| `or` | 2 | logical: `(a != 0 \|\| b != 0) as f64` | `or(0, 3) == 1.0`; `or(0, 0) == 0.0` |
| `not` | 1 | `(x == 0) as f64` | `not(0) == 1.0`; `not(2.5) == 0.0` |
| `xor` | 2 | logical: `((a != 0) != (b != 0)) as f64` | `xor(1, 0) == 1.0`; `xor(1, 1) == 0.0` |

`mtof`/`ftom`/`dbtoa`/`atodb`/`mstosamps`/`sampstoms`/`round` cite `reference/gen/refpages/dsp/gen_dsp_<op>.maxref.xml`; the rest `common`. `switch` rustdoc carries D5 (`# Definition`: both value inputs are evaluated every sample; selection is value-level).

**Commit:** `git commit -am "feat(ops): conversion + misc operators (sign…clamp, switch, rsub/rdiv, round)"`

(`int`, `bool`, `fixnan`, `fixdenorm`, `triangle`, `and`/`or`/`xor` are GSOT/dang-tools demand — see Research Evidence; `fixnan` is the #3 most-used GSOT operator.)

**CHECKPOINT B:** `cargo test --workspace` green, `cargo doc` clean, ops count ≥ 65. Push.

---

## Phase C — Full GenExpr grammar (Tasks 10–16)

Grammar source of truth: PEG facts from `reference/rnbo/genexpr_js/genexpr.pegjs` (Vendor — read from gitignored `reference/`, paraphrase, cite path, never quote or copy) cross-checked with `docs/research/gen_docs/genexpr_ebnf.md` (user-derived, in-repo) and `genexpr_language_reference.md`. Read all before starting Task 11. The pegjs itself is NEVER vendored into the repo (user ruling — see Research Evidence).

### Task 10: Structured error types with source locations

**TDD scenario:** Modifying tested code — existing parse/lower tests must keep passing.

**Files:**
- Modify: `crates/opengen-genexpr/src/{lib.rs,lexer.rs,parser.rs,lower.rs,ast.rs}`

**Step 1: Failing tests** (`crates/opengen-genexpr/tests/errors.rs`):

```rust
use opengen_genexpr::parse;

#[test]
fn parse_error_carries_line_and_col() {
    let err = parse("out1 = 1 +;\n").unwrap_err();
    assert_eq!(err.loc.map(|l| l.line), Some(1));
    assert!(err.loc.unwrap().col >= 10);
    assert!(err.to_string().contains("1:"), "display includes location: {err}");
}

#[test]
fn lower_error_carries_statement_location() {
    let ast = parse("out1 = 1;\nx = bogus_op(2);").unwrap();
    let err = opengen_genexpr::lower(&ast).unwrap_err();
    assert_eq!(err.loc.map(|l| l.line), Some(2));
}
```

**Step 2: Run** `cargo test -p opengen-genexpr errors` — FAIL (no `loc` field).

**Step 3: Implement.**
- `ast.rs`: `#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct SourceLoc { pub line: u32, pub col: u32 }`; `Statement` becomes a struct `{ pub kind: StatementKind, pub loc: SourceLoc }` (rename the old enum to `StatementKind`); expressions do NOT carry locations in M2 (documented — statement granularity).
- `lexer.rs`: track `line`/`col` per token (`struct Spanned { tok: Token, loc: SourceLoc }`).
- `ParseError { pub msg: String, pub loc: Option<SourceLoc> }`, Display = `"{line}:{col}: {msg}"`; same shape for `LowerError`. `parse_and_lower` keeps returning `Result<Graph, String>` (formats the structured error) so testkit/analysis/CLI don't change.

**Step 4: Run** `cargo test --workspace` — PASS (update any test matching old error strings).

**Step 5: Commit** `git commit -am "feat(genexpr): structured ParseError/LowerError with source locations"`

---

### Task 11: Lexer v2 — comments, numeric forms, full token set

**TDD scenario:** New feature — full TDD.

**Files:**
- Modify: `crates/opengen-genexpr/src/lexer.rs`
- Test: `crates/opengen-genexpr/tests/lexer_v2.rs`

**Step 1: Failing tests:**

```rust
use opengen_genexpr::parse;

#[test]
fn comments_and_numeric_forms() {
    // line + block comments, trailing-dot and leading-dot floats, sci notation
    parse("// line comment\n/* block\ncomment */\nout1 = 1. + .5 + 1e-3 + 2.5E+2;").unwrap();
}

#[test]
fn new_operator_tokens_lex() {
    parse("out1 = (1 < 2) && !(3 > 4) || (1 ^^ 0);").unwrap();
    parse("out1 = (5 & 3) | (1 ^ 2) + (1 << 2) + (8 >> 1);").unwrap();
    parse("x = 1; x += 2; x -= 1; x *= 3; x /= 2; x %= 2; out1 = x;").unwrap();
    parse("out1 = 1 ? 2 : 3;").unwrap();
}
```

(These also exercise Task 12's parser — write them now, leave `#[ignore]` on the second test until Task 12 if needed; remove before Task 12's commit.)

**Step 2–4:** FAIL → implement: skip `//…\n` and `/*…*/` (unterminated block comment = error with loc); number rule accepts `1.`, `.5`, exponent part `[eE][+-]?digits` (PEG fact); new tokens: `Question Colon AndAnd OrOr Bang Caret CaretCaret Amp Pipe Shl Shr PlusEq MinusEq StarEq SlashEq PercentEq Dot LBrace RBrace` → PASS.

**Step 5: Commit** `git commit -am "feat(genexpr): lexer v2 — comments, float/sci forms, full operator token set"`

---

### Task 12: Parser v2 — expressions, precedence, declarations, statements

**TDD scenario:** New feature — full TDD.

**Files:**
- Modify: `crates/opengen-genexpr/src/{parser.rs,ast.rs}`
- Test: `crates/opengen-genexpr/tests/parser_v2.rs`

Scope (PEG facts + user EBNF):
- Precedence (low→high): ternary `?:` (right-assoc) → `||` → `&&` → `|` → `^` → `^^` → `&` → equality → relational → shifts → additive → multiplicative → unary (`-`, `!`) → postfix (call, member-call) → primary.
- Compound assignment desugars in the parser: `x += e` → `Assign { name: x, expr: BinOp(Add, Ident(x), e) }`.
- Typed declarations with declarator lists and call-style init args incl. **named args**: `History a(0), b(0);`, `Param size(1.0, min=0.1, max=2);`, `Data buf(512);`, `Delay d(1024);`, `Buffer b(512);` → `StatementKind::Decl { ty: DeclType, items: Vec<Declarator> }` where `Declarator { name, args: Vec<Expr>, named_args: Vec<(String, Expr)> }`.
- Statements: expression statements (`d.write(x);`, `poke(buf, v, i);`), blocks `{ … }`, `if`/`else if`/`else` (single-statement bodies without braces — user-docs fact), `while`, `do … while(…);`, numeric `for (init; cond; step)`, `break;`, `continue;`, `return e1, e2, …;`, multi-assign `a, b = f(x);`, function declarations `name(p1, p2) { … }` (before other statements per file-structure order; parser accepts them anywhere, lowering enforces nothing — lenient), `require "file";` → parsed into `StatementKind::Require(String)`.
- Member calls: `ident.method(args)` → `Expr::MemberCall { object, method, args }`.
- Bare final expression (`in1 * 0.5` with no `out1 =` and no semicolon) → sugar for `out1 = <expr>;` (user-docs fact, minimal-codebox form).

**Step 1: Failing tests** — one per construct, asserting AST shape (follow `parser_v2.rs` pattern: parse, match on `ast.statements[i].kind`). Include sad paths: `parse("if (1)")` missing body; `parse("x +== 1;")`; `return` outside function is a PARSE success (lowering rejects later).

**Steps 2–4:** FAIL → implement → PASS. Keep the recursive-descent + precedence-climbing structure; one parse fn per level.

**Step 5: Commit** `git commit -am "feat(genexpr): parser v2 — full expression grammar, declarations, control-flow statements"`

---

### Task 13: Lowering v2 — straight-line constructs

**TDD scenario:** New feature — full TDD. (Control flow + functions are Tasks 14–16; this task lowers everything that stays a pure dataflow graph.)

**Files:**
- Modify: `crates/opengen-genexpr/src/lower.rs`
- Test: `crates/opengen-genexpr/tests/lower_v2.rs` + doctest updates

Lowering table additions:
- Ternary `c ? a : b` → `switch` op (D5). `&&`→`and`, `||`→`or`, `!`→`not`, `^^`→`xor`; `&`→`bitand`, `|`→`bitor`, `^`→`bitxor`, `<<`→`shl`, `>>`→`shr` — register these five bitwise kernels in `crates/opengen-ops/src/bitwise.rs` in THIS task (Definition: operate on `i64` via `trunc` cast, result back to f64; shift counts masked `& 63`; doctests `shl(1,3)==8`, `bitand(5,3)==1`).
- Builtin constants in identifier position (unless shadowed, D15): `pi`, `twopi`, `halfpi`, `invpi`, `e`, `ln2`, `ln10`, `log2e`, `log10e`, `sqrt2`, `sqrt1_2`, `degtorad`, `radtodeg` → `Node::constant(…)` (values from `std::f64::consts`, cite the GenExpr guide chapter under `# Documented` on a small `consts` table in lower.rs). `samplerate` → arity-0 op `"samplerate"` (kernel returns `sr`; register in ops). `vectorsize` → constant 1.0 (`# Divergence`: per-sample engine).
- `History h(init);` decl → history node with `args: vec![init]` (Task 6's `init` hook prefills state); subsequent `h = expr;` connects port 0 (write); reads of `h` anywhere = node output. Un-written History = held init forever. Doctest: `History h(5); h = h + 1; out1 = h;` → `[5, 6, 7]`.
- `Param p(d, min=lo, max=hi);` → Param node (min/max parsed, stored as node args, runtime-ignored in M2 — documented).
- `Data`/`Buffer`/`Delay` decls → record in a decl table for Phase D tasks; using them before Phase D → clear LowerError ("not yet implemented").
- Expression statements → lower expr, no binding (sinks like `poke` later).
- `require` → LowerError "require unsupported in M2" (D16).
- Shadowing per D15.
- Multi-assign without functions → LowerError until Task 16.

**Steps:** failing tests per row → implement → `cargo test --workspace` → commit:
`git commit -am "feat(genexpr,ops): lowering v2 — ternary/logical/bitwise, constants, History decls, samplerate"`

---

### Task 14: Region IR + compile execution (D6)

**TDD scenario:** New feature — full TDD. The hardest task in M2; read D6 and the design doc's "structured regions" line first.

**Files:**
- Create: `crates/opengen-ir/src/proc.rs` (`pub mod proc;` in lib.rs)
- Modify: `crates/opengen-ir/src/lib.rs` (`NodeKind::Region`)
- Modify: `crates/opengen-compile/src/lib.rs`
- Test: `crates/opengen-compile/tests/regions.rs`

**Step 1: IR types** (complete, in `proc.rs`):

```rust
//! Structured procedural regions: codebox control flow lowers to one Region
//! node embedded in the dataflow graph (design doc, "structured regions").

/// Expression tree, fully resolved at lowering time: locals/inputs/state by
/// index, op calls by registry name (kernel pointers resolve at compile).
#[derive(Debug, Clone, PartialEq)]
pub enum PExpr {
    Const(f64),
    /// Region-local variable slot (zero-initialized every sample).
    Local(u32),
    /// Region input port (fed by graph edges).
    In(u16),
    /// Region persistent state slot (History reads).
    State(u32),
    Call {
        op: String,
        args: Vec<PExpr>,
        /// Offset into the region's state block for this call instance
        /// (stateful ops get unique instances per call site); `u32::MAX` if stateless.
        state_base: u32,
        /// Named data region (peek/poke); resolved to an arena range at compile.
        data_ref: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PStmt {
    SetLocal { dst: u32, expr: PExpr },
    SetOut { index: u16, expr: PExpr },
    /// History writes — immediate, per genlib codebox semantics (a History is a
    /// plain persistent variable; `# Vendor` cite genlib_ops.h; note the
    /// deliberate contrast with graph-level history's deferred update).
    SetState { index: u32, expr: PExpr },
    /// Side-effect expression statement (poke).
    Eval(PExpr),
    If { cond: PExpr, then_body: Vec<PStmt>, else_body: Vec<PStmt> },
    While { cond: PExpr, body: Vec<PStmt> },
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProcRegion {
    pub n_inputs: u16,
    pub n_outputs: u16,
    pub n_locals: u32,
    pub n_state: u32,
    /// Initial values for the region state block (History inits), len == n_state.
    pub state_init: Vec<f64>,
    pub body: Vec<PStmt>,
}
```

`NodeKind` gains `Region(proc::ProcRegion)`; `Node::region(r)` constructor; `Node::state()` for a region returns `StateDecl::Slots(r.n_state)`.

**Step 2: Failing compile tests** (`regions.rs`) — build a Region by hand (no parser dependency):

```rust
use opengen_ir::{proc::*, Graph, Node, Port, StateDecl};

/// out0 = (in0 > 0.5) ? in0 * 2 : -1;  via If statement
#[test]
fn region_if_else_selects_branch() {
    let region = ProcRegion {
        n_inputs: 1, n_outputs: 1, n_locals: 0, n_state: 0,
        state_init: vec![],
        body: vec![PStmt::If {
            cond: PExpr::Call { op: "gt".into(),
                args: vec![PExpr::In(0), PExpr::Const(0.5)], state_base: u32::MAX, data_ref: None },
            then_body: vec![PStmt::SetOut { index: 0, expr: PExpr::Call { op: "mul".into(),
                args: vec![PExpr::In(0), PExpr::Const(2.0)], state_base: u32::MAX, data_ref: None } }],
            else_body: vec![PStmt::SetOut { index: 0, expr: PExpr::Const(-1.0) }],
        }],
    };
    let mut g = Graph::new();
    let inp = g.add_node(Node::input(0));
    let r = g.add_node(Node::region(region));
    let out = g.add_node(Node::output(0));
    g.connect(Port { node: inp, index: 0 }, Port { node: r, index: 0 });
    g.connect(Port { node: r, index: 0 }, Port { node: out, index: 0 });
    let mut p = opengen_compile::compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap();
    assert_eq!(p.process(&[0.75]), vec![1.5]);
    assert_eq!(p.process(&[0.25]), vec![-1.0]);
}

/// while accumulation with break: count = 0; while(1) { count = count + 1; if (count >= in0) break; }
#[test]
fn region_while_with_break() { /* same pattern; expect out == in0 for in0 in 1..5 */ }

/// History counter inside region: state[0] = state[0] + 1 each sample, out = state pre-increment.
#[test]
fn region_state_persists_across_samples() { /* SetState + State read; expect 0,1,2 */ }

/// Unsupported op name inside region -> CompileError mentioning the name.
#[test]
fn region_unknown_op_errors() { /* op: "bogus" */ }
```

**Step 3: Compile implementation.**
- Multi-port value slots: replace direct `src.node.0 as usize` slot lookups with a `slot_of: HashMap<(NodeId, u16), usize>` built up front — port 0 of every node maps to `node.0`; Region output ports `1..n_outputs` get slots appended after `node_count`. `values` length = `node_count + extra`.
- Region nodes: arity = `n_inputs` (edge checks as for ops; all ports blocking); state range allocated from `n_state` with `state_init` prefilled; locals allocated in a new `Patch.scratch: Vec<f64>` arena (`locals_range` per region, zeroed at region entry each sample).
- Compiled representation (in compile, not IR):

```rust
enum RExpr {
    Const(f64),
    Local(usize),            // index into scratch (absolute)
    Slot(usize),             // graph value slot (region inputs, resolved)
    State(usize),            // absolute state-arena index
    Call { kernel: opengen_ops::Kernel, args: Vec<RExpr>, state_range: std::ops::Range<usize> },
}
enum RStep {
    SetLocal { dst: usize, e: RExpr },
    SetOut { slot: usize, e: RExpr },
    SetState { idx: usize, e: RExpr },
    Eval(RExpr),
    If { cond: RExpr, then_b: Vec<RStep>, else_b: Vec<RStep> },
    While { cond: RExpr, body: Vec<RStep> },
    Break,
    Continue,
}
```

- Lower `PExpr::Call` by resolving kernels through the registry at compile; stateful call instances get `state_range = region_state_base + state_base ..` sub-ranges; `data_ref` resolves via the Data map (Task 17 — until then `data_ref: Some(_)` → "data regions land in Task 17" error). Region rejects ops with `update: Some(_)` or non-empty `deferred_ports` inside (history/delay belong to decls, not region calls) with a clear message.
- Execution: free functions to satisfy the borrow checker (fields split):

```rust
enum Flow { Normal, Break, Continue }
fn run_rsteps(steps: &[RStep], values: &mut [f64], state: &mut [f64],
              scratch: &mut [f64], sr: f64) -> Flow { /* sequential; loops re-check cond */ }
fn eval_rexpr(e: &RExpr, values: &[f64], state: &mut [f64],
              scratch: &[f64], sr: f64) -> f64 { /* recursive */ }
```

`StepKind::Region { steps: Vec<RStep>, locals_range, out_slots: Vec<usize> }`; at execution: zero the locals range, run, `Flow::Break/Continue` at top level is a no-op. Determinism: purely sequential statement order — document in `Patch::process` rustdoc.
- No iteration guard on `while` (gen~ has none; documented).

**Step 4: Run** `cargo test -p opengen-compile` — PASS; full workspace + doc check.

**Step 5: Commit** `git commit -am "feat(ir,compile): structured Region nodes — ProcRegion IR, nested step execution"`

---

### Task 15: Lower genexpr control flow into Regions

**TDD scenario:** New feature — full TDD.

**Files:**
- Modify: `crates/opengen-genexpr/src/lower.rs` (region path)
- Test: `crates/opengen-genexpr/tests/control_flow.rs`

**Step 1: Failing tests:**

```rust
use opengen_testkit::{render, render_with_inputs};

#[test]
fn if_else_lowers_and_runs() {
    let out = render_with_inputs(
        "x = 0;\nif (in1 > 0.5) { x = in1 * 2; } else { x = 0 - 1; }\nout1 = x;",
        48_000.0, &[&[0.75, 0.25]]);
    assert_eq!(out.ch(0), &[1.5, -1.0]);
}

#[test]
fn for_loop_accumulates() {
    let out = render("acc = 0; for (i = 0; i < 4; i += 1) { acc += i; } out1 = acc;", 48_000.0, 1);
    assert_eq!(out.ch(0)[0], 6.0);
}

#[test]
fn history_inside_control_flow_program() {
    // History decl + if: whole body lowers to one region with persistent state
    let src = "History h(0);\nif (in1 > 0) { h = h + in1; }\nout1 = h;";
    let out = render_with_inputs(src, 48_000.0, &[&[1.0, 0.0, 2.0]]);
    assert_eq!(out.ch(0), &[0.0, 1.0, 1.0]); // reads precede writes textually? NO:
    // h read for out1 happens AFTER the write in the same sample (immediate-write
    // semantics, D6/genlib) => expected [1.0, 1.0, 3.0]. Work the example by hand
    // before coding; the assertion above is the WRONG one — use [1.0, 1.0, 3.0].
}

#[test]
fn params_feed_regions() {
    let src = "Param g(2);\ny = 0;\nif (1) { y = in1 * g; }\nout1 = y;";
    let out = render_with_inputs(src, 48_000.0, &[&[3.0]]);
    assert_eq!(out.ch(0), &[6.0]);
}
```

(Note inside `history_inside_control_flow_program`: the plan deliberately shows the
wrong-then-right reasoning — the committed test asserts `[1.0, 1.0, 3.0]`.)

**Step 2–4:** FAIL → implement: lowering detects control flow (`If/While/DoWhile/For/Break/Continue` anywhere after inlining) → region path: allocate locals (every assigned non-out ident), region inputs (each distinct `inN` + each referenced Param; graph wires Input/Param nodes to region input ports), History decls → region state with `state_init`, `for` desugars to `while`, `do…while` desugars to body + `while`, outputs → `SetOut`. Programs WITHOUT control flow keep the M1 graph path byte-for-byte (probes/bindings preserved — regression-test `h = history(h + 1)`).
Stateful op calls inside regions (`phasor`/`cycle`/`noise`): allocate per-call-site `state_base`. `history(…)` CALLS inside region → error pointing at `History` decls.

**Step 5: Commit** `git commit -am "feat(genexpr): control flow lowers to structured regions"`

---

### Task 16: User-defined functions + multi-assign + return

**TDD scenario:** New feature — full TDD.

**Files:**
- Create: `crates/opengen-genexpr/src/inline.rs` (AST-level pass)
- Modify: `crates/opengen-genexpr/src/{lib.rs,lower.rs}`
- Test: `crates/opengen-genexpr/tests/functions.rs`

**Step 1: Failing tests:**

```rust
use opengen_testkit::render;

#[test]
fn function_inlines_per_call_site() {
    let src = "double(x) { return x * 2; }\nout1 = double(3) + double(4);";
    assert_eq!(render(src, 48_000.0, 1).ch(0)[0], 14.0);
}

#[test]
fn multi_return_destructures() {
    let src = "mm(a, b) { return min(a, b), max(a, b); }\nlo, hi = mm(7, 3);\nout1 = lo; out2 = hi;";
    let out = render(src, 48_000.0, 1);
    assert_eq!((out.ch(0)[0], out.ch(1)[0]), (3.0, 7.0));
}

#[test]
fn function_state_is_per_call_site() {
    // History inside a function: each call site gets independent state (user-docs fact)
    let src = "count() { History h(0); h = h + 1; return h; }\nout1 = count(); out2 = count();";
    let out = render(src, 48_000.0, 3);
    assert_eq!(out.ch(0), out.ch(1)); // both independent counters: 1,2,3 each
}

#[test]
fn recursion_is_rejected() {
    let err = opengen_genexpr::parse_and_lower("f(x) { return f(x); }\nout1 = f(1);").unwrap_err();
    assert!(err.contains("recursi"), "got: {err}");
}
```

**Step 2–4:** FAIL → implement `inline.rs`: collect function decls; substitute call sites bottom-up with renamed locals (`__inl<N>_<name>`); `return` → value bindings (multi-value only legal in multi-assign position); History-in-function → fresh decl per inline instance; recursion (direct or via cycle in the call graph) → LowerError with the cycle named. Multi-assign from non-function calls → error (until D16 abstraction calls in Task 24). Functions containing control flow force the region path after inlining (already handled by Task 15).

**Step 5: Commit** `git commit -am "feat(genexpr): user functions — AST inlining, multi-return, per-call-site state"`

**CHECKPOINT C:** Full grammar parses; `cargo test --workspace` green; doc build clean. Manually spot-check: parse all 80 vendor genexpr examples (`reference/rnbo/genexpr_js/genexprs/*.genexpr`) in a throwaway `#[test] #[ignore]` and record the pass count in the checkpoint summary (do not commit assertions on EULA corpus content — count only). Push.

---

## Phase D — Memory + small stateful operators (Tasks 17–21)

### Task 17: `data` + `peek` + `poke` (+ `buffer` alias) (D8)

**Files:**
- Modify: `crates/opengen-ir/src/lib.rs` (`NodeKind::Data { name, size }`, `Node::data`, `data_ref` on Op via `Node::op_with_data`)
- Modify: `crates/opengen-compile/src/lib.rs` (named ranges; region `data_ref` resolution)
- Create: `crates/opengen-ops/src/memory.rs` (`peek`, `poke` kernels)
- Modify: `crates/opengen-genexpr/src/lower.rs` (`Data`/`Buffer` decls; `peek(name, i)`, `poke(name, v, i)` in both graph and region paths)
- Tests: ops doctests + `crates/opengen-genexpr/tests/data.rs`

Spec anchors: `reference/gen/refpages/dsp/gen_dsp_{data,peek,poke,buffer}.maxref.xml`; D8 scope (1 channel, replace-write, boundmode ignore). **Verify `Data name(size)` vs `Data name(channels, size)` argument order against the refpage before implementing** (user docs and refpage disagree — refpage wins, record the resolution in rustdoc).

Doctest targets:

```rust
/// Data + poke + peek round-trip, including out-of-range reads -> 0:
/// Data d(4); poke(d, in1, 1); out1 = peek(d, 1); out2 = peek(d, 9);
/// With in1 = [42.0]: out1 == [42.0] (NodeId order: poke before peek — document),
/// out2 == [0.0].
```

Sad paths (`#[test]`): `peek(unknown, 0)` → lower error; `poke` OOB writes nothing; region path: `poke` inside `if` (the dang-tools pattern) round-trips.

**Commit:** `git commit -am "feat(ops,ir,compile,genexpr): data/peek/poke — named state regions, buffer alias"`

### Task 18: `delay` — writer + multi-tap readers (D7)

**Files:**
- Modify: `crates/opengen-ops/src/memory.rs` (`delay_write` kernel+update, `delay_read` kernel — linear + none interp via node args)
- Modify: `crates/opengen-genexpr/src/lower.rs` (`Delay d(size);` decl, `d.write(x);`, `d.read(t)` — multiple reads allowed; member calls inside regions → clear error)
- Tests: doctests + `crates/opengen-genexpr/tests/delay.rs`

State layout: synthetic `Data("__delay_<id>", size+1)`; slot 0 = write cursor, 1.. = ring. `delay_write`: deferred port 0, kernel returns 0.0, `update` writes ring + advances cursor. `delay_read`: arity 1 (tap time), `data_ref` to the buffer, kernel clamps `t` to `[1, size]` and interpolates per D7 (genlib facts cited).

Doctests:

```rust
/// Delay d(4); d.write(in1); out1 = d.read(1);
/// in1 = [1,0,0] -> out1 == [0,1,0]   (1-sample echo; reads precede update-phase write)
/// Delay d(8); d.write(in1); out1 = d.read(1.5);
/// in1 = [1,0,0] -> out1 == [0, 0.5, 0.5]  (linear interp between taps 1 and 2)
```

Unit tests: feedback loop `Delay d(64); y = d.read(10) * 0.5 + in1; d.write(y); out1 = y;` compiles (write port deferred) and is `assert_stable!`; unused `Delay` decl (dang-tools `dang_01iv_voice`) compiles silently; tap > size clamps; graph-box form is exercised in Phase E.

**Commit:** `git commit -am "feat(ops,genexpr): delay — deferred ring writer, multi-tap interpolated readers"`

### Task 19: `dcblock` + `slide`

**Files:** `crates/opengen-ops/src/filter.rs`

- `dcblock`: Slots(2) kernel-managed, blocking input. The refpage **documents the exact equivalent GenExpr** (`y = in1 - x1 + y1*0.9997`) — transcribe as the `# Definition` formula, cite `# Documented reference/gen/refpages/dsp/gen_dsp_dcblock.maxref.xml`. Doctest: DC input 1.0 → output starts at 1.0, decays toward 0 (`out[100] < 0.05`); analysis test in `crates/opengen-analysis/tests/filters.rs`: `freq_response("out1 = dcblock(in1);", …)` — `db_at(10.0) < -20`, `db_at(20_000.0) > -0.1`.
- `slide`: Slots(1), arity 3 (input, up, down): `y += (x - y_prev)/max(slide,1)` choosing `up` when rising, `down` when falling; `@init` via node args + `init` hook. Cite refpage + RNBO `reference/rnbo/operators/slide.js` (facts only). Doctests: `slide(x, 1, 1)` is identity; step 0→1 with up=2 → `[0.5, 0.75, 0.875]`.

**Commit:** `git commit -am "feat(ops): dcblock + slide — documented filter semantics"`

### Task 20: Small stateful ops — `sah`, `latch`, `delta`, `change`, `accum`

(Demand: Fors `sah` 282 / `delta` 172, GSOT `latch` 87 / `change` 8 / `accum` 11.)

**Files:** `crates/opengen-ops/src/sample.rs`

| op | arity | state | Definition (verify each refpage digest before committing) | doctest sketch |
|---|---|---|---|---|
| `sah` | 3 (in, ctrl, thresh) | 2 | output held value; sample input when ctrl crosses thresh upward (prev <= thresh < ctrl); `@init` | ctrl ramp crossing 0.5 latches once |
| `latch` | 2 (in, ctrl) | 1 | ctrl != 0 → pass + store input; else output stored | `[5,7,9]` with ctrl `[1,0,1]` → `[5,5,9]` |
| `delta` | 1 | 1 | `x - x[n-1]`, first sample = x - 0 | `[1,4,9]` → `[1,3,5]` |
| `change` | 1 | 1 | sign of delta: +1/-1/0 | `[1,4,4,2]` → `[1,1,0,-1]` |
| `accum` | 2 (in, reset) | 1 | running sum; reset != 0 zeroes before accumulating (verify refpage for pre/post reset order) | `[1,1,1]` → `[1,2,3]`; reset mid-stream |

All kernel-managed state (no deferred ports — inputs are read at compute time). Cite `reference/gen/refpages/dsp/gen_dsp_{sah,latch,delta,change,accum}.maxref.xml` (confirm each path exists; `delta`/`change`/`accum` may live under `common` — cite whichever exists).

**Commit:** `git commit -am "feat(ops): sah latch delta change accum — sample/hold + difference operators"`

### Task 21: Region `data_ref` integration test sweep

Close the loop on D6/D8: a `#[test]` reproducing the dang-tools voice-allocator shape — `Data` + `for` loop + `poke`/`peek` + `History` in one program — asserts a round-robin allocation table evolves correctly over 8 samples, plus `assert_stable!` on outputs. Fix anything it flushes out.

**Commit:** `git commit -am "test(genexpr,compile): region data access — voice-allocator-shaped integration"`

**CHECKPOINT D:** workspace green, docs clean. Push.

---

## Phase E — `.gendsp` loader (Tasks 22–25)

New crate `crates/opengen-gendsp`. **Zero external deps** (workspace-internal: opengen-ir, opengen-ops, opengen-genexpr). Structure-of-format facts: derive from the corpus at runtime + `docs/research/gen_docs/{gendsp_ebnf.md,gendsp_schema.ts}` (user-owned, citable); cross-check against `reference/gen/examples/` (cite paths only).

### Task 22: Crate + minimal JSON parser

**Files:**
- Create: `crates/opengen-gendsp/{Cargo.toml,src/lib.rs,src/json.rs}`

Complete `json.rs` value model + recursive-descent parser (~150 lines):

```rust
//! Minimal JSON parser (zero deps). Supports the full JSON grammar incl.
//! \uXXXX escapes (surrogate pairs), scientific-notation numbers (f64).
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null, Bool(bool), Num(f64), Str(String),
    Arr(Vec<Json>), Obj(Vec<(String, Json)>),
}
impl Json {
    pub fn get(&self, key: &str) -> Option<&Json> { /* Obj lookup, first match */ }
    pub fn as_f64(&self) -> Option<f64> { /* Num */ }
    pub fn as_str(&self) -> Option<&str> { /* Str */ }
    pub fn as_arr(&self) -> Option<&[Json]> { /* Arr */ }
}
pub fn parse(src: &str) -> Result<Json, JsonError>; // JsonError { msg, offset }
/// Parse the first JSON document in a possibly binary-wrapped buffer
/// (.amxd containers: seek to first '{', raw-decode, ignore trailing bytes).
pub fn parse_embedded(bytes: &[u8]) -> Result<Json, JsonError>;
```

TDD: tests for scalars, nesting, escapes (incl. `\u00e4` and a surrogate pair), numbers (`1e-5`, `-0.5`), trailing-garbage tolerance in `parse_embedded`, error offsets on malformed input. Then verify against reality: `#[test]` parses every file matching `reference/gen/examples/*.gendsp` when `reference/` exists (skip otherwise), asserting `patcher.boxes` is an array.

**Commit:** `git commit -am "feat(gendsp): crate skeleton + zero-dep JSON parser (amxd-embedded aware)"`

### Task 23: Patcher model + box-text parser

**Files:** `crates/opengen-gendsp/src/{model.rs,boxtext.rs}`

`model.rs`: extract `Patcher { boxes: Vec<GBox>, lines: Vec<Line> }`, `GBox { id, maxclass, text, code, numinlets, numoutlets, subpatcher: Option<Patcher> }`, `Line { src: (String, u16), dst: (String, u16) }` from `Json` (fields per corpus facts: `patcher.boxes[].box`, `patcher.lines[].patchline.{source,destination}`).

`boxtext.rs`: tokenize `text` on whitespace; tokens from the first `@`-prefixed token onward are attribute pairs (`@attr value…`); leading token = op/class name; remaining = positional args. Each positional arg is parsed with the **opengen-genexpr expression parser** (corpus fact: args can be expressions — `twopi/samplerate`, `spread`). Box kinds (corpus-derived table — the loader's matching logic, with tests per row):

| text shape | meaning |
|---|---|
| `in N [name…] [@comment c]` / `out N …` | I/O ports (1-based) |
| `param NAME [default…] [@min e] [@max e]` | Param (attrs parsed, runtime-ignored, D9) |
| `setparam NAME` | D13 rewiring |
| `f V` / bare numeric text (`0.5`, `75`) | constant |
| `send NAME` / `s NAME`, `receive NAME` / `r NAME` | bus (D9; `r`/`s` aliases — Fors fact) |
| `history [NAME]` | history node, optional binding name |
| `delay SIZE [TAPS]` | delay (TAPS > 1 → clear error, M3) |
| `data NAME SIZE…` / `buffer NAME…` | data region (D10) |
| `gen @file NAME` / `gen @gen NAME` / bare `NAME` (unknown op) | subpatcher/abstraction (Task 25) |
| `expr EXPRESSION…` | expression box: parse text after `expr` as genexpr expression; `inN` identifiers map to box inlets (Fors fact, 66 uses) |
| `mc_channel` | constant 1.0 (D14) |
| `codebox` (maxclass) | embedded GenExpr (Task 24) |
| anything in the op registry | operator box; positional args fill TRAILING inlets; unconnected+unfilled inlets default 0.0 |

**Commit:** `git commit -am "feat(gendsp): patcher model + box-text parser with expression args"`

### Task 24: Graph builder — single patcher

**Files:** `crates/opengen-gendsp/src/build.rs`

Build one `opengen_ir::Graph` from a `Patcher`: nodes per box (table above), edges per `lines` (box outlet → inlet, mapping to IR ports), constant-arg filling, bus resolution (one `send`/bus; every `receive` aliases its source port; multiple sends to one bus → sum via `add` chain — document), named history bindings (`graph.bind`), `setparam` rewiring (D13), codebox splicing: parse `code` with opengen-genexpr and lower **into the host graph** — add `opengen_genexpr::lower_embedded(program, inputs: &[Port], graph: &mut Graph) -> Result<Vec<Port>, LowerError>` (refactor `Lowerer` to take a target graph + seeded `inN` bindings and capture `outN` ports instead of creating Output nodes; codebox `numinlets`/`numoutlets` give the port counts; programs with control flow produce their Region node inside the host graph — this works because regions are just nodes).

TDD anchor tests (authored fixtures in `crates/opengen-gendsp/tests/fixtures/*.gendsp`, hand-written original JSON, committed):
- minimal: `in 1` → `* 0.5` → `out 1`; render via testkit `render_graph_with_inputs` → exact halving.
- param-arg: `param g 3` + `* g` box → `in1 * 3`.
- expression arg: `* twopi/samplerate` matches genexpr render of same formula.
- bus: `send`/`receive` roundtrip; `r`/`s` aliases.
- codebox: a fixture embedding `out1 = in1 + 1;` codebox.
- setparam: param consumers see the driven signal.

**Commit:** `git commit -am "feat(gendsp): graph builder — boxes, lines, buses, codebox splicing, setparam"`

### Task 25: Subpatchers, abstractions, public API

**Files:** `crates/opengen-gendsp/src/{lib.rs,flatten.rs}`

```rust
pub struct LoadOptions { pub search_paths: Vec<std::path::PathBuf> }
pub fn load_gendsp(path: &std::path::Path, opts: &LoadOptions) -> Result<opengen_ir::Graph, GendspError>;
pub fn parse_gendsp_bytes(bytes: &[u8], base_dir: Option<&std::path::Path>, opts: &LoadOptions) -> Result<opengen_ir::Graph, GendspError>;
```

Resolution order per box (D9/D16): embedded `subpatcher` → `<name>.gendsp` in the loaded file's dir → each `search_paths` entry. Flattening: recursively build sub-graph into the host with port mapping; prefix nested param names + bindings `sub<N>/`. Abstraction-as-function from codeboxes (D16): `lower_embedded` gains a callback for unresolved call names → loader inlines the abstraction graph per call site (positional args → `in N`, named args override `Param` defaults, multi-return → `out N`); inside regions → clear error. Cycle of abstraction includes (a.gendsp → b.gendsp → a.gendsp) → error naming the cycle.

Exit-style tests (skip-if-missing): load all 6 `reference/gen/examples` corpus files — assert node counts > 0 and compile succeeds for `crossover` + `freeverb_comb`; full corpus assertions live in Phase G.

**Commit:** `git commit -am "feat(gendsp): subpatcher flattening, abstraction resolution, load_gendsp API"`

**CHECKPOINT E:** loader end-to-end on authored fixtures + crossover/freeverb_comb compile. Push.

---

## Phase F — Conformance harness (Tasks 26–27)

### Task 26: Harness layout, authored patches, skip-if-missing runner

**Files:**
- Create: `conformance/README.md`, `conformance/patches/*.genexpr` (authored ORIGINALS — never copies of reference/corpus patches), `conformance/tolerances.rs` is embedded in the test
- Create: `crates/opengen-analysis/tests/conformance.rs`

Authored patch set (each self-driving, no external input; render protocol: sr 48 000, 4096 samples, golden = mono WAV per output channel `conformance/golden/<stem>.ch<N>.wav`):
- `phasor_incr_order.genexpr` — `out1 = phasor(997);` (settles the M1 `# Observed` wrap/increment-order question; odd freq avoids bin-aligned coincidences)
- `cycle_440.genexpr`, `history_counter.genexpr` (impulse-at-zero via `h = history(h+1); imp = h == 0;`), `delay_echo.genexpr` (impulse into `Delay`+taps), `slide_step.genexpr`, `dcblock_step.genexpr`, `range_inverted_bounds.genexpr` (upgrades Task 5 to `# Observed`), `triangle_duty.genexpr`, `sah_latch.genexpr`

Runner: for each patch, if golden missing → `eprintln!("conformance: SKIP <name> (no golden)")`; else render and compare per-sample against `read_wav` with per-op tolerances:

```rust
fn tolerance(stem: &str) -> f64 {
    match stem {
        "cycle_440" => 5e-3,       // documented # Divergence: f64::sin vs gen~ wavetable
        s if s.starts_with("phasor") => 1e-6, // float32 golden quantization
        _ => 1e-6,
    }
}
```

(`noise` is NEVER conformance-tested — PRNGs differ by design; document in README.) Extend `read_wav` to accept 32-bit float WAVs if it doesn't already. `OPENGEN_BLESS=1` writes our own render as golden (for regression-pinning before Max renders exist — bless-mode goldens live in `conformance/golden-self/`, separate from Max-rendered `conformance/golden/`).

**Commit:** `git commit -am "feat(conformance): authored patch set + skip-if-missing tolerance runner"`

### Task 27: Max-side render kit + machine validation + human checklist

**Files:**
- Create: `conformance/render/render_host.maxpat` (authored original: gen~ loading each patch via `@gen`, `record~` into `buffer~`, `buffer~ write` to WAV — driven by `conformance/render/render_runner.js`)
- Create: `tools/validate-with-genbo.sh` + `tools/validate_gendsp.js` — adapt the user's validator (`docs/research/gen_docs/validate_gendsp.js`, user-owned) to validate (a) every authored conformance patch and (b) every committed loader test fixture with **Max's own genbo parser** via Max's bundled Node. This is a machine-checkable conformance ring that needs no listening and no rendering.
- Create: `conformance/CHECKLIST.md` — the human-in-the-loop steps: open `render_host.maxpat` in Max 9, run the runner script, verify N WAVs appear in `conformance/golden/`, run `cargo test -p opengen-analysis --test conformance`, commit goldens. Includes the explicit note: until goldens land, `phasor` increment-order and `range_inverted_bounds` stay `# Observed`-pending.

Verification for this task (no Max in CI): `bash tools/validate-with-genbo.sh` exits 0 on this machine (Max installed); the script itself skips cleanly with a warning when Max is absent. Run it; paste the output into the commit message body.

**Commit:** `git commit -am "feat(conformance): Max render kit, genbo machine-validation, human checklist"`

**CHECKPOINT F:** conformance suite green (all SKIP or PASS), genbo validation green locally. Push.

---

## Phase G — M2 exit tests + docs + release (Tasks 28–30)

### Task 28: M2 exit tests

**Files:**
- Create: `crates/opengen-analysis/tests/m2_exit.rs` (dev-dep: `opengen-gendsp`)

Shared helpers: `fn reference_dir() -> Option<PathBuf>` (repo `reference/`, skip when absent); `fn corpus_env(var: &str, default: &str) -> Option<PathBuf>`.

**Deep assertions (PRIMARY — Wakefield/official):**

```rust
#[test]
fn exit_crossover_complementary_response() {
    // load reference/gen/examples/crossover.gendsp; impulse on in1; 8192 samples
    // lo (out1): db_at(100) > -1.0, db_at(20_000) < -40.0
    // hi (out2): db_at(20_000) > -3.0, db_at(100) < -40.0
    // lo+hi: |db_at(f)| < 1.0 for f in [100, 1k, 10k] (LR crossover sums allpass-flat)
}
#[test]
fn exit_freeverb_impulse_tail() {
    // freeverb.gendsp (resolves comb/allpass abstractions from the same dir);
    // impulse; 2s render; assert_stable!; rms(0.0-0.5s) > rms(1.5-2.0s) > 0.0
}
#[test]
fn exit_resonator_peaks_at_drive_freq() {
    // gen_resonator.gendsp; in3 freq=440, in4 amp=1, in5 bw≈0.995, in1 impulse;
    // 1s render; assert_stable!; spectrum peak within 10 Hz of 440
}
#[test]
fn exit_gsot_corpus_ratchet() {
    // walk reference/packages/gsot/**/*.gendsp with search_paths = [gsot patchers dir];
    // count load+compile+render(256 samples, silence in)+assert-finite successes;
    // print per-file failures; assert!(ok >= PINNED_GSOT) — pin the observed count
    // at implementation time (D17) and record it in the test's doc comment.
}
```

**Stress + smoke (skip-if-missing, D17 ratchets):**

```rust
#[test]
fn exit_dang_tools_ratchet() { /* OPENGEN_DANG_TOOLS; same ratchet pattern over 36 files */ }
#[test]
fn exit_fors_smoke_ratchet() {
    // OPENGEN_FORS (default /Users/dangayle/Music/Ableton/User Library/Presets/M4L/Fors);
    // parse_embedded each .amxd, walk to classnamespace == "dsp.gen" sub-patchers,
    // build each through the loader; ratchet on build+compile success count.
}
#[test]
fn exit_dattorro_plate_stress() {
    // the hardest single patch: dang-tools dattorro_plate.gendsp — load, compile,
    // impulse, 2s; assert_stable!; tail rms decays. Exercises Delay multi-tap +
    // declarator lists + History inits end-to-end.
}
```

TDD here = write all exit tests first, watch which fail, fix root causes (each fix in the crate that owns it, with its own regression test), iterate until green. **Do not weaken an assertion to pass** — if a corpus file needs an unplanned operator, either add it via the Phase B pattern (one commit) or document it in the ratchet's failure list.

**Commit:** `git commit -am "test: M2 exit — crossover/freeverb/resonator assertions + corpus ratchets"`

### Task 29: Documentation closeout

**Files:**
- Modify: `docs/plans/2026-06-09-opengen-design.md` — Open Items: remove resolved (testkit gaps, phase_at, inverted bounds, scalar ops, structured errors, genlib extraction); add new findings (phasor `# Observed` status incl. conformance-patch path; M3 backlog: multi-channel data, delay multi-tap-in-regions, `wave`, `gate`/multi-out ops, `require`, bitwise `^^` semantics confirmation, Fors `expr`-box edge cases). Update the corpus section with the hierarchy (Wakefield primary / Fors secondary / dang-tools stress) and `OPENGEN_DANG_TOOLS`/`OPENGEN_FORS` env vars.
- Modify: `CLAUDE.md` — production line additions: gendsp evidence protocol (corpus paths, ratchet rule), genlib citation rule (`reference/genlib/gen_dsp/genlib_ops.h`, eula — facts only), `docs/research/gen_docs/` as in-repo research, the update-phase/deferred-port kernel contract, and the genbo machine-validation step for authored patches.
- Modify: `README.md` if crate list changed (opengen-gendsp).

**Commit:** `git commit -am "docs: M2 closeout — open items, corpus hierarchy, production-line updates"`

### Task 30: Release

```bash
cargo test --workspace            # green
cargo doc --workspace --no-deps   # warning-free
git push origin master
git tag v0.2.0-m2 -m "M2: full GenExpr grammar, memory ops, .gendsp loader, conformance harness"
git push origin v0.2.0-m2
```

**Final summary must list:** `# Observed`/`# Divergence` notes added; ambiguities resolved (with D-numbers); human-in-the-loop items awaiting Max renders (golden WAVs per CHECKLIST.md); corpus ratchet numbers (GSOT / dang-tools / Fors); open items carried to M3.

---

## Final checkpoint

- [ ] `cargo test --workspace` green (incl. doctests, conformance SKIP/PASS, exit tests on this machine)
- [ ] `cargo doc --workspace --no-deps` warning-free
- [ ] All corpora load through their ratchets; numbers recorded in test doc comments
- [ ] `reference/`, Fors, dang-tools never committed; no verbatim EULA text anywhere in the repo
- [ ] Design doc + CLAUDE.md updated; pushed; tagged `v0.2.0-m2`
