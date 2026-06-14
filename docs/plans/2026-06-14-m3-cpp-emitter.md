# M3 — C++ Emitter and Language Completion

> **REQUIRED SUB-SKILL:** Use the executing-plans skill to implement this plan task-by-task.

**Goal:** Emit dependency-free C++ from the IR that matches the Rust backend bit-for-bit, plus resolve the accumulated M2 language-completion backlog (remaining parse failures, missing operators, strict-mode lints, and conformance gaps).

**Architecture:** A new `opengen-emit-cpp` crate reads the same typed dataflow `Graph` that `opengen-compile` uses. It produces a single self-contained `.h`/`.cpp` pair: a flat `f64` state arena, a topo-sorted per-sample `process(float* in, float* out)` function, and a `set_param(name, value)` API. Kernels are emitted as inline C++ functions from descriptor tables in the IR. The shared determinism contract (IEEE-754 f64, spec'd evaluation order, no fast-math, seeded PRNG) is the cross-backend test — `cargo test` renders both backends and asserts bit-identical output.

**Tech Stack:** Rust (emitter), C++17 (output), existing IR/registry/compile crates unchanged.

---

## Phase 1: C++ Emitter Core (Tasks 1–7)

This is the M3 milestone itself. The emitter produces a `Patch`-equivalent C++ artifact that can be compiled standalone (no Max/MSP dependency).

### Architecture

```
opengen-emit-cpp/
  src/
    lib.rs          # Public API: emit_cpp(g, reg) -> CppSource
    emit_graph.rs   # Graph-level emission: state arena layout, topo sort, per-sample loop
    emit_kernel.rs  # Kernel emission: one fn per operator from registry descriptor tables
    emit_regions.rs # Region emission (control flow, local variables)
    emit_types.rs   # Type mapping (f64 → double, port indices → int)
    templates.rs    # Template fragments for header/footer boilerplate
  tests/
    bit_identical.rs # Cross-backend tests: Rust render vs C++ render
    compile_cpp.rs   # Tests that compile and run emitted C++ via `cc` crate
```

**Emitted C++ API:**
```cpp
// opengen_patch.h — self-contained, no dependencies beyond C++17 stdlib
struct Patch {
    int n_inputs, n_outputs, n_params;
    std::vector<double> state;  // flat state arena
    // Per-sample: reads from in[], writes to out[]
    void process(const double* in, double* out);
    // Parameter update between buffers
    void set_param(const std::string& name, double value);
};
```

### Task 1: Create `opengen-emit-cpp` crate skeleton

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-emit-cpp/Cargo.toml`
- Create: `crates/opengen-emit-cpp/src/lib.rs`
- Create: `crates/opengen-emit-cpp/src/emit_graph.rs`
- Create: `crates/opengen-emit-cpp/src/emit_kernel.rs`
- Modify: `Cargo.toml` (workspace members)

**Step 1: Add crate to workspace**

Add `"crates/opengen-emit-cpp"` to workspace `members` in root `Cargo.toml`. Create `Cargo.toml` with dependencies on `opengen-ir` and `opengen-ops`.

**Step 2: Write the failing cross-backend smoke test**

```rust
// crates/opengen-emit-cpp/tests/bit_identical.rs
use opengen_emit_cpp::emit_cpp;
use opengen_genexpr::parse_and_lower;

#[test]
fn constant_to_output_matches_rust_backend() {
    let src = "out1 = 0.5;";
    let graph = parse_and_lower(src).unwrap();
    let cpp_source = emit_cpp(&graph, &opengen_ops::Registry::core())
        .expect("emission should succeed");

    // The emitted C++ must contain the constant 0.5
    assert!(cpp_source.body.contains("0.5"),
        "emitted C++ should contain the constant value");
}
```

**Step 3: Run test to verify it fails**

Run: `cargo test -p opengen-emit-cpp -- bit_identical`
Expected: FAIL — crate not yet declared in workspace or `emit_cpp` not found

**Step 4: Implement minimal `emit_cpp`**

```rust
pub struct CppSource {
    pub header: String,
    pub body: String,
}

pub fn emit_cpp(graph: &Graph, reg: &Registry) -> Result<CppSource, String> {
    // Minimal: just emit constants for now
    let mut body = String::from("void Patch::process(const double* in, double* out) {\n");
    for (id, node) in graph.nodes() {
        match &node.kind {
            NodeKind::Constant(v) => {
                body.push_str(&format!("    // constant: {}\n", v));
            }
            _ => {}
        }
    }
    body.push_str("}\n");
    Ok(CppSource {
        header: String::new(),
        body,
    })
}
```

**Step 5: Run test to verify it passes**

Run: `cargo test -p opengen-emit-cpp -- bit_identical`
Expected: PASS

**Step 6: Commit**

```bash
git add Cargo.toml crates/opengen-emit-cpp/
git commit -m "feat(opengen-emit-cpp): skeleton crate with minimal emit_cpp"
```

---

### Task 2: Emit state arena layout

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-emit-cpp/src/emit_graph.rs`

**Goal:** Walk the graph, collect all operator state declarations, and emit a flat `std::vector<double>` with computed offsets. The layout must match `opengen_compile`'s state arena exactly so cross-backend bit-identity is achievable.

**Step 1: Write failing test**

```rust
#[test]
fn history_node_produces_state_allocation() {
    let src = "h = history(in1); out1 = h;";
    let graph = parse_and_lower(src).unwrap();
    let cpp = emit_cpp(&graph, &Registry::core()).unwrap();
    // Should contain state vector initialization
    assert!(cpp.header.contains("std::vector<double> state"),
        "header should declare state vector");
    assert!(cpp.body.contains("state["),
        "body should reference state array for history");
}
```

**Step 2–4: Implement + verify + commit**

Implement `layout_state()` that mirrors `opengen_compile`'s state layout: walk topo-sorted nodes, assign contiguous indices for each `StateDecl::Slots(n)`, produce a `HashMap<NodeId, Range<usize>>`.

**Step 5: Commit**

```bash
git commit -m "feat(emit-cpp): state arena layout matching compile backend"
```

---

### Task 3: Emit per-sample process loop

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-emit-cpp/src/emit_graph.rs`

**Goal:** Emit the topo-sorted per-sample compute loop. Each node becomes a C++ statement that writes its result to a value slot.

**Step 1: Write failing test — minimal graph emits compilable C++**

```rust
#[test]
fn minimal_graph_compiles_and_runs() {
    let src = "out1 = in1 * 0.5;";
    let graph = parse_and_lower(src).unwrap();
    let cpp = emit_cpp(&graph, &Registry::core()).unwrap();
    // Compile and run the emitted C++ via `cc` crate
    let result = compile_and_run_cpp(&cpp, &[2.0, 4.0], 2);
    assert_eq!(result, vec![1.0, 2.0]);
}
```

Helper `compile_and_run_cpp` writes the C++ to a temp file, invokes `cc::Build`, links a small harness, and returns the output buffer.

**Step 2–4: Implement + verify + commit**

Emit topo-sorted nodes with value slots. Each node's output is stored in `double v[N]` and consumed by downstream nodes via index references.

**Step 5: Commit**

```bash
git commit -m "feat(emit-cpp): topo-sorted per-sample process loop"
```

---

### Task 4: Emit operator kernels

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-emit-cpp/src/emit_kernel.rs`
- Possibly modify: `crates/opengen-ops/src/registry.rs` (add C++ kernel strings or a `cpp_body` method)

**Goal:** Each operator kernel must be emitted as C++ code. There are two strategies:

**Strategy A (chosen — descriptor tables):** Each `OpDef` gets an optional `cpp_body: Option<&'static str>` field. The emitter inlines this body into a switch/match. This keeps kernel definitions next to the Rust spec (single source of truth for math).

**Strategy B (rejected):** Emit per-operator C++ functions from Rust code. Fragile — risks Rust→C++ translation drift.

**Implementation:** Add a `cpp_body` field to `OpDef` and populate it for arithmetic operators first (`add`, `sub`, `mul`, `div`, `mod`). The emitter generates:

```cpp
double kernel_add(double a, double b) { return a + b; }
double kernel_sub(double a, double b) { return a - b; }
// ...
```

With inline `__attribute__((always_inline))` or similar so the C++ compiler can optimize. The process loop calls these through the descriptor table.

**Step 1: Write test — arithmetic op matches Rust backend**

```rust
#[test]
fn add_op_bit_identical() {
    let src = "out1 = in1 + 2.25;";
    let graph = parse_and_lower(src).unwrap();
    let cpp = emit_cpp(&graph, &Registry::core()).unwrap();
    let rust_out = opengen_testkit::render(src, 48000.0, 3);
    let cpp_out = compile_and_run_cpp(&cpp, &rust_out.ch(0), 3); // feed same input
    assert_eq!(rust_out.ch(0), &cpp_out[..]);
}
```

**Step 2–4: Implement + verify (start with add/sub/mul/div, expand incrementally)**

**Step 5: Commit** (one commit per operator family or batch)

---

### Task 5: Emit stateful operators (history, delay, data, phasor, noise)

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-emit-cpp/src/emit_kernel.rs`
- Add: `cpt_body` entries for stateful operators in their respective opengen-ops modules

**Goal:** Emit stateful operator kernels with correct state indexing. The state arena layout from Task 2 means each operator reads/writes `state[offset + N]`.

**Operators:**
- `history` — state[off+0] = current input; return previous
- `delay_write` / `delay_read` — ring buffer access via data node
- `data` — raw read from data buffer
- `phasor` — accumulator with phase state
- `noise` — xoshiro256++ with seed state (seed=0x0123456789ABCDEF)
- `dcblock` — x1, y1 state (2 slots)
- `slide` — target state
- `sah` — held value state

**Test pattern:** For each stateful operator, write a bit-identical cross-backend test — render with Rust, render with C++, compare.

**Step 1–5 per operator:** TDD cycle (failing test → implement → verify → commit)

---

### Task 6: Emit region/control-flow constructs

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Create: `crates/opengen-emit-cpp/src/emit_regions.rs`

**Goal:** Emit `if`/`else`/`for`/`while`/`iter` constructs from region nodes. The IR's `NodeKind::Region` contains a resolved `RExpr`/`RStep` tree (produced by opengen-compile's lowering). The emitter traverses this tree and emits C++ control flow.

**Pattern:**
```cpp
// if (cond) { ... } else { ... }
if (v[cond_slot] > 0.0) {
    v[true_slot] = v[in_slot];
} else {
    v[false_slot] = v[in_slot];
}
```

**Test:** Codebox with control flow emitting bit-identical output to Rust backend.

---

### Task 7: Cross-backend determinism validation suite

**TDD scenario:** Modifying tested code — run existing tests first

**Files:**
- Modify: `crates/opengen-emit-cpp/tests/bit_identical.rs`
- Modify: `crates/opengen-analysis/tests/conformance.rs` (add C++ render path)

**Goal:** Every conformance test that passes the Rust backend must also pass the C++ backend with bit-identical output. This is the M3 exit criterion.

**Implementation:**
1. Add a `compile_and_render_cpp(graph, sr, inputs, n_samples) -> Render` function
2. Extend the conformance test harness to optionally run both backends and assert identical output
3. Run the full 71-operator conformance sweep through both backends

**Step 1: Failing test — conformance sweep through C++ backend**

```rust
#[test]
fn all_operators_bit_identical_cross_backend() {
    for patch in glob("conformance/patches/ops/op_*.genexpr") {
        let src = std::fs::read_to_string(patch).unwrap();
        let graph = opengen_genexpr::parse_and_lower(&src).unwrap();
        let cpp = emit_cpp(&graph, &Registry::core()).unwrap();
        let rust_out = render(&src, 48000.0, 4096);
        let cpp_out = compile_and_render_cpp(&cpp, 48000.0, 4096);
        for ch in 0..rust_out.n_channels() {
            assert_eq!(rust_out.ch(ch), &cpp_out.ch(ch)[..],
                "channel {} of {} diverges between Rust and C++ backends", ch, patch);
        }
    }
}
```

This test will fail until the emitter handles all 71 operators. The failure list is the implementation tracker — each passing operator is a win.

**Step 2–5: Fix operators one by one, re-running the cross-backend test each time**

**Exit:** All 71 operators produce bit-identical output across Rust and C++ backends.

---

## Phase 2: Language Completion (Tasks 8–14)

These are the M2 backlog items accumulated during conformance work that didn't block the M2 exit but need resolution for a complete GenExpr implementation.

### Task 8: Fix 4 remaining vendor genexpr parse failures (76/80 → 80/80)

**TDD scenario:** New feature — full TDD cycle (one per failure)

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/lexer.rs`
- Modify: `crates/opengen-genexpr/tests/` (corpus test)

**Goal:** Pass all 80 vendor genexpr examples from `reference/gen_exprs/genexpr_js/genexprs/`. Currently at 76/80. The 4 remaining failures are:
- Comma expressions in `for` init contexts
- Named arguments inside function calls (`foo(bar=baz)`)
- Two other edge cases from the vendor corpus

**Approach:**
1. Extract the 4 failing `.genexpr` files from the vendor corpus
2. For each, write a minimal reproducing test
3. Fix the parser AST or lexer
4. Verify the fix against the full vendor corpus
5. Commit each fix separately

**Step 1: Write failing corpus test that asserts 80/80**

```rust
#[test]
fn vendor_genexpr_corpus_80_of_80_pass() {
    let failures = run_vendor_corpus(); // parse all 80, collect failures
    assert_eq!(failures.len(), 0, "expected 0 failures, got: {:?}", failures);
}
```

**Step 2–5: Fix each failure with TDD**

---

### Task 9: Declaration-ordering strict mode

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/ast.rs` (maybe a StrictMode flag)
- Modify: `crates/opengen-genexpr/tests/` (new test file)

**Goal:** Real gen~ rejects declarations after expression statements with "declarations must come before expressions" (observed Max 9 2026-06-10). opengen's parser is lenient. Implement a strict mode that enforces gen~'s ordering so authored patches stay gen~-loadable.

**Design options:**
- **Option A:** `parse()` is lenient (current behavior); `parse_strict()` rejects decl-after-expr
- **Option B:** A lint warning emitted from `parse()` when decl-after-expr is detected

Choose Option A (clean separation). The strict mode is opt-in via the parse API; the CLI uses it for `opengen check --strict`.

**Test:**
```rust
#[test]
fn strict_mode_rejects_decl_after_expression() {
    let src = "out1 = in1; h = history(in1);";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("declaration after expression"));
}
```

**Commit:** One commit for parser changes + test.

---

### Task 10: Self-referential history lint

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/tests/`

