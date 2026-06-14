# GenExpr Language Reference

**For gen~, gen, jit.gen, and jit.pix in Max/MSP**

This document is a complete language reference for GenExpr, the textual programming language used inside Max/MSP Gen patchers. It is intended to enable accurate translation of algorithms (DSP, signal processing, filters, synthesis) into GenExpr code.

Sources: Extracted from local Max installation (`@rnbo/genexpr_js` package), official Cycling '74 userguide JSON, and `genexpr.pegjs` (the PEG grammar).

---

## 1. Execution Model

- A Gen patcher (and its GenExpr code) is compiled to **native machine code** (C++) at load time.
- All computation runs at **sample rate**. Every expression executes once per audio sample.
- There is **no heap allocation**, no dynamic memory, no garbage collection.
- Everything is a **64-bit float** unless explicitly converted with `int()`.
- The patcher is a **directed graph**. Feedback is only possible through explicit state (`History`, `Delay`).
- gen~ inputs arrive as `in1`, `in2`, etc. Outputs are written to `out1`, `out2`, etc.

### Domains

| Domain   | Object    | Rate            | Notes                          |
|----------|-----------|-----------------|--------------------------------|
| `dsp`    | gen~      | Audio sample rate | Primary domain for this ref   |
| `common` | gen~/gen  | Both            | Works in all Gen contexts      |
| `dsp`    | pfft~     | FFT frame rate  | Adds `FFTSIZE`, `FFTHOP`, etc. |

---

## 2. File Structure

A GenExpr codebox (translation unit) has this structure. **Order matters:**

```
[compiler commands]      // require "file"  (optional)
[function declarations]  // must come before declarations
[declarations]           // History, Delay, Data, Param, Buffer
[body statements]        // expressions, assignments, control flow
```

A minimal valid codebox (single expression, no semicolon needed):

```
in1 * 0.5
```

Equivalent explicit form:

```
out1 = in1 * 0.5;
```

---

## 3. Syntax Basics

### Comments

```genexpr
// single line comment

/* multi-line
   comment */
```

### Semicolons

Semicolons are **required** only when there are multiple statements. A single expression needs no semicolon. Inside function bodies and control flow blocks, semicolons are always required.

### Identifiers

- Must start with a letter or `_`
- Case-sensitive
- Reserved: `return`, `continue`, `break`, `true`, `false`, `null`

### Literals

```genexpr
42          // integer
3.14        // float
0.5e-3      // float with exponent
0xFF        // hex integer
true        // bool (1.0)
false       // bool (0.0)
null        // null (0.0)
"linear"    // string (for attribute values only)
```

### Types

GenExpr is **typeless**. All values are 64-bit float. Type context is inferred by the compiler. The `int()` and `float()` functions force integer or float interpretation. Declared state types (`History`, `Delay`, etc.) are special objects, not value types.

---

## 4. Variables & Declarations

### Implicit Variables

Any identifier assigned without a declaration keyword becomes an **implicit local variable** (scoped to its block):

```genexpr
x = in1 * 2;       // implicit local
y = x + in2;
out1 = y;
```

### Typed Declarations (State Objects)

State objects must be declared before use and are initialized once at load. They persist across samples.

```genexpr
History name(initial_value);
Delay   name(max_size_samples);
Data    name(channels, size);
Param   name(default, min=lo, max=hi);
```

Multiple declarations on one line:

```genexpr
History x(0), y(0), z(0);
```

---

## 5. State Objects

### History: Single-Sample Delay (z⁻¹)

`History` stores a single value that persists one sample. It is the fundamental feedback primitive (equivalent to the unit delay z⁻¹ in DSP).

```genexpr
History prev(0);      // initial value = 0
out1 = prev;          // read history (previous sample's value)
prev = in1;           // write history (takes effect next sample)
```

**Key rule**: In gen~, reading a `History` returns the *previous* sample's value. Writing sets the *next* sample's value. You can read before writing.

Example: one-pole lowpass filter:

```genexpr
History y(0);
y = y + 0.1 * (in1 - y);
out1 = y;
```

### Delay: Variable-Length Delay Line

`Delay` creates a sample buffer for delay lines. Read before write (standard feedback pattern).

```genexpr
Delay myDelay(samplerate);              // max delay = 1 second
tap = myDelay.read(delayTimeInSamples); // read (with optional interp attribute)
myDelay.write(in1);                     // write current sample
out1 = tap;
```

Read attributes:
- `interp`: `"none"`, `"linear"` (default), `"cubic"`, `"spline"`

