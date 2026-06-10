//! Integration tests: load `.gendsp` fixture files, build a Graph, compile,
//! render, and assert exact outputs.

use opengen_gendsp::build;
use opengen_gendsp::json;
use opengen_gendsp::model;
use opengen_ops::Registry;
use opengen_testkit::render_graph_with_inputs;

/// minimal: in 1 → `* 0.5` → out 1 — exact halving.
#[test]
fn fixture_minimal_halving() {
    let content = include_str!("fixtures/minimal.gendsp");
    let j = json::parse(content).unwrap();
    let patcher = model::Patcher::from_json(&j).unwrap();
    let graph = build::build_graph(&patcher, &Registry::core()).unwrap();
    let out = render_graph_with_inputs(&graph, 48000.0, &[&[2.0, 4.0, 6.0]], 3);
    assert_eq!(out.ch(0), &[1.0, 2.0, 3.0]);
}

/// param_arg: `param g 3` + `* g` (param name as arg) → in1 × 3.
#[test]
fn fixture_param_times_three() {
    let content = include_str!("fixtures/param_arg.gendsp");
    let j = json::parse(content).unwrap();
    let patcher = model::Patcher::from_json(&j).unwrap();
    let graph = build::build_graph(&patcher, &Registry::core()).unwrap();
    let out = render_graph_with_inputs(&graph, 48000.0, &[&[5.0]], 1);
    assert_eq!(out.ch(0), &[15.0]); // 5 * 3
}

/// expr_arg: `* twopi/samplerate` — expression arg lowered in situ.
#[test]
fn fixture_expr_arg_twopi_over_sr() {
    let content = include_str!("fixtures/expr_arg.gendsp");
    let j = json::parse(content).unwrap();
    let patcher = model::Patcher::from_json(&j).unwrap();
    let graph = build::build_graph(&patcher, &Registry::core()).unwrap();
    let expected_factor = std::f64::consts::TAU / 48000.0;
    let out = render_graph_with_inputs(&graph, 48000.0, &[&[1.0]], 1);
    assert!((out.ch(0)[0] - expected_factor).abs() < 1e-15);
}

/// bus: `s mybus` / `r mybus` aliases roundtrip a signal.
#[test]
fn fixture_bus_send_receive_aliases() {
    let content = include_str!("fixtures/bus.gendsp");
    let j = json::parse(content).unwrap();
    let patcher = model::Patcher::from_json(&j).unwrap();
    let graph = build::build_graph(&patcher, &Registry::core()).unwrap();
    let out = render_graph_with_inputs(&graph, 48000.0, &[&[42.0]], 1);
    assert_eq!(out.ch(0), &[42.0]);
}

/// codebox: embedded `out1 = in1 + 1;` code.
#[test]
fn fixture_codebox_plus_one() {
    let content = include_str!("fixtures/codebox.gendsp");
    let j = json::parse(content).unwrap();
    let patcher = model::Patcher::from_json(&j).unwrap();
    let graph = build::build_graph(&patcher, &Registry::core()).unwrap();
    let out = render_graph_with_inputs(&graph, 48000.0, &[&[5.0, 10.0]], 2);
    assert_eq!(out.ch(0), &[6.0, 11.0]);
}

/// setparam: setparam g drives param g consumers from the driving signal.
#[test]
fn fixture_setparam_drives_consumers() {
    let content = include_str!("fixtures/setparam.gendsp");
    let j = json::parse(content).unwrap();
    let patcher = model::Patcher::from_json(&j).unwrap();
    let graph = build::build_graph(&patcher, &Registry::core()).unwrap();
    // With in1 = 2:
    //   in1 drives setparam g → g is now 2 (from driving signal)
    //   in1 → * inlet 0
    //   param g (rewired via setparam from in1) → * inlet 1
    //   out1 = 2 * 2 = 4
    let out = render_graph_with_inputs(&graph, 48000.0, &[&[2.0]], 1);
    assert_eq!(out.ch(0), &[4.0]);
}

/// End-to-end: verify every fixture at least loads and renders without error.
#[test]
fn fixture_every_fixture_loads_and_renders() {
    let fixtures: &[(&str, &str)] = &[
        ("minimal", include_str!("fixtures/minimal.gendsp")),
        ("param_arg", include_str!("fixtures/param_arg.gendsp")),
        ("expr_arg", include_str!("fixtures/expr_arg.gendsp")),
        ("bus", include_str!("fixtures/bus.gendsp")),
        ("codebox", include_str!("fixtures/codebox.gendsp")),
        ("setparam", include_str!("fixtures/setparam.gendsp")),
    ];
    for (name, content) in fixtures {
        let j = json::parse(content)
            .unwrap_or_else(|e| panic!("{}: JSON parse error: {}", name, e));
        let patcher = model::Patcher::from_json(&j)
            .unwrap_or_else(|e| panic!("{}: model error: {}", name, e));
        let graph = build::build_graph(&patcher, &Registry::core())
            .unwrap_or_else(|e| panic!("{}: build error: {}", name, e));
        let _out = render_graph_with_inputs(&graph, 48000.0, &[&[1.0]], 1);
    }
}
