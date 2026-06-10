# M1 Vertical Slice Implementation Plan

> **REQUIRED SUB-SKILL:** Use the executing-plans skill to implement this plan task-by-task.

**Goal:** End-to-end pipeline — `.genexpr` text → IR → compiled Rust closure patch → analysis assertions — proven by a one-pole lowpass and a phasor-driven oscillator with passing frequency-response tests and working probes.

**Architecture:** Cargo workspace. `opengen-ir` holds a typed dataflow graph; `opengen-ops` is the executable spec (rustdoc + doctests per operator, provenance-tagged); `opengen-genexpr` parses and lowers to IR; `opengen-compile` topo-sorts and builds a `Patch` (flat `f64` state arena + per-sample closure chain); `opengen-analysis` provides impulse/frequency-response assertions; `opengen-testkit` is the doctest façade. Determinism contract: IEEE-754 f64, spec'd evaluation order, seeded PRNG, no fast-math.

**Tech Stack:** Rust (edition 2024), `rustfft` + `hound` (analysis only), `clap` (CLI, Phase D). No deps in `opengen-ir`/`opengen-ops`/`opengen-compile` cores.

**Reference design:** `docs/plans/2026-06-09-opengen-design.md` — read it first, especially Spec Provenance System and Testing rings.

**Conventions for every task:**
- TDD unless marked otherwise. Doctests in `opengen-ops` are spec; sad paths go in `#[test]`.
- Run `cargo test --workspace` before every commit; it must pass.
- Commit messages: conventional commits (`feat:`, `test:`, `docs:`, `chore:`).
- Operator rustdoc uses provenance headings: `# Definition`, then `# Documented` / `# Vendor` / `# Observed` / `# Divergence` as applicable, each with a citation line.

---

## Phase A — Foundation (Tasks 1–2)

### Task 1: Convert to Cargo workspace with crate skeleton

**TDD scenario:** Trivial scaffolding — verification is `cargo test --workspace` passing.

**Files:**
- Modify: `Cargo.toml` (workspace root — replaces the `cargo new` package)
- Delete: `src/main.rs`, `src/`
- Create: `crates/opengen-ir/{Cargo.toml,src/lib.rs}`
- Create: `crates/opengen-ops/{Cargo.toml,src/lib.rs}`
- Create: `crates/opengen-genexpr/{Cargo.toml,src/lib.rs}`
- Create: `crates/opengen-compile/{Cargo.toml,src/lib.rs}`
- Create: `crates/opengen-testkit/{Cargo.toml,src/lib.rs}`
- Create: `crates/opengen-analysis/{Cargo.toml,src/lib.rs}`

**Step 1: Replace root Cargo.toml**

```toml
[workspace]
resolver = "3"
members = ["crates/*"]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"

[workspace.dependencies]
opengen-ir = { path = "crates/opengen-ir" }
opengen-ops = { path = "crates/opengen-ops" }
opengen-genexpr = { path = "crates/opengen-genexpr" }
opengen-compile = { path = "crates/opengen-compile" }
opengen-testkit = { path = "crates/opengen-testkit" }
opengen-analysis = { path = "crates/opengen-analysis" }
```

**Step 2: Remove the binary scaffold**

```bash
rm -rf src
```

**Step 3: Create each crate**

Each `crates/<name>/Cargo.toml` follows this pattern (shown for `opengen-ir`; adjust `name` and `dependencies` per the table below):

```toml
[package]
name = "opengen-ir"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
```

| Crate | `[dependencies]` | `[dev-dependencies]` |
|---|---|---|
| opengen-ir | — | — |
| opengen-ops | opengen-ir | opengen-testkit |
| opengen-genexpr | opengen-ir, opengen-ops | — |
| opengen-compile | opengen-ir, opengen-ops | — |
| opengen-testkit | opengen-genexpr, opengen-compile | — |
| opengen-analysis | opengen-compile, opengen-testkit | — |

(Dev-dependency cycle ops→testkit→…→ops is legal in Cargo.)