```genexpr
Delay d(44100);
tap = d.read(100, interp="linear");
d.write(in1);
```

Named delay lines can share a single buffer:

```genexpr
delay myLine(size=samplerate, name="shared");
```

### Data: Fixed Sample Buffer (local array)

`Data` creates a locally-stored array of 64-bit samples.

```genexpr
Data myBuf(1, 512);   // 1 channel, 512 samples
```

Read/write with `peek` and `poke` (no interpolation) or `sample` (interpolated):

```genexpr
val = peek(myBuf, index);          // read by sample index
poke(myBuf, value, index);         // write
val = sample(myBuf, phase);        // read by phase (0..1), linear interp
```

### Buffer: Reference to External buffer~

`Buffer` references a named `buffer~` object in the parent Max patcher.

```genexpr
Buffer buf("myBufferName");
val = peek(buf, index);
```

Or via the `buffer` operator with attributes:

```genexpr
buffer(name="myBuffer", buffername="parentBufferName");
```

### Param: Externally Controllable Parameter

`Param` creates a named parameter that can be set from the parent Max patcher via message or automation.

```genexpr
Param freq(440, min=20, max=20000);
Param gain(0.5, min=0, max=1);
```

Parameters appear as inlets on the gen~ object in the parent patcher. Their value is a constant within any single sample computation.

---

## 6. Expressions & Operators

### Arithmetic

| Operator | Alias | Description |
|----------|-------|-------------|
| `+`      | `add` | Add |
| `-`      | `sub` | Subtract |
| `*`      | `mul` | Multiply |
| `/`      | `div` | Divide (safe: returns 0 if divisor is 0) |
| `%`      | `mod` | Modulo |
| `!-`     | `rsub` | Reverse subtract: `in2 - in1` |
| `!/`     | `rdiv` | Reverse divide: `in2 / in1` |
| `!%`     | `rmod` | Reverse modulo: `mod(in2, in1)` |

### Comparison

| Operator | Alias  | Description |
|----------|--------|-------------|
| `==`     | `eq`   | Equal (returns 0 or 1) |
| `!=`     | `neq`  | Not equal |
| `<`      | `lt`   | Less than |
| `>`      | `gt`   | Greater than |
| `<=`     | `lte`  | Less than or equal |
| `>=`     | `gte`  | Greater than or equal |
| `==p`    | `eqp`  | Pass-equal: returns `in1` if equal, else 0 |
| `!=p`    | `neqp` | Pass-not-equal |
| `<p`     | `ltp`  | Pass-less-than |
| `>p`     | `gtp`  | Pass-greater-than |
| `<=p`    | `ltep` | Pass-less-than-or-equal |
| `>=p`    | `gtep` | Pass-greater-than-or-equal |

### Logical Operators

| Operator | Alias | Description |
|----------|-------|-------------|
| `&&`     | `and` | Logical AND |
| `\|\|`   | `or`  | Logical OR |
| `^^`     | `xor` | Logical XOR |
| `!`      | `not` | Logical NOT |

### Assignment Operators

```genexpr
x = expr       // simple assignment
x += expr      // add-assign
x -= expr      // subtract-assign
x *= expr      // multiply-assign
x /= expr      // divide-assign
x %= expr      // modulo-assign
```

Note: `+=` and `*=` are also operator aliases for `plusequals` and `mulequals` accumulators (stateful), not just syntactic sugar. In expression context they behave as expected.

### Ternary Operator

```genexpr
condition ? value_if_true : value_if_false
```

### Operator Precedence (high to low binding)

| Precedence | Operators |
|-----------|-----------|
| 1 (highest) | `*`, `/`, `%` |
| 2 | `+`, `-` |
| 3 | `<`, `>`, `<=`, `>=`, `<p`, `>p`, `<=p`, `>=p` |
| 4 | `==`, `!=`, `==p`, `!=p` |
| 5 | `&&` |
| 6 | `^^` |
| 7 (lowest) | `\|\|` |

---

## 7. Multiple Return Values

Functions and some operators can return multiple values. Destructuring uses comma-separated left-hand side:

```genexpr
r, theta = cartopol(x, y);   // r = magnitude, theta = angle
a, b = someFunction(x);
```

Rules:
- Extra LHS variables get `0` if RHS has fewer values
- Extra RHS values are ignored if LHS has fewer variables
- In a comma-separated RHS list, only the *last* item expands multi-values

```genexpr
out1, out2 = cartopol(in1, in2);   // two outputs from one operator
```

