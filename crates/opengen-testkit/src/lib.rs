//! Doctest/test façade: compile GenExpr source and render n samples.
use opengen_compile::compile;

pub struct Render {
    channels: Vec<Vec<f64>>,
}

impl Render {
    pub fn ch(&self, i: usize) -> &[f64] {
        &self.channels[i]
    }
}

/// Compile `src` at samplerate `sr` and render `n` samples (no inputs).
/// Panics on compile error — doctests want loud failures.
pub fn render(src: &str, sr: f64, n: usize) -> Render {
    let graph = opengen_genexpr::parse_and_lower(src).expect("parse");
    let mut patch = compile(&graph, &opengen_ops::Registry::core(), sr).expect("compile");
    let outs = patch.output_count();
    let mut channels = vec![Vec::with_capacity(n); outs];
    for _ in 0..n {
        let frame = patch.process(&[]);
        for (c, v) in channels.iter_mut().zip(frame) {
            c.push(v);
        }
    }
    Render { channels }
}
