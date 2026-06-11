#!/usr/bin/env python3
"""Generate conformance/render/render_host.maxpat.

Static render host: one gen~ per conformance patch with the GenExpr source
EMBEDDED as a codebox inside a dsp.gen subpatcher (the corpus-verified
structure — see gen~.resonator_bank_v2.maxpat). There is no file resolution
at load time: gen~ has no documented way to reference a .genexpr file from
its box text (.genexpr files only load via `require()` for function
libraries, per the gen userguide), so embedding is the only reliable path.

One record~ + buffer~ pair per output channel. node.script orchestrates
buffer sizing (sizeinsamps 4096), record arming, and absolute-path WAV
writes — no runtime code injection (vanilla gen~ has no `code` message).

Regenerate after adding/editing a conformance patch:
    python3 tools/gen_render_host.py
"""
import json
from pathlib import Path

import re

REPO = Path(__file__).resolve().parent.parent
PATCHES_DIR = REPO / "conformance" / "patches"

# stem -> (source, n_out) derived from conformance/patches/*.genexpr
def load_patches():
    patches = {}
    for f in sorted(PATCHES_DIR.glob("*.genexpr")):
        src = f.read_text()
        outs = max(int(m) for m in re.findall(r"out(\d+)\s*=", src))
        ins = [int(m) for m in re.findall(r"\bin(\d+)\b", src)]
        assert not ins, f"{f.stem}: render host assumes input-free patches"
        patches[f.stem] = (src, outs)
    return patches

PATCHES = load_patches()

boxes = []
lines = []
_id = [0]

def add_box(maxclass, text, x, y, w, h, n_in, n_out, outlettype=None, extra=None):
    _id[0] += 1
    bid = f"obj-{_id[0]}"
    box = {
        "id": bid,
        "maxclass": maxclass,
        "numinlets": n_in,
        "numoutlets": n_out,
        "patching_rect": [x, y, w, h],
    }
    if text is not None:
        box["text"] = text
    if n_out > 0:
        box["outlettype"] = outlettype or [""] * n_out
    if extra:
        box.update(extra)
    boxes.append({"box": box})
    return bid

def connect(src, src_idx, dst, dst_idx):
    lines.append({"patchline": {"source": [src, src_idx], "destination": [dst, dst_idx]}})

def gen_subpatcher(src, n_out):
    """Build a dsp.gen patcher embedding `src` in a codebox wired to out boxes.

    Structure verified against the vendor corpus (gen~ box with embedded
    patcher, classnamespace dsp.gen, codebox with `code` attribute,
    `out N` newobj boxes).
    """
    gboxes = [{"box": {
        "id": "cb-1",
        "maxclass": "codebox",
        "code": src,
        "numinlets": 0,
        "numoutlets": n_out,
        "outlettype": [""] * n_out,
        "patching_rect": [30.0, 30.0, 480.0, 300.0],
    }}]
    glines = []
    for k in range(n_out):
        oid = f"go-{k + 1}"
        gboxes.append({"box": {
            "id": oid,
            "maxclass": "newobj",
            "text": f"out {k + 1}",
            "numinlets": 1,
            "numoutlets": 0,
            "patching_rect": [30.0 + k * 80.0, 360.0, 60.0, 22.0],
        }})
        glines.append({"patchline": {"source": ["cb-1", k], "destination": [oid, 0]}})
    return {
        "fileversion": 1,
        "appversion": {
            "major": 9, "minor": 0, "revision": 0,
            "architecture": "x64", "modernui": 1,
        },
        "classnamespace": "dsp.gen",
        "rect": [100.0, 100.0, 600.0, 450.0],
        "boxes": gboxes,
        "lines": glines,
    }

