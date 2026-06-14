#!/usr/bin/env python3
"""Generate per-operator conformance patches (one per OpDef in opengen-ops).

Two modes, both runtime-laundered so gen~ runs its real per-sample kernel
(not its constant folder, whose edge-case behavior diverges — observed
2026-06-11: constant sqrt(-1) folds to 0, runtime sqrt(-1) is NaN):

  SWEEP  (smooth/continuous ops): a runtime ramp `p = h/4095` drives the
         operator densely across its valid domain. Catches divergence
         anywhere in the range.

  POINTS (discontinuous ops): discrete laundered input tuples, each emitted
         as one output, placed inside each regime and AWAY from the
         discontinuity. Sweeping a discontinuous op across its jump makes
         two correct implementations diverge by a full step (a sub-ULP
         operand difference — gen~ FMA vs opengen — straddles the jump;
         `mod` at a/b = -1 demonstrated this). Boundary semantics are
         specified analytically by each operator's doctests instead.

Launder primitives: `p = h/4095` (runtime ramp 0..1); `c + h*0` (runtime
constant — the compiler can't fold x*0 for floats, since inf*0 = NaN).

Regenerate:  python3 tools/gen_op_sweeps.py
Then:        python3 tools/gen_render_host.py
"""
import re
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
OPS_SRC = REPO / "crates" / "opengen-ops" / "src"
OUT_DIR = REPO / "conformance" / "patches" / "ops"

EXCLUDE = {"noise"}  # different PRNG by design — never comparable

# Operators opengen implements that the IN-MAX gen~ codebox compiler refuses
# to compile, so no gen~ golden is obtainable (their opengen doctests remain
# the spec). Observed Max 9, 2026-06-13: the bitwise operators are rejected
# in BOTH the named-call form (`bitand(a,b)` -> "operator bitand not defined")
# AND the infix form (`a & b` -> "statement missing ';'"). Note the genbo /
# RNBO genexpr grammar (and our genexpr_language_reference.md, derived from it)
# DO list these operators — the third genbo-vs-in-Max-gen~ divergence this
# session. Revisit if a future Max build adds codebox bitwise support.
GEN_UNSUPPORTED = {"bitand", "bitor", "bitxor", "shl", "shr"}

# ── SWEEP domains ────────────────────────────────────────────────────────────
# Unary smooth ops: (lo, hi). Domains stay strictly inside the valid range;
# domain violations are covered by dedicated edge probes, not here.
SWEEP1 = {
    # trig (radians); tan avoids the pi/2 asymptote
    "sin": (-3.14159, 3.14159), "cos": (-3.14159, 3.14159), "tan": (-1.4, 1.4),
    "asin": (-1.0, 1.0), "acos": (-1.0, 1.0), "atan": (-4.0, 4.0),
    # math
    "neg": (-2.5, 2.5), "abs": (-2.5, 2.5), "sqrt": (0.0, 4.0),
    "exp": (-4.0, 4.0), "exp2": (-4.0, 4.0),
    "ln": (0.01, 8.0), "log": (0.01, 8.0), "log10": (0.01, 8.0), "log2": (0.01, 8.0),
    # convert: hyperbolic + inverse
    "sinh": (-3.0, 3.0), "cosh": (-3.0, 3.0), "tanh": (-3.0, 3.0),
    "asinh": (-3.0, 3.0), "acosh": (1.1, 5.0), "atanh": (-0.95, 0.95),
    # convert: scaling / unit conversions
    "degrees": (-6.0, 6.0), "radians": (-360.0, 360.0),
    "atodb": (0.001, 4.0), "dbtoa": (-60.0, 12.0),
    "mstosamps": (0.0, 100.0), "sampstoms": (0.0, 4800.0),
    # convert: identity on finite/normal inputs
    "fixnan": (-3.0, 3.0), "fixdenorm": (-3.0, 3.0),
    # convert: continuous within an integer interval
    "fract": (2.001, 2.999),
}
# Binary smooth ops: ((alo, ahi), (blo, bhi)). a ascends, b descends.
SWEEP2 = {
    "add": ((-2, 2), (2, -2)), "sub": ((-2, 2), (2, -2)), "mul": ((-2, 2), (2, -2)),
    "min": ((-2, 2), (2, -2)), "max": ((-2, 2), (2, -2)),
    "hypot": ((-3, 3), (3, -3)), "absdiff": ((-3, 3), (3, -3)),
    "atan2": ((-3, 3), (3, -3)),
    "div": ((-2, 2), (1, 3)), "rdiv": ((1, 3), (-2, 2)),
    "rsub": ((-2, 2), (2, -2)),
    "pow": ((0.1, 3), (-2, 3)),
    "mtof": ((0, 127), (440, 440)), "ftom": ((20, 8000), (440, 440)),
    "triangle": ((0, 1), (0.3, 0.7)),
}

