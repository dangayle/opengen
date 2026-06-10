# GenDSP Patcher Format: EBNF & Semantic Constraints

Formal specification of the `.gendsp` file format as used by gen~, gen, jit.gen, and jit.pix in Max/MSP.

Covers the parts of the format that are not expressible in the Zod schema (`gendsp_schema.ts`):
the grammar of the `text` field inside `newobj` boxes, codebox inlet/outlet derivation,
and cross-field semantic constraints enforced by Max at load time.

Sources: Reverse-engineered from `@rnbo/xam/lib/parser.js` (box text tokenizer),
`@rnbo/xam/lib/operators.js` (operator table), and `@rnbo/genexpr_js/operators.json` (operator metadata).

---

## 1. Object Text Grammar (`newobj` box `text` field)

The parser (`@rnbo/xam/lib/parser.js`) uses a **simple whitespace-delimited tokenizer**,
not a recursive grammar. Quoted strings are preserved as single tokens.

```ebnf
object_text = classname , { " " , positional_arg } , { " " , attribute } ;

classname      = gen_operator_name
               | special_object_name ;

positional_arg = token ;            (* interpreted per the operator's defaultargs list *)
attribute      = "@" , identifier , " " , attr_value ;
attr_value     = token | string_literal ;

token          = number_literal | identifier ;
string_literal = '"' , { any_char_except_quote } , '"'
               | "'" , { any_char_except_single_quote } , "'" ;

number_literal = integer_literal | float_literal ;
integer_literal = [ "-" ] , digit , { digit }
                | "0x" , hex_digit , { hex_digit } ;
float_literal   = [ "-" ] , digit , { digit } , "." , { digit } [ exponent ]
                | [ "-" ] , "." , digit , { digit } [ exponent ] ;
exponent        = ( "e" | "E" ) , [ "+" | "-" ] , digit , { digit } ;

identifier = ( letter | "_" ) , { letter | digit | "_" } ;
```

The classname is looked up in the operator table (`operators.json` + `simpleoperators.json`)
to determine valid positional argument count and types, valid attribute names, and
inlet/outlet counts for the visual box.

---

## 2. Gen~ Operator Names

Valid operator names for the `classname` production. All entries from `operators.json`.
`[dsp]` = gen~ only (domain: "dsp"); `[common]` = all Gen contexts.

### Buffer / Data
`buffer` `channels` `cycle` `data` `dim` `lookup` `nearest` `peek` `poke` `read` `sample` `splat` `wave` `write`: `[dsp]`

### Comparison / Selection
`eq` `eqp` `gt` `gte` `gtep` `gtp` `lt` `lte` `ltep` `ltp` `max` `min` `neq` `neqp` `step`: `[common]`

### Constants
`degtorad` `e` `halfpi` `invpi` `ln10` `ln2` `log10e` `log2e` `phi` `pi` `radtodeg` `sqrt1_2` `sqrt2` `twopi`: `[common]`

`FFTFULLSPECT` `FFTHOP` `FFTOFFSET` `FFTSIZE` `SAMPLERATE` `VECTORSIZE` `constant` `float` `int`: `[dsp]`

### Conversion
`atodb` `dbtoa` `ftom` `mstosamps` `mtof` `sampstoms`: `[dsp]`

### DSP Utilities
`fixdenorm` `fixnan` `isdenorm` `isnan` `t60` `t60time`: `[dsp]`

### Feedback / State (as inline GenExpr, also available as declarations)
`delay` `history`: `[dsp]`

### Filter / Signal Processing
`change` `dcblock` `delta` `interp` `latch` `phasewrap` `sah` `slide`: `[dsp]`

### Global
`elapsed` `voice`: `[dsp]`

### Integrators
`counter` `mulequals` `plusequals`: `[dsp]`

### Logic
`and` `bool` `not` `or` `xor`: `[common]`

### Math
`absdiff` `add` `cartopol` `div` `mod` `mul` `neg` `poltocar` `rdiv` `rmod` `rsub` `sub`: `[common]`

### Numeric
`abs` `ceil` `floor` `fract` `round` `sign` `trunc`: `[common]`

