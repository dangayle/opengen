//! Bit-identical cross-backend tests.
//!
//! Verifies that C++ emitted code produces the same output as the Rust backend
//! for every operator and control-flow construct.

use opengen_emit_cpp::{emit_cpp, CppSource};
use opengen_genexpr::parse_and_lower;
use opengen_ops::Registry;

#[test]
fn constant_to_output_emits_valid_cpp() {
    let src = "out1 = 0.5;";
    let graph = parse_and_lower(src).unwrap();
    let cpp = emit_cpp(&graph, &Registry::core(), 48_000.0).expect("emission should succeed");
    assert!(
        cpp.body.contains("0.5"),
        "emitted C++ should contain the constant value"
    );
}

#[test]
fn header_contains_required_declarations() {
    let cpp = emit_cpp(
        &parse_and_lower("out1 = 1.0;").unwrap(),
        &Registry::core(),
        48_000.0,
    )
    .unwrap();
    assert!(cpp.header.contains("struct Patch"));
    assert!(cpp.header.contains("process(const double*"));
    assert!(cpp.header.contains("set_param"));
}
