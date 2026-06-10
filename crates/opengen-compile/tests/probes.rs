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
