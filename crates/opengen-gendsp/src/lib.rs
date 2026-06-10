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

pub mod json;
pub mod model;
pub mod boxtext;