Each `src/lib.rs` starts as:

```rust
//! <one-line crate purpose from the design doc>
```

**Step 4: Verify**

Run: `cargo test --workspace`
Expected: compiles, `0 passed; 0 failed` per crate.

**Step 5: Commit**

```bash
git add -A
git commit -m "chore: workspace skeleton (ir, ops, genexpr, compile, testkit, analysis)"
```

---

### Task 2: Max.app reference extraction script

**TDD scenario:** Script — verify by running it; no unit tests.

**Files:**
- Create: `tools/extract-max-refs.sh`
- Create: `tools/max-refs.manifest` (one entry per line: `<license>\t<src-rel-path>\t<dest-rel-path>`)
- Modify: `.gitignore` (add `/reference/`)

**Step 1: Write the manifest** — `tools/max-refs.manifest`:

```text
# license	source (relative to /Applications/Max.app/Contents/Resources)	dest (relative to reference/)
eula	C74/packages/RNBO/server/node_modules/@rnbo/genexpr_js	rnbo/genexpr_js
eula	C74/packages/RNBO/source/operators	rnbo/operators
mit	C74/packages/RNBO/source/rnbo	rnbo/core-mit
eula	C74/packages/Gen/docs/refpages	gen/refpages
eula	C74/docs/userguide/content/gen	gen/userguide
eula	Examples/gen	gen/examples
eula	C74/help/msp	max/help-msp
```

**Step 2: Write the script** — `tools/extract-max-refs.sh`:

```bash
#!/usr/bin/env bash
# Extract reference material from a licensed Max install into ./reference/ (gitignored).
# EULA-tagged items are reference-only: never commit, never quote verbatim.
set -euo pipefail
MAX_RES="${MAX_RES:-/Applications/Max.app/Contents/Resources}"
DEST_ROOT="$(cd "$(dirname "$0")/.." && pwd)/reference"
MAX_VERSION=$(defaults read /Applications/Max.app/Contents/Info.plist CFBundleShortVersionString)

[ -d "$MAX_RES" ] || { echo "Max not found at $MAX_RES (set MAX_RES=...)"; exit 1; }
mkdir -p "$DEST_ROOT"
echo "max_version: $MAX_VERSION" > "$DEST_ROOT/EXTRACTED.txt"
echo "extracted: $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$DEST_ROOT/EXTRACTED.txt"

while IFS=$'\t' read -r license src dest; do
  [[ "$license" =~ ^#.*$ || -z "$license" ]] && continue
  echo "[$license] $src -> reference/$dest"
  mkdir -p "$DEST_ROOT/$dest"
  rsync -a --exclude node_modules "$MAX_RES/$src/" "$DEST_ROOT/$dest/"
  echo "$license	$src	$dest" >> "$DEST_ROOT/EXTRACTED.txt"
done < "$(dirname "$0")/max-refs.manifest"
echo "Done. reference/ is gitignored; EULA items are read-only reference."
```

**Step 3: Make executable, gitignore, run**

```bash
chmod +x tools/extract-max-refs.sh
echo "/reference/" >> .gitignore
./tools/extract-max-refs.sh
```

Expected: `reference/` populated; `git status` shows only `tools/` + `.gitignore`.

**Step 4: Commit**

```bash
git add tools .gitignore
git commit -m "chore: Max.app reference extraction script + manifest (reference/ gitignored)"
```

> Post-M1 follow-ups recorded here, not done now: stash one gen~ code export (genlib) into `reference/genlib/`; clone Cycling74 `gen-plugin-export`.

**CHECKPOINT A:** workspace builds, references extracted. Continue to Phase B.

---

## Phase B — Thin vertical: constants through output (Tasks 3–7)

### Task 3: IR core types

**TDD scenario:** New feature — full TDD.

**Files:**
- Create: `crates/opengen-ir/src/lib.rs` (replace stub)
- Test: same file, `#[cfg(test)] mod tests`

