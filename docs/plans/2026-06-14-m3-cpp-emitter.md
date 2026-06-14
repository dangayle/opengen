# M3 — C++ Emitter and Language Completion

> **REQUIRED SUB-SKILL:** Use the executing-plans skill to implement this plan task-by-task.

**Goal:** Language-complete the GenExpr frontend (fix remaining parse failures, implement missing operators, add strict-mode lints), then emit dependency-free C++ from the IR that matches the Rust backend bit-for-bit.

**Architecture:** A new `opengen-emit-cpp` crate reads the same typed dataflow `Graph` that `opengen-compile` uses. It produces a single self-contained `.h`/`.cpp` pair: a flat `f64` state arena, a topo-sorted per-sample `process(float* in, float* out)` function, and a `set_param(name, value)` API. Kernels are emitted by a **function-based template system** — each `OpDef` carries an `emit_cpp` function pointer that receives the state offset and sample rate at emission time, avoiding the static-string problem. The shared determinism contract (IEEE-754 f64, spec'd evaluation order, no fast-math, seeded PRNG) is the cross-backend test — `cargo test` renders both backends and asserts bit-identical output.

**Phasing strategy:** Language completion (Phase 1) runs first so the IR and operator set are complete before codegen starts. The C++ emitter (Phase 2) targets a fixed set of operators — no moving target. Conformance and polish (Phase 3) wraps up.

**Tech Stack:** Rust (emitter), C++17 (output), existing IR/registry/compile crates unchanged.

---

## Phase 1: Language Completion (Tasks 1–15)

These tasks resolve the accumulated M2 backlog. Every operator and parse fix lands here before the emitter touches the IR, so Phase 2's exit criteria are against a stable operator count.

---

### Task 1: Characterize the 4 remaining vendor parse failures

**TDD scenario:** New feature — write tests first (characterization = test discovery)

**Files:**
- Create: `crates/opengen-genexpr/tests/vendor_corpus.rs`

**Goal:** Before fixing, identify exactly what the 4 remaining failures are. The vendor corpus lives at `reference/gen_exprs/genexpr_js/genexprs/` (80 `.genexpr` files). Currently 76/80 parse. Run the corpus, extract the 4 failing filenames, and write a minimal reproducing test for each.

**Step 1: Write a corpus runner that reports per-file pass/fail**

```rust
#[test]
fn vendor_genexpr_corpus_report() {
    // Lists every file and its parse result — run once to identify failures
    let failures = run_vendor_corpus(); // returns Vec<(filename, error_message)>
    for (file, err) in &failures {
        eprintln!("FAIL {}: {}", file, err);
    }
    // Not an assertion — informational. Once all pass, change to assert 80/80.
}
```

**Step 2: Run and record the 4 failing files + error messages**

Run: `cargo test -p opengen-genexpr -- vendor_genexpr_corpus_report`
Expected: 4 failure lines printed to stderr. Record each.

**Step 3: Write a minimal reproducing test per failure**

```rust
// For each of the 4 failures, write a test like:
#[test]
fn parse_failure_1_comma_in_for_init() {
    let src = "for(i=0, j=0; i<10; i=i+1) { out1 = i; }";
    let result = opengen_genexpr::parse(src);
    assert!(result.is_ok(), "comma in for-init should parse: {}", result.unwrap_err());
}
```

**Step 4: Commit**

```bash
git commit -m "test(genexpr): characterize 4 remaining vendor parse failures"
```

---

### Task 2: Fix parser failures — comma expressions in for-init

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/ast.rs`
- Modify: `crates/opengen-genexpr/tests/vendor_corpus.rs`

**Goal:** `for(i=0, j=0; i<10; i=i+1)` — the comma between `i=0` and `j=0` is currently rejected. The parser's for-init path needs to handle comma-separated assignment/declaration expressions.

**Step 1–5:** TDD cycle. Verify against minimal test + full vendor corpus.

**Commit:** One commit.

---

### Task 3: Fix parser failures — named arguments inside function calls

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/tests/vendor_corpus.rs`

**Goal:** `foo(bar=baz)` — named/keyword arguments in function calls are currently unparsed. The parser needs to accept `name=expr` inside call argument lists.

**Step 1–5:** TDD cycle.

**Commit:** One commit.

---

### Task 4: Fix remaining 2 parser failures

**TDD scenario:** New feature — full TDD cycle (one per failure)

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/lexer.rs`
- Modify: `crates/opengen-genexpr/tests/vendor_corpus.rs`

**Goal:** Fix the 2 remaining failures identified in Task 1. Each gets its own TDD commit.

**Step 1–5 per failure:** TDD cycle.

**Commit:** One commit per failure.

**Exit:** Vendor corpus assertion changed to `assert_eq!(failures.len(), 0)` — 80/80 parse.

---

### Task 5: Declaration-ordering strict mode

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/ast.rs` (add `StrictMode` flag or `parse_strict` entry point)
- Modify: `crates/opengen-genexpr/tests/` (new test file `strict_mode.rs`)

**Goal:** Real gen~ rejects declarations after expression statements with "declarations must come before expressions" (observed Max 9 2026-06-10). opengen's parser is lenient by default. Implement a strict mode that enforces gen~'s ordering.

**Design:** `parse_strict(src)` is a separate entry point. It parses normally, then validates that all `Decl` AST nodes precede all `Stmt::Expr` nodes. Returns `Err` on violation.

**Test:**
```rust
#[test]
fn strict_mode_rejects_decl_after_expression() {
    // out1 = in1 is an expression, h = history(...) is a declaration
    let src = "out1 = in1; h = history(in1);";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("declaration after expression"));
}

#[test]
fn strict_mode_allows_decl_before_expr() {
    let src = "h = history(in1); out1 = h;";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_ok());
}
```

**Commit:** One commit.

---

### Task 6: Self-referential history lint

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs` (strict mode validation)
- Create: `crates/opengen-genexpr/tests/strict_mode.rs`

**Goal:** `h = history(h + 1)` is an opengen leniency — real gen~ errors "variable h is not defined" (observed Max 9 2026-06-10). gen~ requires the `History h` extern-style declaration before use. In strict mode, detect self-referential history and reject it.

**Design:** In `parse_strict()`, walk the program tree and check: if a `history()` call's first argument references a variable whose LHS assignment is the same name, emit an error.

**Test:**
```rust
#[test]
fn strict_mode_rejects_self_referential_history() {
    let src = "h = history(h + 1);";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not defined"));
}

#[test]
fn strict_mode_allows_non_self_referential_history() {
    let src = "x = in1; h = history(x); out1 = h;";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_ok());
}
```

**Commit:** One commit.

---

### Task 7: `^^` precedence conformance cross-check

**TDD scenario:** Modifying tested code — run existing tests first

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/tests/parser_v2.rs`