---

## 8. Functions

Functions must be declared before all other code (before declarations and body).

```genexpr
myFunc(a, b, c)
{
    History state(0);      // local state, private to this function
    result = a * b + c;
    state = result;
    return result;
}

// Parameters can have defaults:
myFilter(input, freq=1000, Q=1.0)
{
    // ...
    return output;
}
```

### Calling Functions

```genexpr
out1 = myFunc(in1, in2, in3);
out1, out2 = myFilter(in1);              // destructure multiple returns
out1 = myFilter(in1, freq=440, Q=2.0);  // named (attribute-style) arguments
```

### Requiring External GenExpr Files

```genexpr
require "msplib_biquad"   // loads msplib_biquad.genexpr from search path
// or:
require("msplib_adsr");
```

After requiring, the functions defined in that file are available.

---

## 9. Control Flow

### if / else if / else

```genexpr
if (condition) {
    // ...
} else if (other) {
    // ...
} else {
    // ...
}
```

Single-statement bodies don't need braces:

```genexpr
if (x > 0) out1 = x;
```

### for Loop

```genexpr
for (i = 0; i < 8; i += 1) {
    // ...
}
```

Note: loop variables are floats. Use a fudge factor if needed (`i < 8.0001`).

### while Loop

```genexpr
while (condition) {
    // ...
}
```

### do-while Loop

```genexpr
do {
    // ...
} while (condition);
```

### break / continue / return

```genexpr
break;      // exit loop
continue;   // skip to next iteration
return x;   // return from function (single value)
return x, y; // return multiple values
return;     // void return
```

**Important**: Infinite loops are detected and terminated by gen~, but they will stall processing. All loops must be designed to terminate.

---

## 10. Operator Attributes

Many gen~ operators take attribute arguments that configure their behavior. Attributes use `@name value` syntax in the visual patcher, or `name="value"` in function call style within GenExpr:

```genexpr
// Visual patcher style (in codebox, object declaration):
Delay d(samplerate, @interp "cubic");

// Call/read style:
tap = d.read(time, interp="cubic");

// In sample/peek/poke:
val = sample(buf, phase, interp="linear", boundmode="wrap");
```

Attribute values are typically strings or numbers depending on context.

---

## 11. Send / Receive

Named signals can be sent and received anywhere within a gen patcher (not across patcher boundaries):

```genexpr
send(sig, name="mySignal");   // send
sig = receive(name="mySignal"); // receive
// aliases:
s(sig, name="mySignal");
sig = r(name="mySignal");
```

---

## 12. Subpatchers and Abstractions

Gen subpatchers allow composing multiple gen objects hierarchically. A `gen` box object in a patcher creates an inline subpatcher; loading a saved `.gendsp` file as an abstraction is equivalent.

**Creating subpatchers:**

```
gen                        // inline subpatcher (empty, edit by double-clicking)
gen @gen example.filter    // loads example.filter.gendsp as an abstraction
gen @file example.filter   // identical, @file and @gen are aliases
example.filter             // shorthand: object name = file name (no @gen needed)
```

The `@title` attribute sets the titlebar display name only and has no functional effect.

File extensions: `.gendsp` for gen~/gen abstractions; `.genjit` for Jitter gen abstractions.

**Signal inlets:**

The parent patcher's `in 1`, `in 2`, etc. feed directly into the child's corresponding `in 1`, `in 2`, etc. No explicit connection to the `gen` box is required. Audio inlets propagate implicitly.

**Parameters across subpatcher boundaries:**

`Param` objects inside a subpatcher act as named inlets with defaults. If nothing drives them from the parent, they are constants. The `setparam` box object in the parent connects a signal to a named param inside the child.

```
// Parent patcher:
//   param foo 200  →  setparam subfoo  →  gen
//
// Child subpatcher (inline or .gendsp abstraction):
//   param subfoo        ← driven by parent's setparam subfoo
//   in 1  →  * subfoo  →  out 1
```

The parent `param` name (`foo`) and the `setparam` target name (`subfoo`) are **independent**. `setparam` specifies the child's internal `Param` name, which need not match the parent's external param name.

**Parameter scoping:** Setting `@foo 10` as an attribute on the parent `gen~` object in Max only sets top-level params. It does not propagate into nested subpatchers.

**Visual patcher note:** `gen` and `setparam` are visual box objects used in `.gendsp` patchers. They are not GenExpr expressions. From within a codebox, the mechanism for invoking an abstraction is different. See below.