**Goal:** `h = history(h + 1)` is an opengen leniency — real gen~ errors "variable h is not defined" (observed Max 9 2026-06-10). gen~ requires history to be declared as `History h` extern before use. opengen allows the shorthand. Add a lint/error in strict mode.

**Design:** In `parse_strict()`, detect when a `history()` call references the variable being defined on the LHS. Emit an error matching gen~'s message.

**Test:**
```rust
#[test]
fn strict_mode_rejects_self_referential_history() {
    let src = "h = history(h + 1);";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not defined"));
}
```

---

### Task 11: `^^` precedence conformance cross-check

**TDD scenario:** Modifying tested code — run existing tests first

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/tests/parser_v2.rs`

**Goal:** The original plan's precedence ladder was mis-transcribed — the vendor PEG grammar defines `||` → `^^` → `&&` → `|` → `^` → `&`. Verify and fix if needed. Also, since `|` and `^` and `&` are now removed (not gen~), this simplifies to `||` → `^^` → `&&`.

**Approach:**
1. Write a conformance test that asserts the expected precedence: `a || b ^^ c && d` parses as `a || (b ^^ (c && d))`
2. Verify the current parser produces this tree
3. If not, fix the precedence chain in `parse_` methods
4. Add a gen~ conformance golden for extra confidence

**Test:**
```rust
#[test]
fn logical_precedence_matches_gen() {
    let src = "out1 = in1 || in2 ^^ in3 && in4;";
    let program = opengen_genexpr::parse(src).unwrap();
    // Should parse as: in1 || (in2 ^^ (in3 && in4))
    // Verify via lowering + render comparison with explicit grouping
}
```

---

### Task 12: Implement missing operators (`selector`, `gate`, `elapsed`, `wave`)

**TDD scenario:** New feature — full TDD cycle (one per operator)

**Files:**
- Create: `crates/opengen-ops/src/selector.rs`
- Create: `crates/opengen-ops/src/gate.rs`
- Create: `crates/opengen-ops/src/elapsed.rs`
- Create: `crates/opengen-ops/src/wave.rs`
- Modify: `crates/opengen-ops/src/registry.rs`

**Goal:** Implement these operators using the standard operator production line: research provenance → write rustdoc spec → implement kernel → verify with doctests + conformance golden.

**`selector`:** Multi-input switch — selects one of N inputs based on an integer index. gen~ semantics: index 0 → in1, index 1 → in2, etc. Out-of-range clamps to last input.

**`gate`:** Conditional pass-through — when gate signal > 0, passes input; otherwise outputs 0 (or previous value? Check gen~ docs). Research needed.

**`elapsed`:** Sample counter since reset. Outputs sample count since last non-zero trigger input. Resets to 0 when trigger > 0.

**`wave`:** Wavetable oscillator. Reads from a `Data`/`Buffer` using a phase input (0–1). Linear interpolation between samples.

**Commit:** One commit per operator (4 commits).

---

### Task 13: Implement `require` declaration

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-genexpr/src/ast.rs`
- Modify: `crates/opengen-genexpr/src/lower.rs`

