#!/usr/bin/env python3
"""Generate conformance/render/render_host.maxpat (v4).

Capture happens INSIDE each gen~ patcher: the conformance GenExpr source is
embedded as a codebox (untouched), and each output channel is poked into a
named external buffer~ at index `elapsed`. Provenance for the mechanism
(facts, cited by path):
  - `elapsed` = samples since patcher DSP began
    (reference/gen/refpages/dsp/gen_dsp_elapsed.maxref.xml) — shares the
    exact t=0 at which codebox History state initializes, so capture is
    sample-aligned BY CONSTRUCTION (no record~, no arming, no DSP timing).
  - `poke <buffer> <channel>`: inlet 0 = value, inlet 1 = index;
    out-of-range index writes nothing
    (reference/gen/refpages/dsp/gen_dsp_poke.maxref.xml) — capture
    self-terminates once elapsed >= 4096.
  - External buffer~ access via a [buffer <name>] declaration box in the gen
    patcher (corpus idiom, e.g. gen~.resonator_bank_v2.maxpat).

node.script sizes the buffers to exactly 4096 samples (`sizeinsamps`) and
writes float32 WAVs (`writewave <path> float32` — int16 would both quantize
below the comparator's 1e-6 tolerance and clip counter values > 1.0).

Human flow: open patch -> DSP ON ~1s -> DSP OFF -> [writewavs].
Re-run: close and reopen the patch first (fresh gen~ state).

Regenerate after adding/editing a conformance patch:
    python3 tools/gen_render_host.py
"""
import json
import re
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
PATCHES_DIR = REPO / "conformance" / "patches"
N_SAMPLES = 4096


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


def gen_subpatcher(stem, src, n_out):
    """dsp.gen patcher: codebox + per-channel buffer/poke capture + out boxes."""
    gboxes = [{"box": {
        "id": "cb-1",
        "maxclass": "codebox",
        "code": src,
        "numinlets": 0,
        "numoutlets": n_out,
        "outlettype": [""] * n_out,
        "patching_rect": [30.0, 30.0, 480.0, 300.0],
    }}, {"box": {
        "id": "el-1",
        "maxclass": "newobj",
        "text": "elapsed",
        "numinlets": 0,
        "numoutlets": 1,
        "outlettype": [""],
        "patching_rect": [550.0, 30.0, 60.0, 22.0],
    }}]
    glines = []
    for k in range(n_out):
        name = f"{stem}_ch{k}"
        x = 30.0 + k * 170.0
        gboxes.append({"box": {
            "id": f"buf-{k}",
            "maxclass": "newobj",
            "text": f"buffer {name}",
            "numinlets": 0,
            "numoutlets": 2,
            "outlettype": ["", ""],
            "patching_rect": [x, 360.0, 150.0, 22.0],
        }})
        gboxes.append({"box": {
            "id": f"pk-{k}",
            "maxclass": "newobj",
            "text": f"poke {name} 1",
            "numinlets": 2,
            "numoutlets": 0,
            "patching_rect": [x, 400.0, 150.0, 22.0],
        }})
        gboxes.append({"box": {
            "id": f"go-{k + 1}",
            "maxclass": "newobj",
            "text": f"out {k + 1}",
            "numinlets": 1,
            "numoutlets": 0,
            "patching_rect": [x, 440.0, 60.0, 22.0],
        }})
        glines.append({"patchline": {"source": ["cb-1", k], "destination": [f"pk-{k}", 0]}})
        glines.append({"patchline": {"source": ["el-1", 0], "destination": [f"pk-{k}", 1]}})
        glines.append({"patchline": {"source": ["cb-1", k], "destination": [f"go-{k + 1}", 0]}})
    return {
        "fileversion": 1,
        "appversion": {
            "major": 9, "minor": 0, "revision": 0,
            "architecture": "x64", "modernui": 1,
        },
        "classnamespace": "dsp.gen",
        "rect": [100.0, 100.0, 760.0, 520.0],
        "boxes": gboxes,
        "lines": glines,
    }


# ── Instructions ─────────────────────────────────────────────────────────────
add_box(
    "comment",
    "GenExpr Conformance Render Host (v4)\n"
    "Capture happens INSIDE each gen~ (poke @ elapsed) — sample-aligned to patch t=0\n"
    "by construction. No record~, no arming.\n"
    "1. Open this patch; check Max console: all 9 gen~ must compile clean;\n"
    "   node.script autostarts and sizes 17 buffers to 4096 samples.\n"
    "2. Turn DSP ON (ezdac~), wait ~1 second, turn DSP OFF.\n"
    "3. Click [writewavs] — 17 float32 WAVs land in conformance/golden/.\n"
    "Re-run? Close and reopen the patch first (fresh gen~ state).",
    20, 10, 640, 110, 1, 0,
)

# ── node.script + control messages ───────────────────────────────────────────
script = add_box(
    "newobj", "node.script render_runner.js @autostart 1 @watch 1",
    20, 140, 330, 22, 1, 2, ["", ""],
)
msg_start = add_box("message", "script start", 20, 180, 80, 22, 2, 1)
msg_write = add_box("message", "writewavs", 110, 180, 70, 22, 2, 1)
for m in (msg_start, msg_write):
    connect(m, 0, script, 0)

add_box("ezdac~", None, 380, 140, 45, 45, 2, 0)

# ── Routing: script outlet 0 -> [route buf] -> per-buffer route ──────────────
route_top = add_box("newobj", "route buf", 20, 230, 70, 22, 1, 2, ["", ""])
connect(script, 0, route_top, 0)

buffer_names = []
for stem, (_, n_ch) in PATCHES.items():
    for ch in range(n_ch):
        buffer_names.append(f"{stem}_ch{ch}")

route_buf = add_box(
    "newobj", "route " + " ".join(buffer_names),
    110, 230, 1000, 22, 1, len(buffer_names) + 1,
    [""] * (len(buffer_names) + 1),
)
connect(route_top, 0, route_buf, 0)

# ── gen~ + buffer~ grid ──────────────────────────────────────────────────────
x0, y0 = 20, 290
col_w, row_h = 280, 100
chan_idx = 0
for i, (stem, (src, n_ch)) in enumerate(PATCHES.items()):
    col, row = i % 4, i // 4
    x, y = x0 + col * col_w, y0 + row * row_h
    add_box(
        "newobj", f"gen~ @title {stem}",
        x, y, 230, 22, 1, n_ch, ["signal"] * n_ch,
        extra={"patcher": gen_subpatcher(stem, src, n_ch)},
    )
    for ch in range(n_ch):
        name = f"{stem}_ch{ch}"
        buf_id = add_box(
            "newobj", f"buffer~ {name} 86",
            x + ch * 78, y + 35, 75, 22, 1, 1, ["float"],
        )
        connect(route_buf, chan_idx, buf_id, 0)  # sizeinsamps / writewave
        chan_idx += 1

patcher = {
    "patcher": {
        "fileversion": 1,
        "appversion": {
            "major": 9, "minor": 0, "revision": 0,
            "architecture": "x64", "modernui": 1,
        },
        "classnamespace": "box",
        "rect": [100.0, 100.0, 1250.0, 750.0],
        "boxes": boxes,
        "lines": lines,
        "dependency_cache": [],
        "autosave": 0,
    }
}

out = REPO / "conformance" / "render" / "render_host.maxpat"
out.write_text(json.dumps(patcher, indent=1) + "\n")
print(f"wrote {out} ({len(boxes)} boxes, {len(lines)} lines, {len(buffer_names)} channels)")
