//! GenDSP patcher model: parse `.gendsp` JSON files into a typed Patcher graph.
//!
//! This crate provides the data model and parsers for the `.gendsp` file format
//! used by Cycling '74's gen~, gen, jit.gen, and jit.pix. It reconstructs the
//! typed component graph from the JSON serialisation format.
//!
//! # Organisation
//!
//! - `json` — Minimal zero-dependency JSON parser (amxd-embedded aware)
//! - `model` — Typed `Patcher`, `GBox`, `Line` models extracted from JSON
//! - `boxtext` — Box-text tokeniser and classifier (operator vs special object)
//! - `build` — Graph builder: convert a `Patcher` into an `opengen_ir::Graph`
//! - `flatten` — Subpatcher flattening, abstraction resolution, file loading

pub mod json;
pub mod model;
pub mod boxtext;
pub mod build;
pub mod flatten;

use std::path::Path;

pub use flatten::{GendspError, LoadOptions, ResolveCtx, build_graph_with};

/// Load a `.gendsp` file, parse it, and build an IR Graph with subpatcher/
/// abstraction resolution.
///
/// Resolution order per box:
/// 1. Embedded `patcher` (subpatcher) in the box JSON
/// 2. `<name>.gendsp` sibling file in the loaded file's directory
/// 3. Each entry in `opts.search_paths`
///
/// # Errors
///
/// Returns `GendspError` for I/O errors, JSON parse failures, cycle detection,
/// or graph-building errors (unknown operators, invalid wiring, etc.).
pub fn load_gendsp(path: &Path, opts: &LoadOptions) -> Result<opengen_ir::Graph, GendspError> {
    let bytes = std::fs::read(path).map_err(GendspError::Io)?;
    parse_gendsp_bytes(&bytes, path.parent(), opts)
}

/// Parse `.gendsp` bytes into an IR Graph, with abstraction resolution.
///
/// `base_dir` specifies the directory for sibling file resolution.
/// Pass `None` if the bytes came from an in-memory source (abstraction
/// sibling resolution will use `search_paths` from `opts` only).
pub fn parse_gendsp_bytes(
    bytes: &[u8],
    base_dir: Option<&Path>,
    opts: &LoadOptions,
) -> Result<opengen_ir::Graph, GendspError> {
    let j = crate::json::parse_embedded(bytes)
        .map_err(|e| GendspError::Json(e.to_string()))?;
    let patcher = crate::model::Patcher::from_json(&j)
        .map_err(GendspError::Build)?;
    let mut resolve_ctx = ResolveCtx::new(
        opts.search_paths.clone(),
        base_dir.map(|p| p.to_path_buf()),
    );
    build_graph_with(&patcher, &opengen_ops::Registry::core(), &mut resolve_ctx)
}
