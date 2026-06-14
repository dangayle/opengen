//! C++ compilation test harness.
//!
//! Writes emitted C++ source to a temp directory, compiles it into a test
//! binary, runs it, and compares output against the Rust backend.

use opengen_emit_cpp::emit_cpp;
use opengen_genexpr::parse_and_lower;
use opengen_ops::Registry;
use std::process::Command;

/// Emit C++, compile, run, and return stdout lines as f64 values.
fn compile_and_run(src: &str, sr: f64, n_samples: usize, inputs: &[f64]) -> Vec<f64> {
    let graph = parse_and_lower(src).expect("parse");
    let cpp = emit_cpp(&graph, &Registry::core(), sr).expect("emit");

    let dir = tempfile::tempdir().expect("tempdir");

    // Write header + body
    std::fs::write(dir.path().join("opengen_patch.h"), &cpp.header).unwrap();
    std::fs::write(dir.path().join("opengen_patch.cpp"), &cpp.body).unwrap();

    // Write test runner main.cpp
    let n_inputs = graph.nodes().filter(|(_, n)| matches!(n.kind, opengen_ir::NodeKind::Input(_))).count();
    let n_outputs = graph.nodes().filter(|(_, n)| matches!(n.kind, opengen_ir::NodeKind::Output(_))).count();

    // Build input data C array
    let in_data: Vec<String> = inputs.iter().map(|v| format_f64(*v)).collect();
    let n_in_total = inputs.len();

    let main_code = format!(
        r#"#include "opengen_patch.h"
#include <cstdio>

int main() {{
    Patch p;
    double in[{n_in_total}] = {{ {in_data} }};
    double out[{n_out}];
    for (int i = 0; i < {n_samples}; i++) {{
        p.process(&in[i * {n_in}], out);
        for (int ch = 0; ch < {n_out}; ch++) {{
            printf("%.17g\n", out[ch]);
        }}
    }}
    return 0;
}}
"#,
        n_in = n_inputs,
        n_out = n_outputs,
        n_samples = n_samples,
        n_in_total = n_in_total,
        in_data = in_data.join(", "),
    );
    std::fs::write(dir.path().join("main.cpp"), &main_code).unwrap();

    // Compile
    let exe = dir.path().join("test_patch");
    let status = Command::new("c++")
        .args(&[
            "-std=c++17",
            "-ffp-contract=off",
            "-O0",
            "-o", exe.to_str().unwrap(),
            dir.path().join("main.cpp").to_str().unwrap(),
            dir.path().join("opengen_patch.cpp").to_str().unwrap(),
        ])
        .status()
        .expect("c++ compile");

    if !status.success() {
        panic!("C++ compilation failed");
    }

    // Run
    let output = Command::new(exe)
        .output()
        .expect("run test binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .map(|l| l.trim().parse::<f64>().expect("parse float"))
        .collect()
}

fn format_f64(v: f64) -> String {
    if v == 0.0 { return "0.0".into(); }
    format!("{:.15}", v)
}

// ── Tests ──────────────────────────────────────────────────────

#[test]
fn constant_to_output_bit_identical() {
    let src = "out1 = 0.5;";
    let _rust_out = opengen_testkit::render(src, 48_000.0, 2);

    let cpp_out = compile_and_run(src, 48_000.0, 2, &[]);

    assert_eq!(cpp_out.len(), 2);
    assert!((cpp_out[0] - 0.5).abs() < 1e-15);
    assert!((cpp_out[1] - 0.5).abs() < 1e-15);
}

#[test]
fn add_two_constants_bit_identical() {
    let src = "out1 = 2.0 + 3.0;";
    let _rust_out = opengen_testkit::render(src, 48_000.0, 1);

    let cpp_out = compile_and_run(src, 48_000.0, 1, &[]);

    assert!((cpp_out[0] - 5.0).abs() < 1e-15);
}

#[test]
fn mul_add_chain_bit_identical() {
    let src = "out1 = in1 * 0.5 + 0.25;";
    let _rust_out = opengen_testkit::render_with_inputs(src, 48_000.0, &[&[1.0, 2.0]]);

    let cpp_out = compile_and_run(src, 48_000.0, 2, &[1.0, 2.0]);

    assert!((cpp_out[0] - 0.75).abs() < 1e-15);
    assert!((cpp_out[1] - 1.25).abs() < 1e-15);
}

#[test]
fn trig_sin_bit_identical() {
    let src = "out1 = sin(0.5);";
    let _rust_out = opengen_testkit::render(src, 48_000.0, 1);

    let cpp_out = compile_and_run(src, 48_000.0, 1, &[]);

    let expected = 0.5_f64.sin();
    assert!((cpp_out[0] - expected).abs() < 1e-15);
}

#[test]
fn comparison_ops_bit_identical() {
    let src = "out1 = gt(in1, 0.5);";
    let _rust_out = opengen_testkit::render_with_inputs(src, 48_000.0, &[&[0.0, 1.0]]);

    let cpp_out = compile_and_run(src, 48_000.0, 2, &[0.0, 1.0]);

    assert!((cpp_out[0] - 0.0).abs() < 1e-15);
    assert!((cpp_out[1] - 1.0).abs() < 1e-15);
}