**Goal:** The vendor PEG grammar defines `||` → `^^` → `&&` (after bitwise `|`/`^`/`&` removal, this is the complete logical precedence chain). Verify the parser's precedence matches and fix if needed.

**Source:** `reference/rnbo/genexpr_js/genexpr.pegjs` — PEG grammar precedence ladder.

**Test:**
```rust
#[test]
fn logical_precedence_xor_binds_tighter_than_or() {
    // Should parse as: a || (b ^^ c), not (a || b) ^^ c
    let src = "out1 = in1 || in2 ^^ in3;";
    let program = opengen_genexpr::parse(src).unwrap();
    // Verify via lowering: render (in1 || (in2 ^^ in3)) and ((in1 || in2) ^^ in3)
    // with known inputs; assert the output matches the ||→^^ tree
}

#[test]
fn logical_precedence_and_binds_tighter_than_xor() {
    let src = "out1 = in1 ^^ in2 && in3;";
    let program = opengen_genexpr::parse(src).unwrap();
    // Should parse as: in1 ^^ (in2 && in3)
}
```

**Commit:** One commit (either fix + test, or just test confirming current behavior is correct).

---

### Task 8: Implement `selector` operator

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-ops/src/selector.rs`
- Modify: `crates/opengen-ops/src/registry.rs` (register)

**Goal:** Implement gen~'s `selector` with correct float-threshold semantics.

**Semantics (verified against gen~ refpage `reference/gen/refpages/common/gen_common_selector.maxref.xml`):**
- `selector(index, in1, in2, ..., inN)` — N signals, one index signal
- Float threshold comparison: output = in1 when index < 1, output = in2 when 1 ≤ index < 2, ..., output = inN when index ≥ N−1
- At exact integer boundaries, the NEXT input is selected (index ≥ 1 → in2)
- Out-of-range: index < 0 → in1; index ≥ N−1 → inN

**NOT integer-indexed.** This is the critical distinction from the original plan's incorrect description.

**Kernel signature:** `fn(inputs: &[f64], state: &mut [f64], sr: f64) -> f64`
Arity: variable (2–17 inputs: 1 index + 1–16 signal inputs).

**Provenance tags:** `# Documented: reference/gen/refpages/common/gen_common_selector.maxref.xml`

**Step 1–5:** Standard operator production line (spec → doctests → kernel → verify → commit).

---

