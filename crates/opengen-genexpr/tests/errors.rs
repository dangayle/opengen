use opengen_genexpr::parse;

#[test]
fn parse_error_carries_line_and_col() {
    let err = parse("out1 = 1 +;\n").unwrap_err();
    assert_eq!(err.loc.map(|l| l.line), Some(1));
    assert!(err.loc.unwrap().col >= 10);
    assert!(err.to_string().contains("1:"), "display includes location: {err}");
}

#[test]
fn lower_error_carries_statement_location() {
    let ast = parse("out1 = 1;\nx = bogus_op(2);").unwrap();
    let err = opengen_genexpr::lower(&ast).unwrap_err();
    assert_eq!(err.loc.map(|l| l.line), Some(2));
}
