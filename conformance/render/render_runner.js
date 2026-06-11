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
 *   2. "writewavs": float32 export. buffer~'s WAV writers (write/writewave)
 *      are int16-only with no format argument (verified against the
 *      buffer~ maxref; 'extra arguments' warning observed live). The ONLY
 *      format-controllable write is `writeraw <path> float32 <frames> <le>`
 *      (headerless). So: buffer~ writes raw float32 LE to a temp file, this
 *      script polls for it, wraps it in a 44-byte IEEE-float WAV header,
 *      and writes conformance/golden/<stem>.ch<N>.wav. float32 is REQUIRED:
 *      int16 quantization (~3e-5) breaks the comparator's 1e-6 tolerance
 *      AND clips counter values > 1.0 (observed, 2026-06-11).
 *
 * Message protocol out of node.script outlet 0 (dispatched in the patch):
 *   ("buf", <name>, "sizeinsamps", 4096)                       -> [route buf] -> [route <names>] -> buffer~
 *   ("buf", <name>, "writeraw", <tmppath>, "float32", 4096, 1) -> same path
 */

"use strict";

const path = require("path");
const fs = require("fs");
const Max = require("max-api");

const os = require("os");

const REPO_ROOT = path.resolve(__dirname, "..", "..");
const GOLDEN_DIR = path.join(REPO_ROOT, "conformance", "golden");
const NUM_SAMPLES = 4096;
const SR = 48000;
const RAW_BYTES = NUM_SAMPLES * 4;

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

function sleep(ms) {
  return new Promise((r) => setTimeout(r, ms));
}

/** Wrap raw float32 LE mono samples in a canonical IEEE-float WAV header. */
function rawToWav(rawPath, wavPath) {
  const data = fs.readFileSync(rawPath);
  const header = Buffer.alloc(44);
  header.write("RIFF", 0);
  header.writeUInt32LE(36 + data.length, 4);
  header.write("WAVE", 8);
  header.write("fmt ", 12);
  header.writeUInt32LE(16, 16);          // fmt chunk size
  header.writeUInt16LE(3, 20);           // audio format 3 = IEEE float
  header.writeUInt16LE(1, 22);           // mono
  header.writeUInt32LE(SR, 24);          // sample rate
  header.writeUInt32LE(SR * 4, 28);      // byte rate
  header.writeUInt16LE(4, 32);           // block align
  header.writeUInt16LE(32, 34);          // bits per sample
  header.write("data", 36);
  header.writeUInt32LE(data.length, 40);
  fs.writeFileSync(wavPath, Buffer.concat([header, data]));
  return data;
}

Max.addHandler("writewavs", async () => {
  if (!fs.existsSync(GOLDEN_DIR)) fs.mkdirSync(GOLDEN_DIR, { recursive: true });
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "opengen-golden-"));
  let ok = 0;
  for (const name of bufferNames()) {
    const rawPath = path.join(tmpDir, `${name}.f32`);
    await Max.outlet("buf", name, "writeraw", rawPath, "float32", NUM_SAMPLES, 1);
    // Poll until Max finishes writing the raw file.
    let tries = 0;
    while (tries < 100) {
      if (fs.existsSync(rawPath) && fs.statSync(rawPath).size >= RAW_BYTES) break;
      await sleep(50);
      tries++;
    }
    if (!fs.existsSync(rawPath)) {
      Max.post(`render_runner: ERROR — ${name}: writeraw produced no file`);
      continue;
    }
    const wavPath = wavPathFor(name);
    const data = rawToWav(rawPath, wavPath);
    fs.unlinkSync(rawPath);
    const v0 = data.readFloatLE(0);
    const v1 = data.readFloatLE(4);
    const v2 = data.readFloatLE(8);
    Max.post(`render_runner: ${path.basename(wavPath)} (${data.length / 4} samples, first: ${v0}, ${v1}, ${v2})`);
    ok++;
  }
  fs.rmSync(tmpDir, { recursive: true, force: true });
  Max.post(`render_runner: done (${ok}/${bufferNames().length}). Verify with: cargo test -p opengen-analysis --test conformance`);
});

(async () => {
  Max.post("=== GenExpr Conformance Render Runner v4 ===");
  Max.post(`goldens -> ${GOLDEN_DIR}`);
  await sizeBuffers();
  Max.post("render_runner: ready. Steps: DSP ON ~1s -> DSP OFF -> [writewavs]");
})();