**Step 1: Write failing tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_constant_to_output_graph() {
        let mut g = Graph::new();
        let c = g.add_node(Node::constant(0.75));
        let out = g.add_node(Node::output(0));
        g.connect(Port { node: c, index: 0 }, Port { node: out, index: 0 });
        assert_eq!(g.nodes().count(), 2);
        assert_eq!(g.input_of(Port { node: out, index: 0 }), Some(Port { node: c, index: 0 }));
    }

    #[test]
    fn op_node_carries_kind_and_state_decl() {
        let n = Node::op("history", vec![], StateDecl::Slots(1));
        assert_eq!(n.op_name(), Some("history"));
        assert_eq!(n.state(), StateDecl::Slots(1));
    }
}
```

**Step 2: Run** `cargo test -p opengen-ir` — Expected: FAIL (types undefined).

**Step 3: Implement minimal types**

```rust
//! Typed dataflow IR for opengen. All signals are f64.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Port { pub node: NodeId, pub index: u16 }

/// Explicit state declaration — state is a visible property of the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateDecl { None, Slots(u32) }

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Constant(f64),
    Param { name: String, default: f64 },
    Input(u16),
    Output(u16),
    Op { name: String, args: Vec<f64>, state: StateDecl },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node { pub kind: NodeKind }

impl Node {
    pub fn constant(v: f64) -> Self { Node { kind: NodeKind::Constant(v) } }
    pub fn output(i: u16) -> Self { Node { kind: NodeKind::Output(i) } }
    pub fn input(i: u16) -> Self { Node { kind: NodeKind::Input(i) } }
    pub fn param(name: &str, default: f64) -> Self {
        Node { kind: NodeKind::Param { name: name.into(), default } }
    }
    pub fn op(name: &str, args: Vec<f64>, state: StateDecl) -> Self {
        Node { kind: NodeKind::Op { name: name.into(), args, state } }
    }
    pub fn op_name(&self) -> Option<&str> {
        match &self.kind { NodeKind::Op { name, .. } => Some(name), _ => None }
    }
    pub fn state(&self) -> StateDecl {
        match &self.kind { NodeKind::Op { state, .. } => *state, _ => StateDecl::None }
    }
}

#[derive(Debug, Default)]
pub struct Graph {
    nodes: Vec<Node>,
    /// dest port -> source port
    edges: std::collections::HashMap<Port, Port>,
}

impl Graph {
    pub fn new() -> Self { Self::default() }
    pub fn add_node(&mut self, n: Node) -> NodeId {
        self.nodes.push(n);
        NodeId(self.nodes.len() as u32 - 1)
    }
    pub fn connect(&mut self, from: Port, to: Port) { self.edges.insert(to, from); }
    pub fn node(&self, id: NodeId) -> &Node { &self.nodes[id.0 as usize] }
    pub fn nodes(&self) -> impl Iterator<Item = (NodeId, &Node)> {
        self.nodes.iter().enumerate().map(|(i, n)| (NodeId(i as u32), n))
    }
    pub fn input_of(&self, p: Port) -> Option<Port> { self.edges.get(&p).copied() }
}
```

**Step 4: Run** `cargo test -p opengen-ir` — Expected: PASS.

**Step 5: Commit** `git commit -am "feat(ir): graph, nodes, ports, explicit state decls"`

---

### Task 4: Operator registry + first kernels (`add`, `mul`)

**TDD scenario:** New feature — full TDD. These two establish the kernel pattern every later operator follows.

**Files:**
- Create: `crates/opengen-ops/src/lib.rs` (replace stub), `crates/opengen-ops/src/registry.rs`, `crates/opengen-ops/src/math.rs`

**Step 1: Failing test** (in `registry.rs`):

```rust
#[test]
fn registry_resolves_add() {
    let reg = Registry::core();
    let op = reg.get("add").expect("add registered");
    assert_eq!(op.arity, 2);
    assert_eq!(op.state, opengen_ir::StateDecl::None);
    assert_eq!((op.kernel)(&[1.5, 2.25], &mut [], 48_000.0), 3.75);
}
```

**Step 2: Run** `cargo test -p opengen-ops` — FAIL.

**Step 3: Implement**

`registry.rs`:

```rust
use opengen_ir::StateDecl;