### Task 9: Implement `gate` operator

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-ops/src/gate.rs`
- Modify: `crates/opengen-ops/src/registry.rs`

**Goal:** Implement `gate` with correct sample-and-hold semantics.

**Semantics (verified against gen~ refpage):**
- `gate(trigger, input)` — two signals
- When trigger > 0: output = input (gate open, passes through)
- When trigger ≤ 0: output = last value passed while gate was open (sample-and-hold)
- Stateful: stores the last passed value in 1 slot of state
- Initial state: 0.0 (gate starts closed, output is 0 until first trigger > 0)

**NOT combinatorial** — this is a stateful operator, unlike `switch` (which is combinatorial). This is critical for correct behavior.

**Kernel:** `fn(inputs: &[f64], state: &mut [f64], sr: f64) -> f64`
Arity: 2. StateDecl: Slots(1).

**Provenance:** `# Documented: reference/gen/refpages/common/gen_common_gate.maxref.xml`

**Step 1–5:** Standard operator production line.

---

### Task 10: Implement `elapsed` operator

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-ops/src/elapsed.rs`
- Modify: `crates/opengen-ops/src/registry.rs`

**Goal:** Implement `elapsed` with correct time-in-milliseconds semantics.

**Semantics (verified against gen~ refpage):**
- `elapsed(trigger)` — one signal
- Output: time in **milliseconds** since the trigger was last non-zero
- When trigger > 0: output = 0, reset counter
- When trigger ≤ 0: output = counter * (1000.0 / sr), counter increments each sample
- Stateful: stores the sample counter in 1 slot of state

**NOT sample count** — the output is milliseconds, using sample-rate-dependent conversion.

**Kernel:** `fn(inputs: &[f64], state: &mut [f64], sr: f64) -> f64`
Arity: 1. StateDecl: Slots(1).

**Provenance:** `# Documented: reference/gen/refpages/common/gen_common_elapsed.maxref.xml`

**Step 1–5:** Standard operator production line.

---

### Task 11: Implement `wave` operator

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-ops/src/wave.rs`
- Modify: `crates/opengen-ops/src/registry.rs`

**Goal:** Wavetable oscillator reading from a `Data`/`Buffer` node.

**Semantics:**
- `wave(data_ref, phase)` — reads a Data/Buffer node at normalised phase (0.0–1.0)
- Linear interpolation between adjacent samples
- Phase wraps: fractional part used for read position
- Reads from the graph's Data node (cross-references via data_ref in Op)

**Design note:** Like `peek`/`poke`, `wave` accesses a `Data` node by name. The Op's `data_ref` field links to the data node. The kernel receives the data buffer via the IR's data-mapping at compile time.

**Kernel:** `fn(inputs: &[f64], state: &mut [f64], sr: f64) -> f64`
Arity: 2 (data_ref resolved at compile time, not a runtime input).

**Step 1–5:** Standard operator production line.

---

### Task 12: Implement remaining gen~ operators (`smoothstep`, `step`, `cartopol`, `poltocar`)

**TDD scenario:** New feature — full TDD cycle (one per operator)

**Files:**
- Create: `crates/opengen-ops/src/smoothstep.rs`
- Create: `crates/opengen-ops/src/step.rs`
- Create: `crates/opengen-ops/src/cartopol.rs`
- Modify: `crates/opengen-ops/src/registry.rs`

**Goal:** Four operators from the gen~ reference set that aren't yet implemented.

**`smoothstep(low, high, x)`** — Hermite interpolation: maps x from [low, high] to [0, 1] using smooth S-curve. Formula: `t = clamp((x-low)/(high-low), 0, 1); return t*t*(3-2*t)`. Arity: 3, stateless.

**`step(threshold, x)`** — Heaviside step function: returns 1.0 when x ≥ threshold, 0.0 otherwise. Arity: 2, stateless. Domain note: gen~ has both `step` (two-arg) and a signal-rate variant; implement the basic form.

**`cartopol(x, y)`** — Cartesian to polar: given (x, y), returns (magnitude, phase). Two outputs — requires multi-out support or the gen~ convention of returning both as a packed signal pair on alternate channels.

**`poltocar(mag, phase)`** — Polar to Cartesian: given (magnitude, phase), returns (x, y). Inverse of `cartopol`. Same multi-out consideration.

**`rmod(a, b)`** — Reverse mod: computes `b % a` (swapped argument order). Arity: 2, stateless. Simpler than `cartopol`/`poltocar` — implement first.

**Multi-out design for `cartopol`/`poltocar`:** For M3, emit them as two separate output channels via the IR's multi-outlet support. Each operator node produces two outputs (outlet 0 = magnitude/x, outlet 1 = phase/y). This avoids a packed-signal convention.

**Step 1–5 per operator:** Standard production line. `rmod` can share a commit with `step` (both trivial stateless ops).

**Commit:** One commit per operator or small batch (`rmod` + `step` together; `smoothstep` alone; `cartopol` + `poltocar` together).

---

### Task 13: Multi-tap delay (TAPS > 1)

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-gendsp/src/build.rs` (remove the `TAPS > 1 → Err` guard; wire multi-tap outlets)
- Modify: `crates/opengen-gendsp/src/flatten.rs` (same for subpatcher flattening path)
- Modify: `crates/opengen-gendsp/tests/build_fixtures.rs` (regression test)

