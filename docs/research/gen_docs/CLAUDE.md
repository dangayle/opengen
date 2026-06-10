# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Purpose

This project produces documentation and tooling support for **gen~**, Cycling '74's signal-rate compiled programming environment embedded in Max/MSP. The docs are useful for anyone working programmatically with gen~: LLM-assisted code generation, syntax highlighters, linters, validators, and other scripting tools. The CLAUDE.md format makes them convenient for use with Claude Code specifically, but that's one application among many.

## Repository Contents

```
docs/
  genexpr_language_reference.md   # Complete language reference (operators, syntax, idioms)
  genexpr_grammar.pegjs           # Formal PEG grammar (extracted from Max installation)
  genexpr_ebnf.md                 # EBNF for GenExpr syntax (derived from pegjs)
  gendsp_ebnf.md                  # EBNF + semantic constraints for the .gendsp file format
  gendsp_schema.ts                # Zod schema for .gendsp (TypeScript, with generation helpers)
  validate_gendsp.js              # Validator using Max's own genbo parser (no npm required)
```

### What These Docs Enable

**GenExpr tooling**: working with GenExpr source code:
- `genexpr_language_reference.md` covers all 155 operators, syntax, idioms, and DSP patterns
- `genexpr_grammar.pegjs` / `genexpr_ebnf.md` provide the formal grammar for parsers, validators, syntax highlighters, and linters

**`.gendsp` patcher tooling**: working with the gen~ patcher file format:
- `gendsp_ebnf.md` specifies the `newobj` text field grammar, all special object forms,
  inlet/outlet derivation rules, and cross-field semantic constraints enforced by Max
- `gendsp_schema.ts` provides the Zod schema with typed generation helpers (`makeNewObj`,
  `makeCodebox`, `makePatchline`) and `PATCHER_DEFAULTS` for Max compatibility
- `validate_gendsp.js` validates `.gendsp` files using Max's own `genbo.parse()`. Run it
  with Max's bundled Node.js to catch errors before opening in Max

## Source Material Locations (local Max installation)

All source material was extracted from the installed Max application:

| Path | Contents |
|------|----------|
| `/Applications/Max.app/Contents/Resources/C74/packages/RNBO/server/node_modules/@rnbo/genexpr_js/genexpr.pegjs` | Official PEG grammar |
| `/Applications/Max.app/Contents/Resources/C74/packages/RNBO/server/node_modules/@rnbo/genexpr_js/operators.json` | 155 operators with full metadata |
| `/Applications/Max.app/Contents/Resources/C74/packages/RNBO/server/node_modules/@rnbo/genexpr_js/operator_exprs.json` | Operator reduction expressions |
| `/Applications/Max.app/Contents/Resources/C74/packages/RNBO/server/node_modules/@rnbo/genexpr_js/genexprs/` | ~70 real GenExpr algorithm examples |
| `/Applications/Max.app/Contents/Resources/C74/docs/userguide/content/gen/` | Userguide JSON (MDX/JSX compiled) |
| `/Applications/Max.app/Contents/Resources/C74/docs/refpages/msp-ref/gen~.maxref.xml` | gen~ object XML reference |

To extract/update operator data from `operators.json`, use `jq`:
```bash
OPS=/Applications/Max.app/Contents/Resources/C74/packages/RNBO/server/node_modules/@rnbo/genexpr_js/operators.json

# List all operator names and categories
jq 'keys[] as $k | "\($k): \(.[$k].category)"' "$OPS"

# Get a single operator's full entry
jq '.["gen"]' "$OPS"

# List all operators in a category (e.g. subpatcher)
jq '[to_entries[] | select(.value.category == "subpatcher") | .key]' "$OPS"

# 155 operators, each with: op, domain, category, inputs, outputs, attributes, aliases, digest
```

## Provenance

All documentation in this repo was derived from source material within the Max application, including userguide docs, XML references, packaged scripts, and internal genexpr_js source files. See "Source Material Locations" above for specific paths.

## Domain Context

### What is gen~?
- gen~ compiles visual/textual DSP code to native C++ running at sample rate
- It supports three authoring modes: visual patching, codebox (GenExpr text), and inline expr boxes
- Operators are signal-rate primitives; every expression executes once per audio sample
- State persists across samples only via `History`, `Delay`, `Data`, and `Buffer` objects

### Key Distinctions
- **gen~**: audio signal rate (this project's focus)
- **gen**: event/control rate (shares most operators but no DSP-only ones)
- **jit.gen** / **jit.pix**: Jitter matrix processing (different coordinate system)
- **RNBO**: uses the same GenExpr language, but targets exportable DSP

### Domain Tags in operators.json
- `"domain": "dsp"`: gen~ / pfft~ only
- `"domain": "common"`: all Gen contexts

## Key Language Semantics

- All values are **64-bit float** unless explicitly integer-ized (`int()`, `trunc()`)
- **No dynamic allocation**: all buffer/array sizes fixed at compile time
- **Feedback requires explicit state** (`History` for 1-sample, `Delay` for variable)
- `samplerate` and `vectorsize` are compile-time constants, not runtime signals
- `History` read returns *previous* sample's value (by design, enabling feedback)
- `Delay.read()` must come *before* `Delay.write()` in the same sample
- Function declarations must appear before all other code in a codebox
- No nested functions; no closures

## Official Documentation URLs

- Gen Overview: https://docs.cycling74.com/userguide/gen/_gen_overview/
- GenExpr Reference: https://docs.cycling74.com/userguide/gen/gen_genexpr/
- gen~ Operators: https://docs.cycling74.com/userguide/gen/gen~_operators/
- Common Operators: https://docs.cycling74.com/userguide/gen/gen_common_operators/
