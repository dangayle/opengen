#!/usr/bin/env node
/**
 * validate_gendsp.js: .gendsp patcher validator
 *
 * Validates .gendsp files using Max's own parser for GenExpr content.
 * No npm install required. Runs with Max's bundled Node.js.
 *
 * Written for Max 9 on macOS. The hardcoded paths below assume a standard
 * macOS installation at /Applications/Max.app. Windows installations use a
 * different path structure and a different Node.js binary location; both
 * GENBO_PATH and the NODE path in the usage example will need to be updated.
 *
 * Usage:
 *   NODE="/Applications/Max.app/Contents/Resources/C74/packages/Node for Max/source/bin/osx/node/node"
 *   $NODE docs/validate_gendsp.js path/to/file.gendsp [path/to/other.gendsp ...]
 *
 * Validates:
 *   - JSON structure (required fields, correct types)
 *   - classnamespace must be "dsp.gen"
 *   - Box id uniqueness
 *   - Patchline source/destination references exist
 *   - codebox code field: parsed with Max's own genbo.parse() (same parser Max uses)
 *   - expr object text: parsed with genbo.parse() via gen start rule
 *
 * Exit code: 0 if all files valid, 1 if any errors found.
 */

"use strict";

const fs = require("fs");

// Max's own GenExpr parser, the same one Max uses internally to compile gen~ patches.
// Source: @rnbo/genexpr_js package bundled with Max 9.
const GENBO_PATH =
  "/Applications/Max.app/Contents/Resources/C74/packages/RNBO/server/node_modules/@rnbo/genexpr_js/genbo.js";

let genbo = null;
try {
  genbo = require(GENBO_PATH);
} catch (e) {
  console.warn(
    `Warning: Could not load genbo parser from Max installation.\n` +
    `  Expected at: ${GENBO_PATH}\n` +
    `  codebox and expr content will not be validated.\n`
  );
}

// ─── Structural validation ────────────────────────────────────────────────────

const VALID_MAXCLASSES = new Set(["newobj", "comment", "codebox", "panel"]);

