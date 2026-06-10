/**
 * GenDSP Patcher Format: Zod Schema
 *
 * Formal type specification for the .gendsp file format used by gen~, gen,
 * jit.gen, and jit.pix in Max/MSP.
 *
 * This schema covers the JSON structure. For the `text` field grammar inside
 * `newobj` boxes and cross-field semantic constraints, see `gendsp_ebnf.md`.
 *
 * Source: Reverse-engineered from @rnbo/xam-runner/lib/patcher.js (field
 * consumption), @rnbo/genexpr_js/operators.json (operator metadata), and
 * direct inspection of 250+ real .gendsp files.
 *
 * Usage:
 *   npm install zod
 *   import { GenDSPSchema, PATCHER_DEFAULTS } from "./gendsp_schema";
 *   const result = GenDSPSchema.safeParse(JSON.parse(fileContent));
 */

import { z } from "zod";

// ─── Primitives ───────────────────────────────────────────────────────────────

/** [x, y, width, height] in pixels */
const Rect = z.tuple([z.number(), z.number(), z.number(), z.number()]);

/** [red, green, blue, alpha] each in range 0.0–1.0 */
const RGBA = z.tuple([z.number(), z.number(), z.number(), z.number()]);

/** Box identifier string, conventionally "obj-N" where N is any positive integer. */
const ObjId = z.string();

// ─── Box Types ────────────────────────────────────────────────────────────────

/**
 * A gen~ operator or special object (in, out, history, delay, param, expr, etc.).
 * The `text` field contains the operator name plus arguments and @attributes.
 * See gendsp_ebnf.md Section 3 for the full text field grammar.
 *
 * Inlet/outlet counts are determined by operators.json for named operators,
 * or by convention for special objects (in, out, param, etc.).
 */
const NewObjBox = z.object({
  id: ObjId,
  maxclass: z.literal("newobj"),
  numinlets: z.number().int().nonnegative(),
  numoutlets: z.number().int().nonnegative(),
  /**
   * Array of outlet type strings, one per outlet. Always [""] for gen~ signals.
   * MUST be present when numoutlets > 0. MUST be omitted when numoutlets == 0.
   */
  outlettype: z.array(z.string()).optional(),
  patching_rect: Rect,
  /** Operator name + positional args + @attributes. See gendsp_ebnf.md. */
  text: z.string(),
  /** Number of text lines displayed. Only affects visual height of the box. */
  linecount: z.number().int().positive().optional(),
  fontname: z.string().optional(),
  fontsize: z.number().positive().optional(),
  /** 0=normal, 1=bold, 2=italic, 3=bold+italic */
  fontface: z.union([z.literal(0), z.literal(1), z.literal(2), z.literal(3)]).optional(),
  style: z.string().optional(),
  /** Position in presentation mode (only relevant when openinpresentation=1). */
  presentation_rect: Rect.optional(),
});

/**
 * A text annotation label. Visual only. No signal processing.
 * Always has numinlets=1, numoutlets=0.
 */
const CommentBox = z.object({
  id: ObjId,
  maxclass: z.literal("comment"),
  numinlets: z.literal(1),
  numoutlets: z.literal(0),
  patching_rect: Rect,
  text: z.string(),
  linecount: z.number().int().positive().optional(),
  fontname: z.string().optional(),
  fontsize: z.number().positive().optional(),
  fontface: z.number().int().min(0).max(3).optional(),
  style: z.string().optional(),
  presentation_rect: Rect.optional(),
});

/**
 * A GenExpr text block. The `code` field contains a complete GenExpr
 * translation_unit (see genexpr_grammar.pegjs).
 *
 * numinlets and numoutlets MUST match the highest inN / outN index appearing
 * in the code. Max derives these by scanning the GenExpr AST.
 * Line endings in `code` are CRLF (\r\n) in files saved by Max.
 */
const CodeboxBox = z.object({
  id: ObjId,
  maxclass: z.literal("codebox"),
  numinlets: z.number().int().nonnegative(),
  numoutlets: z.number().int().nonnegative(),
  outlettype: z.array(z.string()).optional(),
  patching_rect: Rect,
  /** GenExpr source code. Must be valid per genexpr_grammar.pegjs. */
  code: z.string(),
  fontname: z.string().optional(),
  fontsize: z.number().positive().optional(),
  fontface: z.number().int().optional(),
  style: z.string().optional(),
  presentation_rect: Rect.optional(),
});

/**
 * A background UI panel. Visual grouping element. No signal processing.
 * Always has numinlets=1, numoutlets=0.
 */
