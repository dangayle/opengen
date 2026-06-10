# gen_docs

Documentation and tooling support for **gen~**, Cycling '74's signal-rate compiled programming environment embedded in Max/MSP.

These docs are useful for anyone working programmatically with gen~: building syntax highlighters, linters, validators, code generators, LLM integrations, or other scripting tools.

## Contents

```
genexpr_language_reference.md   # Complete language reference (operators, syntax, idioms)
genexpr_grammar.pegjs           # Formal PEG grammar (extracted from Max installation)
genexpr_ebnf.md                 # EBNF for GenExpr syntax (derived from pegjs)
gendsp_ebnf.md                  # EBNF + semantic constraints for the .gendsp file format
gendsp_schema.ts                # Zod schema for .gendsp (TypeScript, with generation helpers)
validate_gendsp.js              # Validator using Max's own genbo parser (no npm required)
```

### GenExpr tooling

`genexpr_language_reference.md` covers all 155 operators, syntax, idioms, and DSP patterns. `genexpr_grammar.pegjs` and `genexpr_ebnf.md` provide the formal grammar for parsers, validators, syntax highlighters, and linters.

### `.gendsp` patcher tooling

`gendsp_ebnf.md` specifies the `newobj` text field grammar, all special object forms, inlet/outlet derivation rules, and cross-field semantic constraints enforced by Max.

`gendsp_schema.ts` provides a Zod schema with typed generation helpers (`makeNewObj`, `makeCodebox`, `makePatchline`) and `PATCHER_DEFAULTS` for Max compatibility.

`validate_gendsp.js` validates `.gendsp` files using Max's own `genbo.parse()`. Run it with Max's bundled Node.js to catch errors before opening in Max. Written for **Max 9 on macOS**; the hardcoded paths will need updating for Windows.

## What is gen~?

gen~ compiles visual/textual DSP code to native C++ running at audio sample rate. It supports three authoring modes: visual patching, codebox (GenExpr text), and inline expr boxes. Operators are signal-rate primitives. Every expression executes once per audio sample. State persists across samples only via `History`, `Delay`, `Data`, and `Buffer` objects.

Related environments that share most of the same operator set: **gen** (control rate), **jit.gen** / **jit.pix** (Jitter matrix processing), **RNBO** (exportable DSP).

## Provenance

All documentation was derived from source material within the Max application, including userguide docs, XML references, packaged scripts, and internal genexpr_js source files. Source material is copyright Cycling '74.

## Official Documentation

- [Gen Overview](https://docs.cycling74.com/userguide/gen/_gen_overview/)
- [GenExpr Reference](https://docs.cycling74.com/userguide/gen/gen_genexpr/)
- [gen~ Operators](https://docs.cycling74.com/userguide/gen/gen~_operators/)
- [Common Operators](https://docs.cycling74.com/userguide/gen/gen_common_operators/)