### Calling Abstractions from a Codebox

A saved `.gendsp` abstraction can be called directly from GenExpr code using its filename as a function name. The file must be on a Max searchable path and its name must be a valid GenExpr identifier (letters, digits, and underscores only; starts with a letter; no dots or spaces).

```genexpr
// myfilter.gendsp on the search path:
out1 = myfilter(in1);

// Multiple outputs via destructuring (maps to out 1, out 2, ... inside the abstraction):
lp, hp, bp = mysvf(in1);

// Named parameter passing (drives Param objects inside the abstraction by name):
out1 = myfilter(in1, cutoff=1000, q=2);
```

Call arguments map positionally to `in 1`, `in 2`, etc. inside the abstraction. Named arguments (`name=value`) drive `Param` objects by name. Return values come from `out 1`, `out 2`, etc.

State inside the abstraction (`History`, `Delay`, `phasor`, `cycle`, etc.) is **per call site**, not per abstraction file. Two calls to `myfilter(...)` at different points in the same codebox each maintain independent state, exactly like `History` declared inside a user-defined function.

---

## 13. Inlets and Outlets

```genexpr
in1, in2, in3, ...    // implicit inlet references (no declaration needed)
out1 = value;          // write to outlet 1
out2 = value;          // write to outlet 2
```

Or using the explicit operator form:

```genexpr
out(value, index=2);   // explicit index
in(index=3);           // explicit inlet reference
```

Inlet/outlet indices are 1-based.

---

## 14. Complete Operator Reference

All 155 operators, grouped by category. `[dsp]` = gen~ only; `[common]` = all Gen domains.

### BUFFER / DATA

| Operator | Domain | Description | Key Attributes |
|----------|--------|-------------|----------------|
| `buffer` | dsp | Reference to external buffer~ | `name`, `buffername` |
| `data` | dsp | Local sample array | channels, dim, name |
| `channels` | dsp | Number of channels of a data/buffer |   |
| `dim` | dsp | Length (samples) of data/buffer |   |
| `cycle` | dsp | Sine/wavetable oscillator | `name`, `index` (freq/phase) |
| `lookup` | dsp | Waveshaping buffer read | `index`, `interp`, `boundmode`, `channelmode` |
| `nearest` | dsp | Multi-channel buffer read, no interp | `index`, `boundmode`, `channelmode` |
| `peek` | dsp | Read from data/buffer (no interp) | `index`, `boundmode`, `channelmode` |
| `poke` | dsp | Write to data/buffer | `index`, `boundmode`, `overdubmode` |
| `sample` | dsp | Linear-interp multi-channel read | `index`, `interp`, `boundmode`, `channelmode` |
| `splat` | dsp | Write with linear-interp overdub | `index`, `boundmode`, `overdubmode` |
| `wave` | dsp | Wavetable synthesis from buffer | `index`, `interp`, `boundmode`, `channelmode` |

**Buffer index modes** (`@index`): `phase` (0..1), `samples` (integer index), `lookup` (-1..1 for waveshaping), `wave` (phase with start/end range)

**Bound modes** (`@boundmode`): `clamp`, `wrap`, `ignore`

**Interp modes** (`@interp`): `none`, `linear`, `cubic`, `spline`

### COMPARISON

| Operator | Alias | Description |
|----------|-------|-------------|
| `eq` | `==` | Equal |
| `neq` | `!=` | Not equal |
| `lt` | `<` | Less than |
| `gt` | `>` | Greater than |
| `lte` | `<=` | Less than or equal |
| `gte` | `>=` | Greater than or equal |
| `ltp` | `<p` | Pass less than (returns in1 or 0) |
| `gtp` | `>p` | Pass greater than |
| `ltep` | `<=p` | Pass less than or equal |
| `gtep` | `>=p` | Pass greater than or equal |
| `eqp` | `==p` | Pass equal |
| `neqp` | `!=p` | Pass not equal |
| `max` | `maximum` | Maximum of two inputs |
| `min` | `minimum` | Minimum of two inputs |
| `step` |   | `not(in1 < in2)`, Heaviside step function |

### CONSTANTS

