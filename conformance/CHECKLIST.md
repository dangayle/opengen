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
opengen's semantics match real gen~ on this patch. Bug reported to the Max
community (Discord, 2026-06-10) with repro patch — see the research doc.
No further action unless upstream ships a fix (the tripwire in
`exit_resonator_vendor_sign_bug_renders_silence` will catch that).

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

## Steps (render kit v4 — rebuilt 2026-06-11)

> Kit history: v1 was never exercised and could not work (no start trigger,
> unwired node.script, a gen~ `code` message that does not exist). v2's
> `@gen <file>.genexpr` failed — gen~ cannot load `.genexpr` from box text
> (`.genexpr` only loads via `require()`). v3 embedded the sources as
> codeboxes (works) but used record~ armed before DSP-on for alignment —
> measured ~0.7 s late — and buffer~'s default `write` produced int16 WAVs
> (quantization breaks the 1e-6 tolerance; counter values > 1.0 clip).
> v4 captures INSIDE each gen~ patcher: every output channel is poked into
> its named buffer~ at index `elapsed` (samples since DSP began — the same
> t=0 as codebox History state, so alignment is correct by construction;
> out-of-range pokes are ignored, so capture self-terminates at 4096).
> WAVs are exported via `writeraw <path> float32 4096 1` + a JS-assembled
> IEEE-float header (buffer~'s WAV writers are int16-only — buffer~ maxref).
> Poke channel args are ZERO-based; channel 1 on a mono buffer is silently
> ignored (this produced an all-zero capture on 2026-06-11). Regenerate
> after adding/editing a patch: `python3 tools/gen_render_host.py`.

### 1. Open Render Host

Open `conformance/render/render_host.maxpat` in Max 9 and check the Max
console: **all 9 gen~ objects must compile with no errors**. Never save the
patch after a failed load: Max prunes patchcords from collapsed gen~
outlets.

The `node.script` autostarts (`@autostart 1`); the console should show
`render_runner: sized 17 buffers to 4096 samples`. If not, click the
`script start` message box.

### 2. Record

Turn DSP ON (ezdac~), wait about 1 second, turn DSP OFF. Capture runs
automatically during the first 4096 samples after DSP start.

### 3. Write the WAVs

Click the `writewavs` message box. The runner has each `buffer~` write raw
float32 to a temp file, wraps it in a WAV header, and saves
`conformance/golden/<stem>.ch<N>.wav` — logging each file with its first
three sample values (instant sanity check: `history_counter.ch1` should
start `0, 1, 2`).

To re-record, close and reopen the patch first (fresh gen~ state), then
repeat from step 2.

### 5. Verify Golden Files

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
- gen~ compile errors in the console → a codebox source didn't compile; the
  console names the offending expression — report it (possible GenExpr
  dialect gap in an authored patch)
- Node for Max not available → node.script won't start
- File permissions → buffer~ write may fail

**Bit-depth check:** the comparator's 1e-6 tolerance requires float32 WAVs.
The runner's `writeraw float32` + JS WAV assembly guarantees this; if
`cargo test` fails with uniform ~1e-5 diffs something regressed in the
export path. (History: `write`/`writewave` are int16-only — quantized AND
clipped counter channels at 1.0 on the first render attempts, 2026-06-11.)

**Alignment check is automatic:** `history_counter.ch1` must read 0,1,2,…
and `cycle`/`phasor` must start at exactly 0. v4's poke-at-elapsed capture
makes this hold by construction; if it fails on sample 0, gen~ state and
`elapsed` did not share t=0 (e.g. a re-run without reopening the patch).

### 6. Run Conformance Tests

With goldens in place, run the conformance test suite:

```sh
cargo test -p opengen-analysis --test conformance
```

Expected output: all tests PASS.

If a test fails, check per-patch tolerances in
`crates/opengen-analysis/tests/conformance.rs`.

### 7. Commit Goldens

```sh
git add conformance/golden/
git commit -m "chore(conformance): add Max-rendered golden WAVs"
```

### 8. Upgrade `# Observed` to `# Observed` (resolved)

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
