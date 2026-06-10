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
        ("abs_fn", include_str!("fixtures/abs_fn.gendsp")),
        ("mr", include_str!("fixtures/mr.gendsp")),
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

// ─── Codebox abstraction-as-function tests (M2) ──────────────────────

/// codebox calls abstraction `y = abs_fn(in1); out1 = y;` with abs_fn.gendsp
/// (in 1 → ×param k default 2 → out 1) → in1 × 2.
#[test]
fn codebox_abstraction_basic() {
    let dir = std::env::temp_dir().join("opengen_test_codebox_abs");
    let _ = std::fs::create_dir_all(&dir);

    // Write abstraction file
    let abs_bytes = include_bytes!("fixtures/abs_fn.gendsp");
    std::fs::write(dir.join("abs_fn.gendsp"), abs_bytes).unwrap();

    // Write host file with codebox calling abs_fn
    let host_content = br#"{
        "patcher": {
            "fileversion": 1,
            "boxes": [
                {"box": {"id": "i1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                {"box": {"id": "cb", "maxclass": "codebox", "numinlets": 1, "numoutlets": 1, "code": "y = abs_fn(in1); out1 = y;"}},
                {"box": {"id": "o1", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
            ],
            "lines": [
                {"patchline": {"source": ["i1", 0], "destination": ["cb", 0]}},
                {"patchline": {"source": ["cb", 0], "destination": ["o1", 0]}}
            ]
        }
    }"#;
    let host_path = dir.join("host.gendsp");
    std::fs::write(&host_path, host_content).unwrap();

    let opts = opengen_gendsp::LoadOptions { search_paths: vec![dir.clone()] };
    let graph = opengen_gendsp::load_gendsp(&host_path, &opts).unwrap();
    let out = opengen_testkit::render_graph_with_inputs(&graph, 48000.0, &[&[7.0]], 1);
    assert_eq!(out.ch(0), &[14.0], "abs_fn with default k=2 should multiply by 2");

    let _ = std::fs::remove_dir_all(&dir);
}

/// named arg `y = abs_fn(in1, k=5);` → in1 × 5.
#[test]
fn codebox_abstraction_named_arg() {
    let dir = std::env::temp_dir().join("opengen_test_codebox_abs_named");
    let _ = std::fs::create_dir_all(&dir);

    let abs_bytes = include_bytes!("fixtures/abs_fn.gendsp");
    std::fs::write(dir.join("abs_fn.gendsp"), abs_bytes).unwrap();

    let host_content = br#"{
        "patcher": {
            "fileversion": 1,
            "boxes": [
                {"box": {"id": "i1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                {"box": {"id": "cb", "maxclass": "codebox", "numinlets": 1, "numoutlets": 1, "code": "y = abs_fn(in1, k=5); out1 = y;"}},
                {"box": {"id": "o1", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
            ],
            "lines": [
                {"patchline": {"source": ["i1", 0], "destination": ["cb", 0]}},
                {"patchline": {"source": ["cb", 0], "destination": ["o1", 0]}}
            ]
        }
    }"#;
    let host_path = dir.join("host.gendsp");
    std::fs::write(&host_path, host_content).unwrap();

    let opts = opengen_gendsp::LoadOptions { search_paths: vec![dir.clone()] };
    let graph = opengen_gendsp::load_gendsp(&host_path, &opts).unwrap();
    let out = opengen_testkit::render_graph_with_inputs(&graph, 48000.0, &[&[3.0]], 1);
    assert_eq!(out.ch(0), &[15.0], "abs_fn with k=5 should multiply by 5");

    let _ = std::fs::remove_dir_all(&dir);
}

/// multi-return `a, b = mr(in1);` → out1 = a + b.
#[test]
fn codebox_abstraction_multi_return() {
    let dir = std::env::temp_dir().join("opengen_test_codebox_mr");
    let _ = std::fs::create_dir_all(&dir);

    let mr_bytes = include_bytes!("fixtures/mr.gendsp");
    std::fs::write(dir.join("mr.gendsp"), mr_bytes).unwrap();

    let host_content = br#"{
        "patcher": {
            "fileversion": 1,
            "boxes": [
                {"box": {"id": "i1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                {"box": {"id": "cb", "maxclass": "codebox", "numinlets": 1, "numoutlets": 1, "code": "a, b = mr(in1); out1 = a + b;"}},
                {"box": {"id": "o1", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
            ],
            "lines": [
                {"patchline": {"source": ["i1", 0], "destination": ["cb", 0]}},
                {"patchline": {"source": ["cb", 0], "destination": ["o1", 0]}}
            ]
        }
    }"#;
    let host_path = dir.join("host.gendsp");
    std::fs::write(&host_path, host_content).unwrap();

    let opts = opengen_gendsp::LoadOptions { search_paths: vec![dir.clone()] };
    let graph = opengen_gendsp::load_gendsp(&host_path, &opts).unwrap();
    // mr: out1 = in1, out2 = in1 * 3. So a + b = in1 * 1 + in1 * 3 = in1 * 4.
    let out = opengen_testkit::render_graph_with_inputs(&graph, 48000.0, &[&[10.0]], 1);
    assert_eq!(out.ch(0), &[40.0], "mr should produce in1 + in1*3 = in1*4");

    let _ = std::fs::remove_dir_all(&dir);
}

/// control-flow codebox calling an abstraction → clear error.
#[test]
fn codebox_abstraction_control_flow_error() {
    let dir = std::env::temp_dir().join("opengen_test_codebox_cf");
    let _ = std::fs::create_dir_all(&dir);

    let abs_bytes = include_bytes!("fixtures/abs_fn.gendsp");
    std::fs::write(dir.join("abs_fn.gendsp"), abs_bytes).unwrap();

    let host_content = br#"{
        "patcher": {
            "fileversion": 1,
            "boxes": [
                {"box": {"id": "i1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                {"box": {"id": "cb", "maxclass": "codebox", "numinlets": 1, "numoutlets": 1, "code": "x = 0; if (in1 > 0.5) { x = abs_fn(in1); } out1 = x;"}},
                {"box": {"id": "o1", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
            ],
            "lines": [
                {"patchline": {"source": ["i1", 0], "destination": ["cb", 0]}},
                {"patchline": {"source": ["cb", 0], "destination": ["o1", 0]}}
            ]
        }
    }"#;
    let host_path = dir.join("host.gendsp");
    std::fs::write(&host_path, host_content).unwrap();

    let opts = opengen_gendsp::LoadOptions { search_paths: vec![dir.clone()] };
    let result = opengen_gendsp::load_gendsp(&host_path, &opts);
    match result {
        Err(e) => {
            let msg = e.to_string();
            assert!(msg.contains("if") || msg.contains("control"),
                "error should mention control flow issue: {}", msg);
            eprintln!("codebox abstraction control flow error: {}", msg);
        }
        Ok(_) => panic!("expected error for codebox with control flow + abstraction call, got Ok"),
    }

    let _ = std::fs::remove_dir_all(&dir);
}

/// unknown name with no resolver match → op-not-found error.
#[test]
fn codebox_abstraction_unknown_name() {
    let dir = std::env::temp_dir().join("opengen_test_codebox_unknown");
    let _ = std::fs::create_dir_all(&dir);

    let host_content = br#"{
        "patcher": {
            "fileversion": 1,
            "boxes": [
                {"box": {"id": "i1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                {"box": {"id": "cb", "maxclass": "codebox", "numinlets": 1, "numoutlets": 1, "code": "out1 = nonexistent(in1);"}},
                {"box": {"id": "o1", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
            ],
            "lines": [
                {"patchline": {"source": ["i1", 0], "destination": ["cb", 0]}},
                {"patchline": {"source": ["cb", 0], "destination": ["o1", 0]}}
            ]
        }
    }"#;
    let host_path = dir.join("host.gendsp");
    std::fs::write(&host_path, host_content).unwrap();

    let opts = opengen_gendsp::LoadOptions { search_paths: vec![dir.clone()] };
    let result = opengen_gendsp::load_gendsp(&host_path, &opts);
    match result {
        Err(e) => {
            let msg = e.to_string();
            // The resolver checks search paths, finds no match, returns Ok(None),
            // and the lowerer falls through to "unknown function" error.
            assert!(msg.contains("unknown function")
                || msg.contains("nonexistent"),
                "error should mention unknown function: {}", msg);
            eprintln!("unknown name error: {}", msg);
        }
        Ok(_) => panic!("expected error for unknown function in codebox, got Ok"),
    }

    let _ = std::fs::remove_dir_all(&dir);
}

// ─── Reference example file tests (skip-if-missing) ────────────────

const REF_EXAMPLES: &[&str] = &[
    "crossover.gendsp",
    "freeverb_allpass.gendsp",
    "freeverb_comb.gendsp",
    "freeverb.gendsp",
    "gen_resonator.gendsp",
    "waveguide_string.gendsp",
];

#[test]
fn load_all_reference_examples() {
    let root = std::path::Path::new("reference/gen/examples");
    if !root.exists() {
        eprintln!("skipping: reference/ directory not available");
        return;
    }
    let opts = opengen_gendsp::LoadOptions { search_paths: vec![root.to_path_buf()] };

    for filename in REF_EXAMPLES {
        let path = root.join(filename);
        if !path.exists() {
            eprintln!("skipping missing: {}", path.display());
            continue;
        }
        let graph = opengen_gendsp::load_gendsp(&path, &opts)
            .unwrap_or_else(|e| panic!("{}: {}", filename, e));
        let count = graph.nodes().count();
        assert!(count > 0, "{}: graph has no nodes", filename);
        eprintln!("{}: {} nodes", filename, count);
    }
}

#[test]
fn compile_crossover() {
    let root = std::path::Path::new("reference/gen/examples");
    if !root.exists() {
        eprintln!("skipping: reference/ directory not available");
        return;
    }
    let path = root.join("crossover.gendsp");
    if !path.exists() {
        eprintln!("skipping missing: crossover.gendsp");
        return;
    }
    let opts = opengen_gendsp::LoadOptions { search_paths: vec![root.to_path_buf()] };
    let graph = opengen_gendsp::load_gendsp(&path, &opts).unwrap();
    let mut patch = opengen_compile::compile(&graph, &opengen_ops::Registry::core(), 48000.0).unwrap();
    let frame = patch.process(&[1.0]);
    // Just verify it produces output without crashing
    assert!(frame.len() > 0, "crossover should have outputs");
    eprintln!("crossover compiled OK, {} outputs, first sample: {}", frame.len(), frame[0]);
}

#[test]
fn compile_freeverb_comb() {
    let root = std::path::Path::new("reference/gen/examples");
    if !root.exists() {
        eprintln!("skipping: reference/ directory not available");
        return;
    }
    let path = root.join("freeverb_comb.gendsp");
    if !path.exists() {
        eprintln!("skipping missing: freeverb_comb.gendsp");
        return;
    }
    let opts = opengen_gendsp::LoadOptions { search_paths: vec![root.to_path_buf()] };
    let graph = opengen_gendsp::load_gendsp(&path, &opts).unwrap();
    let mut patch = opengen_compile::compile(&graph, &opengen_ops::Registry::core(), 48000.0).unwrap();
    let frame = patch.process(&[1.0, 0.5, 0.3, 0.7]);
    assert!(frame.len() > 0, "freeverb_comb should have outputs");
    eprintln!("freeverb_comb compiled OK, {} outputs, first sample: {}", frame.len(), frame[0]);
}

// ─── Search path resolution test ───────────────────────────────────

#[test]
fn abstraction_search_path_resolution() {
    let dir = std::env::temp_dir().join("opengen_test_search_path");
    let _ = std::fs::create_dir_all(&dir);

    // Create a leaf abstraction file
    let leaf_path = dir.join("leaf.gendsp");
    let leaf_content = br#"{
        "patcher": {
            "fileversion": 1,
            "boxes": [
                {"box": {"id": "o1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                {"box": {"id": "o2", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
            ],
            "lines": [
                {"patchline": {"source": ["o1", 0], "destination": ["o2", 0]}}
            ]
        }
    }"#;
    std::fs::write(&leaf_path, leaf_content).unwrap();

    // Create a host file that references "leaf" as an abstraction
    let host_path = dir.join("host.gendsp");
    let host_content = format!(
        r#"{{
            "patcher": {{
                "fileversion": 1,
                "boxes": [
                    {{"box": {{"id": "i1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}}}},
                    {{"box": {{"id": "sub", "maxclass": "newobj", "numinlets": 1, "numoutlets": 1, "text": "leaf"}}}},
                    {{"box": {{"id": "o1", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}}}
                ],
                "lines": [
                    {{"patchline": {{"source": ["i1", 0], "destination": ["sub", 0]}}}},
                    {{"patchline": {{"source": ["sub", 0], "destination": ["o1", 0]}}}}
                ]
            }}
        }}"#
    );
    std::fs::write(&host_path, host_content.as_bytes()).unwrap();

    // Load with search_paths pointing to dir
    let opts = opengen_gendsp::LoadOptions { search_paths: vec![dir.clone()] };
    let graph = opengen_gendsp::load_gendsp(&host_path, &opts).unwrap();
    assert!(graph.nodes().count() > 0, "search path abstraction should work");

    // Render to verify signal passes through
    let out = opengen_testkit::render_graph_with_inputs(&graph, 48000.0, &[&[99.0]], 1);
    assert_eq!(out.ch(0), &[99.0], "abstraction via search path should passthrough");

    let _ = std::fs::remove_dir_all(&dir);
}

// ─── Include-cycle error test ──────────────────────────────────────

#[test]
fn include_cycle_detection() {
    let dir = std::env::temp_dir().join("opengen_test_cycle");
    let _ = std::fs::create_dir_all(&dir);

    // Create a.gendsp that references b
    let a_content = br#"{
        "patcher": {
            "fileversion": 1,
            "boxes": [
                {"box": {"id": "o1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                {"box": {"id": "sub", "maxclass": "newobj", "numinlets": 1, "numoutlets": 1, "text": "b"}},
                {"box": {"id": "o2", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
            ],
            "lines": [
                {"patchline": {"source": ["o1", 0], "destination": ["sub", 0]}},
                {"patchline": {"source": ["sub", 0], "destination": ["o2", 0]}}
            ]
        }
    }"#;
    std::fs::write(dir.join("a.gendsp"), a_content).unwrap();

    // Create b.gendsp that references a (forming a cycle)
    let b_content = br#"{
        "patcher": {
            "fileversion": 1,
            "boxes": [
                {"box": {"id": "o1", "maxclass": "newobj", "numinlets": 0, "numoutlets": 1, "text": "in 1"}},
                {"box": {"id": "sub", "maxclass": "newobj", "numinlets": 1, "numoutlets": 1, "text": "a"}},
                {"box": {"id": "o2", "maxclass": "newobj", "numinlets": 1, "numoutlets": 0, "text": "out 1"}}
            ],
            "lines": [
                {"patchline": {"source": ["o1", 0], "destination": ["sub", 0]}},
                {"patchline": {"source": ["sub", 0], "destination": ["o2", 0]}}
            ]
        }
    }"#;
    std::fs::write(dir.join("b.gendsp"), b_content).unwrap();

    // Load a.gendsp — should detect the cycle
    let path = dir.join("a.gendsp");
    let opts = opengen_gendsp::LoadOptions { search_paths: vec![dir.clone()] };
    let result = opengen_gendsp::load_gendsp(&path, &opts);
    match result {
        Err(opengen_gendsp::GendspError::Cycle(msg)) => {
            assert!(msg.contains("a") || msg.contains("b") || msg.contains("cycle"),
                "cycle error should mention cycle: {}", msg);
            eprintln!("cycle detected: {}", msg);
        }
        other => panic!("expected Cycle error, got: {:?}", other),
    }

    let _ = std::fs::remove_dir_all(&dir);
}