/// A pure-Rust per-sample kernel: (inputs, state slots, samplerate) -> output.
pub type Kernel = fn(&[f64], &mut [f64], f64) -> f64;

pub struct OpDef {
    pub name: &'static str,
    pub arity: u16,
    pub state: StateDecl,
    pub kernel: Kernel,
}

pub struct Registry { ops: std::collections::HashMap<&'static str, OpDef> }

impl Registry {
    pub fn core() -> Self {
        let mut ops = std::collections::HashMap::new();
        for def in crate::math::defs() { ops.insert(def.name, def); }
        Registry { ops }
    }
    pub fn get(&self, name: &str) -> Option<&OpDef> { self.ops.get(name) }
}
```

`math.rs` — spec module format used by ALL future operators:

```rust
//! Arithmetic operators.
use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Add two signals: `out = a + b`.
///
/// # Definition
/// IEEE-754 f64 addition. No saturation, no denormal handling.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_add.maxref.xml` (Max 9.x).
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = 1.5 + 2.25;", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 3.75);
/// ```
pub fn add(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 { inputs[0] + inputs[1] }

/// Multiply two signals: `out = a * b`. (Same shape; doctest `2.0 * 0.75 == 1.5`.)
pub fn mul(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 { inputs[0] * inputs[1] }

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef { name: "add", arity: 2, state: StateDecl::None, kernel: add },
        OpDef { name: "mul", arity: 2, state: StateDecl::None, kernel: mul },
    ]
}
```

`lib.rs`: `pub mod registry; pub mod math; pub use registry::{Registry, OpDef, Kernel};`

NOTE: the doctests will not pass until Task 7 (testkit). Mark them ` ```ignore ` for now; Task 7 removes the `ignore`.

**Step 4: Run** `cargo test -p opengen-ops` — PASS (unit test; doctests ignored).

**Step 5: Commit** `git commit -am "feat(ops): registry + add/mul kernels with spec-format rustdoc"`

---

### Task 5: Compile — topo sort + closure patch

**TDD scenario:** New feature — full TDD.

**Files:**
- Create: `crates/opengen-compile/src/lib.rs` (replace stub)

**Step 1: Failing test**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use opengen_ir::*;

    fn const_add_graph() -> Graph {
        let mut g = Graph::new();
        let a = g.add_node(Node::constant(1.5));
        let b = g.add_node(Node::constant(2.25));
        let add = g.add_node(Node::op("add", vec![], StateDecl::None));
        let out = g.add_node(Node::output(0));
        g.connect(Port { node: a, index: 0 }, Port { node: add, index: 0 });
        g.connect(Port { node: b, index: 0 }, Port { node: add, index: 1 });
        g.connect(Port { node: add, index: 0 }, Port { node: out, index: 0 });
        g
    }

    #[test]
    fn compiles_and_processes_constant_add() {
        let mut patch = compile(&const_add_graph(), &opengen_ops::Registry::core(), 48_000.0).unwrap();
        let out = patch.process(&[]);
        assert_eq!(out, vec![3.75]);
    }

    #[test]
    fn rejects_cycle_without_history() {
        let mut g = Graph::new();
        let add = g.add_node(Node::op("add", vec![], StateDecl::None));
        g.connect(Port { node: add, index: 0 }, Port { node: add, index: 0 }); // self-loop
        let out = g.add_node(Node::output(0));
        g.connect(Port { node: add, index: 0 }, Port { node: out, index: 0 });
        let err = compile(&g, &opengen_ops::Registry::core(), 48_000.0).unwrap_err();
        assert!(err.to_string().contains("feedback requires history or delay"));
    }
}
```

**Step 2: Run** `cargo test -p opengen-compile` — FAIL.

**Step 3: Implement.** Core shape (fill in straightforwardly):

```rust
//! IR -> executable Patch. Deterministic: spec'd order, f64 only.
use opengen_ir::*;
use opengen_ops::Registry;