**Goal:** The current code returns `Err("multi-tap (TAPS=N) not yet supported (M3)")`. Remove this guard and wire the additional delay_read outlets correctly. Multi-tap delay boxes have `numoutlets = TAPS` — each tap is a separate delay_read with a different offset.

**Design:** For `delay SIZE TAPS`:
- One `delay_write` node (inlet 0 = signal)
- `TAPS` `delay_read` nodes (inlets: tap time for each)
- Each read's output maps to a separate box outlet (outlet 0 = tap 1, outlet 1 = tap 2, ...)
- All share the same Data ring buffer

**Step 1: Write failing test** — `delay 1024 3` should produce 3 outlets

**Step 2–5: TDD cycle.**

**Commit:** One commit.

---

### Task 14: for-init comma expressions, early returns, delay members in regions

**TDD scenario:** New feature — full TDD cycle per sub-task

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/lower.rs`
- Modify: `crates/opengen-genexpr/tests/control_flow.rs`

**Goal:** Three parser/lowering edge cases from the M3 backlog:

**14a: for-init comma expressions** — `for(i=0, j=0; ...)` is a subset of Task 2, but verify the full for-init lowering handles multi-variable init correctly.

**14b: Early returns in functions** — `fn foo(x) { if (x) return 1; out1 = x; }` — the `return` statement inside a function body needs AST support + lowering to region exits. Currently, `return` may not be supported or may panic.

**14c: Delay member calls inside regions** — `d = delay(1024); if (cond) { out1 = d.read(50); }` — the `d.read(tap)` member-call syntax on delay nodes inside control-flow regions.

**Step 1–5 per sub-task:** TDD cycle with minimal reproducing test first.

**Commit:** One commit per sub-task (3 commits).

---

### Task 15: peek/poke NaN and (−1,0)-index conformance

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-ops/src/memory.rs`
- Modify: `crates/opengen-analysis/tests/conformance.rs` (add golden)

**Goal:** Two edge cases in `peek`/`poke` that need gen~ conformance verification:

**15a: NaN index** — What does `peek(buf, NaN)` return in gen~? Index clamping to 0? Return NaN? Write a conformance probe, render in real gen~, and match.

**15b: Negative and zero index** — gen~ `peek(buf, -1)` and `peek(buf, 0)` behavior. `poke` at index −1 and 0. Conformance probe → golden → match.

**Step 1:** Write conformance patches that probe these edge cases.
**Step 2:** Render in real gen~ (or use existing goldens if available).
**Step 3:** Update `peek`/`poke` kernels to match observed behavior.
**Step 4:** Add golden comparison tests.

**Commit:** One commit.

---

### Task 15b: Lexer cursor-snapshot refactor

**TDD scenario:** Modifying tested code — run existing tests first

**Files:**
- Modify: `crates/opengen-genexpr/src/lexer.rs`

**Goal:** Replace clone-lexer-for-lookahead with a cursor-snapshot pattern. The lexer currently clones itself for backtracking lookahead — a `snapshot() -> LexerSnapshot` / `restore(snapshot)` pattern is more efficient and avoids potential state inconsistency.

**Test:** Existing lexer tests pass unchanged. No behavioral change — pure refactor.

**Commit:** One commit.

---

