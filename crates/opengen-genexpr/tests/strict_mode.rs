//! Strict-mode tests: enforces gen~ declarator ordering.
//!
//! In gen~, declarations must precede expression statements at the top level.
//! `parse_strict` validates this after a successful parse.

use opengen_genexpr;

#[test]
fn strict_mode_rejects_decl_after_expression() {
    let src = "out1 = in1;\nh = history(in1);";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("declaration after expression"),
        "expected 'declaration after expression' in error, got: {}",
        err
    );
}

#[test]
fn strict_mode_allows_decl_before_expr() {
    let src = "h = history(in1);\nout1 = h;";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_ok(), "strict should accept decl before expr: {:?}", result.err());
}

#[test]
fn lenient_mode_allows_decl_after_expr() {
    let src = "out1 = in1;\nh = history(in1);";
    let result = opengen_genexpr::parse(src);
    assert!(
        result.is_ok(),
        "lenient parse should accept decl after expr: {:?}",
        result.err()
    );
}

#[test]
fn strict_mode_rejects_typed_decl_after_expr() {
    let src = "out1 = in1;\nHistory h(0);";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_err());
}

#[test]
fn strict_mode_allows_only_decls() {
    let src = "h = history(in1);\np = 0.5;\nd = delay(1024);";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_ok());
}

#[test]
fn strict_mode_allows_only_exprs() {
    let src = "out1 = in1 * 0.5;\nout2 = in1 + 0.25;";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_ok());
}

// ── Task 6: Self-referential history lint ──────────────────────

#[test]
fn strict_mode_rejects_self_referential_history() {
    let src = "h = history(h + 1);";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("not defined"),
        "expected 'not defined' in error, got: {}",
        err
    );
}

#[test]
fn strict_mode_allows_non_self_referential_history() {
    let src = "x = in1;\nh = history(x);\nout1 = h;";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_ok());
}

#[test]
fn strict_mode_allows_history_with_constant() {
    let src = "h = history(0);\nout1 = h;";
    let result = opengen_genexpr::parse_strict(src);
    assert!(result.is_ok());
}