### Powers / Logarithms
`exp` `exp2` `fastexp` `fastpow` `log` `log10` `log2` `pow` `sqrt`: `[common]`

### Range / Mapping
`clamp` `fold` `scale` `wrap`: `[common]`

### Routing
`gate` `mix` `receive` `selector` `send` `smoothstep` `switch`: `[common/dsp]`

### Subpatcher
`gen` `setparam`: `[dsp]`

### Trigonometry
`acos` `acosh` `asin` `asinh` `atan` `atan2` `atanh` `cos` `cosh` `degrees` `fastcos` `fastsin` `fasttan` `hypot` `radians` `sin` `sinh` `tan` `tanh`: `[common]`

### Waveforms / Generators
`noise` `pass` `phasor` `rate` `train` `triangle`: `[dsp]`

### FFT (pfft~ only)
`fftinfo`: `[dsp]`

---

## 3. Special Object Text Forms

Special objects have a fixed first positional argument that determines their role,
and fixed inlet/outlet semantics.

```ebnf
(* Patcher I/O: expose gen~ inlets and outlets to the parent patcher *)
(* numinlets=0, numoutlets=1 *)
inlet_obj  = "in"  , " " , pos_int
           , [ " " , "@comment" , " " , string_literal ] ;

(* numinlets=1, numoutlets=0, no outlettype field *)
outlet_obj = "out" , " " , pos_int ;


(* State declarations as visual objects *)
(* numinlets=1, numoutlets=1 *)
history_obj = "history" , [ " " , identifier ] ;   (* optional name arg *)

(* numinlets=1, numoutlets=1 *)
delay_obj = "delay" , " " , pos_number ;            (* max delay in samples *)

(* numinlets=0, numoutlets=1 *)
data_obj = "data" , " " , pos_int , " " , pos_int ; (* channels, dim *)

(* numinlets=0, numoutlets=1 *)
buffer_obj = "buffer~" , " " , string_literal ;     (* external buffer~ name *)


(* Parameter: creates a named inlet on the parent gen~ object *)
(* numinlets=0, numoutlets=1 *)
param_obj = "param" , " " , identifier
          , [ " " , number_literal ]                 (* default value *)
          , [ " " , "@min" , " " , number_literal ]
          , [ " " , "@max" , " " , number_literal ] ;


(* Inline GenExpr expression: remainder of text is parsed as a GenExpr expression *)
(* by genbo.parse() with startRule="gen"; numinlets/numoutlets derived from inN/outN usage *)
expr_obj = "expr" , " " , genexpr_expression ;      (* see genexpr_ebnf.md *)


(* Arithmetic operators with a constant baked into the right-hand side *)
(* numinlets=1, numoutlets=1 *)
binop_const = binary_op_name , " " , const_value ;
binary_op_name = "+" | "-" | "*" | "/" | "%" | "!-" | "!/" | "!%" ;
const_value    = number_literal | identifier ;       (* identifiers like samplerate/2 accepted *)


(* Operators with attribute-style arguments *)
(* numinlets/numoutlets determined by operators.json constructors *)
operator_with_attrs = operator_name , { " " , positional_arg } , { " " , "@" , identifier , " " , attr_value } ;
```

### Attribute Values

Attribute values are almost always unquoted tokens (numbers or identifiers).
String values are used for names (buffer names, interp modes, etc.) and must be quoted:

```ebnf
(* Common attributes and their accepted value sets: *)
(*   @interp   : "none" | "linear" | "cubic" | "spline"    *)
(*   @boundmode: "clamp" | "wrap" | "ignore"                *)
(*   @index    : "phase" | "samples" | "lookup" | "wave"    *)
(*   @comment  : any string literal                          *)
(*   @min, @max, @default: number_literal                   *)
(*   @name     : string_literal (buffer/data/send/receive)  *)
```

---

## 4. Codebox Inlet/Outlet Derivation

Unlike `newobj` boxes (where inlet/outlet counts come from `operators.json`),
`codebox` box `numinlets` and `numoutlets` are derived from the GenExpr code.

Max uses the following algorithm (from `@rnbo/xam/lib/expression.js`):

