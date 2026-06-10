#[test]
fn probe_records_interior_wire() {
    let graph = opengen_genexpr::parse_and_lower("h = history(h + 1); out1 = h * 2;").unwrap();
    let mut patch = opengen_compile::compile_with_probes(
        &graph, &opengen_ops::Registry::core(), 48_000.0, &["h"]).unwrap();
    for _ in 0..3 { patch.process(&[]); }
    assert_eq!(patch.probe("h").unwrap(), &[0.0, 1.0, 2.0]);
}

#[test]
fn probe_unknown_name_errors() {
    let graph = opengen_genexpr::parse_and_lower("out1 = 1.0;").unwrap();
    let result = opengen_compile::compile_with_probes(
        &graph, &opengen_ops::Registry::core(), 48_000.0, &["nope"]);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("nope"), "error message should mention 'nope': {}", err_msg);
}

#[test]
fn batch_probe_retrieval() {
    let graph = opengen_genexpr::parse_and_lower(
        "a = history(a + 1); b = a * 2; out1 = b;").unwrap();
    let mut patch = opengen_compile::compile_with_probes(
        &graph, &opengen_ops::Registry::core(), 48_000.0, &["a", "b"]).unwrap();
    for _ in 0..3 { patch.process(&[]); }
    let mut names = patch.probe_names();
    names.sort();
    assert_eq!(names, vec!["a", "b"]);
    assert_eq!(patch.probe("a").unwrap(), &[0.0, 1.0, 2.0]);
    assert_eq!(patch.probe("b").unwrap(), &[0.0, 2.0, 4.0]);
}
