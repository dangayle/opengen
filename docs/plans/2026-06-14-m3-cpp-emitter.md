# M3 — C++ Emitter and Language Completion Implementation Plan

> **REQUIRED SUB-SKILL:** Use the executing-plans skill to implement this plan task-by-task.

**Goal:** Language-complete the GenExpr frontend (fix remaining parse failures, implement missing operators, add strict-mode lints), then emit dependency-free C++ from the IR that matches the Rust backend bit-for-bit.

**Architecture:** A new `opengen-emit-cpp` crate reads the same typed dataflow `Graph` that `opengen-compile` uses. It produces a single self-contained `.h`/`.cpp` pair: a flat `f64` state arena, a topo-sorted per-sample `process(float* in, float* out)` function, and a `set_param(name, value)` API. Kernels are emitted by a **function-based template system** — each `OpDef` carries an `emit_cpp_call` method that receives state offset, sample rate, and value-slot indices at emission time, so stateful/rate-dependent operators emit correct code. The shared determinism contract (IEEE-754 f64, spec'd evaluation order, no fast-math, seeded PRNG) is the cross-backend test — `cargo test` renders both backends and asserts bit-identical output.

**Phasing strategy:** Language completion (Phase 1, Tasks 1–19) runs first so the IR and operator set are complete before codegen starts. The C++ emitter (Phase 2, Tasks 20–27) targets a fixed set of operators. Integration and polish (Phase 3, Tasks 28–31) wraps up.

**Tech Stack:** Rust (emitter), C++17 (output), `cc` crate (test-only C++ compilation), existing IR/registry/compile crates unchanged.

---

## Phase 1: Language Completion (Tasks 1–19)

Every operator and parse fix lands here before the emitter touches the IR, so Phase 2's exit criteria are against a stable operator count.

---

### Task 1: Characterize the 4 remaining vendor parse failures

**TDD scenario:** New feature — write tests first (characterization = test discovery)

**Files:**
- Create: `crates/opengen-genexpr/tests/vendor_corpus.rs`

**Goal:** Before fixing, identify exactly what the 4 remaining failures are. The vendor corpus lives at `reference/gen_exprs/genexpr_js/genexprs/` (80 `.genexpr` files). Currently 76/80 parse. Run the corpus, extract the 4 failing filenames and error messages, and write a minimal reproducing test for each.

**Step 1: Write a corpus runner that reports per-file pass/fail**

```rust
// crates/opengen-genexpr/tests/vendor_corpus.rs
use opengen_genexpr;

fn run_vendor_corpus() -> Vec<(String, String)> {
    let dir = std::path::Path::new("reference/gen_exprs/genexpr_js/genexprs");
    if !dir.exists() {
        return vec![];
    }
    let mut failures = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("genexpr") {
            continue;
        }
        let src = std::fs::read_to_string(&path).unwrap();
        if let Err(e) = opengen_genexpr::parse(&src) {
            failures.push((
                path.file_name().unwrap().to_string_lossy().to_string(),
                e.to_string(),
            ));
        }
    }
    failures
}

#[test]
fn vendor_genexpr_corpus_report() {
    let failures = run_vendor_corpus();
    for (file, err) in &failures {
        eprintln!("FAIL {}: {}", file, err);
    }
    // Informational — no assertion yet. Once all pass, change to assert 80/80.
}
```

**Step 2: Run test to record the 4 failures**

Run: `cargo test -p opengen-genexpr -- vendor_genexpr_corpus_report`
Expected: 4 `FAIL` lines printed to stderr. Record each.

**Step 3: Write a minimal reproducing test per failure**

```rust
// For each failure, write a test like:
#[test]
fn parse_failure_1_comma_in_for_init() {
    let src = "for(i=0, j=0; i<10; i=i+1) { out1 = i; }";
    let result = opengen_genexpr::parse(src);
    assert!(result.is_ok(), "comma in for-init should parse: {}", result.unwrap_err());
}
```

**Step 4: Commit**

```bash
git add crates/opengen-genexpr/tests/vendor_corpus.rs
git commit -m "test(genexpr): characterize 4 remaining vendor parse failures"
```

---

### Task 2: Fix parser — comma expressions in for-init

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/ast.rs`
- Test: `crates/opengen-genexpr/tests/vendor_corpus.rs`

**Goal:** `for(i=0, j=0; i<10; i=i+1)` — the comma between `i=0` and `j=0` is currently rejected. The parser's for-init path needs to handle comma-separated assignment/declaration expressions.

**Step 1: Write failing test**

```rust
#[test]
fn for_init_comma_expression_parses() {
    let src = "for(i=0, j=0; i<10; i=i+1) { out1 = i; }";
    let result = opengen_genexpr::parse(src);
    assert!(result.is_ok(), "comma in for-init should parse: {}", result.unwrap_err());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p opengen-genexpr -- for_init_comma_expression_parses`
Expected: FAIL

**Step 3: Implement the fix**

Modify `parse_for_init` or equivalent in `crates/opengen-genexpr/src/parser.rs` to accept comma-separated expressions in the init clause. Add a `Comma` AST node or inline multi-assignment.

**Step 4: Run test to verify it passes + full vendor corpus**

Run: `cargo test -p opengen-genexpr -- for_init_comma_expression_parses`
Expected: PASS
Run: `cargo test -p opengen-genexpr -- vendor_genexpr_corpus_report`
Expected: ≤3 failures remaining

**Step 5: Commit**

```bash
git add crates/opengen-genexpr/src/parser.rs crates/opengen-genexpr/src/ast.rs crates/opengen-genexpr/tests/vendor_corpus.rs
git commit -m "fix(genexpr): parse comma expressions in for-init"
```

---

### Task 3: Fix parser — named arguments inside function calls

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Test: `crates/opengen-genexpr/tests/vendor_corpus.rs`

**Goal:** `foo(bar=baz)` — named/keyword arguments in function calls are currently unparsed. The parser needs to accept `name=expr` inside call argument lists.

**Step 1: Write failing test**

```rust
#[test]
fn named_argument_in_call_parses() {
    let src = "out1 = foo(bar=in1);";
    let result = opengen_genexpr::parse(src);
    assert!(result.is_ok(), "named arg in call should parse: {}", result.unwrap_err());
}
```

**Step 2: Run test**

Run: `cargo test -p opengen-genexpr -- named_argument_in_call_parses`
Expected: FAIL

**Step 3: Implement fix** — modify call argument parsing to accept `name=expr` tuples.

**Step 4: Verify**

Run: `cargo test -p opengen-genexpr -- named_argument_in_call_parses vendor_genexpr_corpus_report`
Expected: PASS + ≤2 vendor failures remaining

**Step 5: Commit**

```bash
git commit -m "fix(genexpr): parse named arguments in function calls"
```

---

### Task 4: Fix remaining 2 parser failures

**TDD scenario:** New feature — full TDD cycle (one per failure)

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/lexer.rs` (likely, depending on failure)
- Test: `crates/opengen-genexpr/tests/vendor_corpus.rs`

**Goal:** Fix the 2 remaining failures identified in Task 1. Each gets its own TDD commit.

**Step 1: For each failure, write a minimal test**

```rust
#[test]
fn parse_failure_3_<description>() { /* minimal reproduction */ }
#[test]
fn parse_failure_4_<description>() { /* minimal reproduction */ }
```

**Step 2: Run tests to verify they fail**

**Step 3: Implement fixes**

**Step 4: Verify both pass + full vendor corpus**

**Step 5: Commit each fix separately**

```bash
git commit -m "fix(genexpr): <description of failure 3>"
git commit -m "fix(genexpr): <description of failure 4>"
```

**Exit criterion:** Vendor corpus assertion changed to `assert_eq!(failures.len(), 0)` — 80/80 parse.

---

### Task 5: Declaration-ordering strict mode

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/lib.rs` (add `parse_strict` public function)
- Test: `crates/opengen-genexpr/tests/strict_mode.rs`

**Goal:** Real gen~ rejects declarations after expression statements with "declarations must come before expressions" (observed Max 9 2026-06-10). opengen's parser is lenient by default. Implement a strict mode that enforces gen~'s ordering.

**Design:** `parse_strict(src)` parses normally, then validates that all `Decl` AST nodes precede all `Stmt::Expr` nodes. Returns `Err` on violation. During implementation, verify that the distinction between Decl and Expr is clear in the AST — if not, add a marker or refactor.

**Step 1: Write failing tests**

```rust
// crates/opengen-genexpr/tests/strict_mode.rs
use opengen_genexpr;

#[test]
fn strict_mode_rejects_decl_after_expression() {
    let src = "out1 = in1; h = history(in1);";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("declaration after expression"),
        "expected 'declaration after expression' in error, got: {}", err);
}

#[test]
fn strict_mode_allows_decl_before_expr() {
    let src = "h = history(in1); out1 = h;";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_ok());
}