### Task 15c: Codebox abstraction calls inside control flow

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/lower.rs`
- Modify: `crates/opengen-genexpr/tests/control_flow.rs`

**Goal:** Codebox calls to abstraction files inside `if`/`for` blocks. Currently, abstraction resolution (via `GendspAbstractionResolver`) works at the top level but may fail inside region lowering.

**Test:** Codebox with `if (cond) { myabstraction(in1); }` where `myabstraction.gendsp` is a sibling file.

**Step 1–5:** TDD cycle.

**Commit:** One commit.

---

## Phase 2: C++ Emitter Core (Tasks 16–23)

The emitter produces a `Patch`-equivalent C++ artifact that can be compiled standalone. By this point, the operator set is complete and stable — the emitter targets a fixed IR.

### Architecture

```
opengen-emit-cpp/
  src/
    lib.rs           # Public API: emit_cpp(g, reg, sr) -> CppSource
    emit_graph.rs    # Graph-level emission: state arena layout, topo sort, per-sample loop
    emit_kernel.rs   # Kernel emission: template-based with {s0}, {s1}, {sr} placeholders
    emit_regions.rs  # Region emission (control flow, local variables)
    emit_types.rs    # Type mapping (f64 → double, port indices → int)
    templates.rs     # Template fragments for header/footer boilerplate
  tests/
    bit_identical.rs # Cross-backend tests: Rust render vs C++ render
    compile_cpp.rs   # Tests that compile and run emitted C++ via `cc` crate
    harness.cpp       # Embedded C++ test harness (compiled alongside generated code)
```

**Emitted C++ API:**
```cpp
// opengen_patch.h — self-contained, no dependencies beyond C++17 stdlib
#include <vector>
#include <string>
#include <cmath>
#include <cstdint>

struct Patch {
    int n_inputs, n_outputs;
    std::vector<double> state;       // flat state arena
    std::vector<double> value_slots; // per-node output values for current sample

    explicit Patch(int n_in, int n_out, int n_state, int n_values);

    // Per-sample: reads from in[], writes to out[]
    void process(const double* in, double* out);

    // Parameter update between buffers
    void set_param(const std::string& name, double value);
private:
    // Operator kernel helpers — emitted per-operator
    static double kernel_add(double a, double b);
    // ... one per used operator
};
```

**Kernel emission strategy (template-based, NOT static strings):**

Static strings like `"return a + b;"` cannot express state indexing or sample-rate-dependent values. Instead, each `OpDef` carries an **emitter function**:

```rust
/// Emits C++ code for one operator call.
/// Receives state offset and sample rate so stateful/rate-dependent ops
/// can emit correct indexing.
type CppEmitter = fn(state_base: usize, sr: f64) -> String;
```

The emitter generates code like:
```cpp
v[5] = kernel_add(v[2], v[3]);                           // stateless: add
v[6] = state[2]; state[2] = v[1];                         // history at offset 2
v[7] = state[3] + v[4] * (1000.0 / 48000.0); state[3] += 1.0;  // elapsed at offset 3
```

For the kernel body itself (the `kernel_*` functions), the `OpDef` carries a static `cpp_kernel: Option<&'static str>` for the pure-math part, and the emitter wraps it with state/sr context:

```rust
impl OpDef {
    /// Pure C++ expression body for this operator's math (no state/sr).
    /// Placeholders: {a0}, {a1}, ... for input args.
    /// Example for add: "return {a0} + {a1};"
    pub cpp_kernel: Option<&'static str>,

    /// Emit the full per-call C++ statement, given state offset and sr.
    /// Default implementation uses cpp_kernel with argument substitution.
    pub fn emit_cpp_call(&self, state_off: usize, sr: f64, args: &[String]) -> String {
        // ...
    }
}
```

---

### Task 16: Create `opengen-emit-cpp` crate skeleton + test infrastructure

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-emit-cpp/Cargo.toml`
- Create: `crates/opengen-emit-cpp/src/lib.rs`
- Create: `crates/opengen-emit-cpp/src/emit_graph.rs`
- Create: `crates/opengen-emit-cpp/src/emit_kernel.rs`
- Create: `crates/opengen-emit-cpp/src/emit_regions.rs`
- Create: `crates/opengen-emit-cpp/src/emit_types.rs`
- Create: `crates/opengen-emit-cpp/src/templates.rs`
- Create: `crates/opengen-emit-cpp/tests/bit_identical.rs`
- Create: `crates/opengen-emit-cpp/tests/compile_cpp.rs`
- Create: `crates/opengen-emit-cpp/tests/harness.cpp`
- Modify: `Cargo.toml` (workspace members)

**Dev-dependencies:** `cc` (for compiling C++ in tests), `tempfile` (for temp build dirs).

**C++ test harness (`harness.cpp`):**

The test harness is a small C++ file compiled alongside the generated code. It:
1. `#include "opengen_patch.h"`
2. Provides `extern "C" int run_patch(const double* in, int n_in, int n_samples, double* out)` that instantiates `Patch`, calls `process()` for each sample, and writes to `out`

**Step 1: Add crate to workspace + write smoke test**

