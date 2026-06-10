#!/usr/bin/env node
/**
 * render_runner.js — Drives gen~ conformance rendering in Max 9.
 *
 * Loads each conformance patch into gen~, records 4096 samples, and writes
 * golden WAV files to conformance/golden/<stem>.ch<N>.wav.
 *
 * Intended to run via Max's `node.script` object inside render_host.maxpat.
 * The maxpat must have:
 *   - gen~ object with varname "genpatcher" (receives @gen file / code messages)
 *   - record~ object with varname "recorder" (records into buffer~ "capture")
 *   - buffer~ named "capture" (size 4096, 1 channel)
 *
 * Usage in Max:
 *   1. Open render_host.maxpat in Max 9
 *   2. Turn on audio (DSP)
 *   3. Click the message or send "start" to node.script
 *
 * The script will iterate each .genexpr patch in conformance/patches/,
 * render 4096 samples, and save WAV(s) to conformance/golden/.
 *
 * Dependencies: none beyond Max's built-in Node for Max runtime.
 */

"use strict";

const fs = require("fs");
const path = require("path");
const Max = require("Max");

// ─── Configuration ────────────────────────────────────────────────────────────

// Resolve paths relative to the render_runner.js location
const REPO_ROOT = path.resolve(__dirname, "..", "..");
const PATCHES_DIR = path.join(REPO_ROOT, "conformance", "patches");
const GOLDEN_DIR = path.join(REPO_ROOT, "conformance", "golden");
const BUFFER_NAME = "capture";
const NUM_SAMPLES = 4096;
const SR = 48000;

// ─── Helpers ──────────────────────────────────────────────────────────────────

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * Get the maximum number of outputs for a gen~ patch by scanning for "outN"
 * identifiers. Falls back to 1 (the minimum: a patch must have at least out1).
 */
function countOutputs(code) {
  const regex = /out(\d+)\s*=/g;
  let max = 0;
  let match;
  while ((match = regex.exec(code)) !== null) {
    const n = parseInt(match[1], 10);
    if (n > max) max = n;
  }
  return Math.max(max, 1);
}

/**
 * Load a genexpr patch into gen~ by sending its content as a `code` message.
 * Uses Max's patcher scripting: Max.outlet() to send to the gen~ object.
 */
async function loadPatch(stem, code) {
  return new Promise((resolve, reject) => {
    try {
      // Send the genexpr code directly to the gen~ patcher.
      Max.outlet("genpatcher", "code", code);
      // Give gen~ time to compile (typically < 10 ms for simple patches)
      setTimeout(resolve, 100);
    } catch (e) {
      reject(e);
    }
  });
}

/**
 * Start recording into the buffer by sending a 1 (start) to record~.
 */
async function startRecording() {
  return new Promise((resolve) => {
    Max.outlet("recorder", 1);
    setTimeout(resolve, 10);
  });
}

/**
 * Stop recording by sending a 0 (stop) to record~.
 */
async function stopRecording() {
  return new Promise((resolve) => {
    Max.outlet("recorder", 0);
    setTimeout(resolve, 10);
  });
}

/**
 * Save the buffer content to a mono WAV file by sending a `write` message.
 * Format: "write <filepath>"
 */
async function writeBuffer(filePath) {
  return new Promise((resolve, reject) => {
    try {
      Max.outlet(BUFFER_NAME, "write", filePath);
      setTimeout(resolve, 50);
    } catch (e) {
      reject(e);
    }
  });
}

/**
 * Render a single patch: load, record N samples, save all output channels.
 * Each channel gets its own mono WAV file: <stem>.ch<N>.wav
 */
async function renderPatch(filePath) {
  const stem = path.basename(filePath, ".genexpr");
  const code = fs.readFileSync(filePath, "utf-8");
  const nOutputs = countOutputs(code);

  console.log(`  Rendering ${stem} (${nOutputs} output(s)) ...`);

  // Load the patch
  await loadPatch(stem, code);

  // Record
  await startRecording();
  const renderMs = Math.ceil((NUM_SAMPLES / SR) * 1000) + 50;
  await sleep(renderMs);
  await stopRecording();

  // Save each output channel
  for (let ch = 0; ch < nOutputs; ch++) {
    const wavPath = path.join(GOLDEN_DIR, `${stem}.ch${ch}.wav`);
    await writeBuffer(wavPath);
    console.log(`    → ${wavPath}`);
  }
}

// ─── Main ─────────────────────────────────────────────────────────────────────

async function main() {
  console.log("=== GenExpr Conformance Render Runner ===");
  console.log(`Patches:  ${PATCHES_DIR}`);
  console.log(`Goldens:  ${GOLDEN_DIR}`);
  console.log(`Samples:  ${NUM_SAMPLES} @ ${SR} Hz`);
  console.log("");

  // Ensure golden directory exists
  if (!fs.existsSync(GOLDEN_DIR)) {
    fs.mkdirSync(GOLDEN_DIR, { recursive: true });
  }

  // Collect patches
  const patches = fs.readdirSync(PATCHES_DIR)
    .filter(f => f.endsWith(".genexpr"))
    .sort();

  if (patches.length === 0) {
    console.log("WARN: No .genexpr patches found.");
    return;
  }

  console.log(`Found ${patches.length} patches\n`);

  for (const f of patches) {
    try {
      await renderPatch(path.join(PATCHES_DIR, f));
    } catch (e) {
      console.error(`  ERROR rendering ${f}: ${e.message || e}`);
    }
  }

  console.log("\nDone.");
}

main().catch(e => {
  console.error("Fatal:", e);
});