# ── POINTS (discontinuous ops): op -> list of laundered input tuples ─────────
# Each tuple becomes one output. Points sit inside a regime, away from jumps.
POINTS = {
    # math
    "mod": [(2.3, 2), (-2.3, 2), (5.7, 2), (1.5, 0.4)],   # a/b non-integer
    "floor": [(2.3,), (2.9,), (-1.1,), (-1.9,)],
    "ceil": [(2.1,), (2.7,), (-1.1,), (-1.9,)],
    # convert: rounding / integer
    "int": [(2.3,), (2.9,), (-1.3,), (-1.9,)],
    "trunc": [(2.3,), (2.9,), (-1.3,), (-1.9,)],
    "round": [(2.3, 1), (2.7, 1), (-1.3, 1), (5.2, 2)],
    "sign": [(2.5,), (-2.5,), (0,)],
    # convert: predicates (0 is exact via laundering, so safe)
    "bool": [(2.5,), (-2.5,), (0,)],
    "not": [(2.5,), (0,)],
    "and": [(2, 3), (2, 0), (0, 3), (0, 0)],
    "or": [(2, 3), (2, 0), (0, 3), (0, 0)],
    "xor": [(2, 3), (2, 0), (0, 0)],
    # compare (equal case is correct under folding too: both engines give 1)
    "eq": [(2, 2), (2, 3), (3, 2)],
    "neq": [(2, 2), (2, 3), (3, 2)],
    "gt": [(3, 1), (1, 3), (2, 2)],
    "gte": [(3, 1), (1, 3), (2, 2)],
    "lt": [(1, 3), (3, 1), (2, 2)],
    "lte": [(1, 3), (3, 1), (2, 2)],
    # range: wrap is discontinuous at its bounds; points stay interior
    "wrap": [(0.3, 0, 1), (0.7, 0, 1), (1.3, 0, 1), (2.6, 0, 1), (-0.4, 0, 1)],
}

# ── Bespoke arity-3/5 SWEEP bodies for continuous range ops ──────────────────
def sweep_clip():  # clip is continuous; sweep across and beyond the bounds
    return ["x = -2 + 4 * p;", "out1 = clip(x, -1 + h*0, 1 + h*0);"]
def sweep_fold():  # fold value is continuous (reflection)
    return ["x = -2 + 5 * p;", "out1 = fold(x, 0 + h*0, 1 + h*0);"]
def sweep_mix():   # linear interpolation, continuous in t
    return ["t = 0 + 1 * p;", "out1 = mix(-1 + h*0, 2 + h*0, t);"]
def sweep_clamp(): # clamp == clip (continuous)
    return ["x = -2 + 4 * p;", "out1 = clamp(x, -1 + h*0, 1 + h*0);"]
def sweep_scale(): # linear map, continuous
    return ["x = 0 + 1 * p;", "out1 = scale(x, 0 + h*0, 1 + h*0, -5 + h*0, 5 + h*0);"]
def sweep_switch():  # switch(sel, a, b): discontinuous in sel -> points instead
    return None
BESPOKE = {
    "clip": sweep_clip, "fold": sweep_fold, "mix": sweep_mix,
    "clamp": sweep_clamp, "scale": sweep_scale,
}
# switch(sel,a,b) selects b when sel!=0 — discontinuous at sel=0 -> points
POINTS["switch"] = [(1, 10, 20), (0, 10, 20), (-1, 10, 20)]

HEADER = (
    "// AUTO-GENERATED by tools/gen_op_sweeps.py — do not edit by hand.\n"
    "// Per-operator conformance {mode} for `{name}` ({module}, arity {arity}).\n"
    "// Runtime-laundered: gen~ runs the real per-sample kernel, not the\n"
    "// constant folder. Output = raw operator result.\n"
)


def parse_registry():
    ops, pat = [], re.compile(r'OpDef \{ name: "([a-z_0-9]+)", arity: (\d+)')
    for f in sorted(OPS_SRC.glob("*.rs")):
        for m in pat.finditer(f.read_text()):
            ops.append((f.stem, m.group(1), int(m.group(2))))
    return ops


def lin(lo, hi):
    return f"{lo} + ({hi} - ({lo})) * p"


def body_for(module, name, arity):
    """Return (mode, lines) or None to skip."""
    if name in BESPOKE:
        return "sweep", BESPOKE[name]()
    if name in POINTS:
        lines = []
        for i, args in enumerate(POINTS[name], 1):
            launder = ", ".join(f"{a} + h*0" for a in args)
            lines.append(f"out{i} = {name}({launder});")
        return "points", lines
    if arity == 0:
        return "sweep", [f"out1 = {name} + h * 0;"]
    if arity == 1 and name in SWEEP1:
        lo, hi = SWEEP1[name]
        return "sweep", [f"s = {lin(lo, hi)};", f"out1 = {name}(s);"]
    if arity == 2 and name in SWEEP2:
        (alo, ahi), (blo, bhi) = SWEEP2[name]
        return "sweep", [f"a = {lin(alo, ahi)};", f"b = {lin(blo, bhi)};",
                         f"out1 = {name}(a, b);"]
    return None  # unclassified — surfaced so we never silently miss one


def main():
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    # clear stale generated patches
    for f in OUT_DIR.glob("op_*.genexpr"):
        f.unlink()
    ops = parse_registry()
    written, unclassified = 0, []
    for module, name, arity in ops:
        if name in EXCLUDE or name in GEN_UNSUPPORTED:
            continue
        res = body_for(module, name, arity)
        if res is None:
            unclassified.append(f"{module}/{name}(a{arity})")
            continue
        mode, lines = res
        src = HEADER.format(mode=mode, name=name, module=module, arity=arity)
        src += "History h(0);\np = h / 4095;\n"
        src += "\n".join(lines) + "\n"
        src += "h = h + 1;\n"
        (OUT_DIR / f"op_{name}.genexpr").write_text(src)
        written += 1
    print(f"wrote {written} op-sweep patches to {OUT_DIR}")
    if unclassified:
        print(f"UNCLASSIFIED (add to SWEEP/POINTS): {', '.join(unclassified)}")


if __name__ == "__main__":
    main()
