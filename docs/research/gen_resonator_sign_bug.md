# gen_resonator.gendsp: sign bug in the b-coefficient pole-radius branch

**Status:** root-caused 2026-06-10 against opengen's deterministic engine;
**confirmed in real Max gen~ 2026-06-10** (human probe): as shipped, the
example outputs silence for the drive values below; with obj-30 changed to
`* -0.5` it resonates with a definite spectral peak at 440 Hz and peak
amplitude fluctuating around 1 under noise excitation — matching opengen's
predictions exactly. Pending: report to Cycling '74 (draft below).

**File:** `reference/gen/examples/gen_resonator.gendsp` (shipped Max example,
used by `gen~.resonator_bank_v2.maxpat`; credited to Graham Wakefield). Cited
by path and box id only — the file itself is never committed to this repo.

## Summary

The example implements a two-pole resonator
`y = a·x + b·y1 + c·y2` with coefficients derived from a bandwidth input
`bw` (in5) and a normalized radian frequency `ω` (in3). Its two
pole-radius branches are mathematically inconsistent:

| branch | boxes | computes | canonical form |
|---|---|---|---|
| c (−r²) | obj-45 `* -1` → obj-17 `exp` → obj-18 `* -1` | `c = −e^(−bw)` | `−r²` with `r = e^(−bw/2)` ✓ |
| b (2r·cosω) | obj-30 `* 0.5` → obj-46 `exp` → obj-13 `*` (with `cos`) → obj-14 `* 2` | `b = 2·cosω·e^(+bw/2)` | needs `e^(−bw/2)` ✗ |

`obj-30` multiplies by **+0.5** where the design requires **−0.5**.

## Pole analysis

Poles of `y = a·x + b·y1 + c·y2` satisfy `z² − bz − c = 0`; for complex
poles, radius `R = √(−c)` and angle `cosθ = b/(2R)`.

- **As shipped:** `R = e^(−bw/2)` (stable), but
  `cosθ = cosω·e^(+bw)` — the resonance is detuned **flat** of the
  requested ω by a factor that grows with bw (e.g. ω for 440 Hz at 48 kHz,
  bw = 6.5e-4 → peak ≈ 343 Hz, a 22 % error).
- **Input-kill:** the input coefficient is `a = 1 − min(b − e^(−bw), 1)`
  (obj-40 `min 1` summing two cords, obj-20 `!- 1`). For
  `2·cosω·e^(bw/2) − e^(−bw) ≥ 1`, i.e.
  `bw ≳ (2 − 2cosω)/(1 + cosω)` (≈ 0.00166 at ω for 440 Hz @ 48 kHz),
  `a` clamps to **exactly 0**: the filter receives no input and outputs
  **exact silence**.
- **Sign-fixed (obj-30 = `* -0.5`):** `cosθ = cosω` exactly — the resonator
  tunes to the requested ω for *any* bandwidth, and `a > 0` for all
  `bw > 0`. This invariance is clearly the design intent.

## Why the bug ships unnoticed

The host patcher (`gen~.resonator_bank_v2.maxpat`, codebox in the
`obj-1` subpatcher) pre-scales the bandwidth parameter to
`scale(bw, 0, 1, 0.5, 10, 2) * twopi/samplerate` ∈ [6.5e-5, 1.3e-3] at
48 kHz — below the silence threshold for most of its frequency range, so
the bank still makes sound; the resonances are merely mistuned flat (by an
amount that grows toward low frequencies / large bandwidths, where voices
can also go fully silent: threshold f ≈ 389 Hz at bw = 1.3e-3, 48 kHz).

## Evidence (opengen, deterministic)

- Stage-by-stage probe of the loaded graph: all latches/slides/gates
  converge correctly; `min1 = 1.0`, `a = 0.0` exactly; `y1 = y2 = out = 0`.
- Sign-fixed copy (in memory): impulse ringdown FFT peaks at **439.5 Hz**
  for a 440 Hz drive (32768-point FFT @ 48 kHz, bin spacing 1.46 Hz),
  stable, `a = 0.0033`.
- **Max confirmation (2026-06-10):** real gen~, patch driven per
  `conformance/CHECKLIST.md`: shipped → silence; sign-fixed → definite
  440 Hz peak, peakamp ≈ 1. Both opengen predictions confirmed.
- Regression tests:
  `exit_resonator_vendor_sign_bug_renders_silence` (faithful emulation,
  exact-zero assertion + upstream-fix tripwire on obj-30's text) and
  `exit_resonator_sign_fixed_peaks_at_drive_freq` (440 ± 10 Hz on the
  in-memory sign-fixed variant) in `crates/opengen-analysis/tests/m2_exit.rs`.

## Suggested upstream fix

Change obj-30's expression from `* 0.5` to `* -0.5` so that
`b = 2·cosω·e^(−bw/2)`, consistent with `c = −e^(−bw)`.

## Draft report text (for Cycling '74)

> The gen~ example `gen_resonator.gendsp` (used by
> `gen~.resonator_bank_v2.maxpat`) has a sign inconsistency in its
> resonator coefficients. The `c` coefficient computes `−e^(−bw)` = −r²
> (boxes obj-45/obj-17/obj-18), implying a pole radius r = e^(−bw/2), but
> the `b` coefficient computes `2·cos(ω)·e^(+bw/2)` (boxes
> obj-30/obj-46/obj-13/obj-14) — obj-30 multiplies by +0.5 instead of
> −0.5. Consequences: resonances are detuned flat of the requested
> frequency by a factor e^(bw) in the pole-angle cosine, and for
> bandwidths above ≈ (2 − 2cosω)/(1 + cosω) the input coefficient
> `a = 1 − min(b − r², 1)` clamps to exactly 0, silencing the voice
> entirely. Changing obj-30 to `* -0.5` makes the pole angle equal ω
> exactly for any bandwidth, which appears to be the design intent.