function validateFile(filePath) {
  const errors = [];
  const warnings = [];

  // Parse JSON
  let data;
  try {
    data = JSON.parse(fs.readFileSync(filePath, "utf-8"));
  } catch (e) {
    return { errors: [`JSON parse error: ${e.message}`], warnings: [] };
  }

  // Root structure
  if (typeof data !== "object" || data === null || !data.patcher) {
    return { errors: [`Missing root "patcher" key`], warnings: [] };
  }

  const p = data.patcher;

  // Required patcher fields
  if (p.fileversion !== 1) {
    errors.push(`fileversion must be 1, got: ${JSON.stringify(p.fileversion)}`);
  }
  if (p.classnamespace !== "dsp.gen") {
    // Warning, not error: Max infers gen~ context from .gendsp extension.
    // Files saved before Max 8.1 often omit this field and still load correctly.
    warnings.push(
      p.classnamespace === undefined
        ? `classnamespace missing, recommended: "dsp.gen" (Max infers from .gendsp extension)`
        : `classnamespace is "${p.classnamespace}", expected "dsp.gen"`
    );
  }
  if (!Array.isArray(p.rect) || p.rect.length !== 4) {
    errors.push(`rect must be an array of 4 numbers`);
  }
  if (!Array.isArray(p.boxes)) {
    errors.push(`boxes must be an array`);
    return errors; // Can't continue without boxes
  }
  if (!Array.isArray(p.lines)) {
    errors.push(`lines must be an array`);
  }

  // Box validation
  const ids = new Set();
  for (let i = 0; i < p.boxes.length; i++) {
    const entry = p.boxes[i];
    if (!entry || typeof entry !== "object" || !entry.box) {
      errors.push(`boxes[${i}]: missing "box" wrapper`);
      continue;
    }
    const box = entry.box;
    const loc = `box ${JSON.stringify(box.id || `[${i}]`)}`;

    // id uniqueness
    if (!box.id) {
      errors.push(`${loc}: missing id`);
    } else if (ids.has(box.id)) {
      errors.push(`${loc}: duplicate id`);
    } else {
      ids.add(box.id);
    }

    // maxclass
    if (!VALID_MAXCLASSES.has(box.maxclass)) {
      errors.push(`${loc}: unknown maxclass "${box.maxclass}"`);
    }

    // Required fields by type
    if (!Array.isArray(box.patching_rect) || box.patching_rect.length !== 4) {
      errors.push(`${loc}: patching_rect must be array of 4 numbers`);
    }
    if (typeof box.numinlets !== "number") {
      errors.push(`${loc}: numinlets must be a number`);
    }
    if (typeof box.numoutlets !== "number") {
      errors.push(`${loc}: numoutlets must be a number`);
    }

    // outlettype consistency
    const numouts = box.numoutlets || 0;
    if (numouts > 0 && !Array.isArray(box.outlettype)) {
      errors.push(`${loc}: outlettype must be present when numoutlets=${numouts}`);
    }
    if (numouts === 0 && Array.isArray(box.outlettype) && box.outlettype.length > 0) {
      errors.push(`${loc}: outlettype should be absent when numoutlets=0`);
    }
    if (Array.isArray(box.outlettype) && box.outlettype.length !== numouts) {
      errors.push(`${loc}: outlettype length (${box.outlettype.length}) must equal numoutlets (${numouts})`);
    }

    // Type-specific validation
    if (box.maxclass === "newobj") {
      if (typeof box.text !== "string" || box.text.trim() === "") {
        errors.push(`${loc}: newobj missing text field`);
      } else {
        // Validate expr objects using genbo
        if (genbo && box.text.startsWith("expr ")) {
          const exprCode = box.text.slice(5);
          try {
            genbo.test_expr(exprCode);
          } catch (e) {
            errors.push(`${loc}: expr parse error in "${box.text.slice(0, 40)}": ${e.message || e}`);
          }
        }
      }
    }

    if (box.maxclass === "codebox") {
      if (typeof box.code !== "string") {
        errors.push(`${loc}: codebox missing code field`);
      } else if (genbo) {
        // Validate GenExpr using Max's own parser
        try {
          genbo.parse(box.code);
        } catch (e) {
          // Extract a readable error message
          const msg = (e && (e.message || e.toString())) || String(e);
          const short = msg.split("\n")[0].slice(0, 120);
          errors.push(`${loc}: GenExpr parse error: ${short}`);
        }
      }
    }
  }

  // Patchline validation
  if (Array.isArray(p.lines)) {
    for (let i = 0; i < p.lines.length; i++) {
      const entry = p.lines[i];
      if (!entry || !entry.patchline) {
        errors.push(`lines[${i}]: missing "patchline" wrapper`);
        continue;
      }
      const pl = entry.patchline;
      const loc = `patchline[${i}]`;

      if (!Array.isArray(pl.source) || pl.source.length < 2) {
        errors.push(`${loc}: source must be [id, outlet_index]`);
      } else if (!ids.has(pl.source[0])) {
        errors.push(`${loc}: source box "${pl.source[0]}" not found`);
      }

      if (!Array.isArray(pl.destination) || pl.destination.length < 2) {
        errors.push(`${loc}: destination must be [id, inlet_index]`);
      } else if (!ids.has(pl.destination[0])) {
        errors.push(`${loc}: destination box "${pl.destination[0]}" not found`);
      }
    }
  }

  return { errors, warnings };
}

// ─── CLI ──────────────────────────────────────────────────────────────────────

const files = process.argv.slice(2);

if (files.length === 0) {
  console.error("Usage: validate_gendsp.js <file.gendsp> [file2.gendsp ...]");
  process.exit(1);
}

let allValid = true;
let validCount = 0;
let errorCount = 0;

for (const f of files) {
  if (!fs.existsSync(f)) {
    console.error(`Not found: ${f}`);
    allValid = false;
    errorCount++;
    continue;
  }

  const { errors, warnings } = validateFile(f);

  if (errors.length === 0 && warnings.length === 0) {
    console.log(`ok  ${f}`);
    validCount++;
  } else if (errors.length === 0) {
    console.log(`ok  ${f}`);
    for (const w of warnings) {
      console.warn(`    WARN: ${w}`);
    }
    validCount++;
  } else {
    allValid = false;
    errorCount++;
    console.error(`ERR ${f}`);
    for (const e of errors) {
      console.error(`    ${e}`);
    }
    for (const w of warnings) {
      console.warn(`    WARN: ${w}`);
    }
  }
}

if (files.length > 1) {
  console.log(`\n${validCount} valid, ${errorCount} with errors (${files.length} total)`);
}

process.exit(allValid ? 0 : 1);