```rust
// crates/opengen-emit-cpp/tests/bit_identical.rs
use opengen_emit_cpp::{emit_cpp, CppSource};
use opengen_genexpr::parse_and_lower;

#[test]
fn constant_to_output_emits_valid_cpp() {
    let src = "out1 = 0.5;";
    let graph = parse_and_lower(src).unwrap();
    let cpp = emit_cpp(&graph, &opengen_ops::Registry::core(), 48000.0)
        .expect("emission should succeed");
    assert!(cpp.body.contains("0.5"),
        "emitted C++ should contain the constant value");
}
```

**Step 2: Verify test fails** (crate not built yet)

**Step 3: Implement minimal `emit_cpp`** — emits `CppSource { header, body }` with skeleton Patch struct and placeholder `process()`.

**Step 4: Verify test passes**

**Step 5: Add `compile_and_run_cpp` helper and integration test**

```rust
/// Compiles the emitted C++ source, links with harness.cpp, runs it,
/// and returns the output buffer.
fn compile_and_run_cpp(cpp: &CppSource, inputs: &[f64], n_samples: usize) -> Vec<f64> {
    // 1. Write header + body + harness to temp dir
    // 2. Invoke cc::Build with -std=c++17 -ffp-contract=off -O0
    // 3. Run the compiled binary
    // 4. Parse and return stdout as Vec<f64>
}
```

**Step 6: Commit**

```bash
git add Cargo.toml crates/opengen-emit-cpp/
git commit -m "feat(emit-cpp): skeleton crate with C++ test infrastructure"
```

---

### Task 17: Emit state arena layout

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-emit-cpp/src/emit_graph.rs`

**Goal:** Walk the graph, collect all operator state declarations, and emit a flat state vector with `std::vector<double> state(N)` in the Patch constructor. The layout must match `opengen_compile`'s state arena exactly.

**Implementation:** `layout_state(graph) -> HashMap<NodeId, usize>` assigns contiguous offsets for each `StateDecl::Slots(n)`. This mirrors `opengen_compile`'s `allocate_state`.

**Test:** Emit a graph with `history` + `phasor` — verify the state layout sizes and offsets match the Rust backend.

**Commit:** One commit.

---

### Task 18: Emit per-sample process loop (stateless operators first)

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-emit-cpp/src/emit_graph.rs`

**Goal:** Emit the topo-sorted per-sample compute loop. Each node becomes a C++ statement that computes its output and stores it in `v[N]`. Start with stateless operators only (add, mul, constants, in/out).

**Example emission for `out1 = in1 * 0.5 + 0.25`:**
```cpp
void Patch::process(const double* in, double* out) {
    v[0] = in[0];                      // input 0
    v[1] = 0.5;                        // constant
    v[2] = kernel_mul(v[0], v[1]);     // mul
    v[3] = 0.25;                       // constant
    v[4] = kernel_add(v[2], v[3]);     // add
    out[0] = v[4];                     // output 0
}
```

**Test:** `out1 = in1 * 0.5` — render with Rust, render with C++, assert bit-identical.

**Commit:** One commit.

---

### Task 19: Emit stateless operator kernels (math, compare, logic, trig, convert)

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-emit-cpp/src/emit_kernel.rs`
- Modify: `crates/opengen-ops/src/registry.rs` (add `cpp_kernel` field to `OpDef`)
- Modify: Each operator module (add `cpp_kernel` to each `OpDef`)

**Goal:** Add `cpp_kernel: Option<&'static str>` to `OpDef` and populate it for all stateless operators (~60 operators). Each kernel is a pure C++ expression body with `{a0}`, `{a1}`, etc. placeholders.

**Example entries:**
```rust
OpDef {
    name: "add",
    cpp_kernel: Some("return {a0} + {a1};"),
    // ... existing fields unchanged
}
OpDef {
    name: "sin",
    cpp_kernel: Some("return std::sin({a0});"),
}
OpDef {
    name: "eq",
    cpp_kernel: Some("return ({a0} == {a1}) ? 1.0 : 0.0;"),
}
```

**Implementation strategy:** Batch-populate per module (math.rs → commit, trig.rs → commit, etc.).

**Test per batch:** Emit a patch using those operators, compile C++, run, compare with Rust backend.

**Commit:** One commit per operator module.

---