# ── Instructions comment ─────────────────────────────────────────────────────
add_box(
    "comment",
    "GenExpr Conformance Render Host (v3)\n"
    "GenExpr sources are EMBEDDED (codebox in each gen~) — nothing to resolve.\n"
    "1. Open this patch; check Max console: all 9 gen~ must compile clean.\n"
    "2. node.script autostarts (or click [script start]); console shows buffer sizing.\n"
    "3. Click [arm] with DSP OFF.\n"
    "4. Turn DSP ON (ezdac~), wait 1 second, turn DSP OFF.\n"
    "5. Click [writewavs] — 17 WAVs land in conformance/golden/.\n"
    "Re-run? Close and reopen the patch first (fresh gen~ state).",
    20, 10, 620, 120, 1, 0,
)

# ── node.script + control messages ───────────────────────────────────────────
script = add_box(
    "newobj", "node.script render_runner.js @autostart 1 @watch 1",
    20, 150, 330, 22, 1, 2, ["", ""],
)
msg_start = add_box("message", "script start", 20, 190, 80, 22, 2, 1)
msg_arm = add_box("message", "arm", 110, 190, 40, 22, 2, 1)
msg_disarm = add_box("message", "disarm", 160, 190, 55, 22, 2, 1)
msg_write = add_box("message", "writewavs", 225, 190, 70, 22, 2, 1)
for m in (msg_start, msg_arm, msg_disarm, msg_write):
    connect(m, 0, script, 0)

add_box("ezdac~", None, 380, 150, 45, 45, 2, 0)

# ── Routing: script outlet 0 -> [route rec buf] ──────────────────────────────
route_top = add_box("newobj", "route rec buf", 20, 240, 100, 22, 1, 3, ["", "", ""])
connect(script, 0, route_top, 0)

buffer_names = []
for stem, (_, n_ch) in PATCHES.items():
    for ch in range(n_ch):
        buffer_names.append(f"{stem}_ch{ch}")

route_buf = add_box(
    "newobj", "route " + " ".join(buffer_names),
    140, 240, 1000, 22, 1, len(buffer_names) + 1,
    [""] * (len(buffer_names) + 1),
)
connect(route_top, 1, route_buf, 0)

# ── gen~ / record~ / buffer~ grid ────────────────────────────────────────────
x0, y0 = 20, 300
col_w, row_h = 260, 130
chan_idx = 0
for i, (stem, (src, n_ch)) in enumerate(PATCHES.items()):
    col, row = i % 4, i // 4
    x, y = x0 + col * col_w, y0 + row * row_h
    gen_id = add_box(
        "newobj", f"gen~ @title {stem}",
        x, y, 220, 22, 1, n_ch, ["signal"] * n_ch,
        extra={"patcher": gen_subpatcher(src, n_ch)},
    )
    for ch in range(n_ch):
        name = f"{stem}_ch{ch}"
        rec_id = add_box(
            "newobj", f"record~ {name}",
            x + ch * 75, y + 35, 70, 22, 1, 1, ["signal"],
        )
        buf_id = add_box(
            "newobj", f"buffer~ {name} 86",
            x + ch * 75, y + 70, 70, 22, 1, 1, ["float"],
        )
        connect(gen_id, ch, rec_id, 0)        # signal to record
        connect(route_top, 0, rec_id, 0)      # arm/disarm (1/0)
        connect(route_buf, chan_idx, buf_id, 0)  # sizeinsamps / write
        chan_idx += 1

patcher = {
    "patcher": {
        "fileversion": 1,
        "appversion": {
            "major": 9, "minor": 0, "revision": 0,
            "architecture": "x64", "modernui": 1,
        },
        "classnamespace": "box",
        "rect": [100.0, 100.0, 1200.0, 800.0],
        "boxes": boxes,
        "lines": lines,
        "dependency_cache": [],
        "autosave": 0,
    }
}

render_dir = REPO / "conformance" / "render"
out = render_dir / "render_host.maxpat"
out.write_text(json.dumps(patcher, indent=1) + "\n")
print(f"wrote {out} ({len(boxes)} boxes, {len(lines)} lines, {len(buffer_names)} channels)")