#[derive(Debug)]
pub struct CompileError(pub String);
impl std::fmt::Display for CompileError { /* delegate to .0 */ }
impl std::error::Error for CompileError {}

/// One compiled step: reads value slots, writes its value slot, may touch state arena.
struct Step { kernel: opengen_ops::Kernel, inputs: Vec<usize>, value_slot: usize, state_range: std::ops::Range<usize> }

pub struct Patch {
    steps: Vec<Step>,
    values: Vec<f64>,       // one slot per node output
    state: Vec<f64>,        // flat state arena
    outputs: Vec<usize>,    // value slots feeding Output nodes, by output index
    sr: f64,
}

impl Patch {
    /// Process one sample frame. Deterministic order = topo order (ties broken by NodeId).
    pub fn process(&mut self, inputs: &[f64]) -> Vec<f64> { /* run steps, gather outputs */ }
}

pub fn compile(g: &Graph, reg: &Registry, sr: f64) -> Result<Patch, CompileError> {
    // 1. Kahn topo sort over nodes; stateful ops (StateDecl != None) break cycles
    //    (their input edge is deferred — reads previous sample's state).
    //    Unbreakable cycle -> Err("feedback requires history or delay").
    // 2. Allocate value slot per node, state arena ranges per stateful op.
    // 3. Constants prefill their value slot; Inputs copy from process() args.
    // 4. Build Step list in topo order (ties: ascending NodeId — the determinism contract).
}
```

**Step 4: Run** `cargo test -p opengen-compile` — PASS.

**Step 5: Commit** `git commit -am "feat(compile): topo-sorted closure patch, cycle detection, state arena"`

---

### Task 6: GenExpr parser — expressions, assignment, params

**TDD scenario:** New feature — full TDD. Grammar reference: `reference/rnbo/genexpr_js/genexpr.pegjs` (read for precedence facts; do not copy text — provenance `Vendor`).

**Files:**
- Create: `crates/opengen-genexpr/src/{lib.rs,lexer.rs,parser.rs,ast.rs,lower.rs}`
- Test: `crates/opengen-genexpr/tests/parse.rs`

Scope for M1: numeric literals, identifiers, `+ - * /` with C precedence, unary `-`, parens, function-call syntax `name(arg, ...)`, statements `ident = expr;`, outputs `out1..outN`, inputs `in1..inN`, `Param name(default);`. Everything else is a clear parse error.

**Step 1: Failing tests** (`tests/parse.rs`):

```rust
use opengen_genexpr::parse;

#[test]
fn parses_precedence() {
    let ast = parse("out1 = 1 + 2 * 3;").unwrap();
    assert_eq!(format!("{ast:?}").contains("Add"), true); // top node is Add(1, Mul(2,3))
}

#[test]
fn parses_param_and_call() {
    parse("Param freq(440); out1 = cycle(freq);").unwrap();
}

#[test]
fn rejects_unknown_statement() {
    assert!(parse("for (;;) {}").is_err()); // not in M1 scope — clear error
}
```

Plus a lowering test: `lower(parse("out1 = 0.5 + 0.25;")?)` produces a Graph that compiles (via opengen-compile) and outputs `0.75`.

**Step 2: Run** — FAIL.

**Step 3: Implement** hand-written lexer (tokens: number, ident, punct) + recursive-descent parser (precedence climbing) + `lower.rs` mapping AST → `opengen_ir::Graph`, resolving call names and binary ops through `opengen_ops::Registry` (binary `+` → `"add"`, `*` → `"mul"`, etc.). Unknown operator name → `LowerError` listing the unknown name.

**Step 4: Run** `cargo test -p opengen-genexpr` — PASS.

**Step 5: Commit** `git commit -am "feat(genexpr): lexer, parser (expr/assign/Param), lowering to IR"`

---

### Task 7: Testkit façade + un-ignore spec doctests

**TDD scenario:** New feature — the doctests written in Task 4 are the failing tests.

**Files:**
- Create: `crates/opengen-testkit/src/lib.rs` (replace stub)
- Modify: `crates/opengen-ops/src/math.rs` (remove ` ```ignore `)