**Goal:** `require name` is a gen~ declaration that imports a named binding from the host patcher (used in `.gendsp` codeboxes to reference host params/data). Implement as a no-op in standalone mode (codebox has no host) and as a lint check in `build_graph`'s codebox splicing.

**Design:** `require myparam` declares that `myparam` must exist in the seeding context. If the host graph doesn't provide it, the codebox lowering emits an error.

---

### Task 14: History read-after-write divergence decision

**TDD scenario:** Modifying tested code — run existing tests first

**Files:**
- Modify: `crates/opengen-genexpr/src/lower.rs` (if changing behavior)
- Modify: `docs/research/gen_docs/genexpr_language_reference.md`
- Modify: `crates/opengen-analysis/tests/conformance.rs` (update KNOWN_DIVERGENCES if needed)

**Goal:** The session documented this divergence (gen~ is write-through, opengen is dataflow). Decide whether to:
- **Keep as-is** (dataflow semantics — cleaner for compilation, documented divergence) — RECOMMENDED
- **Change to match gen~** (more complex compilation but full compatibility)

**Decision record:** Write a 1-page decision document at `docs/research/history_read_after_write_decision.md` summarizing the tradeoffs and recording the final decision. The conformance KNOWN_DIVERGENCES map stays as-is regardless.

