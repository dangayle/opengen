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

/// Compile `src` and render with per-channel input samples.
/// `n` = longest input channel. Short channels are zero-padded.
/// If the graph declares more inputs than provided channels, missing inputs default to 0.0.
/// Extra provided channels are ignored.
/// Panics on compile error — tests want loud failures.
pub fn render_with_inputs(src: &str, sr: f64, inputs: &[&[f64]]) -> Render {
    let n = inputs.iter().map(|c| c.len()).max().unwrap_or(0);
    render_with_inputs_n(src, sr, inputs, n)
}

/// Like `render_with_inputs` but renders exactly `n` samples (zero-padding all inputs).
pub fn render_with_inputs_n(src: &str, sr: f64, inputs: &[&[f64]], n: usize) -> Render {
    let graph = opengen_genexpr::parse_and_lower(src).expect("parse");
    render_graph_with_inputs(&graph, sr, inputs, n)
}

/// Render an already-lowered Graph with inputs.
/// Use this when you already have a lowered `Graph` (e.g., from a non-genexpr source
/// like `.gendsp`). For genexpr source strings, use `render_with_inputs` or
/// `render_with_inputs_n` instead.
pub fn render_graph_with_inputs(
    graph: &opengen_ir::Graph,
    sr: f64,
    inputs: &[&[f64]],
    n: usize,
) -> Render {
    let mut patch = compile(graph, &opengen_ops::Registry::core(), sr).expect("compile");
    let outs = patch.output_count();
    let mut channels = vec![Vec::with_capacity(n); outs];
    let mut frame_in = vec![0.0; inputs.len()];
    for i in 0..n {
        for (k, ch) in inputs.iter().enumerate() {
            frame_in[k] = ch.get(i).copied().unwrap_or(0.0);
        }
        let frame = patch.process(&frame_in);
        for (c, v) in channels.iter_mut().zip(frame) {
            c.push(v);
        }
    }
    Render { channels }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_with_inputs_feeds_channels() {
        // out1 = in1 + in2, driven by two 3-sample channels
        let out = render_with_inputs(
            "out1 = in1 + in2;",
            48_000.0,
            &[&[1.0, 2.0, 3.0], &[10.0, 20.0, 30.0]],
        );
        assert_eq!(out.ch(0), &[11.0, 22.0, 33.0]);
    }

    #[test]
    fn render_with_inputs_short_channel_pads_zero() {
        let out = render_with_inputs("out1 = in1;", 48_000.0, &[&[1.0]]);
        assert_eq!(out.ch(0), &[1.0]);
        let out = render_with_inputs_n("out1 = in1;", 48_000.0, &[&[1.0]], 3);
        assert_eq!(out.ch(0), &[1.0, 0.0, 0.0]);
    }

    #[test]
    fn render_with_inputs_missing_channel_defaults_zero() {
        // Graph expects in1 + in2, but only in1 provided
        let out = render_with_inputs("out1 = in1 + in2;", 48_000.0, &[&[5.0]]);
        assert_eq!(out.ch(0)[0], 5.0); // in2 defaults to 0
    }
}