| Operator | Value | Notes |
|----------|-------|-------|
| `pi` / `PI` | 3.14159… | |
| `twopi` / `TWOPI` | 2π | |
| `halfpi` / `HALFPI` | π/2 | |
| `invpi` / `INVPI` | 1/π | |
| `e` / `E` | 2.71828… | |
| `ln2` / `LN2` | 0.69314… | |
| `ln10` | 2.30258… | |
| `log2e` / `LOG2E` | 1.44269… | |
| `log10e` / `LOG10E` | 0.43429… | |
| `sqrt2` / `SQRT2` | 1.41421… | |
| `sqrt1_2` / `SQRT1_2` | 0.70710… | |
| `phi` / `PHI` | 1.61803… | Golden ratio |
| `degtorad` / `DEGTORAD` | π/180 | |
| `radtodeg` / `RADTODEG` | 180/π | |
| `SAMPLERATE` / `samplerate` |   | Current sample rate (dsp only) |
| `VECTORSIZE` / `vectorsize` |   | DSP vector size (dsp only) |
| `FFTSIZE` / `fftsize` |   | FFT frame size (pfft~ only) |
| `FFTHOP` / `ffthop` |   | FFT hop size (pfft~ only) |
| `FFTOFFSET` / `fftoffset` |   | FFT bin offset (pfft~ only) |
| `FFTFULLSPECT` / `fftfullspect` |   | Full spectrum flag (pfft~ only) |
| `constant` |   | Constant value (`@value n`) |

### CONVERT (dsp only)

| Operator | Description |
|----------|-------------|
| `atodb` | Linear amplitude → dB |
| `dbtoa` | dB → linear amplitude |
| `mtof(note, tuning=440)` | MIDI note → Hz |
| `ftom(freq, tuning=440)` | Hz → MIDI note |
| `mstosamps(ms)` | Milliseconds → samples |
| `sampstoms(samples)` | Samples → milliseconds |

### DECLARE

| Operator | Description | Attributes |
|----------|-------------|------------|
| `param` / `Param` | Externally controllable parameter | `name`, `default`, `min`, `max` |

### DSP UTILITIES (dsp only)

| Operator | Description |
|----------|-------------|
| `fixdenorm(x)` | Replace denormal values with 0 |
| `fixnan(x)` | Replace NaN with 0 |
| `isdenorm(x)` | Returns 1 if denormal, else 0 |
| `isnan(x)` | Returns 1 if NaN, else 0 |
| `t60(time_samples)` | Sample-rate multiplier for 60 dB decay in N samples |
| `t60time(multiplier)` | Inverse of t60 |

### FEEDBACK / STATE (dsp only)

| Operator | Description | Attributes |
|----------|-------------|------------|
| `history` / `History` | Single-sample delay (z⁻¹) | `name`, `value` |
| `delay` / `Delay` | Variable delay line | `name`, `interp`, `feedback` |
| `read` | Read from delay line: `delayObj.read(time)` | `interp` |
| `write` | Write to delay line: `delayObj.write(value)` |   |

### FILTER / SIGNAL PROCESSING (dsp only)

| Operator | Description | Attributes |
|----------|-------------|------------|
| `change(x)` | Sign of derivative: -1, 0, +1 | `init` |
| `delta(x)` | Discrete derivative (x - prev_x) | `init` |
| `dcblock(x)` | DC blocking filter |   |
| `interp(t, a, b, c, d)` | Interpolate inputs | `mode` |
| `latch(input, control)` | Sample and hold (gate) | `init` |
| `phasewrap(x)` | Wrap to -π..+π |   |
| `sah(input, control, thresh)` | Sample-and-hold with Schmitt trigger | `init` |
| `slide(input, up, down)` | Logarithmic slew/portamento filter | `init` |

### GLOBAL (dsp only)

| Operator | Description |
|----------|-------------|
| `elapsed` | Samples since load/reset |
| `voice` | Voice index (in poly~) |

### INPUT / OUTPUT

| Operator | Description | Attributes |
|----------|-------------|------------|
| `in` | Gen patcher input | `index`, `min`, `max`, `comment` |
| `out(value)` | Gen patcher output | `index` |
| `pass(x)` | Pass-through (identity) |   |

### INTEGRATORS (dsp only)

| Operator | Alias | Description | Attributes |
|----------|-------|-------------|------------|
| `counter(max, reset, incr)` |   | Sample-rate counter | `init` |
| `plusequals(incr, reset)` | `+=`, `accum` | Additive accumulator | `init`, `min`, `max`, `resetmode` |
| `mulequals(incr, reset)` | `*=` | Multiplicative accumulator | `init`, `min`, `max`, `resetmode` |

### LOGIC

| Operator | Alias | Description |
|----------|-------|-------------|
| `and(a, b)` | `&&` | Logical AND |
| `or(a, b)` | `\|\|` | Logical OR |
| `xor(a, b)` | `^^` | Logical XOR |
| `not(a)` | `!` | Logical NOT |
| `bool(x)` |   | Convert to boolean (0 or 1) |

