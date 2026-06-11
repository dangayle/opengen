# Conformance Checklist — Human-in-the-Loop Rendering

This checklist documents the steps to produce authoritative golden WAV files
by rendering conformance patches through **real gen~ in Max 9**.

Until goldens land, the following operators remain `# Observed`-pending in
their rustdoc:

- **`phasor`** — increment-order / wrap semantics (`conformance/patches/phasor_incr_order.genexpr`)
- **`range_inverted_bounds`** — clip inverse-pin / wrap-fold-swap (`conformance/patches/range_inverted_bounds.genexpr`)

## ~~Pending~~ CONFIRMED probe: gen_resonator silence (vendor sign bug)

**Confirmed in real Max gen~ 2026-06-10:** as shipped → silence; with obj-30
changed to `* -0.5` → definite 440 Hz peak, peakamp ≈ 1 (noise-driven).
opengen's semantics match real gen~ on this patch. Remaining action: submit
the bug report to Cycling '74 (draft in the research doc).

opengen root-caused a sign bug in the shipped `gen_resonator.gendsp` example
(obj-30 `* 0.5` should be `* -0.5`; full analysis in
`docs/research/gen_resonator_sign_bug.md`). Prediction: **real gen~ also
outputs exact silence** when the patch is driven with in2 = 0, in3 = ω for
440 Hz (≈ 0.0576 rad @ 48 kHz), in4 = 1, in5 = 0.005, and any in1
excitation — because the input coefficient `a` clamps to 0.

Human step: load `gen_resonator.gendsp` in a gen~ host in Max, drive the
inputs with those constants plus noise on in1, and confirm silence. If Max
is NOT silent, our `?`/`min`/`!-`/multi-cord-summation semantics diverge
somewhere — file an opengen bug and re-open the resonator investigation
(`exit_resonator_vendor_sign_bug_renders_silence` pins current behavior).
Also report the patch bug to Cycling '74 (draft text in the research doc).

---

## Prerequisites

- **Max 9** installed (tested with Max 9.0+)
- **Audio enabled** (DSP turned on — any output device works)
- This repository checked out at the working directory

## Steps

### 1. Open Render Host

Open `conformance/render/render_host.maxpat` in Max 9.

The patcher contains:
- A `gen~` object (will load each conformance patch)
- A `gain~` for level control
- A `record~` object recording into a `buffer~`
- A `node.script` object that runs `tools/render_runner.js`

### 2. Enable Audio

Turn on DSP in Max (click the audio status in the toolbar, or press ⌘+.)

**Important:** gen~ will not compile patches without audio enabled.

### 3. Trigger the Runner

Click the `start` message box or send a bang to the `node.script` object.

This runs `tools/render_runner.js`, which for each patch in
`conformance/patches/`:

1. Sends the genexpr `code` to the gen~ object
2. Waits 100 ms for gen~ to compile
3. Starts recording for ~135 ms (≈ 4096 samples at 48 kHz)
4. Saves a mono WAV per output channel to `conformance/golden/<stem>.ch<N>.wav`

### 4. Verify Golden Files

After the runner completes, verify that WAV files appear in
`conformance/golden/`. Expected files (9 patches × N outputs):

| Patch | Outputs | Files |
|---|---|---|
| `phasor_incr_order` | 1 | `phasor_incr_order.ch0.wav` |
| `cycle_440` | 1 | `cycle_440.ch0.wav` |
| `history_counter` | 2 | `history_counter.ch0.wav`, `history_counter.ch1.wav` |
| `delay_echo` | 3 | `delay_echo.ch0.wav` … `.ch1.wav` … `.ch2.wav` |
| `slide_step` | 1 | `slide_step.ch0.wav` |
| `dcblock_step` | 1 | `dcblock_step.ch0.wav` |
| `range_inverted_bounds` | 3 | `range_inverted_bounds.ch0.wav` … `.ch1.wav` … `.ch2.wav` |
| `triangle_duty` | 3 | `triangle_duty.ch0.wav` … `.ch1.wav` … `.ch2.wav` |
| `sah_latch` | 2 | `sah_latch.ch0.wav`, `sah_latch.ch1.wav` |

**Total: 17 WAV files** (all mono, 32-bit float, 48 kHz, 4096 samples)

If some files are missing, check the Max window for error messages from
the runner script. Common issues:
- Audio not enabled → gen~ won't compile
- Node for Max not available → node.script won't start
- File permissions → buffer~ write may fail

### 5. Run Conformance Tests

With goldens in place, run the conformance test suite:

```sh
cargo test -p opengen-analysis --test conformance
```

Expected output: all tests PASS.

If a test fails, check per-patch tolerances in
`crates/opengen-analysis/tests/conformance.rs`.

### 6. Commit Goldens

```sh
git add conformance/golden/
git commit -m "chore(conformance): add Max-rendered golden WAVs"
```

### 7. Upgrade `# Observed` to `# Observed` (resolved)

After goldens are committed, the following operators' rustdoc can be updated
from `# Observed`-pending to `# Observed` (confirmed):

- `phasor` — increment order (cite `conformance/patches/phasor_incr_order.genexpr` + golden)
- `clip`/`wrap`/`fold` — inverted bounds (cite `conformance/patches/range_inverted_bounds.genexpr` + golden)

---

## Troubleshooting

### gen~ fails to load a patch

Check that the genexpr file is valid GenExpr by running:

```sh
bash tools/validate-with-genbo.sh
```

This validates all patches using Max's own genbo parser (no Max GUI needed).

### record~ doesn't capture

Make sure:
- The buffer~ is large enough (4096 samples)
- Audio is enabled
- gen~ is actually outputting signal (try connecting to ezdac~ directly)

### node.script not found

Max may need the `Node for Max` package installed separately. Verify with
Max's Package Manager.

The runner expects Max's bundled Node at:
`/Applications/Max.app/Contents/Resources/C74/packages/Node for Max/source/bin/osx/node/node`

---

## Background

This conformance harness is part of the opengen M2 milestone. The machine-
checkable genbo validation (`tools/validate-with-genbo.sh`) runs in CI (exits
0 when Max is absent). The golden WAV rendering is a human step because it
requires Max's gen~ DSP engine which cannot run in CI.