#[test]
fn lenient_mode_allows_decl_after_expr() {
    let src = "out1 = in1; h = history(in1);";
    let result = opengen_genexpr::parse(src);
    assert!(result.is_ok(), "lenient parse should accept decl after expr");
}
```

**Step 2: Run tests to verify they fail** (strict mode tests fail)

**Step 3: Implement `parse_strict`**

```rust
pub fn parse_strict(src: &str) -> Result<Program, ParseError> {
    let program = parse(src)?;
    // Walk statements: once an Expr is seen, any subsequent Decl is an error
    let mut seen_expr = false;
    for stmt in &program.stmts {
        match stmt {
            Stmt::Decl { .. } if seen_expr => {
                return Err(ParseError::new("declaration after expression"));
            }
            Stmt::Expr { .. } => seen_expr = true,
            _ => {}
        }
    }
    Ok(program)
}
```

**Step 4: Run tests**

Run: `cargo test -p opengen-genexpr -- strict_mode`
Expected: all 3 tests PASS

**Step 5: Commit**

```bash
git add crates/opengen-genexpr/src/parser.rs crates/opengen-genexpr/src/lib.rs crates/opengen-genexpr/tests/strict_mode.rs
git commit -m "feat(genexpr): add parse_strict enforcing gen~ declarator ordering"
```

---

### Task 6: Self-referential history lint

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs` (strict mode validation)
- Test: `crates/opengen-genexpr/tests/strict_mode.rs`

**Goal:** `h = history(h + 1)` is an opengen leniency — real gen~ errors "variable h is not defined" (observed Max 9 2026-06-10). gen~ requires `History h` extern-style declaration before use. In strict mode, detect self-referential history and reject it.

**Step 1: Write failing tests**

```rust
#[test]
fn strict_mode_rejects_self_referential_history() {
    let src = "h = history(h + 1);";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("not defined"),
        "expected 'not defined' in error, got: {}", err);
}

#[test]
fn strict_mode_allows_non_self_referential_history() {
    let src = "x = in1; h = history(x); out1 = h;";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_ok());
}

#[test]
fn strict_mode_allows_history_with_constant() {
    let src = "h = history(0); out1 = h;";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_ok());
}
```

**Step 2: Run tests to verify they fail**

**Step 3: Implement the validation** — in `parse_strict`, when a `Decl` for variable `h` initialises with `history(...)` where the first argument is an `Expr::Ident("h")`, emit error.

**Step 4: Run tests**

Run: `cargo test -p opengen-genexpr -- strict_mode`
Expected: all 6 tests PASS (3 from Task 5 + 3 new)

**Step 5: Commit**

```bash
git commit -m "feat(genexpr): reject self-referential history in strict mode"
```

---

### Task 7: `^^` precedence conformance cross-check

**TDD scenario:** Modifying tested code — run existing tests first

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs` (only if fix needed)
- Test: `crates/opengen-genexpr/tests/parser_v2.rs`

**Goal:** The vendor PEG grammar defines `||` → `^^` → `&&`. Verify the parser's precedence matches and fix if needed.

**Source:** `reference/rnbo/genexpr_js/genexpr.pegjs`

**Step 1: Run existing tests**

Run: `cargo test -p opengen-genexpr`
Expected: all existing tests PASS (establish baseline)

**Step 2: Write conformance test**

```rust
#[test]
fn logical_precedence_xor_binds_tighter_than_or() {
    // Should parse as: in1 || (in2 ^^ in3), not (in1 || in2) ^^ in3
    let src = "out1 = in1 || in2 ^^ in3;";
    let program = opengen_genexpr::parse(src).unwrap();
    // Verify by lowering both possible trees, rendering with known inputs,
    // and asserting the output matches the ||→^^ grouping
}

#[test]
fn logical_precedence_and_binds_tighter_than_xor() {
    let src = "out1 = in1 ^^ in2 && in3;";
    let program = opengen_genexpr::parse(src).unwrap();
    // Should parse as: in1 ^^ (in2 && in3)
}
```

**Step 3: If tests fail, fix the precedence chain in `parse_or`/`parse_xor`/`parse_and` methods**

**Step 4: Verify**

Run: `cargo test -p opengen-genexpr`
Expected: all tests PASS

**Step 5: Commit**

```bash
git commit -m "fix(genexpr): verify/fix ^^ precedence to match gen~ PEG grammar"
```

---

### Task 8: Implement `selector` operator

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-ops/src/selector.rs`
- Modify: `crates/opengen-ops/src/lib.rs`
- Modify: `crates/opengen-ops/src/registry.rs`

**Goal:** Implement gen~'s `selector` with correct float-threshold semantics.