### MATH

| Operator | Alias | Description |
|----------|-------|-------------|
| `add(a, b)` | `+` | Add |
| `sub(a, b)` | `-` | Subtract |
| `mul(a, b)` | `*` | Multiply |
| `div(a, b)` | `/` | Divide (0 if divisor is 0) |
| `mod(a, b)` | `%` | Modulo |
| `neg(a)` | unary `-` | Negate |
| `rsub(a, b)` | `!-` | b - a |
| `rdiv(a, b)` | `!/` | b / a |
| `rmod(a, b)` | `!%` | b % a |
| `absdiff(a, b)` |   | `abs(a - b)` |
| `cartopol(x, y)` |   | Cartesian → polar (returns r, theta) |
| `poltocar(r, theta)` |   | Polar → Cartesian (returns x, y) |

### NUMERIC

| Operator | Description |
|----------|-------------|
| `abs(x)` | Absolute value |
| `ceil(x)` | Round up to integer |
| `floor(x)` | Round down (toward -∞) |
| `trunc(x)` | Round toward zero |
| `fract(x)` | Fractional part |
| `round(x, base=1)` | Round to nearest (or multiple of base) |
| `sign(x)` | Sign: -1, 0, or +1 |

### POWERS / LOGARITHMS

| Operator | Alias | Description |
|----------|-------|-------------|
| `pow(x, y)` |   | xʸ |
| `sqrt(x)` |   | √x |
| `exp(x)` |   | eˣ |
| `exp2(x)` |   | 2ˣ |
| `log(x)` | `ln` | Natural log |
| `log2(x)` |   | Log base 2 |
| `log10(x)` |   | Log base 10 |
| `fastexp(x)` |   | Approximated eˣ |
| `fastpow(x, y)` |   | Approximated xʸ |

### RANGE / MAPPING

| Operator | Description |
|----------|-------------|
| `clamp(x, lo, hi)` / `clip` | Clamp to [lo, hi] |
| `fold(x, lo, hi)` | Fold out-of-range values back |
| `wrap(x, lo, hi)` | Wrap modulo-style |
| `scale(x, ilo, ihi, olo, ohi, exp=1)` | Map input range to output range |

### ROUTING

| Operator | Description | Attributes |
|----------|-------------|------------|
| `mix(a, b, t)` | Linear crossfade: `a + t*(b-a)` |   |
| `smoothstep(a, b, t)` | Smooth crossfade |   |
| `gate(input, choose)` | Route input to one of N outputs | `choices` |
| `selector(choose, ...)` | Select one of N inputs | `choices` |
| `switch(cond, t, f)` | Ternary: alias for `?` |   |
| `send(v)` / `s` | Named signal send | `name` |
| `receive` / `r` | Named signal receive | `name` |

### SUBPATCHERS

| Operator | Description | Attributes |
|----------|-------------|------------|
| `gen` | Inline subpatcher or abstraction (visual box). Signal inlets feed into the child's `in 1`, `in 2`, etc. From a codebox, abstractions are called by filename instead. See Section 12. | `@gen` / `@file`: abstraction filename (interchangeable); `@title`: display title only |
| `setparam(v)` | Drive a named `Param` inside a child subpatcher from the parent. Target name must match the child's `Param` name exactly, independent of any parent `param` name. | `@name` (required): child param name to target |
| `expr` / `codebox` | Inline GenExpr expression | `@code` |

### TRIGONOMETRY

| Operator | Description |
|----------|-------------|
| `sin(x)` | Sine (radians) |
| `cos(x)` | Cosine (radians) |
| `tan(x)` | Tangent (radians) |
| `asin(x)` | Arc sine |
| `acos(x)` | Arc cosine |
| `atan(x)` | Arc tangent |
| `atan2(y, x)` | Arc tangent of y/x |
| `sinh(x)` | Hyperbolic sine |
| `cosh(x)` | Hyperbolic cosine |
| `tanh(x)` | Hyperbolic tangent |
| `asinh(x)` | Inverse hyperbolic sine |
| `acosh(x)` | Inverse hyperbolic cosine |
| `atanh(x)` | Inverse hyperbolic tangent |
| `hypot(x, y)` | √(x²+y²) |
| `fastsin(x)` | Approximated sine |
| `fastcos(x)` | Approximated cosine |
| `fasttan(x)` | Approximated tangent |
| `radians(deg)` | Degrees → radians |
| `degrees(rad)` | Radians → degrees |

