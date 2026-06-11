#!/usr/bin/env node
/**
 * render_runner.js — orchestrates gen~ conformance golden rendering (v2).
 *
 * Runs inside Max via `node.script` in render_host.maxpat (regenerate the
 * host with tools/gen_render_host.py). The host is fully static: one gen~
 * per conformance patch (loaded via @gen), one record~ + buffer~ pair per
 * output channel. This script only:
 *
 *   1. On start: sizes every capture buffer to exactly 4096 samples
 *      (`sizeinsamps` — buffer~ args are in ms, which cannot hit an exact
 *      sample count).
 *   2. "arm" / "disarm": broadcasts 1/0 to every record~ (arm with DSP OFF
 *      so recording starts at the first processed vector = patch t=0).
 *   3. "writewavs": sends each buffer~ a `write` message with the absolute
 *      path conformance/golden/<stem>.ch<N>.wav.
 *
 * Message protocol out of node.script outlet 0 (dispatched in the patch):
 *   ("rec", 1|0)                          -> [route rec buf] outlet 0 -> all record~
 *   ("buf", <name>, "sizeinsamps", 4096)  -> outlet 1 -> [route <names>] -> buffer~
 *   ("buf", <name>, "write", <abspath>)   -> same path
 *
 * No runtime code injection: vanilla gen~ has no `code` message (verified
 * against the gen~ help/reference, 2026-06-10).
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

Max.addHandler("arm", async () => {
  await Max.outlet("rec", 1);
  Max.post("render_runner: ARMED. Now turn DSP ON for ~1s, then OFF, then click [writewavs].");
});

Max.addHandler("disarm", async () => {
  await Max.outlet("rec", 0);
  Max.post("render_runner: disarmed.");
});

Max.addHandler("writewavs", async () => {
  if (!fs.existsSync(GOLDEN_DIR)) fs.mkdirSync(GOLDEN_DIR, { recursive: true });
  for (const name of bufferNames()) {
    const p = wavPathFor(name);
    await Max.outlet("buf", name, "write", p);
    Max.post(`render_runner: write ${p}`);
  }
  Max.post("render_runner: done. Verify with: cargo test -p opengen-analysis --test conformance");
});

(async () => {
  Max.post("=== GenExpr Conformance Render Runner v2 ===");
  Max.post(`goldens -> ${GOLDEN_DIR}`);
  await sizeBuffers();
  Max.post("render_runner: ready. Steps: [arm] (DSP off) -> DSP ON 1s -> DSP OFF -> [writewavs]");
})();
