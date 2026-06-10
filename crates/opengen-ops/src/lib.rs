//! THE SPEC. One module per operator; rustdoc is the normative definition, doctests are the executable spec.

pub mod registry;
pub mod math;
pub mod compare;
pub mod range;
pub use registry::{Registry, OpDef, Kernel};