**Step 1: Implement the façade**

```rust
//! Doctest/test façade: compile GenExpr source and render n samples.
use opengen_compile::{compile, Patch};

pub struct Render { channels: Vec<Vec<f64>> }
impl Render {
    pub fn ch(&self, i: usize) -> &[f64] { &self.channels[i] }
}

/// Compile `src` at samplerate `sr` and render `n` samples (no inputs).
/// Panics on compile error — doctests want loud failures.
pub fn render(src: &str, sr: f64, n: usize) -> Render {
    let graph = opengen_genexpr::parse_and_lower(src).expect("parse");
    let mut patch = compile(&graph, &opengen_ops::Registry::core(), sr).expect("compile");
    let outs = patch.output_count();
    let mut channels = vec![Vec::with_capacity(n); outs];
    for _ in 0..n {
        let frame = patch.process(&[]);
        for (c, v) in channels.iter_mut().zip(frame) { c.push(v); }
    }
    Render { channels }
}
```

(Add `Patch::output_count()` and `opengen_genexpr::parse_and_lower` as needed.)

**Step 2: Remove `ignore` from Task 4 doctests.**

**Step 3: Run** `cargo test --workspace` — Expected: PASS including doctests. **This is the thin vertical slice working end-to-end.**

**Step 4: Commit** `git commit -am "feat(testkit): render() façade; ops doctests now executable spec"`

**CHECKPOINT B:** `render("out1 = 1.5 + 2.25;", 48000.0, 1)` works via doctest. Pipeline proven. Review before Phase C.

---

## Phase C — State, oscillators, probes (Tasks 8–11)

### Task 8: Remaining stateless math + comparison operators

**TDD scenario:** Repetition of the Task 4 pattern. For each operator: spec rustdoc (Definition + provenance citations) → doctest → kernel → `cargo test -p opengen-ops` → commit per group.

**Files:** extend `crates/opengen-ops/src/math.rs`, create `compare.rs`, `range.rs`.

Operators and their defining doctest assertions (consult `reference/gen/refpages/common/` for documented behavior; `reference/rnbo/genexpr_js/operator_exprs.json` for `Vendor` evidence):

- `sub`, `div`, `mod` (gen `%` follows C `fmod` semantics — cite refpage), `neg`, `abs`
- `min`, `max`, `pow`, `sqrt`, `floor`, `ceil`
- `gt/gte/lt/lte/eq/neq` (return exactly `0.0`/`1.0`)
- `clip(x, lo, hi)` — boundary doctest: `clip(1.0, 0.0, 1.0) == 1.0`
- `wrap(x, lo, hi)` — **boundary spec**: `wrap(1.0, 0.0, 1.0) == 0.0` (high bound exclusive); negative input doctest `wrap(-0.25, 0.0, 1.0) == 0.75`
- `fold(x, lo, hi)` — `fold(1.25, 0.0, 1.0) == 0.75`
- `scale(x, inlo, inhi, outlo, outhi)` (linear M1 form), `mix(a, b, t) == a + t*(b - a)`

Parser additions: `-` `/` `%` binary ops and comparison operators map to these kernels (extend Task 6 lowering table; one test per token).

**Commit:** one commit per module (`feat(ops): comparison operators`, etc.).

---

### Task 9: `history` — first stateful operator + feedback

**TDD scenario:** New feature — full TDD. This validates the state arena and cycle-breaking from Task 5.

**Files:** create `crates/opengen-ops/src/state.rs`; modify compile if deferred-edge handling needs completion.

**Step 1: Spec doctest** (in `state.rs`):

