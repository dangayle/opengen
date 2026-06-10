use opengen_genexpr::parse;

#[test]
fn comments_and_numeric_forms() {
    // line + block comments, trailing-dot and leading-dot floats, sci notation
    parse("// line comment\n/* block\ncomment */\nout1 = 1. + .5 + 1e-3 + 2.5E+2;").unwrap();
}

#[test]
#[ignore = "parser v2 (Task 12)"]
fn new_operator_tokens_parse() {
    parse("out1 = (1 < 2) && !(3 > 4) || (1 ^^ 0);").unwrap();
    parse("out1 = (5 & 3) | (1 ^ 2) + (1 << 2) + (8 >> 1);").unwrap();
    parse("x = 1; x += 2; x -= 1; x *= 3; x /= 2; x %= 2; out1 = x;").unwrap();
    parse("out1 = 1 ? 2 : 3;").unwrap();
}

#[test]
fn unterminated_block_comment_errors() {
    let err = parse("/* unterminated block comment").unwrap_err();
    assert!(
        err.msg.contains("unterminated"),
        "error msg should mention unterminated: {}",
        err.msg
    );
}