### Task 20: Emit stateful operator kernels

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-emit-cpp/src/emit_kernel.rs`
- Modify: Each stateful operator module (add `cpp_kernel` + custom `emit_cpp_call` if needed)

**Goal:** Emit stateful operators with correct state indexing. Stateful ops need custom `emit_cpp_call` overrides because the default template expansion can't handle state reads/writes.

**Operators and their C++ emission:**
- `history` → `v[{out}] = state[{s0}]; state[{s0}] = v[{in}];` (read old, write new)
- `phasor` → `v[{out}] = state[{s0}]; state[{s0}] = fmod(state[{s0}] + {a0} / {sr}, 1.0);`
- `noise` → xoshiro256++ with 4 state slots; emit the full update algorithm
- `dcblock` → `v[{out}] = v[{in}] - state[{s0}] + 0.9997 * state[{s1}]; state[{s0}] = v[{in}]; state[{s1}] = v[{out}];`
- `slide` → `v[{out}] = state[{s0}] + (v[{in}] - state[{s0}]) * (1.0 - exp(...)); state[{s0}] = v[{out}];`
- `sah` → `if (v[{in0}] > 0.0) state[{s0}] = v[{in1}]; v[{out}] = state[{s0}];`
- `latch` → `if (v[{in0}] > 0.0 && latch != v[{in0}]) state[{s0}] = v[{in1}]; if (v[{in0}] == 0.0) latch = 0.0;`
- `delta` → `v[{out}] = v[{in}] - state[{s0}]; state[{s0}] = v[{in}];`
- `gate` (from Task 9) → `if (v[{in0}] > 0.0) state[{s0}] = v[{in1}]; v[{out}] = state[{s0}];`
- `elapsed` (from Task 10) → `if (v[{in0}] > 0.0) { state[{s0}] = 0.0; v[{out}] = 0.0; } else { v[{out}] = state[{s0}] * (1000.0 / {sr}); state[{s0}] += 1.0; }`
- `delay_write` / `delay_read` → ring buffer access via data node indexing
- `peek` / `poke` → data node access with index clamping

**Test per operator:** Render with Rust, emit C++, compile, run, compare bit-identical.

**Commit:** One operator per commit (stateful ops are complex enough to justify individual review).

---

### Task 21: Emit region/control-flow constructs

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-emit-cpp/src/emit_regions.rs`

**Goal:** Emit `if`/`else`/`for`/`while`/`iter` constructs. The IR's `NodeKind::Region(ProcRegion)` contains lowered `RExpr`/`RStep` trees (from `opengen_ir::proc`). The emitter traverses and emits C++ with local variables for value slots.

**Pattern:**
```cpp
// if (v[cond] > 0.0) { v[out] = v[then_val]; } else { v[out] = v[else_val]; }
// for (int i = 0; i < v[limit]; i++) { v[acc] = kernel_add(v[acc], v[i]); }
```

**Note:** `RExpr`/`RStep` are defined in `opengen_ir::proc`, NOT in `opengen_compile`. The emitter reads them directly from the IR — no circular dependency.

**Test:** Codebox with `if`/`for` emitting bit-identical output to Rust backend.

**Commit:** One commit.

---

### Task 22: Cross-backend determinism validation suite

**TDD scenario:** Modifying tested code — run existing tests first

**Files:**
- Modify: `crates/opengen-emit-cpp/tests/bit_identical.rs`
- Modify: `crates/opengen-analysis/tests/conformance.rs` (add C++ render path)

**Goal:** Every conformance patch renders bit-identical across Rust and C++ backends. This is the M3 exit criterion.

**Implementation:**
1. Add `compile_and_render_cpp(src, sr, n_samples) -> Render` to the test infrastructure
2. Add a sweep test that iterates all `conformance/patches/ops/op_*.genexpr` and runs both backends
3. Assert bit-identical per channel

```rust
#[test]
fn all_operators_bit_identical_cross_backend() {
    for patch in glob("conformance/patches/ops/op_*.genexpr") {
        let src = std::fs::read_to_string(&patch).unwrap();
        let graph = opengen_genexpr::parse_and_lower(&src).unwrap();
        let cpp = emit_cpp(&graph, &Registry::core(), 48000.0).unwrap();

        let rust_out = opengen_testkit::render(&src, 48000.0, 4096);
        let cpp_out = compile_and_render_cpp(&cpp, 48000.0, 4096);

        for ch in 0..rust_out.n_channels() {
            assert_eq!(
                rust_out.ch(ch), &cpp_out.ch(ch)[..],
                "channel {} of {} diverges between Rust and C++ backends",
                ch, patch.display()
            );
        }
    }
}
```

**Exit:** All operators produce bit-identical output across backends. This test stays in CI.

**Commit:** One commit.

---

### Task 23: CLI `emit` integration

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-cli/src/main.rs` (add `emit` subcommand)

**Goal:** `opengen emit file.genexpr --target cpp` emits C++ to stdout or a file. Analogous to `opengen run`.

**CLI design:**
```
opengen emit <file> [--target cpp] [--output <dir>] [--sample-rate <hz>]
```

Default target is `cpp`. Writes `opengen_patch.h` and `opengen_patch.cpp` to the output directory.

**Test:** Roundtrip test — `parse_and_lower → emit_cpp → compile C++ → run → compare output with `opengen run`.

