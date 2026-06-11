#!/usr/bin/env node
/**
 * render_runner.js — orchestrates gen~ conformance golden rendering (v4).
 *
 * Runs inside Max via `node.script` in render_host.maxpat (regenerate the
 * host with tools/gen_render_host.py). Capture happens INSIDE each gen~
 * patcher (poke @ elapsed — sample-aligned to patch t=0 by construction),
 * so this script only:
 *
 *   1. On start: sizes every capture buffer to exactly 4096 samples
 *      (`sizeinsamps` — buffer~ args are in ms, which cannot hit an exact
 *      sample count).
 *   2. "writewavs": sends each buffer~ `writewave <abspath> float32`.
 *      float32 is REQUIRED: int16 quantization (~3e-5) breaks the
 *      comparator's 1e-6 tolerance AND clips counter values > 1.0
 *      (observed on the first int16 render attempt, 2026-06-11).
 *
 * Message protocol out of node.script outlet 0 (dispatched in the patch):
 *   ("buf", <name>, "sizeinsamps", 4096)            -> [route buf] -> [route <names>] -> buffer~
 *   ("buf", <name>, "writewave", <abspath>, "float32") -> same path
 */

"use strict";

const path = require("path");
const fs = require("fs");
const Max = require("max-api");

const REPO_ROOT = path.resolve(__dirname, "..", "..");
const GOLDEN_DIR = path.join(REPO_ROOT, "conformance", "golden");
const NUM_SAMPLES = 4096;

// stem -> channel count. Keep in sync with conformance/patches/ and
// tools/gen_render_host.py.
const PATCHES = {
  cycle_440: 1,
  dcblock_step: 1,
  delay_echo: 3,
  history_counter: 2,
  phasor_incr_order: 1,
  range_inverted_bounds: 3,
  sah_latch: 2,
  slide_step: 1,
  triangle_duty: 3,
};

function bufferNames() {
  const names = [];
  for (const [stem, nCh] of Object.entries(PATCHES)) {
    for (let ch = 0; ch < nCh; ch++) names.push(`${stem}_ch${ch}`);
  }
  return names;
}

function wavPathFor(bufName) {
  // <stem>_ch<N>  ->  <stem>.ch<N>.wav
  const m = bufName.match(/^(.*)_ch(\d+)$/);
  return path.join(GOLDEN_DIR, `${m[1]}.ch${m[2]}.wav`);
}

async function sizeBuffers() {
  for (const name of bufferNames()) {
    await Max.outlet("buf", name, "sizeinsamps", NUM_SAMPLES);
  }
  Max.post(`render_runner: sized ${bufferNames().length} buffers to ${NUM_SAMPLES} samples`);
}

Max.addHandler("writewavs", async () => {
  if (!fs.existsSync(GOLDEN_DIR)) fs.mkdirSync(GOLDEN_DIR, { recursive: true });
  for (const name of bufferNames()) {
    const p = wavPathFor(name);
    await Max.outlet("buf", name, "writewave", p, "float32");
    Max.post(`render_runner: writewave ${p} float32`);
  }
  Max.post("render_runner: done. Verify with: cargo test -p opengen-analysis --test conformance");
});

(async () => {
  Max.post("=== GenExpr Conformance Render Runner v4 ===");
  Max.post(`goldens -> ${GOLDEN_DIR}`);
  await sizeBuffers();
  Max.post("render_runner: ready. Steps: DSP ON ~1s -> DSP OFF -> [writewavs]");
})();