**Commit:** Decision document only (no code change if keeping dataflow semantics).

---

## Phase 3: Conformance and Polish (Tasks 15–17)

### Task 15: Ratchet climbing — improve GSOT coverage

**TDD scenario:** Modifying tested code — run existing tests first

**Files:**
- Modify: `crates/opengen-analysis/tests/m2_exit.rs` (re-pin ratchet)

**Goal:** GSOT pinned at 121/189. Any session fixes (comment-box skip, subpatcher binding) that improved coverage should be reflected in an upward re-pin.

**Approach:**
1. Run `cargo test -p opengen-analysis -- m2_exit` and note current pass count
2. If higher than 121, re-pin the ratchet value
3. If unchanged, investigate which tests are failing and whether they're actionable this session

---

### Task 16: Multi-channel data support

**TDD scenario:** New feature — full TDD cycle

**Files:**
- Modify: `crates/opengen-genexpr/src/lower.rs`
- Modify: `crates/opengen-genexpr/src/parser.rs`
- Modify: `crates/opengen-ops/src/data.rs` (or new `mc_data.rs`)

**Goal:** `Data d(4, 2)` declares multi-channel data (width > 1). Currently only single-channel data is supported. Add multi-channel semantics: channel index becomes the second dimension in data node access.

---

### Task 17: FMA/determinism constraint documentation