**Commit:** One commit.

---

## Phase 3: Conformance and Polish (Tasks 24–27)

### Task 24: `require` declaration

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/ast.rs`
- Modify: `crates/opengen-genexpr/src/lower.rs`
- Modify: `crates/opengen-gendsp/src/build.rs` (codebox splicing validation)

**Goal:** `require name` is a gen~ declaration that imports a named binding from the host patcher. The host graph's `seeded_inputs` or param bindings must include the required name, or codebox lowering emits an error.

**Design:** Lower `require x` to a node that reads the seeded binding. If `x` is not in the seeding context, emit `Err("undeclared identifier 'x' — add 'require x' or ensure the host provides it")`.

**Commit:** One commit.

---

### Task 25: History read-after-write divergence decision

**TDD scenario:** Trivial change — use judgment

**Files:**
- Create: `docs/research/history_read_after_write_decision.md`
- Modify: `docs/research/gen_docs/genexpr_language_reference.md` (document decision)

**Goal:** Formalize the decision: keep dataflow semantics (reads always see previous sample) as a documented divergence from gen~'s write-through behavior. Rationale: dataflow semantics are cleaner for compilation, and the conformance patches avoid the read-after-write pattern so both engines agree.

**Decision document structure:**
1. Problem statement (gen~'s write-through vs. opengen's dataflow)
2. Evidence (conformance goldens at 44.1k and 48k confirm the difference)
3. Options considered (keep vs. change)
4. Decision: keep dataflow semantics
5. Rationale: compilation simplicity, no impact on conformance (patches avoid the pattern)
6. Future: if strong user demand for gen~ compatibility, revisit with a `#[gen_compat]` attribute

**Commit:** Decision document only.

---

### Task 26: Ratchet climbing — improve GSOT coverage

**TDD scenario:** Modifying tested code — run existing tests first

**Files:**
- Modify: `crates/opengen-analysis/tests/m2_exit.rs` (re-pin ratchet)

**Goal:** Phase 1 fixes (comment-box skip, subpatcher binding, parse fixes, new operators) may have improved GSOT coverage beyond the current 121/189 pin. Re-run and re-pin upward.

**Approach:**
1. Run `cargo test -p opengen-analysis -- m2_exit` — observe current pass count
2. If > 121, update the ratchet constant
3. If unchanged, note which tests fail and whether they're blocked by known missing features (reference unavailable, subpatcher flattening issues, etc.)

**Commit:** One commit (pin update only).

---

### Task 27: Multi-channel data + FMA documentation

**TDD scenario:** New feature (data) + trivial (FMA doc)

**Files:**
- Modify: `crates/opengen-genexpr/src/lower.rs` (multi-channel data)
- Modify: `crates/opengen-genexpr/src/parser.rs` (multi-channel data syntax)
- Create: `docs/research/fma_determinism.md`

**Goal 27a — Multi-channel data:** `Data d(4, 2)` declares 2-channel data of size 4. Channel index selects the channel for `peek`/`poke`/`wave`. Currently only single-channel data works.

**Goal 27b — FMA documentation:** Document the fused multiply-add risk across backends:
- C++: `-ffp-contract=off` disables FMA contraction
- Rust: no FMA by default in `f64` operations
- Cross-backend test suite catches any divergence
- Rule: neither backend constant-folds in a precision-changing way

**Commit:** Two commits (data + FMA doc).

---

## Exit Criteria

1. **Vendor genexpr corpus: 80/80 parse** (Tasks 1–4)
2. **Strict mode rejects decl-after-expr and self-referential history** (Tasks 5–6)
3. **All gen~ operators implemented and conformance-goldened:** `selector`, `gate`, `elapsed`, `wave`, `smoothstep`, `step`, `cartopol`, `poltocar`, `rmod` (Tasks 8–12)
4. **Multi-tap delay (TAPS > 1) works** (Task 13)
5. **Remaining M3 backlog cleared:** for-init comma, early returns, delay members in regions, peek/poke NaN, lexer refactor, codebox abstraction in control flow (Tasks 14–15c)
6. **`require` declaration works** (Task 24)
7. **History read-after-write divergence decision documented** (Task 25)
8. **C++ emitter produces bit-identical output** for all operators across all conformance patches (Task 22)
9. **CLI `emit` command integrated** (Task 23)
10. **`cargo test --workspace` green**, zero failures
11. **`cargo doc --workspace --no-deps` zero warnings**
12. **GSOT ratchet re-pinned upward** (Task 26)
