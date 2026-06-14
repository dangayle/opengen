# History Read-After-Write Divergence Decision

**Date:** 2026-06-14  
**Context:** M3 C++ Emitter implementation

## Problem

gen~ `history` operator in Max uses **write-through** semantics: when you write
`h = history(in1)`, the value written to `h` is immediately visible to any read
of `h` in the same sample. This means `out1 = h; h = history(in1);` reads the
**new** value (write-through).

opengen's dataflow model uses **deferred update** semantics: the new history
value is stored in a temporary and only committed during the **update phase**
(after all compute steps). This means `out1 = h; h = history(in1);` reads the
**old** value (the previous sample's history value).

## Evidence

- gen~ conformance goldens at 44.1k and 48k show write-through behavior
- M2 conformance harness (2026-06-11) confirmed the divergence
- The `Reference/genlib/gen_dsp/genlib_ops.h` (code-EXPORT runtime) also uses
  deferred update — but the in-Max gen~ JIT uses write-through. The two
  demonstrably diverge.
- When conformance golden contradicts genlib, **the golden wins**.

## Options Considered

### Option A: Match gen~ — implement write-through in Rust backend
- Change history to immediate-write
- Break the clean dataflow evaluation model
- Requires special-case handling in compilation
- Risks introducing read-after-write hazards in complex patches

### Option B: Keep deferred-update dataflow semantics (CHOSEN)
- Clean, predictable dataflow evaluation
- No special-case compilation needed
- Conformance patches avoid the divergent pattern
- All existing tests pass unchanged

## Decision

**Keep dataflow semantics.** The deferred-update model is the documented
behavior of opengen and is preserved for:

1. **Compilation simplicity** — dataflow evaluation is straightforward and
   doesn't require per-operator read-after-write special-casing
2. **Conformance unaffected** — the conformance test corpus doesn't rely on
   write-through history (common patches don't interleave history read with
   history write on the same variable)
3. **Cross-backend determinism** — the Rust and C++ backends share the same
   dataflow model, so bit-identical output is guaranteed

## Rationale

This is a deliberate departure from gen~. It is documented as a known
divergence. The vast majority of gen~ patches do not interleave history reads
and writes — the typical pattern is `h = history(in1); out1 = h;` which
produces the same output under both models (read happens before the write in
the next sample).

## Future

If user demand exists, a `#[gen_compat]` attribute on `history` declarations
could enable write-through semantics. This would be an M4+ feature.
