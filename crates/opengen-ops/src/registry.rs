use opengen_ir::StateDecl;

/// A pure-Rust per-sample kernel: (inputs, state slots, samplerate) -> output.
/// Kernels MUST NOT read input values arriving on `deferred_ports` (update-phase only).
pub type Kernel = fn(&[f64], &mut [f64], f64) -> f64;
/// End-of-sample state update: (inputs, state slots, samplerate).
/// Runs after ALL Compute steps, in ascending NodeId order (determinism contract).
pub type UpdateFn = fn(&[f64], &mut [f64], f64);
/// One-time state initializer at compile: (IR node args, state slots, samplerate).
pub type InitFn = fn(&[f64], &mut [f64], f64);

pub struct OpDef {
    pub name: &'static str,
    pub arity: u16,
    pub state: StateDecl,
    /// Input ports whose incoming edges do NOT block topological scheduling
    /// (the "write" ports of feedback-capable ops: history port 0, delay port 0).
    pub deferred_ports: &'static [u16],
    /// End-of-sample state writer; None = stateless or kernel-managed state.
    pub update: Option<UpdateFn>,
    /// One-time state initializer from IR node args; None = zero-init.
    pub init: Option<InitFn>,
    pub kernel: Kernel,
}

pub struct Registry { ops: std::collections::HashMap<&'static str, OpDef> }

impl Registry {
    pub fn core() -> Self {
        let mut ops = std::collections::HashMap::new();
        for def in crate::math::defs() { ops.insert(def.name, def); }
        for def in crate::compare::defs() { ops.insert(def.name, def); }
        for def in crate::range::defs() { ops.insert(def.name, def); }
        for def in crate::state::defs() { ops.insert(def.name, def); }
        for def in crate::osc::defs() { ops.insert(def.name, def); }
        for def in crate::trig::defs() { ops.insert(def.name, def); }
        for def in crate::convert::defs() { ops.insert(def.name, def); }
        for def in crate::bitwise::defs() { ops.insert(def.name, def); }
        for def in crate::memory::defs() { ops.insert(def.name, def); }
        for def in crate::filter::defs() { ops.insert(def.name, def); }
        Registry { ops }
    }
    pub fn get(&self, name: &str) -> Option<&OpDef> { self.ops.get(name) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_resolves_add() {
        let reg = Registry::core();
        let op = reg.get("add").expect("add registered");
        assert_eq!(op.arity, 2);
        assert_eq!(op.state, opengen_ir::StateDecl::None);
        assert!(op.update.is_none());
        assert!(op.deferred_ports.is_empty());
        assert_eq!((op.kernel)(&[1.5, 2.25], &mut [], 48_000.0), 3.75);
    }
}