```rust
/// Single-sample delay. Read returns the PREVIOUS sample's written value.
///
/// # Definition
/// y[n] = x[n-1]; y[0] = init (default 0.0).
///
/// # Documented
/// reference/gen/refpages/dsp/gen_dsp_history.maxref.xml (Max 9.x).
///
/// ```
/// use opengen_testkit::render;
/// // counter via feedback: acc = history(acc + 1)
/// let out = render("h = history(h + 1); out1 = h;", 48000.0, 3);
/// assert_eq!(out.ch(0), &[0.0, 1.0, 2.0]);
/// ```
```

**Step 2–4:** FAIL → implement kernel (`state[0]` read-then-write ordering per definition) + ensure lowering allows an identifier to be self-referential only through `history` → PASS.

**Step 5:** Sad-path `#[test]`: direct cycle without history still errors (regression on Task 5).

**Step 6: Commit** `git commit -am "feat(ops): history — stateful kernel, feedback through state"`

---

### Task 10: `phasor`, `cycle`, `noise`

**TDD scenario:** Full TDD; these are the exactness showcases.

**Files:** extend `crates/opengen-ops/src/state.rs` (or `osc.rs`).

- **`phasor(freq)`** — Definition: `y[n] = wrap(y[n-1] + freq/sr, 0, 1)`, `y[0] = 0.0`. Doctest: `render("out1 = phasor(1000);", 48000.0, 3).ch(0) == &[0.0, 1000.0/48000.0, 2000.0/48000.0]` (exact `==`). Provenance: `# Vendor` cite `reference/rnbo/operators/phasor.js` (notes RNBO emits pre-increment value and applies wrap before increment — verify our definition against gen~ conformance later; record as open `# Observed` item).
- **`cycle(freq)`** — Definition: `sin(2π · phase)` where phase is a phasor. Doctest: first sample `== 0.0` exactly; sample at quarter period within 1 ulp of `1.0`. `# Divergence`: gen~ uses interpolated wavetable; we use `f64::sin`. Document it.
- **`noise()`** — Definition: uniform [-1, 1) from a seeded xoshiro256++ (implement in-crate, no dep; cite the public xoshiro reference). Patch seed defaults to a fixed constant; doctest: two renders produce identical output (determinism), values within [-1, 1).

**Commit per operator.**

---

### Task 11: Probes

**TDD scenario:** New feature — full TDD.

**Files:** modify `crates/opengen-compile/src/lib.rs`; test in `crates/opengen-compile/tests/probes.rs`.

**Step 1: Failing test**

```rust
#[test]
fn probe_records_interior_wire() {
    let graph = opengen_genexpr::parse_and_lower("h = history(h + 1); out1 = h * 2;").unwrap();
    let mut patch = opengen_compile::compile_with_probes(
        &graph, &opengen_ops::Registry::core(), 48_000.0, &["h"]).unwrap();
    for _ in 0..3 { patch.process(&[]); }
    assert_eq!(patch.probe("h").unwrap(), &[0.0, 1.0, 2.0]);
}
```

**Step 2–4:** FAIL → implement: lowering records `name -> NodeId` bindings in the Graph (add a `bindings` map to `opengen_ir::Graph`); `compile_with_probes` tags those value slots and copies them into per-probe `Vec<f64>` each sample → PASS.

**Step 5: Commit** `git commit -am "feat(compile): named probes record interior wires per sample"`

**CHECKPOINT C:** stateful ops + probes working. Review before Phase D.

---

## Phase D — Analysis + M1 exit criteria (Tasks 12–14)

### Task 12: Analysis crate v1

**TDD scenario:** New feature — full TDD. Add deps to `opengen-analysis/Cargo.toml`: `rustfft = "6"`, `hound = "3"`.

**Files:** `crates/opengen-analysis/src/{lib.rs,response.rs,wav.rs}`; tests inline.

**Step 1: Failing test** — validate `freq_response` against an analytically known filter:

