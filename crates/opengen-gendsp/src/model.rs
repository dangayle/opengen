//! Patcher data model — parsed from `.gendsp` JSON.
//!
//! The data model follows the structure documented in
//! `docs/research/gen_docs/gendsp_schema.ts` (field consumption from
//! `@rnbo/xam-runner/lib/patcher.js`) and `gendsp_ebnf.md` (box text grammar).
//!
//! # Provenance
//!
//! Schema reverse-engineered from `@rnbo/xam-runner/lib/patcher.js` and
//! inspection of 250+ real `.gendsp` files. Cross-checked against
//! `reference/gen/examples/*.gendsp` (cite paths only).

use crate::json::Json;

/// A parsed GenDSP patcher.
#[derive(Debug, Clone, PartialEq)]
pub struct Patcher {
    pub boxes: Vec<GBox>,
    pub lines: Vec<Line>,
}

/// A box (node) in the patcher.
#[derive(Debug, Clone, PartialEq)]
pub struct GBox {
    /// Box identifier, e.g. "obj-1"
    pub id: String,
    /// Max class: "newobj", "codebox", "comment", "panel"
    pub maxclass: String,
    /// Box text (operator name + args + @attrs), present for newobj
    pub text: String,
    /// GenExpr source code, present for codebox
    pub code: String,
    pub numinlets: u16,
    pub numoutlets: u16,
    /// Nested subpatcher (for gen @file / gen @gen / abstractions)
    pub subpatcher: Option<Box<Patcher>>,
}

/// A signal connection between two boxes.
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    /// (box_id, outlet_index) — outlet_index is 0-based
    pub src: (String, u16),
    /// (box_id, inlet_index) — inlet_index is 0-based
    pub dst: (String, u16),
}

impl Patcher {
    /// Extract a `Patcher` from a parsed `.gendsp` JSON value.
    ///
    /// Expects the root `{"patcher": {...}}` structure (for `.gendsp` files) or
    /// an inline patcher object with `boxes`/`lines` directly (for embedded
    /// dsp.gen sub-patchers inside `.amxd` containers).
    pub fn from_json(json: &Json) -> Result<Self, String> {
        match json.get("patcher") {
            Some(patcher) => Self::from_json_value(patcher),
            None => {
                // Embedded sub-patcher (e.g., Fors .amxd): boxes/lines are
                // direct keys, not wrapped in a "patcher" key.
                Self::from_json_value(json)
            }
        }
    }

    /// Extract a `Patcher` from a "patcher" JSON object (without the wrapper).
    /// Used both by the top-level `from_json` and by embedded subpatcher parsing.
    pub fn from_json_value(patcher: &Json) -> Result<Self, String> {

        let boxes_arr = patcher.get("boxes")
            .and_then(|j| j.as_arr())
            .ok_or_else(|| "missing or invalid 'boxes' array".to_string())?;

        let lines_arr = patcher.get("lines")
            .and_then(|j| j.as_arr())
            .ok_or_else(|| "missing or invalid 'lines' array".to_string())?;

        let mut boxes = Vec::new();
        for bv in boxes_arr {
            let bx = bv.get("box").ok_or_else(|| "box entry missing 'box' key".to_string())?;
            boxes.push(GBox::from_json(bx)?);
        }

        let mut lines = Vec::new();
        for lv in lines_arr {
            let pl = lv.get("patchline").ok_or_else(|| "line entry missing 'patchline' key".to_string())?;
            lines.push(Line::from_json(pl)?);
        }

        Ok(Patcher { boxes, lines })
    }
}

impl GBox {
    /// Extract a `GBox` from a `box` JSON object.
    /// Looks for an embedded subpatcher under the JSON keys `patcher` (used by real
    /// `.gendsp` files) or `subpatcher` (legacy field name).
    pub fn from_json(json: &Json) -> Result<Self, String> {
        let id = json.get("id")
            .and_then(|j| j.as_str())
            .ok_or_else(|| "box missing 'id'".to_string())?
            .to_string();

        let maxclass = json.get("maxclass")
            .and_then(|j| j.as_str())
            .ok_or_else(|| format!("box '{}' missing 'maxclass'", id))?
            .to_string();

        let text = json.get("text").and_then(|j| j.as_str()).unwrap_or("").to_string();
        let code = json.get("code").and_then(|j| j.as_str()).unwrap_or("").to_string();

        let numinlets = json.get("numinlets")
            .and_then(|j| j.as_f64())
            .map(|n| n as u16)
            .unwrap_or(0);

        let numoutlets = json.get("numoutlets")
            .and_then(|j| j.as_f64())
            .map(|n| n as u16)
            .unwrap_or(0);

        // Parse embedded subpatcher. Real .gendsp files use the "patcher" key on the
        // box object (not "subpatcher"). Check "patcher" first, then "subpatcher".
        let subpatcher = match json.get("patcher").or_else(|| json.get("subpatcher")) {
            Some(sp_json) => {
                // The subpatcher JSON might be wrapped in {"patcher": {...}} or
                // be the patcher object directly. Normalize by checking for "patcher" key.
                let inner = sp_json.get("patcher").unwrap_or(sp_json);
                Some(Box::new(Patcher::from_json_value(inner)
                    .map_err(|e| format!("box '{}' subpatcher: {}", id, e))?))
            }
            None => None,
        };

        Ok(GBox {
            id,
            maxclass,
            text,
            code,
            numinlets,
            numoutlets,
            subpatcher,
        })
    }
}