**Semantics (verified against `reference/gen/refpages/common/gen_common_selector.maxref.xml`):**
- `selector(index, in1, in2, ..., inN)` — one index signal, N signal inputs
- Float threshold comparison: output = in1 when index < 1, output = in2 when 1 ≤ index < 2, ..., output = inN when index ≥ N−1
- At exact integer boundaries, the NEXT input is selected (index ≥ 1 → in2)
- Out-of-range: index < 0 → in1; index ≥ N−1 → inN
- **NOT integer-indexed** — this is float threshold, not `inputs[floor(index)]`

**Variable-arity design decision:** The current `OpDef.arity: u16` is fixed. For M3, register `selector3` (1 index + 2 signals, the most common case) and `selector5` (1 index + 4 signals) as separate OpDefs. Variable-arity `selector` is deferred to M4. This is a pragmatic tradeoff — zero IR changes, covers the common cases, and real gen~ patches rarely exceed 4 signal inputs to `selector`.

**Step 1: Write spec doctest**

```rust
// crates/opengen-ops/src/selector.rs
/// Select one of N signals based on a float threshold index.
///
/// # Definition
/// ```
/// use opengen_testkit::render;
/// // selector3(index, in1, in2): index < 1 → in1, index ≥ 1 → in2
/// let out = render("out1 = selector3(in1, 10, 20);", 48000.0, 3);
/// // in1 from testkit is 0.0 for render() with no explicit input
/// // 0.0 < 1 → out1 = 10
/// assert_eq!(out.ch(0)[0], 10.0);
/// ```
```

**Step 2: Run test to verify it fails** (operator not registered yet)

**Step 3: Implement kernel + OpDef**

```rust
pub fn selector3(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let index = inputs[0];
    if index < 1.0 { return inputs[1]; }
    inputs[2]  // index ≥ 1
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef {
            name: "selector3",
            arity: 3,
            state: StateDecl::None,
            deferred_ports: &[],
            update: None,
            init: None,
            kernel: selector3,
            cpp_kernel: None, // filled in Phase 2
            emit_cpp_call: None,
        },
    ]
}
```

**Step 4: Run tests**

Run: `cargo test -p opengen-ops`
Expected: doctest PASS

**Step 5: Commit**

```bash
git commit -m "feat(ops): implement selector3 operator (float-threshold selector)"
```

---

### Task 9: Implement `gate` operator

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-ops/src/gate.rs`
- Modify: `crates/opengen-ops/src/lib.rs`
- Modify: `crates/opengen-ops/src/registry.rs`

**Goal:** Implement `gate` with correct sample-and-hold semantics.

**Semantics (verified against `reference/gen/refpages/common/gen_common_gate.maxref.xml`):**
- `gate(trigger, input)` — two signals
- When trigger > 0: output = input (gate open, passes through)
- When trigger ≤ 0: output = last value passed while gate was open (sample-and-hold)
- Stateful: stores the last passed value in 1 slot of state
- Initial state: 0.0 (gate starts closed)

**NOT combinatorial** — unlike `switch`, this has memory.

**Step 1: Write spec doctest**

```rust
/// Gate with sample-and-hold.
///
/// # Definition
/// When trigger > 0, output = input. When trigger ≤ 0, output = last passed value.
///
/// ```
/// use opengen_testkit::render;
/// // Render a patch that drives gate with a phasor ramp: gate opens/closes
/// let src = "t = phasor(100); g = gate(t > 0.5, t); out1 = g;";
/// let out = render(src, 48000.0, 480);
/// // Gate holds the value when trigger ≤ 0.5
/// assert!(out.ch(0)[240] > 0.49); // near peak, held during closed phase
/// ```
```

**Step 2: Run test to verify it fails**

**Step 3: Implement kernel**

```rust
pub fn gate(inputs: &[f64], state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] > 0.0 {
        state[0] = inputs[1];
    }
    state[0]
}
```

Arity: 2. StateDecl: Slots(1).

**Step 4: Run tests**

Run: `cargo test -p opengen-ops`
Expected: doctest PASS

**Step 5: Commit**

```bash
git commit -m "feat(ops): implement gate operator with sample-and-hold semantics"
```

---

### Task 10: Implement `elapsed` operator

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-ops/src/elapsed.rs`
- Modify: `crates/opengen-ops/src/lib.rs`
- Modify: `crates/opengen-ops/src/registry.rs`

**Goal:** Implement `elapsed` with correct time-in-milliseconds semantics.

**Semantics (verified against `reference/gen/refpages/common/gen_common_elapsed.maxref.xml`):**
- `elapsed(trigger)` — one signal
- Output: time in **milliseconds** since the trigger was last non-zero
- When trigger > 0: output = 0, reset counter
- When trigger ≤ 0: output = counter * (1000.0 / sr), counter increments each sample
- Stateful: stores the sample counter in 1 slot of state

**NOT sample count** — the output uses sample-rate-dependent conversion.

**Step 1: Write spec doctest**

```rust
/// Elapsed time in milliseconds since trigger was last non-zero.
///
/// # Definition
/// When trigger > 0: reset counter, output 0.
/// When trigger ≤ 0: output = counter * (1000.0 / sr), counter += 1.
///
/// ```
/// use opengen_testkit::render;
/// let src = "out1 = elapsed(0);"; // trigger always 0, counter runs
/// let out = render(src, 1000.0, 3);
/// // After 1 sample at 1000 Hz: 1 * (1000/1000) = 1.0
/// // After 2 samples: 2.0, after 3 samples: 3.0
/// assert!((out.ch(0)[2] - 2.0).abs() < 1e-9);
/// ```
```

**Step 2: Run test to verify it fails**

**Step 3: Implement kernel**

```rust
pub fn elapsed(inputs: &[f64], state: &mut [f64], sr: f64) -> f64 {
    if inputs[0] > 0.0 {
        state[0] = 0.0;
        return 0.0;
    }
    let ms_per_sample = 1000.0 / sr;
    let result = state[0] * ms_per_sample;
    state[0] += 1.0;
    result
}
```

Arity: 1. StateDecl: Slots(1).

**Step 4: Run tests**

**Step 5: Commit**

```bash
git commit -m "feat(ops): implement elapsed operator (milliseconds since trigger)"
```

---

### Task 11: Implement `wave` operator

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-ops/src/wave.rs`
- Modify: `crates/opengen-ops/src/lib.rs`
- Modify: `crates/opengen-ops/src/registry.rs`

**Goal:** Wavetable oscillator reading from a `Data`/`Buffer` node at normalised phase (0.0–1.0) with linear interpolation.

**Design:** Like `peek`/`poke`, `wave` references a `Data` node by name via `NodeKind::Op { data_ref }`. The kernel receives the data buffer through the IR's data-mapping at compile time (same mechanism as `peek`/`poke`).

**Step 1: Write spec doctest**

