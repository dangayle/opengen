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
        ("delay_echo", include_str!("fixtures/delay_echo.gendsp")),
        ("bus_multi_send_sums", include_str!("fixtures/bus_multi_send_sums.gendsp")),
        ("codebox_with_control_flow", include_str!("fixtures/codebox_with_control_flow.gendsp")),
        ("mc_channel_constant_one", include_str!("fixtures/mc_channel_constant_one.gendsp")),
        ("history_named_e2e", include_str!("fixtures/history_named_e2e.gendsp")),
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

// ── New e2e tests (Task 24) ─────────────────────────────────────────

/// delay_echo: delay 4 with tap=1 → 1-sample echo [0,1,0] for input [1,0,0].
#[test]
fn fixture_delay_echo() {
    let content = include_str!("fixtures/delay_echo.gendsp");
    let j = json::parse(content).unwrap();
    let patcher = model::Patcher::from_json(&j).unwrap();
    let graph = build::build_graph(&patcher, &Registry::core()).unwrap();
    let out = render_graph_with_inputs(&graph, 48000.0, &[&[1.0, 0.0, 0.0]], 3);
    assert_eq!(out.ch(0), &[0.0, 1.0, 0.0]);
}

/// delay multi-tap: delay 8 2 → clear M3 error.
#[test]
fn fixture_delay_multi_tap_errors() {
    let content = include_str!("fixtures/delay_multi_tap.gendsp");
    let j = json::parse(content).unwrap();
    let patcher = model::Patcher::from_json(&j).unwrap();
    let result = build::build_graph(&patcher, &Registry::core());
    assert!(result.is_err(), "multi-tap delay should error");
    let err = result.unwrap_err();
    assert!(err.contains("M3"), "error should mention M3: {}", err);
}

/// bus_multi_send_sums: two sends to one bus name + one receive → output = sum.
#[test]
fn fixture_bus_multi_send_sums() {
    let content = include_str!("fixtures/bus_multi_send_sums.gendsp");
    let j = json::parse(content).unwrap();
    let patcher = model::Patcher::from_json(&j).unwrap();
    let graph = build::build_graph(&patcher, &Registry::core()).unwrap();
    // in1 drives 1st send (obj-2), f 3 drives 2nd send (obj-3), receive sums them
    let out = render_graph_with_inputs(&graph, 48000.0, &[&[10.0]], 1);
    assert_eq!(out.ch(0), &[13.0]); // 10.0 + 3.0
}

/// codebox_with_control_flow: if/else in codebox renders correctly.
#[test]
fn fixture_codebox_with_control_flow() {
    let content = include_str!("fixtures/codebox_with_control_flow.gendsp");
    let j = json::parse(content).unwrap();
    let patcher = model::Patcher::from_json(&j).unwrap();
    let graph = build::build_graph(&patcher, &Registry::core()).unwrap();
    // in1=0.75 > 0.5 → x = 0.75*2 = 1.5; in1=0.25 ≤ 0.5 → x = 0-1 = -1.0
    let out = render_graph_with_inputs(&graph, 48000.0, &[&[0.75, 0.25]], 2);
    assert_eq!(out.ch(0), &[1.5, -1.0]);
}

/// mc_channel_constant_one: mc_channel feeds out 1 → 1.0.
#[test]
fn fixture_mc_channel_constant_one() {
    let content = include_str!("fixtures/mc_channel_constant_one.gendsp");
    let j = json::parse(content).unwrap();
    let patcher = model::Patcher::from_json(&j).unwrap();
    let graph = build::build_graph(&patcher, &Registry::core()).unwrap();
    let out = render_graph_with_inputs(&graph, 48000.0, &[&[0.0]], 1);
    assert_eq!(out.ch(0), &[1.0]);
}

/// history_named_e2e: named history h1 — compile_with_probes succeeds.
#[test]
fn fixture_history_named_e2e() {
    let content = include_str!("fixtures/history_named_e2e.gendsp");
    let j = json::parse(content).unwrap();
    let patcher = model::Patcher::from_json(&j).unwrap();
    let graph = build::build_graph(&patcher, &Registry::core()).unwrap();
    // compile_with_probes on a named history binding should succeed
    let mut patch = opengen_compile::compile_with_probes(
        &graph, &Registry::core(), 48000.0, &["h1"],
    ).expect("compile_with_probes should succeed for named history h1");
    // Process a few samples to exercise the history
    for &v in &[1.0, 0.0] {
        patch.process(&[v]);
    }
    let trace = patch.probe("h1").unwrap();
    // h1 starts at 0, first sample: history(in1=1.0) → 0, second: history(in1=0.0) → 1.0
    assert_eq!(trace.len(), 2);
}
