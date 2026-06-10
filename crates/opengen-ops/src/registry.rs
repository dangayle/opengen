use opengen_ir::StateDecl;

/// A pure-Rust per-sample kernel: (inputs, state slots, samplerate) -> output.
pub type Kernel = fn(&[f64], &mut [f64], f64) -> f64;

pub struct OpDef {
    pub name: &'static str,
    pub arity: u16,
    pub state: StateDecl,
    /// If true, the compile layer emits a StateUpdate step copying `inputs[0] → state[0]`.
    /// If false, the kernel manages its own state internally.
    pub auto_state_update: bool,
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
        assert_eq!(op.auto_state_update, true);
        assert_eq!((op.kernel)(&[1.5, 2.25], &mut [], 48_000.0), 3.75);
    }
}