```rust
/// Wavetable oscillator with linear interpolation.
///
/// # Definition
/// Reads from a Data buffer at normalised phase 0.0–1.0.
/// Linearly interpolates between adjacent samples.
///
/// ```
/// use opengen_testkit::render;
/// // Data d with [1, 2, 3, 4] — phase 0.0 reads 1, phase 0.5 reads 2.5 (interp between idx 1 and 2)
/// // For M3: test with actual data node + peek for known values
/// ```
```

**Step 2: Run test to verify it fails**

**Step 3: Implement kernel**

Arity: 2 (phase, data pointer resolved at compile time via data_ref). Uses the same data-access pattern as `peek`/`poke`.

**Step 4: Run tests**

**Step 5: Commit**

```bash
git commit -m "feat(ops): implement wave operator (wavetable oscillator)"
```

---

### Task 12: Implement remaining gen~ operators

**TDD scenario:** New feature — full TDD cycle (one per operator)

**Files:**
- Create: `crates/opengen-ops/src/step.rs`
- Create: `crates/opengen-ops/src/smoothstep.rs`
- Create: `crates/opengen-ops/src/rmod.rs`
- Modify: `crates/opengen-ops/src/lib.rs`
- Modify: `crates/opengen-ops/src/registry.rs`

**Goal:** Implement `step`, `smoothstep`, `rmod`. Defer `cartopol`/`poltocar` to M4 — they require multi-output node support which the current IR doesn't have (each Op node has exactly one output on `index: 0`). Adding multi-out support is a non-trivial IR change, not scoped for M3.

**12a: `step(threshold, x)`** — Heaviside step: returns 1.0 when x ≥ threshold, 0.0 otherwise. Arity: 2, stateless.

```rust
pub fn step(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[1] >= inputs[0] { 1.0 } else { 0.0 }
}
```

**12b: `smoothstep(low, high, x)`** — Hermite interpolation. Formula: `t = clamp((x-low)/(high-low), 0, 1); return t*t*(3-2*t)`. Arity: 3, stateless.

```rust
pub fn smoothstep(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    let t = ((inputs[2] - inputs[0]) / (inputs[1] - inputs[0])).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
```

**12c: `rmod(a, b)`** — Reverse mod: computes `b % a` (swapped argument order). Arity: 2, stateless.

```rust
pub fn rmod(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    inputs[1] % inputs[0]
}
```

**Step 1 per operator:** Write doctest → run (fails) → implement → run (passes)

**Commit:** `rmod` + `step` together (both trivial); `smoothstep` separate.

---

### Task 13: Multi-tap delay (TAPS > 1)

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-gendsp/src/build.rs` (remove `TAPS > 1 → Err` guard; wire multi-tap outlets)
- Modify: `crates/opengen-gendsp/src/flatten.rs` (same for subpatcher flattening path)
- Test: `crates/opengen-gendsp/tests/build_fixtures.rs`

**Goal:** `delay SIZE TAPS` with TAPS > 1 currently returns `Err`. Remove the guard and wire the additional delay_read outlets.

**Design:** For `delay SIZE TAPS`:
- One `delay_write` node (inlet 0 = signal)
- `TAPS` `delay_read` nodes, all sharing the same Data ring buffer
- Each read's output maps to a separate box outlet (outlet 0 = tap 1, outlet 1 = tap 2, ...)
- Each read has its own tap-time inlet

**Step 1: Write failing test**

```rust
#[test]
fn delay_multi_tap_produces_multiple_outlets() {
    // Build a graph with `delay 1024 3` and verify it has 3 output ports
}
```

**Step 2–4: TDD cycle**

**Step 5: Commit**

```bash
git commit -m "feat(gendsp): implement multi-tap delay (TAPS > 1)"
```

---

### Task 14: for-init comma lowering verification

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Test: `crates/opengen-genexpr/tests/control_flow.rs`

**Goal:** Task 2 fixed the parser for `for(i=0, j=0; ...)`. Now verify the **lowering** handles multi-variable for-init correctly — each variable gets its own local slot in the region.

**Step 1: Write failing test**

```rust
#[test]
fn for_multi_init_lowers_and_renders_correctly() {
    let src = "for(i=0, j=10; i<3; i=i+1) { j = j + 1; out1 = j; }";
    let out = opengen_testkit::render(src, 48000.0, 3);
    // i=0 j=10 → j=11 out=11 → i=1 j=12 out=12 → i=2 j=13 out=13
    assert_eq!(out.ch(0)[0], 11.0);
    assert_eq!(out.ch(0)[1], 12.0);
    assert_eq!(out.ch(0)[2], 13.0);
}
```

**Step 2–4: TDD cycle — fix lowering if needed**

**Step 5: Commit**

```bash
git commit -m "fix(genexpr): verify for-init multi-variable lowering"
```

---

### Task 15: Early returns in functions

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/lower.rs`
- Test: `crates/opengen-genexpr/tests/control_flow.rs`

**Goal:** `fn foo(x) { if (x) return 1; out1 = x; }` — the `return` statement inside a function body needs AST support and region lowering. Currently, `return` may not be supported.

**Step 1: Write failing test**

```rust
#[test]
fn early_return_in_function_renders_correctly() {
    let src = "fn pick(x) { if (x > 0) return 10; return 20; } out1 = pick(in1);";
    let out = opengen_testkit::render_with_inputs(src, 48000.0, &[&[-1.0, 1.0]], 2);
    assert_eq!(out.ch(0)[0], 20.0); // x=-1 → return 20
    assert_eq!(out.ch(0)[1], 10.0); // x=1  → return 10
}
```

**Step 2–4: TDD cycle — add `Return` AST node, lower to region exit**

**Step 5: Commit**

```bash
git commit -m "feat(genexpr): implement early return in functions"
```

---

### Task 16: Delay member calls inside regions

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/lower.rs`
- Test: `crates/opengen-genexpr/tests/control_flow.rs`

**Goal:** The `delay` variable inside a codebox supports member-call syntax: `d.read(tap)`. Verify this works inside control-flow regions (`if`/`for` blocks). Currently, delay member calls may fail during region lowering because the delay's data node is resolved outside the region.

**Step 1: Write failing test**

```rust
#[test]
fn delay_member_call_inside_if_block() {
    let src = "d = delay(1024); if (in1 > 0) { out1 = d.read(0); } else { out1 = d.read(100); }";
    // Verify both branches access the delay correctly
}
```

**Step 2–4: TDD cycle**

**Step 5: Commit**

```bash
git commit -m "fix(genexpr): support delay member calls inside control-flow regions"
```

---

### Task 17: peek/poke NaN and negative-index conformance

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-ops/src/memory.rs`
- Modify: `crates/opengen-analysis/tests/conformance.rs`
- Create: `conformance/patches/peek_nan_index.genexpr`
- Create: `conformance/patches/peek_neg_index.genexpr`

**Goal:** gen~ edge cases for `peek`/`poke` index clamping:

**17a: NaN index** — What does `peek(buf, NaN)` return in gen~? Write a conformance probe patch, render in real gen~, and match the behavior.

**17b: Negative/zero index** — `peek(buf, -1)` and `peek(buf, 0)` behavior. Same for `poke`.

**Step 1: Write conformance patches**

```genexpr
// conformance/patches/peek_nan_index.genexpr
Data d(4);
out1 = peek(d, sqrt(-1));  // NaN index
```

**Step 2: Render in real gen~ → golden WAV**

**Step 3: Update `peek`/`poke` kernels to match observed behavior**

**Step 4: Add golden comparison tests**

**Step 5: Commit**

---

### Task 18: Lexer cursor-snapshot refactor

**TDD scenario:** Modifying tested code — run existing tests first

**Files:**
- Modify: `crates/opengen-genexpr/src/lexer.rs`

**Goal:** Replace clone-lexer-for-lookahead with a cursor-snapshot pattern (`snapshot() -> LexerSnapshot` / `restore(snapshot)`). Pure refactor — no behavioral change.

**Step 1: Run existing tests**

Run: `cargo test -p opengen-genexpr`
Expected: all PASS (establish baseline)

**Step 2: Implement snapshot/restore and replace clones**

**Step 3: Run tests again**

Run: `cargo test -p opengen-genexpr`
Expected: all PASS (unchanged behavior)

**Step 4: Commit**

```bash
git commit -m "refactor(genexpr): lexer cursor-snapshot pattern replaces clone lookahead"
```

---

### Task 19: Codebox abstraction calls inside control flow

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/lower.rs`
- Test: `crates/opengen-genexpr/tests/control_flow.rs`

**Goal:** Codebox calls to abstraction `.gendsp` files inside `if`/`for` blocks. Currently, abstraction resolution via `GendspAbstractionResolver` works at the top level but may fail inside region lowering.

**Step 1: Write failing test**

```rust
#[test]
fn abstraction_call_inside_if_block() {
    // Requires a sibling .gendsp file on the search path.
    // Skip if reference/ not available.
    let dir = std::env::temp_dir().join("opengen_test_abs_in_if");
    // Write a minimal .gendsp abstraction file
    // Write host .gendsp with: if (in1) { myabs(in1); }
    // Assert the abstraction is resolved and rendered
}
```

**Step 2–4: TDD cycle**

**Step 5: Commit**

---

## Phase 2: C++ Emitter Core (Tasks 20–27)

By this point, the operator set is complete and stable — the emitter targets a fixed IR.

### Architecture

```
opengen-emit-cpp/
  src/
    lib.rs           # Public API: emit_cpp(g, reg, sr) -> CppSource
    emit_graph.rs    # Graph emission: state arena layout, topo sort, per-sample loop
    emit_kernel.rs   # Kernel emission: template-based with {a0}, {a1}, {sr} placeholders
    emit_regions.rs  # Region emission (control flow, local variables)
    emit_types.rs    # Type mapping (f64 → double, port indices → int)
    templates.rs     # Template fragments for header/footer boilerplate
  tests/
    bit_identical.rs # Cross-backend tests: Rust render vs C++ render
    compile_cpp.rs   # Helper: compile_and_run_cpp via `cc` crate
    harness.cpp       # Embedded C++ test harness
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
    std::vector<double> state;       // flat state arena (op state + data buffers)
    std::vector<double> v;           // per-node value slots for current sample

    explicit Patch(int n_in, int n_out, int n_state, int n_values);

    void process(const double* in, double* out);

    void set_param(const std::string& name, double value);
private:
    // Emitted per-operator kernel helpers
    static double kernel_add(double a, double b);
    static double kernel_sin(double a);
    // ... one per used operator
};
```

**Kernel emission strategy (template-based, NOT static strings):**

Static strings cannot express state indexing or sample-rate-dependent values. Instead, each `OpDef` carries two optional fields:

```rust
// Added to OpDef in crates/opengen-ops/src/registry.rs:
pub struct OpDef {
    // ... existing fields unchanged ...
    pub kernel: Kernel,  // existing: Rust kernel fn

    /// Pure C++ expression body with {a0}, {a1}, ... placeholders for inputs.
    /// Example for add: "return {a0} + {a1};"
    /// None for stateful ops that need custom emit_cpp_call.
    pub cpp_kernel: Option<&'static str>,

    /// Emit the full per-call C++ statement for stateful/rate-dependent ops.
    /// Receives value-slot indices and state offset so the emitted code
    /// reads/writes v[N] and state[N] correctly.
    /// None for stateless ops (cpp_kernel handles those).
    pub emit_cpp_call: Option<fn(out_slot: usize, in_slots: &[usize], state_off: usize, sr: f64) -> String>,
}
```

The graph emitter generates code like:
```cpp
void Patch::process(const double* in, double* out) {
    v[0] = in[0];                                    // input 0
    v[1] = 0.5;                                      // constant
    v[2] = kernel_mul(v[0], v[1]);                   // stateless: add
    v[3] = 0.25;                                     // constant
    v[4] = kernel_add(v[2], v[3]);                   // stateless: add
    // stateful ops use emit_cpp_call:
    // history at state offset 2: "v[5] = state[2]; state[2] = v[4];"
    // elapsed at state offset 3: "if (v[6] > 0.0) { state[3] = 0.0; v[7] = 0.0; } ..."
    out[0] = v[4];                                   // output 0
}
```

Placeholder convention: `{a0}` = first input value-slot, `{a1}` = second, etc. The graph emitter handles `v[{out}] = ...` prefix. The `emit_cpp_call` function produces the full statement (including `v[N] = ...` assignment) for stateful ops.

---

### Task 20: Create `opengen-emit-cpp` crate skeleton + C++ test infrastructure

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
- Modify: `Cargo.toml` (add workspace member)

**Dev-dependencies:** `cc` (compiling C++ in tests), `tempfile` (temp build dirs).

**C++ test harness (`harness.cpp`):**
```cpp
// Compiled alongside the generated opengen_patch.h/opengen_patch.cpp
// Provides a C ABI entry point for Rust tests to call.
#include "opengen_patch.h"
#include <cstring>

extern "C" int run_patch(
    const double* in, int n_in, int n_samples, double* out
) {
    Patch p(n_in, 1, /* n_state */ 0, /* n_values */ 0); // FIXME after Task 21
    for (int i = 0; i < n_samples; i++) {
        p.process(&in[i * n_in], &out[i]);
    }
    return 0;
}
```

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

**Step 2: Run test to verify it fails**

Run: `cargo test -p opengen-emit-cpp -- constant_to_output`
Expected: FAIL — crate not yet in workspace

**Step 3: Implement minimal `emit_cpp`**

```rust
// crates/opengen-emit-cpp/src/lib.rs
use opengen_ir::Graph;
use opengen_ops::Registry;

pub struct CppSource {
    pub header: String,
    pub body: String,
}

pub fn emit_cpp(graph: &Graph, reg: &Registry, sr: f64) -> Result<CppSource, String> {
    let mut body = String::from("void Patch::process(const double* in, double* out) {\n");
    for (_id, node) in graph.nodes() {
        if let opengen_ir::NodeKind::Constant(v) = &node.kind {
            body.push_str(&format!("    // constant: {}\n", v));
        }
    }
    body.push_str("}\n");
    Ok(CppSource { header: String::new(), body })
}
```

**Step 4: Run test to verify it passes**

**Step 5: Add `compile_and_run_cpp` helper**

```rust
// crates/opengen-emit-cpp/tests/compile_cpp.rs
use opengen_emit_cpp::CppSource;
use std::process::Command;

/// Compiles emitted C++ source with harness.cpp, runs it, returns output buffer.
/// Uses cc crate for compilation. Sets -std=c++17 -ffp-contract=off -O0.
pub fn compile_and_run_cpp(cpp: &CppSource, inputs: &[f64], n_samples: usize) -> Vec<f64> {
    let dir = tempfile::tempdir().unwrap();
    // 1. Write header + body to dir
    std::fs::write(dir.path().join("opengen_patch.h"), &cpp.header).unwrap();
    std::fs::write(dir.path().join("opengen_patch.cpp"), &cpp.body).unwrap();
    // 2. Compile with cc
    let lib = cc::Build::new()
        .cpp(true)
        .std("c++17")
        .flag("-ffp-contract=off")
        .opt_level(0)
        .include(dir.path())
        .file(dir.path().join("opengen_patch.cpp"))
        .file("tests/harness.cpp")
        .compile("opengen_patch_test");
    // 3. Run and parse output
    // (simplified — actual impl links a test binary)
    todo!("full impl requires linking a binary; see cc crate docs for executable targets")
}
```

**Step 6: Commit**

```bash
git add Cargo.toml crates/opengen-emit-cpp/
git commit -m "feat(emit-cpp): skeleton crate with C++ test infrastructure"
```

---

### Task 21: Emit state arena layout + data buffers

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-emit-cpp/src/emit_graph.rs`

**Goal:** Walk the graph and compute the state arena layout. Emit `std::vector<double> state(N)` in the Patch constructor. The layout must match `opengen_compile`'s state arena exactly. Also emit Data/Buffer node allocations as contiguous regions within the flat state vector (or as separate vectors — match whatever opengen_compile does).

**Implementation:**
1. Walk topo-sorted nodes, assign contiguous offsets for each `StateDecl::Slots(n)` → `HashMap<NodeId, usize>`
2. Walk Data nodes, assign offsets within the same arena (or separate arena — verify by reading `crates/opengen-compile/src/lib.rs` state allocation)
3. Emit `state.resize(total_slots, 0.0)` in the constructor
4. Emit `v.resize(n_value_slots, 0.0)` in the constructor

**Step 1: Write failing test**

```rust
#[test]
fn history_node_produces_state_allocation() {
    let src = "h = history(in1); out1 = h;";
    let graph = parse_and_lower(src).unwrap();
    let cpp = emit_cpp(&graph, &Registry::core(), 48000.0).unwrap();
    assert!(cpp.header.contains("std::vector<double> state"),
        "header should declare state vector");
    assert!(cpp.body.contains("state["),
        "body should reference state array for history");
    // History has 1 state slot → state should have size ≥ 1
    assert!(cpp.header.contains("state.resize("),
        "constructor should resize state vector");
}
```

**Step 2–4: Implement + verify**

**Step 5: Commit**

```bash
git commit -m "feat(emit-cpp): state arena layout matching compile backend"
```

---

### Task 22: Emit per-sample process loop (stateless operators first)

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-emit-cpp/src/emit_graph.rs`

**Goal:** Emit the topo-sorted per-sample compute loop. Each node becomes a C++ statement. Start with stateless operators only (constants, inputs, outputs, add, mul).

**Emission for `out1 = in1 * 0.5 + 0.25`:**
```cpp
void Patch::process(const double* in, double* out) {
    v[0] = in[0];
    v[1] = 0.5;
    v[2] = kernel_mul(v[0], v[1]);
    v[3] = 0.25;
    v[4] = kernel_add(v[2], v[3]);
    out[0] = v[4];
}
```

**Step 1: Write failing test**

```rust
#[test]
fn minimal_graph_emits_compilable_cpp() {
    let src = "out1 = in1 * 0.5;";
    let graph = parse_and_lower(src).unwrap();
    let cpp = emit_cpp(&graph, &Registry::core(), 48000.0).unwrap();
    // Verify structural elements present
    assert!(cpp.body.contains("void Patch::process"));
    assert!(cpp.body.contains("kernel_mul"));
    assert!(cpp.body.contains("out[0] ="));
}
```

**Step 2–4: Implement + verify**

**Step 5: Commit**

```bash
git commit -m "feat(emit-cpp): topo-sorted per-sample process loop"
```

---

### Task 23: Emit stateless operator kernels (math, compare, logic, trig, convert)

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-emit-cpp/src/emit_kernel.rs`
- Modify: `crates/opengen-ops/src/registry.rs` (add `cpp_kernel` and `emit_cpp_call` fields)
- Modify: Each operator module in `crates/opengen-ops/src/` (add `cpp_kernel` to each `OpDef`)

**Goal:** Add `cpp_kernel: Option<&'static str>` and `emit_cpp_call: Option<...>` to `OpDef`. Populate `cpp_kernel` for all stateless operators (~55 operators across math, compare, logic, trig, convert, range, sample). Each entry is a pure C++ expression with `{a0}`, `{a1}`, etc.

**Example entries:**
```rust
// In crates/opengen-ops/src/math.rs
OpDef {
    name: "add",
    cpp_kernel: Some("return {a0} + {a1};"),
    emit_cpp_call: None, // stateless — cpp_kernel is sufficient
    // ... all other fields unchanged
}

// In crates/opengen-ops/src/trig.rs
OpDef {
    name: "sin",
    cpp_kernel: Some("return std::sin({a0});"),
    emit_cpp_call: None,
}

// In crates/opengen-ops/src/convert.rs
OpDef {
    name: "eq",
    cpp_kernel: Some("return ({a0} == {a1}) ? 1.0 : 0.0;"),
    emit_cpp_call: None,
}
```

**Implementation strategy:** Batch-populate per module. Order: math.rs → compare.rs → logic module → trig.rs → convert.rs → range.rs → sample.rs.

**Step 1: Add fields to OpDef**

```rust
// crates/opengen-ops/src/registry.rs
pub struct OpDef {
    pub name: &'static str,
    pub arity: u16,
    pub state: StateDecl,
    pub deferred_ports: &'static [u16],
    pub update: Option<UpdateFn>,
    pub init: Option<InitFn>,
    pub kernel: Kernel,
    /// Pure C++ expression body with {a0}, {a1}, ... placeholders.
    /// None for stateful ops that need custom emit_cpp_call.
    pub cpp_kernel: Option<&'static str>,
    /// Emit full per-call C++ for stateful/sr-dependent ops.
    pub emit_cpp_call: Option<fn(
        out_slot: usize, in_slots: &[usize], state_off: usize, sr: f64
    ) -> String>,
}
```

Update every existing `OpDef` construction to include `cpp_kernel: None, emit_cpp_call: None`.

**Step 2: Write test for one module**

```rust
#[test]
fn add_op_emits_correct_cpp_kernel() {
    let op = Registry::core().get("add").unwrap();
    assert!(op.cpp_kernel.is_some(), "add should have a cpp_kernel");
    let kernel = op.cpp_kernel.unwrap();
    assert!(kernel.contains("{a0}"), "should use {a0} placeholder");
    assert!(kernel.contains("{a1}"), "should use {a1} placeholder");
}
```

**Step 3: Populate math.rs, run test, verify**

**Step 4: Commit**

```bash
git commit -m "feat(emit-cpp): add cpp_kernel to OpDef, populate math operators"
```

**Step 5: Repeat for each operator module** — one commit each.

---

### Task 24: Emit stateful operator kernels

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-emit-cpp/src/emit_kernel.rs`
- Modify: Each stateful operator module (add `emit_cpp_call` function)

**Goal:** Emit stateful operators using custom `emit_cpp_call` functions. Each function receives `(out_slot, in_slots, state_off, sr)` and returns a C++ statement string with correct `v[N]` and `state[N]` indexing.

**Operators and their C++ emission:**

| Operator | emit_cpp_call returns |
|---|---|
| `history` | `"v[{out}] = state[{s0}]; state[{s0}] = v[{in0}];"` |
| `phasor` | `"v[{out}] = state[{s0}]; state[{s0}] = fmod(state[{s0}] + v[{in0}] / {sr}, 1.0);"` |
| `noise` | xoshiro256++ with 4 state slots; emit full algorithm referencing `state[{s0}]..state[{s3}]` |
| `dcblock` | `"v[{out}] = v[{in0}] - state[{s0}] + 0.9997 * state[{s1}]; state[{s0}] = v[{in0}]; state[{s1}] = v[{out}];"` |
| `slide` | `"v[{out}] = state[{s0}] + (v[{in0}] - state[{s0}]) * (1.0 - std::exp(..."` |
| `sah` | `"if (v[{in0}] > 0.0) state[{s0}] = v[{in1}]; v[{out}] = state[{s0}];"` |
| `latch` | edge-detection latch with extra state for previous trigger |
| `delta` | `"v[{out}] = v[{in0}] - state[{s0}]; state[{s0}] = v[{in0}];"` |
| `gate` | `"if (v[{in0}] > 0.0) state[{s0}] = v[{in1}]; v[{out}] = state[{s0}];"` |
| `elapsed` | `"if (v[{in0}] > 0.0) { state[{s0}] = 0.0; v[{out}] = 0.0; } else { v[{out}] = state[{s0}] * (1000.0 / {sr}); state[{s0}] += 1.0; }"` |
| `delay_write` | ring buffer write: `"state[data_off + (state[{s0}] % data_len)] = v[{in0}]; state[{s0}] = fmod(state[{s0}] + 1.0, data_len);"` |
| `delay_read` | ring buffer read at tap offset |
| `peek` | `"v[{out}] = state[data_off + clamped_index];"` with index clamping |
| `poke` | `"state[data_off + clamped_index] = v[{in1}]; v[{out}] = v[{in1}];"` |

**{s0}, {s1}** = `state_off + 0`, `state_off + 1`, etc. **{in0}, {in1}** = `in_slots[0]`, `in_slots[1]`. **{out}** = `out_slot`. **{sr}** = `sr` (f64 literal).

**Step 1: Write test for history**

```rust
#[test]
fn history_emit_cpp_call_uses_correct_state_indexing() {
    let op = Registry::core().get("history").unwrap();
    assert!(op.emit_cpp_call.is_some(), "history should have emit_cpp_call");
    let call = (op.emit_cpp_call.unwrap())(7, &[3], 5, 48000.0);
    assert!(call.contains("v[7]"), "should write to output slot 7");
    assert!(call.contains("state[5]"), "should read/write state at offset 5");
    assert!(call.contains("v[3]"), "should read input slot 3");
}
```

**Step 2: Run test to verify it fails** (emit_cpp_call not set yet)

**Step 3: Implement emit_cpp_call for history, run test**

**Step 4: Commit**

```bash
git commit -m "feat(emit-cpp): emit stateful kernel for history"
```

**Step 5: Repeat for each stateful operator** — one commit each.

---

### Task 25: Emit region/control-flow constructs

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-emit-cpp/src/emit_regions.rs`

**Goal:** Emit `if`/`else`/`for`/`while`/`iter` from `NodeKind::Region(ProcRegion)`. The region IR lives in `opengen_ir::proc` — the emitter reads it directly.

**Pattern:**
```cpp
if (v[cond_slot] > 0.0) {
    v[true_slot] = v[input_slot];
} else {
    v[false_slot] = v[input_slot];
}
```

**Step 1: Write failing test with codebox**

```rust
#[test]
fn if_else_codebox_emits_correct_cpp_control_flow() {
    let src = "if (in1 > 0) { out1 = 10; } else { out1 = 20; }";
    let graph = parse_and_lower(src).unwrap();
    let cpp = emit_cpp(&graph, &Registry::core(), 48000.0).unwrap();
    assert!(cpp.body.contains("if ("), "should contain if statement");
    assert!(cpp.body.contains("else"), "should contain else");
}
```

**Step 2–4: Implement + verify**

**Step 5: Commit**

```bash
git commit -m "feat(emit-cpp): emit if/else/for control flow from region IR"
```

---

### Task 26: Cross-backend determinism validation suite

**TDD scenario:** Modifying tested code — run existing tests first

**Files:**
- Modify: `crates/opengen-emit-cpp/tests/bit_identical.rs`
- Modify: `crates/opengen-analysis/tests/conformance.rs`

**Goal:** Every conformance patch renders bit-identical across Rust and C++ backends. This is the M3 exit criterion.

**Step 1: Add `compile_and_render_cpp` helper**

```rust
/// Parse, emit C++, compile, run, and return a Render.
fn compile_and_render_cpp(src: &str, sr: f64, n_samples: usize) -> Render {
    let graph = opengen_genexpr::parse_and_lower(src).unwrap();
    let cpp = emit_cpp(&graph, &Registry::core(), sr).unwrap();
    // Determine inputs from the graph's input count, zero-fill
    let n_inputs = graph.nodes().filter(|(_, n)| matches!(n.kind, NodeKind::Input(_))).count();
    let inputs = vec![0.0; n_inputs * n_samples];
    let output = compile_and_run_cpp(&cpp, &inputs, n_samples);
    // Wrap in Render struct for channel access
    todo!("wrap output in Render")
}
```

**Step 2: Write sweep test**

```rust
#[test]
fn all_operators_bit_identical_cross_backend() {
    use glob::glob;
    for entry in glob("conformance/patches/ops/op_*.genexpr").unwrap() {
        let path = entry.unwrap();
        let src = std::fs::read_to_string(&path).unwrap();

        let rust_out = opengen_testkit::render(&src, 48000.0, 4096);
        let cpp_out = compile_and_render_cpp(&src, 48000.0, 4096);

        for ch in 0..rust_out.n_channels() {
            assert_eq!(
                rust_out.ch(ch), &cpp_out.ch(ch)[..],
                "channel {} of {} diverges between Rust and C++ backends",
                ch, path.display()
            );
        }
    }
}
```

**Step 3: Run test** — will fail until all operators are emitted.

Run: `cargo test -p opengen-emit-cpp -- all_operators_bit_identical`
Expected: FAIL with list of diverging operators

**Step 4: Fix each diverging operator** — re-run after each fix until all pass.

**Step 5: Commit**

```bash
git commit -m "test(emit-cpp): cross-backend determinism suite for all operators"
```

**Exit:** All operators produce bit-identical output across backends.

---

### Task 27: CLI `emit` integration

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-cli/src/main.rs`

**Goal:** `opengen emit file.genexpr --target cpp` emits C++ to stdout or files.

**Step 1: Write failing roundtrip test**

```rust
// In crates/opengen-cli/tests/ (or integration test)
#[test]
fn emit_cpp_roundtrip_matches_rust_run() {
    // parse_and_lower → emit_cpp → compile C++ → run → compare with opengen run output
}
```

**Step 2: Implement `emit` subcommand**

```
opengen emit <file> [--target cpp] [--output <dir>] [--sample-rate <hz>]
```

Writes `opengen_patch.h` and `opengen_patch.cpp` to the output directory (default: current dir).

**Step 3: Run test to verify**

**Step 4: Commit**

```bash
git commit -m "feat(cli): add emit subcommand for C++ codegen"
```

---

## Phase 3: Integration and Polish (Tasks 28–31)

### Task 28: `require` declaration

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/ast.rs`
- Modify: `crates/opengen-genexpr/src/lower.rs`
- Modify: `crates/opengen-gendsp/src/build.rs`

**Goal:** `require name` imports a named binding from the host patcher. The host graph's `seeded_inputs` or param bindings must include the required name, or codebox lowering emits an error.

**Step 1: Write failing test**

```rust
#[test]
fn require_binds_host_param_to_codebox() {
    // Host gendsp with param g → codebox with `require g; out1 = g;`
}
```

**Step 2–4: Implement + verify**

**Step 5: Commit**

---

### Task 29: History read-after-write divergence decision

**TDD scenario:** Trivial change — use judgment

**Files:**
- Create: `docs/research/history_read_after_write_decision.md`

**Goal:** Formalize the decision: keep dataflow semantics as a documented divergence.

**Decision document structure:**
1. Problem: gen~ write-through vs. opengen dataflow
2. Evidence: conformance goldens at 44.1k and 48k
3. Options: keep vs. change
4. Decision: keep dataflow semantics
5. Rationale: compilation simplicity, conformance unaffected
6. Future: revisit with `#[gen_compat]` attribute if user demand exists

**Step 1: Write document → commit**

---

### Task 30: Ratchet climbing — re-pin GSOT coverage

**TDD scenario:** Modifying tested code — run existing tests first

**Files:**
- Modify: `crates/opengen-analysis/tests/m2_exit.rs`

**Goal:** Phase 1 fixes may have improved GSOT coverage beyond 121/189. Re-run and re-pin.

**Step 1: Run test**

Run: `cargo test -p opengen-analysis -- m2_exit`
Expected: observe current pass count

**Step 2: If > 121, update the ratchet constant**

**Step 3: Commit**

---

### Task 31: Multi-channel data + FMA documentation

**TDD scenario:** New feature (data) + trivial (FMA doc)

**Files:**
- Modify: `crates/opengen-genexpr/src/lower.rs`
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Create: `docs/research/fma_determinism.md`

**31a — Multi-channel data:** `Data d(4, 2)` declares 2-channel data of size 4. Channel index selects the channel.

**31b — FMA documentation:** Document FMA risk, compiler flags (`-ffp-contract=off`), and the rule against precision-changing constant folding in either backend.

**Commit:** Two commits.

---

## Design Decisions (recorded for reference)

| Decision | Choice | Rationale |
|---|---|---|
| Kernel emission | Template-based with `emit_cpp_call` per operator | Static strings can't express state indices or sr-dependent values |
| `selector` variable arity | Register `selector3`, `selector5` as fixed-arity OpDefs | Zero IR changes; covers 99% of gen~ patches; M4 revisit |
| `cartopol`/`poltocar` | Deferred to M4 | IR has no multi-output node support; adding it is non-trivial |
| `require` phase | Phase 3 | Doesn't block emitter; can be done anytime after lowering is stable |
| History divergence | Keep dataflow semantics | Cleaner compilation; conformance patches avoid the pattern |
| FMA determinism | `-ffp-contract=off` in C++, no constant folding in either backend | Shared test suite validates bit-identity |

---

## Exit Criteria

1. **Vendor genexpr corpus: 80/80 parse** (Tasks 1–4)
2. **Strict mode rejects decl-after-expr and self-referential history** (Tasks 5–6)
3. **All gen~ operators implemented:** `selector`, `gate`, `elapsed`, `wave`, `smoothstep`, `step`, `rmod` (Tasks 8–12)
4. **Multi-tap delay (TAPS > 1) works** (Task 13)
5. **Remaining M3 backlog cleared:** for-init comma lowering, early returns, delay members in regions, peek/poke NaN conformance, lexer refactor, codebox abstraction in control flow (Tasks 14–19)
6. **`require` declaration works** (Task 28)
7. **History divergence decision documented** (Task 29)
8. **C++ emitter produces bit-identical output** for all operators (Task 26)
9. **CLI `emit` command integrated** (Task 27)
10. **`cargo test --workspace` green**, zero failures
11. **`cargo doc --workspace --no-deps` zero warnings**
12. **GSOT ratchet re-pinned upward** (Task 30)