### WAVEFORMS / GENERATORS (dsp only)

| Operator | Description | Attributes |
|----------|-------------|------------|
| `noise` | White noise (random float) |   |
| `phasor(freq, reset=0)` | Sawtooth (0..1 ramp) | `phase` |
| `rate(multiplier, phase)` | Time-scale a phasor | `sync` |
| `train(period, width=0.5, onset=0)` | Pulse train | `phase` |
| `triangle(phase, duty=0.5)` | Triangle/ramp |   |
| `cycle(freq, phase=0)` | Sine oscillator / wavetable | `name`, `index` |

---

## 15. Composition Patterns and Constraints

These patterns illustrate how gen~'s state primitives and language features compose. They are not recipes. They show *why* certain constructs exist and how they interact, including non-obvious constraints that arise from the sample-rate execution model.

GenExpr has three levels of code modularity, each with the same per-call-site state semantics:

1. **User-defined functions**: defined in the same codebox, called by name
2. **External abstractions**: saved as `.gendsp` files, called by filename
3. **Visual subpatchers**: `gen` box objects in the `.gendsp` patcher, wired with patchcords

### Feedback with History

`History` reads the value written in the *previous* sample. This one-sample delay is what makes feedback loops possible without a circular dependency within a single sample.

```genexpr
History fb(0);
out1 = in1 + fb * 0.5;   // read: returns last sample's written value
fb = out1;                // write: available to read next sample
```

The read always precedes the write conceptually. Even if the lines appear in any order, gen~ enforces the previous-sample semantics.

### History Inside a Function

`History` declared inside a function is *per-call-site* state, not per-function. Each invocation of the function in the same patch maintains its own independent state register.

```genexpr
onepole(input, freq)
{
    History y(0);
    coeff = clamp(sin(twopi * freq / samplerate), 0.00001, 0.99999);
    y = mix(y, input, coeff);
    return y;
}

// These two calls each have their own independent y register:
smoothed_a = onepole(in1, 10);
smoothed_b = onepole(in2, 200);
```

### Calling an External Abstraction

A `.gendsp` file on the Max search path can be called like a function using its filename (without extension). The filename must be a valid GenExpr identifier. State inside the abstraction is per call site, identical to the `History`-inside-a-function rule above.

```genexpr
// mysvf.gendsp defines a state variable filter with Param cutoff and q.
// in 1 maps to the first call argument; out 1, out 2, out 3 map to return values.
lp, hp, bp = mysvf(in1, cutoff=1200, q=3);

// Two independent calls each maintain their own internal History/Delay state:
lp_a, hp_a, bp_a = mysvf(in1, cutoff=800,  q=2);
lp_b, hp_b, bp_b = mysvf(in2, cutoff=3000, q=5);
```

Named arguments drive `Param` objects inside the abstraction by name. Positional arguments map to `in 1`, `in 2`, etc. in order.

### Delay Line (read-before-write ordering)

`Delay` must be read before it is written within the same sample. Reading after writing returns the just-written value rather than the delayed signal, which is almost never correct.

```genexpr
Delay d(44100);
tap = d.read(delayTimeSamples);   // read first
d.write(in1);                     // write after
out1 = tap;
```

The maximum delay size (in samples) is fixed at compile time as the `Delay` constructor argument. Runtime read times must be ≤ that maximum.

### Phasor: Manual vs. Built-in

The `wrap()` + `History` pattern shows how a phasor accumulates phase across samples. This is functionally equivalent to the built-in `phasor()` operator, but composable. You can insert modulation, reset logic, or non-linear mapping into the accumulation step.

```genexpr
// Manual, composable
History phase(0);
phase = wrap(phase + in1/samplerate, 0, 1);
out1 = phase;

// Built-in, idiomatic shorthand
out1 = phasor(in1);
```

### Multiple Return Values and Destructuring

Functions can return multiple values, and built-in operators like `cartopol`/`poltocar` use this convention. Assignment destructuring works on both sides.

```genexpr
// Destructuring a built-in that returns multiple values
r, theta = cartopol(x, y);
x, y = poltocar(r, theta);

// User function returning multiple values
svf_outputs(input, freq, Q)
{
    // ... filter state ...
    return lowpass, highpass, bandpass;
}
lp, hp, bp = svf_outputs(in1, 1000, 2);
```

Unneeded return values can be ignored by assigning to a throwaway variable; there is no `_` discard syntax.