const PanelBox = z.object({
  id: ObjId,
  maxclass: z.literal("panel"),
  numinlets: z.literal(1),
  numoutlets: z.literal(0),
  patching_rect: Rect,
  /** 0=fill, 1=border */
  mode: z.union([z.literal(0), z.literal(1)]).optional(),
  /** Corner rounding proportion (0.0–1.0) */
  proportion: z.number().min(0).max(1).optional(),
  style: z.string().optional(),
  presentation_rect: Rect.optional(),
});

/** Discriminated union on maxclass. All box types found in real .gendsp files. */
const Box = z.discriminatedUnion("maxclass", [
  NewObjBox,
  CommentBox,
  CodeboxBox,
  PanelBox,
]);

// ─── Patchline ────────────────────────────────────────────────────────────────

/**
 * A signal connection between two boxes.
 *
 * IMPORTANT: source[1] and destination[1] are 0-based outlet/inlet indices,
 * even though `in 1` / `out 1` object text uses 1-based numbering.
 */
const Patchline = z.object({
  /** [box_id, outlet_index], outlet_index is 0-based */
  source: z.tuple([ObjId, z.number().int().nonnegative()]),
  /** [box_id, inlet_index], inlet_index is 0-based */
  destination: z.tuple([ObjId, z.number().int().nonnegative()]),
  /**
   * Visual routing waypoints [x1, y1, x2, y2, ...].
   * Purely cosmetic. Omitting produces straight-line connections.
   */
  midpoints: z.array(z.number()).optional(),
  /**
   * Draw order when multiple patchlines share the same source outlet.
   * Does not affect signal evaluation order.
   */
  order: z.number().int().optional(),
  /** 1 = Max ignores this connection; 0 = active (default) */
  disabled: z.union([z.literal(0), z.literal(1)]).optional(),
  /** 1 = not drawn in the patcher UI; 0 = visible (default) */
  hidden: z.union([z.literal(0), z.literal(1)]).optional(),
});

// ─── Patcher ──────────────────────────────────────────────────────────────────

const AppVersion = z.object({
  major: z.number().int(),
  minor: z.number().int(),
  revision: z.number().int(),
  /** Always "x64" in current Max installations */
  architecture: z.string(),
  /** Always 1 in current Max installations */
  modernui: z.number().int(),
});

const Patcher = z.object({
  // ── Required fields ────────────────────────────────────────────
  /** Always 1 */
  fileversion: z.literal(1),
  /**
   * CRITICAL: Must be "dsp.gen". Without this field, Max will not load the
   * file as a gen~ patcher. Older files (Max 7.x) sometimes omit it.
   */
  classnamespace: z.literal("dsp.gen"),
  /** Window position and size: [x, y, width, height] in screen pixels */
  rect: Rect,
  /** Array of { box: Box } objects. Order is not significant for signal flow. */
  boxes: z.array(z.object({ box: Box })),
  /** Array of { patchline: Patchline } signal connections */
  lines: z.array(z.object({ patchline: Patchline })),

  // ── Metadata (optional; include when generating for Max compatibility) ──
  appversion: AppVersion.optional(),
  description: z.string().optional(),
  digest: z.string().optional(),
  tags: z.string().optional(),

  // ── UI state flags (optional; Max uses built-in defaults when absent) ──
  /** 1 = patcher editing is locked */
  bglocked: z.number().int().optional(),
  /** 1 = open in presentation mode by default */
  openinpresentation: z.number().int().optional(),
  default_fontsize: z.number().optional(),
  /** 0=normal, 1=bold, 2=italic, 3=bold+italic */
  default_fontface: z.number().int().optional(),
  default_fontname: z.string().optional(),
  /** 1 = show grid when patcher opens */
  gridonopen: z.number().int().optional(),
  /** Grid cell size [width, height] in pixels */
  gridsize: z.tuple([z.number(), z.number()]).optional(),
  /** 1 = snap objects to grid */
  gridsnaponopen: z.number().int().optional(),
  objectsnaponopen: z.number().int().optional(),
  statusbarvisible: z.number().int().optional(),
  toolbarvisible: z.number().int().optional(),
  lefttoolbarpinned: z.number().int().optional(),
  toptoolbarpinned: z.number().int().optional(),
  righttoolbarpinned: z.number().int().optional(),
  bottomtoolbarpinned: z.number().int().optional(),
  toolbars_unpinned_last_save: z.number().int().optional(),
  tallnewobj: z.number().int().optional(),
  boxanimatetime: z.number().int().optional(),
  enablehscroll: z.number().int().optional(),
  enablevscroll: z.number().int().optional(),
  devicewidth: z.number().optional(),
  style: z.string().optional(),
  subpatcher_template: z.string().optional(),
  assistshowspatchername: z.number().int().optional(),
  /** Background color [r, g, b, a], each 0.0–1.0 */
  bgcolor: RGBA.optional(),
  /** Background color in editing mode */
  editing_bgcolor: RGBA.optional(),
});

