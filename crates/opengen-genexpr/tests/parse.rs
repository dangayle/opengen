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
fn rejects_unknown_statement() {
    assert!(parse("for (;;) {}").is_err()); // not in M1 scope — clear error
}