```rust
#[test]
fn one_pole_lowpass_minus_3db_near_cutoff() {
    // y = mix(y[n-1], x, g) one-pole; g chosen for fc ≈ 1 kHz at 48 kHz:
    // g = 1 - exp(-2π·fc/sr)  (cite: standard one-pole relation; verified against
    // scipy.signal.freqz at authoring time — constants below from that session)
    let src = "h = history(mix(h, in1, 0.12278));
               out1 = h;";
    let h = freq_response(src, 48_000.0, 8192);
    let db = h.db_at(1_000.0);
    assert!((db - (-3.01)).abs() < 0.1, "got {db} dB at 1 kHz");
}
```

**Step 2–4:** FAIL → implement:
- `impulse_response(src, sr, n) -> Vec<f64>` — render with `in1` = unit impulse (needs `Patch::process(&[x])` input support; wire `in1..inN` through testkit).
- `freq_response(src, sr, nfft) -> Response` — FFT of impulse response; `Response::db_at(hz)`, `phase_at(hz)` with bin interpolation.
- `wav.rs`: `write_wav(path, &[f64], sr)`, `read_wav(path)`, and `assert_render_matches!(src, golden_path, tol)` (skipped-if-missing + `OPENGEN_BLESS=1` to write goldens).

→ PASS.

**Step 5: Commit** `git commit -am "feat(analysis): impulse/freq response, WAV golden infra"`

---

### Task 13: M1 exit tests

**TDD scenario:** These ARE the tests — integration tests proving M1's exit criteria.

**Files:** `crates/opengen-analysis/tests/m1_exit.rs`

```rust
//! M1 exit criteria (design doc, Milestones).
use opengen_analysis::*;

#[test]
fn exit_one_pole_lowpass_response() {
    let src = "Param g(0.12278); h = history(mix(h, in1, g)); out1 = h;";
    let h = freq_response(src, 48_000.0, 8192);
    assert!((h.db_at(1_000.0) + 3.01).abs() < 0.1);
    assert!(h.db_at(100.0) > -0.2);          // passband flat
    assert!(h.db_at(20_000.0) < -20.0);      // stopband falling
}

#[test]
fn exit_phasor_driven_oscillator() {
    let src = "out1 = cycle(440);";
    let r = opengen_testkit::render(src, 48_000.0, 48_000);
    let h = spectrum(r.ch(0), 48_000.0);
    assert!((h.peak_hz() - 440.0).abs() < 1.0);   // fundamental where expected
    assert!(h.db_at(880.0) < -90.0);              // pure sine: no harmonics
}

#[test]
fn exit_probes_work_on_real_patch() { /* probe "h" in the lowpass; assert non-empty, monotone-converging on DC input */ }
```

Implement `spectrum()`/`peak_hz()` in analysis as needed. Run: `cargo test --workspace` — ALL PASS = **M1 complete**.

**Commit** `git commit -am "test: M1 exit criteria — lowpass response, oscillator spectrum, probes"`

---

### Task 14: CLI (`run`, `plot`, `probe`)

**TDD scenario:** Thin shell over tested libraries — judgment; one smoke test.

**Files:** `crates/opengen-cli/{Cargo.toml,src/main.rs}` (deps: workspace crates + `clap = { version = "4", features = ["derive"] }`; add `plotters = "0.3"` to analysis for SVG when wiring `plot`).

Subcommands:
- `opengen run patch.genexpr --sr 48000 --samples N [--wav out.wav]` — render to stdout stats or WAV
- `opengen plot patch.genexpr --response out.svg` — freq-response SVG via analysis
- `opengen probe patch.genexpr --tap h --samples 100` — print tapped values

Smoke test: `tests/cli.rs` runs the binary via `std::process::Command` on a fixture patch; asserts exit 0 and expected stdout shape.

**Commit** `git commit -am "feat(cli): run/plot/probe subcommands"`

---

## Final checkpoint

- [ ] `cargo test --workspace` green
- [ ] `cargo doc --workspace --no-deps` renders the operator spec readably
- [ ] Design doc + this plan committed; push to private GitHub remote
- [ ] Open items for M2 recorded in design doc (conformance harness, full grammar, `.gendsp` loader, gen~ phasor semantics `# Observed` follow-up from Task 10)
