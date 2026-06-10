use opengen_genexpr::{parse, lower};

#[test]
fn lowers_and_compiles_constant_expression() {
    let ast = parse("out1 = 0.5 + 0.25;").unwrap();
    let graph = lower(&ast).unwrap();
    
    // Compile and execute to verify correctness
    let mut patch = opengen_compile::compile(
        &graph,
        &opengen_ops::Registry::core(),
        48_000.0
    ).unwrap();
    
    let output = patch.process(&[]);
    assert_eq!(output, vec![0.75]);
}