**TDD scenario:** Trivial change — use judgment

**Files:**
- Create: `docs/research/fma_determinism.md`
- Modify: `CLAUDE.md` (add determinism constraint note)

**Goal:** The session discovered that C++ and Rust backends must fold identically — or neither folds — to maintain bit-identical output. FMA (fused multiply-add) is a key risk: different compiler flags (`-ffp-contract=off` in C++, no FMA in Rust) can produce different results for `a * b + c`.

**Document:**
1. The FMA risk and mitigation strategy
2. Compiler flags required for bit-identical output
3. Test strategy: cross-backend test suite validates bit-identity
4. Rule: no constant folding that changes precision in either backend

---

## Exit Criteria

1. **All 71 operators produce bit-identical output across Rust and C++ backends**
2. **`cargo test --workspace` green**, zero failures
3. **`cargo doc --workspace --no-deps` zero warnings**
4. **Vendor genexpr corpus: 80/80 parse** (4 remaining failures fixed)
5. **Strict mode rejects decl-after-expr and self-referential history**
6. **Missing operators implemented:** `selector`, `gate`, `elapsed`, `wave`
7. **`require` declaration works in codebox contexts**
8. **History read-after-write divergence decision documented**
9. **GSOT ratchet re-pinned upward** (any coverage improvements captured)
