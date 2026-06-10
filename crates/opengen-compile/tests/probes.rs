#[test]
fn probe_records_interior_wire() {
    let graph = opengen_genexpr::parse_and_lower("h = history(h + 1); out1 = h * 2;").unwrap();
    let mut patch = opengen_compile::compile_with_probes(
        &graph, &opengen_ops::Registry::core(), 48_000.0, &["h"]).unwrap();
    for _ in 0..3 { patch.process(&[]); }
    assert_eq!(patch.probe("h").unwrap(), &[0.0, 1.0, 2.0]);
}