impl Line {
    /// Extract a `Line` from a `patchline` JSON object.
    pub fn from_json(json: &Json) -> Result<Self, String> {
        let src_arr = json.get("source")
            .and_then(|j| j.as_arr())
            .ok_or_else(|| "patchline missing 'source' array".to_string())?;

        let dst_arr = json.get("destination")
            .and_then(|j| j.as_arr())
            .ok_or_else(|| "patchline missing 'destination' array".to_string())?;

        if src_arr.len() < 2 || dst_arr.len() < 2 {
            return Err("patchline source/destination must be [id, index]".to_string());
        }

        let src_id = src_arr[0].as_str()
            .ok_or_else(|| "patchline source[0] must be a string".to_string())?
            .to_string();
        let src_idx = src_arr[1].as_f64()
            .ok_or_else(|| "patchline source[1] must be a number".to_string())?
            as u16;

        let dst_id = dst_arr[0].as_str()
            .ok_or_else(|| "patchline destination[0] must be a string".to_string())?
            .to_string();
        let dst_idx = dst_arr[1].as_f64()
            .ok_or_else(|| "patchline destination[1] must be a number".to_string())?
            as u16;

        Ok(Line { src: (src_id, src_idx), dst: (dst_id, dst_idx) })
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json;

    #[test]
    fn from_json_minimal_patcher() {
        let src = r#"{
            "patcher": {
                "fileversion": 1,
                "classnamespace": "dsp.gen",
                "rect": [0, 0, 400, 300],
                "boxes": [
                    {"box": {"id": "obj-1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "outlettype": [""], "patching_rect": [50, 30, 30, 22], "text": "in 1"}},
                    {"box": {"id": "obj-2", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "patching_rect": [50, 220, 37, 22], "text": "out 1"}}
                ],
                "lines": [
                    {"patchline": {"source": ["obj-1", 0], "destination": ["obj-2", 0]}}
                ]
            }
        }"#;

        let j = json::parse(src).unwrap();
        let patcher = Patcher::from_json(&j).unwrap();

        assert_eq!(patcher.boxes.len(), 2);
        assert_eq!(patcher.lines.len(), 1);

        assert_eq!(patcher.boxes[0].id, "obj-1");
        assert_eq!(patcher.boxes[0].maxclass, "newobj");
        assert_eq!(patcher.boxes[0].text, "in 1");
        assert_eq!(patcher.boxes[0].numinlets, 0);
        assert_eq!(patcher.boxes[0].numoutlets, 1);
        assert_eq!(patcher.boxes[0].code, "");

        assert_eq!(patcher.boxes[1].id, "obj-2");
        assert_eq!(patcher.boxes[1].text, "out 1");

        assert_eq!(patcher.lines[0].src, ("obj-1".to_string(), 0));
        assert_eq!(patcher.lines[0].dst, ("obj-2".to_string(), 0));
    }

    #[test]
    fn from_json_codebox() {
        let src = r#"{
            "patcher": {
                "fileversion": 1,
                "classnamespace": "dsp.gen",
                "rect": [0, 0, 400, 300],
                "boxes": [
                    {"box": {"id": "obj-1", "maxclass": "codebox", "numinlets": 1, "numoutlets": 1, "outlettype": [""], "patching_rect": [50, 30, 200, 100], "code": "out1 = in1 * 2;"}}
                ],
                "lines": []
            }
        }"#;

        let j = json::parse(src).unwrap();
        let patcher = Patcher::from_json(&j).unwrap();

        assert_eq!(patcher.boxes.len(), 1);
        assert_eq!(patcher.boxes[0].maxclass, "codebox");
        assert_eq!(patcher.boxes[0].code, "out1 = in1 * 2;");
        assert_eq!(patcher.boxes[0].text, "");
    }

    #[test]
    fn from_json_missing_patcher() {
        let j = json::parse("{}").unwrap();
        assert!(Patcher::from_json(&j).is_err());
    }

    #[test]
    fn from_json_missing_boxes() {
        let src = r#"{"patcher": {"fileversion": 1, "rect": [0, 0, 400, 300]}}"#;
        let j = json::parse(src).unwrap();
        assert!(Patcher::from_json(&j).is_err());
    }

    #[test]
    fn from_json_conformance_all_examples() {
        let root = std::path::Path::new("reference/gen/examples");
        if !root.exists() {
            eprintln!("skipping: reference/ directory not available");
            return;
        }

        let mut count = 0;
        for entry in std::fs::read_dir(root).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("gendsp") {
                continue;
            }
            let content = std::fs::read(&path).unwrap();
            let j = crate::json::parse_embedded(&content)
                .unwrap_or_else(|e| panic!("{}: parse error: {}", path.display(), e));

            let patcher = Patcher::from_json(&j)
                .unwrap_or_else(|e| panic!("{}: model error: {}", path.display(), e));

            // Every box has valid id and maxclass
            for bx in &patcher.boxes {
                assert!(!bx.id.is_empty(), "{}: empty box id", path.display());
                assert!(!bx.maxclass.is_empty(), "{}: empty maxclass", path.display());
            }

            // Every line refers to existing box IDs
            for line in &patcher.lines {
                let has_src = patcher.boxes.iter().any(|b| b.id == line.src.0);
                let has_dst = patcher.boxes.iter().any(|b| b.id == line.dst.0);
                assert!(has_src, "{}: line source '{}' not found", path.display(), line.src.0);
                assert!(has_dst, "{}: line destination '{}' not found", path.display(), line.dst.0);
            }

            count += 1;
        }

        assert!(count > 0, "no .gendsp files found in reference/gen/examples");
    }
}
