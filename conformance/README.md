# Conformance Suite

Authored `.genexpr` patches that exercise opengen operator semantics. Each patch is
rendered by opengen and compared sample-by-sample against a golden WAV file.

## Render Protocol

- **Sample rate:** 48 kHz
- **Duration:** 4096 samples (~85 ms)
- **No inputs:** all patches are self-contained (constants, hist, Param, etc.)

## Golden Files

| Directory | Origin | Purpose |
|-----------|--------|---------|
| `golden/` | Max gen~ (authoritative) | Cross-verified reference renders (external, not committed) |
| `golden-self/` | opengen self-render | Regression baseline: committed to repo, overwritten on `OPENGEN_BLESS` |

Resolution order: `golden/` first, then `golden-self/`. If neither exists, the test
SKIPs with a message and does not fail.

## Noise Exclusion

Patches containing `noise` (or any PRNG-based operator) are excluded from conformance
because opengen uses xoshiro256++ (fixed seed) while gen~ uses an internal PRNG.
No such patches exist in the current set.

## Blessing (updating golden-self)

```sh
OPENGEN_BLESS=1 cargo test -p opengen-analysis --test conformance
```

This writes `conformance/golden-self/<stem>.ch<N>.wav` (32-bit float WAV) for every
output channel of every patch. After blessing, the next run compares against these new
files.

## Running

```sh
cargo test -p opengen-analysis --test conformance
```

## Tolerances

| Pattern | Tolerance | Rationale |
|---------|-----------|-----------|
| `cycle` | 5e-3 | Transcendental (sin via polyphase approximation) |
| `phasor` | 1e-6 | Deterministic ramp with wrap |
| default | 1e-6 | Deterministic arithmetic |

## Checklist

See `CHECKLIST.md` for the full Task 27 exit criteria.
