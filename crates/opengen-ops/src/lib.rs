//! THE SPEC. One module per operator; rustdoc is the normative definition, doctests are the executable spec.

pub mod registry;
pub mod math;
pub mod compare;
pub mod range;
pub mod state;
pub mod osc;
pub mod trig;
pub mod convert;
pub mod samplerate;
pub mod memory;
pub mod filter;
pub mod sample;
pub mod selector;
pub mod gate;
pub mod elapsed;
pub mod wave;
pub mod step_smoothstep_rmod;
pub use registry::{Registry, OpDef, Kernel, UpdateFn, InitFn};