```ebnf
(* Inlets: highest N where inN appears anywhere in code *)
(* Outlets: highest N where outN appears as an assignment target *)
(* Both use the regex: \b(in|out)([1-9]\d*)\b *)

codebox_numinlets  = max { N : "in"  N appears in code } ;
codebox_numoutlets = max { N : "out" N appears in code } ;

(* Gaps are filled: if in3 appears but not in2, inlet 2 still exists *)
(* Indices are 1-based in code; 0-based in patchlines *)
```

---

## 5. Semantic Constraints

These constraints are enforced by Max at load time. They cannot be expressed
in the Zod schema (`gendsp_schema.ts`) and must be maintained manually when
generating `.gendsp` files.

### Identity and Reference Integrity

- All `box.id` values within a patcher MUST be unique strings
- `patchline.source[0]` MUST reference an existing `box.id` in the same patcher
- `patchline.destination[0]` MUST reference an existing `box.id` in the same patcher
- `patchline.source[1]` (outlet index, 0-based) MUST be `< numoutlets` of the source box
- `patchline.destination[1]` (inlet index, 0-based) MUST be `< numinlets` of the destination box

### Index Conventions

- Inlet/outlet `N` in text is **1-based**: `in 1`, `out 2`, `param freq`
- Patchline `source[1]` and `destination[1]` are **0-based**: `["obj-1", 0]`
- Box positions in `patching_rect` are in pixels from top-left of the patcher window

### Box Field Constraints

- `outlettype` array MUST be present when `numoutlets > 0`; each element is `""`
- `outlettype` MUST be omitted (not `[]`) when `numoutlets == 0`
- `numinlets` for `in N` objects MUST be `0`; `numoutlets` MUST be `1`
- `numinlets` for `out N` objects MUST be `1`; `numoutlets` MUST be `0`
- `numinlets` and `numoutlets` for `newobj` operators MUST match `operators.json` constructors
- `numinlets` and `numoutlets` for `codebox` MUST reflect actual `inN`/`outN` usage

### Content Constraints

- `classnamespace` MUST be `"dsp.gen"`. Max will not load the file as gen~ without it
- `fileversion` MUST be `1`
- `codebox` `code` field MUST be valid GenExpr per `genexpr_grammar.pegjs`
- `expr` object text (the portion after `"expr "`) MUST be a valid GenExpr `expression` production
- `newobj` classnames MUST be valid gen~ operators (see Section 2) or special objects (Section 3)
- `param` names MUST be valid identifiers and unique within the patcher

### Ordering

- Boxes are sorted by `patching_rect[0]` (x position) before processing
- This affects implicit inlet/outlet index assignment for unnamed `in`/`out` objects
- Explicit `in N` / `out N` index arguments override position-based assignment

### Patchline Ordering

- When multiple patchlines share the same `source` outlet, their relative evaluation order
  is unspecified unless `order` fields are set
- `disabled: 1` patchlines are ignored by Max; `hidden: 1` lines are not drawn

---

## 6. Minimal Valid .gendsp Skeleton

A passthrough patch routing `in1` to `out1`:

```json
{
  "patcher": {
    "fileversion": 1,
    "classnamespace": "dsp.gen",
    "rect": [100.0, 100.0, 400.0, 300.0],
    "boxes": [
      {
        "box": {
          "id": "obj-1",
          "maxclass": "newobj",
          "numinlets": 0,
          "numoutlets": 1,
          "outlettype": [""],
          "patching_rect": [50.0, 30.0, 30.0, 22.0],
          "text": "in 1"
        }
      },
      {
        "box": {
          "id": "obj-2",
          "maxclass": "newobj",
          "numinlets": 1,
          "numoutlets": 0,
          "patching_rect": [50.0, 220.0, 37.0, 22.0],
          "text": "out 1"
        }
      }
    ],
    "lines": [
      {
        "patchline": {
          "source": ["obj-1", 0],
          "destination": ["obj-2", 0]
        }
      }
    ]
  }
}
```

Only `fileversion`, `classnamespace`, `rect`, `boxes`, and `lines` are required.
All other patcher fields are optional (Max uses built-in defaults).
