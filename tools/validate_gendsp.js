#!/usr/bin/env node
/**
 * validate_gendsp.js — .gendsp + .genexpr validator using Max's own genbo parser
 *
 * Validates:
 *   (a) conformance/patches/*.genexpr — raw GenExpr, validated with genbo.parse()
 *   (b) crates/opengen-gendsp/tests/fixtures/*.gendsp — JSON patchers with codebox
 *       code and expr-object text validated with genbo.parse() / genbo.test_expr()
 *
 * No npm install required. Runs with Max's bundled Node.js.
 *
 * Written for Max 9 on macOS. Assumes standard installation at /Applications/Max.app.
 *
 * Usage:
 *   NODE="/Applications/Max.app/Contents/Resources/C74/packages/Node for Max/source/bin/osx/node/node"
 *   $NODE tools/validate_gendsp.js
 *
 * The tool auto-discovers patch directories relative to the repo root (its own dir).
 * See also: tools/validate-with-genbo.sh (shell wrapper)
 *
 * Exit code: 0 if all files valid, 1 if any errors found.
 */

"use strict";

const fs = require("fs");
const path = require("path");

// ─── Path resolution ──────────────────────────────────────────────────────────

// Resolve repo root: tools/validate_gendsp.js → repo/
const REPO_ROOT = path.resolve(__dirname, "..");

// ─── Genbo loader ─────────────────────────────────────────────────────────────

const GENBO_PATH =
  "/Applications/Max.app/Contents/Resources/C74/packages/RNBO/server/node_modules/@rnbo/genexpr_js/genbo.js";

let genbo = null;
try {
  genbo = require(GENBO_PATH);
} catch (e) {
  console.warn(
    `Warning: Could not load genbo parser from Max installation.\n` +
    `  Expected at: ${GENBO_PATH}\n` +
    `  GenExpr content will not be validated.\n`
  );
}

// ─── Structural validation (gendsp JSON) ──────────────────────────────────────

const VALID_MAXCLASSES = new Set(["newobj", "comment", "codebox", "panel"]);

function validateGendspFile(filePath) {
  const errors = [];
  const warnings = [];

  let data;
  try {
    data = JSON.parse(fs.readFileSync(filePath, "utf-8"));
  } catch (e) {
    return { errors: [`JSON parse error: ${e.message}`], warnings: [] };
  }

  if (typeof data !== "object" || data === null || !data.patcher) {
    return { errors: [`Missing root "patcher" key`], warnings: [] };
  }

  const p = data.patcher;

  // Required patcher fields
  if (p.fileversion !== 1) {
    errors.push(`fileversion must be 1, got: ${JSON.stringify(p.fileversion)}`);
  }
  if (p.classnamespace !== "dsp.gen") {
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
    return { errors, warnings };
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
    // patching_rect and outlettype are UI layout fields — warn, don't error.
    // They are absent in minimal test fixtures but the gendsp is still valid.
    if (!Array.isArray(box.patching_rect) || box.patching_rect.length !== 4) {
      warnings.push(`${loc}: patching_rect missing or invalid (minimal fixture — fine)`);
    }
    if (typeof box.numinlets !== "number") {
      warnings.push(`${loc}: numinlets must be a number`);
    }
    if (typeof box.numoutlets !== "number") {
      warnings.push(`${loc}: numoutlets must be a number`);
    }

    // outlettype consistency
    const numouts = box.numoutlets || 0;
    if (numouts > 0 && !Array.isArray(box.outlettype)) {
      warnings.push(`${loc}: outlettype absent (minimal fixture — fine)`);
    }
    if (numouts === 0 && Array.isArray(box.outlettype) && box.outlettype.length > 0) {
      warnings.push(`${loc}: outlettype should be absent when numoutlets=0`);
    }
    if (Array.isArray(box.outlettype) && box.outlettype.length !== numouts) {
      warnings.push(`${loc}: outlettype length (${box.outlettype.length}) must equal numoutlets (${numouts})`);
    }

    // Type-specific validation with genbo
    if (box.maxclass === "newobj" && typeof box.text === "string" && box.text.startsWith("expr ")) {
      const exprCode = box.text.slice(5);
      if (genbo) {
        try {
          genbo.test_expr(exprCode);
        } catch (e) {
          const msg = (e && (e.message || e.toString())) || String(e);
          const short = msg.split("\n")[0].slice(0, 120);
          errors.push(`${loc}: expr parse error in "${box.text.slice(0, 40)}": ${short}`);
        }
      }
    }

    if (box.maxclass === "codebox" && typeof box.code === "string") {
      if (genbo) {
        try {
          genbo.parse(box.code);
        } catch (e) {
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

// ─── Raw GenExpr validation (conformance patches) ─────────────────────────────

function validateGenexprFile(filePath) {
  const errors = [];
  const warnings = [];

  let code;
  try {
    code = fs.readFileSync(filePath, "utf-8");
  } catch (e) {
    return { errors: [`read error: ${e.message}`], warnings: [] };
  }

  if (genbo) {
    try {
      genbo.parse(code);
    } catch (e) {
      const msg = (e && (e.message || e.toString())) || String(e);
      const short = msg.split("\n")[0].slice(0, 200);
      errors.push(`GenExpr parse error: ${short}`);
    }
  }

  return { errors, warnings };
}

// ─── CLI ──────────────────────────────────────────────────────────────────────

let allValid = true;
let validCount = 0;
let errorCount = 0;
let totalCount = 0;

function validateStem(dir, label, ext, validatorFn) {
  const fullDir = path.resolve(REPO_ROOT, dir);
  if (!fs.existsSync(fullDir)) {
    console.warn(`WARN: directory not found: ${fullDir}`);
    return;
  }

  const files = fs.readdirSync(fullDir)
    .filter(f => f.endsWith(ext))
    .sort();

  if (files.length === 0) {
    console.warn(`WARN: no ${ext} files found in ${dir}`);
    return;
  }

  console.log(`--- ${label} (${files.length} files) ---`);

  for (const f of files) {
    totalCount++;
    const filePath = path.join(fullDir, f);
    const { errors, warnings } = validatorFn(filePath);

    if (errors.length === 0 && warnings.length === 0) {
      console.log(`  ok  ${f}`);
      validCount++;
    } else if (errors.length === 0) {
      console.log(`  ok  ${f}`);
      for (const w of warnings) {
        console.warn(`      WARN: ${w}`);
      }
      validCount++;
    } else {
      allValid = false;
      errorCount++;
      console.error(`  ERR ${f}`);
      for (const e of errors) {
        console.error(`      ${e}`);
      }
      for (const w of warnings) {
        console.warn(`      WARN: ${w}`);
      }
    }
  }
}

// Validate conformance patches (*.genexpr)
validateStem("conformance/patches", "Conformance patches", ".genexpr", validateGenexprFile);

// Validate gendsp test fixtures (*.gendsp)
validateStem("crates/opengen-gendsp/tests/fixtures", "Gendsp fixtures", ".gendsp", validateGendspFile);

console.log(`\n${validCount} valid, ${errorCount} with errors (${totalCount} total)`);

process.exit(allValid ? 0 : 1);