// ─── Root ─────────────────────────────────────────────────────────────────────

/** The complete .gendsp file schema. Root object wraps a single "patcher" key. */
export const GenDSPSchema = z.object({ patcher: Patcher });

// ─── Exported Types ───────────────────────────────────────────────────────────

export type GenDSP = z.infer<typeof GenDSPSchema>;
export type GenDSPPatcher = z.infer<typeof Patcher>;
export type GenDSPBox = z.infer<typeof Box>;
export type GenDSPNewObj = z.infer<typeof NewObjBox>;
export type GenDSPComment = z.infer<typeof CommentBox>;
export type GenDSPCodebox = z.infer<typeof CodeboxBox>;
export type GenDSPPanel = z.infer<typeof PanelBox>;
export type GenDSPPatchline = z.infer<typeof Patchline>;

// ─── Generation Helpers ───────────────────────────────────────────────────────

/**
 * Recommended default values for all optional patcher UI fields.
 * Include these when generating .gendsp files for maximum Max compatibility.
 *
 * Only `fileversion`, `classnamespace`, `rect`, `boxes`, and `lines` are
 * strictly required for Max to load the file.
 */
export const PATCHER_DEFAULTS = {
  fileversion: 1 as const,
  classnamespace: "dsp.gen" as const,
  appversion: { major: 8, minor: 6, revision: 0, architecture: "x64", modernui: 1 },
  bglocked: 0,
  openinpresentation: 0,
  default_fontsize: 12.0,
  default_fontface: 0,
  default_fontname: "Arial",
  gridonopen: 1,
  gridsize: [15.0, 15.0] as [number, number],
  gridsnaponopen: 1,
  objectsnaponopen: 1,
  statusbarvisible: 2,
  toolbarvisible: 1,
  lefttoolbarpinned: 0,
  toptoolbarpinned: 0,
  righttoolbarpinned: 0,
  bottomtoolbarpinned: 0,
  toolbars_unpinned_last_save: 0,
  tallnewobj: 0,
  boxanimatetime: 200,
  enablehscroll: 1,
  enablevscroll: 1,
  devicewidth: 0.0,
  description: "",
  digest: "",
  tags: "",
  style: "",
  subpatcher_template: "",
} satisfies Partial<GenDSPPatcher>;

/**
 * Helper to build a `newobj` box. Outlet type array is auto-populated.
 * Position arguments are [x, y] in patcher canvas pixels.
 */
export function makeNewObj(params: {
  id: string;
  text: string;
  numinlets: number;
  numoutlets: number;
  x?: number;
  y?: number;
  width?: number;
}): { box: GenDSPNewObj } {
  const { id, text, numinlets, numoutlets, x = 50, y = 50, width = 22 * text.length * 0.6 + 10 } = params;
  const box: GenDSPNewObj = {
    id,
    maxclass: "newobj",
    numinlets,
    numoutlets,
    patching_rect: [x, y, Math.max(22, width), 22.0],
    text,
  };
  if (numoutlets > 0) {
    box.outlettype = Array(numoutlets).fill("");
  }
  return { box };
}

/**
 * Helper to build a `codebox` box.
 */
export function makeCodebox(params: {
  id: string;
  code: string;
  numinlets: number;
  numoutlets: number;
  x?: number;
  y?: number;
  width?: number;
  height?: number;
}): { box: GenDSPCodebox } {
  const { id, code, numinlets, numoutlets, x = 50, y = 50, width = 300, height = 200 } = params;
  const box: GenDSPCodebox = {
    id,
    maxclass: "codebox",
    numinlets,
    numoutlets,
    patching_rect: [x, y, width, height],
    code,
  };
  if (numoutlets > 0) {
    box.outlettype = Array(numoutlets).fill("");
  }
  return { box };
}

/**
 * Helper to build a patchline connection.
 */
export function makePatchline(
  sourceId: string,
  sourceOutlet: number,
  destId: string,
  destInlet: number,
): { patchline: GenDSPPatchline } {
  return {
    patchline: {
      source: [sourceId, sourceOutlet],
      destination: [destId, destInlet],
    },
  };
}
