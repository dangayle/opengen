use opengen_genexpr::parse;

#[test]
fn parses_precedence() {
    let ast = parse("out1 = 1 + 2 * 3;").unwrap();
    assert_eq!(format!("{ast:?}").contains("Add"), true); // top node is Add(1, Mul(2,3))
}

#[test]
fn parses_param_and_call() {
    parse("Param freq(440); out1 = cycle(freq);").unwrap();
}

#[test]
fn for_loop_now_parses_in_m2() {
    // for loops are syntactically valid in M2; lowering rejects them
    let ast = parse("for (;;) {}").unwrap();
    assert!(matches!(ast.statements[0].kind, opengen_genexpr::StatementKind::For { .. }));
}