### send/receive Across Codeboxes

`send` and `receive` pass signals between separate codebox objects within the same gen~ patch. The `name` attribute is the only coupling between sender and receiver. There is no explicit wiring in the patcher.

```genexpr
// in one codebox:
send(mySignal, name="carrier");

// in another codebox, order of evaluation relative to the sender is not guaranteed:
sig = receive(name="carrier");
```

Use this for routing signals without visual patchcord clutter, or when the signal source and destination are in logically separate codebox modules.

### All Values Are Float

Every variable and expression is a 64-bit float. There is no integer type. Operations that require integer semantics (array indices, loop counters) must be explicitly converted.

```genexpr
// Array indexing truncates, but being explicit avoids ambiguity
idx = trunc(phase * 1024);
val = peek(table, idx);
```

### Float Loop Counters

Because loop counters are floats, floating-point rounding can cause `i < N` to fail on the last iteration when `i` accumulates to something like `7.9999...` instead of `8.0`. Use a small epsilon in the upper bound for fixed-count loops.

```genexpr
// May silently skip the last iteration
for (i = 0; i < 8; i += 1) { ... }

// Safe
for (i = 0; i < 8.0001; i += 1) { ... }
```

Avoid unbounded loop counts driven by runtime signals, as gen~ will terminate infinite loops but they stall the audio thread while doing so.

### No Dynamic Allocation

All buffer and array sizes are fixed at compile time. `Data`, `Buffer`, and `Delay` constructors take literal sizes; runtime values are not accepted.

```genexpr
Data table(1024);    // always 1024 samples regardless of runtime input
Delay d(44100);      // max 44100 samples; runtime read time must stay ≤ this
```

### samplerate and vectorsize Are Compile-Time Constants

`samplerate` and `vectorsize` are baked in when the patch compiles, not updated as live signals. If Max's audio settings change, the patch must be recompiled to reflect the new values.

```genexpr
// Fine, a constant ratio computed once at compile time
coeff = exp(-twopi * freq / samplerate);

// Not a live signal; won't update if the user changes sample rate in Max prefs
```

### Params Are Constant Per Vector

A `Param`'s value is fixed for the entire audio vector (`vectorsize` samples). It cannot vary sample-by-sample within a single processing block, even if the upstream Max parameter is automated.

```genexpr
Param cutoff(1000, min=20, max=20000);
// cutoff holds the same value for all samples in the vector;
// changes take effect on the next vector boundary
```

### Function Declarations Must Come First

All function definitions must appear before any top-level declarations (`History`, `Delay`, `Param`, etc.) and before any statements.

```genexpr
// Correct order
myFilter(x) { ... }
myOsc(f) { ... }

History state(0);
Param freq(440);

out1 = myFilter(myOsc(freq));
```

```genexpr
// Incorrect, function declared after a statement
x = in1 * 2;
myFunc(a) { return a; }   // compile error
```

### No Nested Functions

Functions cannot be defined inside other functions.

```genexpr
// Incorrect, nested function definition
outer(x)
{
    inner(y) { return y * 2; }  // compile error
    return inner(x);
}

// Correct, define both at top level
inner(y) { return y * 2; }
outer(x) { return inner(x); }
```

### Division by Zero

Both `/` and `div` return `0` when dividing by zero, not `NaN` or `Inf`. This is intentional and silent.

```genexpr
out1 = 1.0 / 0.0;     // → 0.0
out1 = in1 / in2;     // → 0.0 when in2 is 0, no error or signal discontinuity
```

Guard explicit divisions where a zero denominator would produce incorrect silence rather than a meaningful zero.

### Strings Are Attribute-Only

String values exist only as compile-time attributes on operators. They cannot be stored in variables, computed, or passed as arguments.

```genexpr
// OK, string as a compile-time attribute
tap = d.read(time, interp="cubic");

// Not valid, strings cannot be assigned or computed at runtime
mode = "cubic";              // error
tap = d.read(time, interp=mode);  // error
```

---

## 16. File Format

GenExpr files use the `.genexpr` extension. A `.genexpr` file defines one or more functions and/or top-level expressions that can be `require`d by other patchers.

Naming convention: `mylib_filtername.genexpr` (underscore-separated, no spaces).

A `.gendsp` file is a JSON Gen patcher (not text). A `.maxpat` or `.gendsp` can reference GenExpr via the `@gen` attribute or via codebox objects.

gen~ automatically reloads a `.gendsp` file when it is modified on disk. The parent Max patcher does not need to be reloaded.
