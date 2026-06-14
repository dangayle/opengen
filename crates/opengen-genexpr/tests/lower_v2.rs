//! Lowering v2 tests: ternary/logical, constants, History decls, samplerate, shadows.
//! Bitwise operators were removed (gen~ does not support them, 2026-06-13).
//!
//! TDD: each test covers one row of the M2 lowering table. Write → observe fail → implement → pass.

use opengen_genexpr::{parse, lower};

/// Helper: parse and lower, unwrapping result.
fn parse_and_lower(src: &str) -> Result<opengen_ir::Graph, String> {
    let ast = parse(src).map_err(|e| e.to_string())?;
    lower(&ast).map_err(|e| e.to_string())
}

/// Helper: parse, lower, compile, render, return output channel.
fn render(src: &str, n: usize) -> Vec<f64> {
    let graph = parse_and_lower(src).unwrap();
    let mut patch = opengen_compile::compile(
        &graph,
        &opengen_ops::Registry::core(),
        48000.0,
    ).unwrap();
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let frame = patch.process(&[]);
        out.push(frame[0]);
    }
    out
}

// ═══════════════════════════════════════════════════════════════════
//  Ternary
// ═══════════════════════════════════════════════════════════════════

#[test]
fn ternary_selects_true_branch() {
    // cond is nonzero → selects true_expr
    let out = render("out1 = 1 ? 100 : 200;", 1);
    assert_eq!(out, vec![100.0]);
}

#[test]
fn ternary_selects_false_branch() {
    let out = render("out1 = 0 ? 100 : 200;", 1);
    assert_eq!(out, vec![200.0]);
}

#[test]
fn ternary_eager_evaluation() {
    // Both branches always evaluated (dataflow). This test verifies the switch op
    // produces correct output regardless.
    let out = render("out1 = (2 > 1) ? 3.5 + 1.5 : 10.0 / 2.0;", 1);
    assert_eq!(out, vec![5.0]);
}

// ═══════════════════════════════════════════════════════════════════
//  Logical operators (&&, ||, ! via BinOp/Unary → existing and/or/not ops)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn logical_and_true() {
    let out = render("out1 = 1 && 2;", 1);
    assert_eq!(out, vec![1.0]);
}

#[test]
fn logical_and_false() {
    let out = render("out1 = 1 && 0;", 1);
    assert_eq!(out, vec![0.0]);
}

#[test]
fn logical_or_true() {
    let out = render("out1 = 0 || 2;", 1);
    assert_eq!(out, vec![1.0]);
}

#[test]
fn logical_or_false() {
    let out = render("out1 = 0 || 0;", 1);
    assert_eq!(out, vec![0.0]);
}

#[test]
fn logical_not() {
    let out = render("out1 = !0;", 1);
    assert_eq!(out, vec![1.0]);
    let out2 = render("out1 = !5;", 1);
    assert_eq!(out2, vec![0.0]);
}

#[test]
fn logical_xor() {
    let out = render("out1 = 1 ^^ 0;", 1);
    assert_eq!(out, vec![1.0]);
    let out2 = render("out1 = 1 ^^ 1;", 1);
    assert_eq!(out2, vec![0.0]);
}

// ═══════════════════════════════════════════════════════════════════
//  Bitwise operators
// ═══════════════════════════════════════════════════════════════════

#[test]
fn bitwise_ops_rejected() {
    // gen~ does not support bitwise operators (verified 2026-06-13).
    // Expressions containing & | ^ << >> should fail to parse/lex.
    assert!(opengen_genexpr::parse_and_lower("out1 = 5 & 3;").is_err());
    assert!(opengen_genexpr::parse_and_lower("out1 = 5 | 1;").is_err());
    assert!(opengen_genexpr::parse_and_lower("out1 = 5 ^ 2;").is_err());
    assert!(opengen_genexpr::parse_and_lower("out1 = 1 << 3;").is_err());
    assert!(opengen_genexpr::parse_and_lower("out1 = 8 >> 3;").is_err());
}

// ═══════════════════════════════════════════════════════════════════
//  Builtin constants
// ═══════════════════════════════════════════════════════════════════

#[test]
fn constant_pi() {
    let out = render("out1 = pi;", 1);
    assert_eq!(out[0], std::f64::consts::PI);
}

#[test]
fn constant_twopi() {
    let out = render("out1 = twopi;", 1);
    assert_eq!(out[0], std::f64::consts::TAU);
}

#[test]
fn constant_halfpi() {
    let out = render("out1 = halfpi;", 1);
    assert_eq!(out[0], std::f64::consts::FRAC_PI_2);
}

#[test]
fn constant_invpi() {
    let out = render("out1 = invpi;", 1);
    assert_eq!(out[0], std::f64::consts::FRAC_1_PI);
}

#[test]
fn constant_e() {
    let out = render("out1 = e;", 1);
    assert_eq!(out[0], std::f64::consts::E);
}

#[test]
fn constant_ln2() {
    let out = render("out1 = ln2;", 1);
    assert_eq!(out[0], std::f64::consts::LN_2);
}

#[test]
fn constant_ln10() {
    let out = render("out1 = ln10;", 1);
    assert_eq!(out[0], std::f64::consts::LN_10);
}

#[test]
fn constant_log2e() {
    let out = render("out1 = log2e;", 1);
    assert_eq!(out[0], std::f64::consts::LOG2_E);
}

#[test]
fn constant_log10e() {
    let out = render("out1 = log10e;", 1);
    assert_eq!(out[0], std::f64::consts::LOG10_E);
}

#[test]
fn constant_sqrt2() {
    let out = render("out1 = sqrt2;", 1);
    assert_eq!(out[0], std::f64::consts::SQRT_2);
}

#[test]
fn constant_sqrt1_2() {
    let out = render("out1 = sqrt1_2;", 1);
    assert_eq!(out[0], std::f64::consts::FRAC_1_SQRT_2);
}

#[test]
fn constant_degtorad() {
    let out = render("out1 = degtorad;", 1);
    assert_eq!(out[0], std::f64::consts::PI / 180.0);
}

#[test]
fn constant_radtodeg() {
    let out = render("out1 = radtodeg;", 1);
    assert_eq!(out[0], 180.0 / std::f64::consts::PI);
}

// ═══════════════════════════════════════════════════════════════════
//  Samplerate & vectorsize
// ═══════════════════════════════════════════════════════════════════

#[test]
fn samplerate_at_48k() {
    let graph = parse_and_lower("out1 = samplerate;").unwrap();
    let mut patch = opengen_compile::compile(
        &graph,
        &opengen_ops::Registry::core(),
        48000.0,
    ).unwrap();
    let out = patch.process(&[]);
    assert_eq!(out[0], 48000.0);
}

#[test]
fn vectorsize_is_one() {
    let out = render("out1 = vectorsize;", 1);
    assert_eq!(out[0], 1.0);
}

// ═══════════════════════════════════════════════════════════════════
//  History decl
// ═══════════════════════════════════════════════════════════════════

#[test]
fn history_decl_with_init() {
    // History h(5); h = h + 1; out1 = h;
    // Sample 0: reads init 5, writes 5+1=6 → returns 5
    // Sample 1: reads 6, writes 6+1=7 → returns 6
    // Sample 2: reads 7, writes 7+1=8 → returns 7
    let out = render("History h(5); h = h + 1; out1 = h;", 3);
    assert_eq!(out, vec![5.0, 6.0, 7.0]);
}

#[test]
fn history_decl_without_write() {
    // History h(5); out1 = h;
    // No write → held init forever
    let out = render("History h(5); out1 = h;", 3);
    assert_eq!(out, vec![5.0, 5.0, 5.0]);
}

// ═══════════════════════════════════════════════════════════════════
//  Param decl with named args (min/max ignored in M2)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn param_decl_with_named_args() {
    let graph = parse_and_lower("Param freq(440, min=20, max=20000); out1 = freq;").unwrap();
    let mut patch = opengen_compile::compile(
        &graph,
        &opengen_ops::Registry::core(),
        48000.0,
    ).unwrap();
    let out = patch.process(&[]);
    assert_eq!(out[0], 440.0);
}

#[test]
fn param_decl_without_args() {
    // Param with no initializer → defaults to 0.0
    let graph = parse_and_lower("Param freq; out1 = freq;").unwrap();
    let mut patch = opengen_compile::compile(
        &graph,
        &opengen_ops::Registry::core(),
        48000.0,
    ).unwrap();
    let out = patch.process(&[]);
    assert_eq!(out[0], 0.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Data / Buffer / Delay → "not yet implemented"
// ═══════════════════════════════════════════════════════════════════

#[test]
fn data_decl_works() {
    // Data d(1024); creates a Data node with size 1024
    let graph = lower(&parse("Data d(1024); out1 = 0;").unwrap()).unwrap();
    // Should have: Data node, Constant(0), Output node
    let data_count = graph.nodes().filter(|(_, n)| matches!(n.kind, opengen_ir::NodeKind::Data { .. })).count();
    assert_eq!(data_count, 1);
}

#[test]
fn buffer_decl_works_as_data_alias() {
    // Buffer is an alias for Data in opengen
    let graph = lower(&parse("Buffer b(1024); out1 = 0;").unwrap()).unwrap();
    let data_count = graph.nodes().filter(|(_, n)| matches!(n.kind, opengen_ir::NodeKind::Data { .. })).count();
    assert_eq!(data_count, 1);
}

#[test]
fn delay_decl_works() {
    // Unused Delay decl compiles silently (creates synthetic Data node)
    use opengen_genexpr::lower;
    use opengen_genexpr::parse;
    let ast = parse("Delay d(100); out1 = 0;").unwrap();
    let graph = lower(&ast).unwrap();
    // Should have: synthetic Data node, constant 0, output
    assert!(graph.nodes().count() >= 3);
}

// ═══════════════════════════════════════════════════════════════════
//  Require → "require unsupported in M2"
// ═══════════════════════════════════════════════════════════════════

#[test]
fn require_unsupported() {
    let ast = parse("require \"foo.genexpr\"; out1 = 0;").unwrap();
    let err = lower(&ast).unwrap_err();
    assert!(err.msg.contains("require unsupported"), "got: {}", err.msg);
}

#[test]
fn require_with_parens_unsupported() {
    let ast = parse("require(\"foo.genexpr\"); out1 = 0;").unwrap();
    let err = lower(&ast).unwrap_err();
    assert!(err.msg.contains("require unsupported"), "got: {}", err.msg);
}

// ═══════════════════════════════════════════════════════════════════
//  Shadowing: locals/params shadow operator names AND builtin constants
// ═══════════════════════════════════════════════════════════════════

#[test]
fn local_shadows_builtin_constant() {
    // "e" is a builtin constant (~2.718), but local assign shadows it
    let out = render("e = 2; out1 = e;", 1);
    assert_eq!(out[0], 2.0);
}

#[test]
fn param_shadows_op_name() {
    // "mix" is an operator name, but Param decl shadows it
    let graph = parse_and_lower("Param mix(0.33); out1 = mix;").unwrap();
    let mut patch = opengen_compile::compile(
        &graph,
        &opengen_ops::Registry::core(),
        48000.0,
    ).unwrap();
    let out = patch.process(&[]);
    assert_eq!(out[0], 0.33);
}

#[test]
fn param_shadows_builtin_constant() {
    // "pi" is a builtin constant, but Param decl shadows it
    let graph = parse_and_lower("Param pi(3.0); out1 = pi;").unwrap();
    let mut patch = opengen_compile::compile(
        &graph,
        &opengen_ops::Registry::core(),
        48000.0,
    ).unwrap();
    let out = patch.process(&[]);
    assert_eq!(out[0], 3.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Expression statements
// ═══════════════════════════════════════════════════════════════════

#[test]
fn expression_statement_lowerable() {
    // Expression statements (e.g. side-effect expressions like poke) should lower
    // without error, but produce no binding. Here we use a simple expr stmt.
    let src = "1 + 2; out1 = 42;";
    let out = render(src, 1);
    assert_eq!(out[0], 42.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Multi-assign → error until Task 16
// ═══════════════════════════════════════════════════════════════════

#[test]
fn multi_assign_not_yet_implemented() {
    let ast = parse("x, y = 42; out1 = x;").unwrap();
    let err = lower(&ast).unwrap_err();
    assert!(err.msg.contains("multi-assign"), "got: {}", err.msg);
}

// ═══════════════════════════════════════════════════════════════════
//  M1 backward compatibility — existing patterns must still work
// ═══════════════════════════════════════════════════════════════════

#[test]
fn m1_history_with_explicit_op() {
    // Original M1 pattern: h = history(mix(h, in1, g));
    let graph = parse_and_lower("h = history(mix(h, in1, 0.5)); out1 = h;").unwrap();
    let mut patch = opengen_compile::compile(
        &graph,
        &opengen_ops::Registry::core(),
        48000.0,
    ).unwrap();
    // First sample: h=0, mix(0, in1, 0.5)=0 → write 0
    // Second sample: h=0, mix(0, 0.5, 0.5)=0.25 → write 0.25
    // (with in1=1.0)
    let out1 = patch.process(&[1.0])[0];
    assert_eq!(out1, 0.0);
    let out2 = patch.process(&[1.0])[0];
    assert!((out2 - 0.5).abs() < 0.001);
}

#[test]
fn m1_param_decl_old_syntax() {
    let graph = parse_and_lower("Param freq(440); out1 = freq;").unwrap();
    let mut patch = opengen_compile::compile(
        &graph,
        &opengen_ops::Registry::core(),
        48000.0,
    ).unwrap();
    assert_eq!(patch.process(&[])[0], 440.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Negative init in History decl (const-fold unary negation)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn history_decl_with_negative_init() {
    // History h(-5); out1 = h; → first sample should be -5.0
    let out = render("History h(-5); out1 = h;", 3);
    assert_eq!(out, vec![-5.0, -5.0, -5.0]);
}

#[test]
fn history_decl_with_double_negative_init() {
    // History h(-(-5)); out1 = h; → first sample should be 5.0
    let out = render("History h(-(-5)); out1 = h;", 3);
    assert_eq!(out, vec![5.0, 5.0, 5.0]);
}

// ═══════════════════════════════════════════════════════════════════
//  Stress: constant expression in non-constant init position
// ═══════════════════════════════════════════════════════════════════

#[test]
fn history_decl_nonconstant_init_errors() {
    // Init must be a literal constant expression (not a variable)
    let ast = parse("Param x(5); History h(x); out1 = h;").unwrap();
    let err = lower(&ast).unwrap_err();
    assert!(err.msg.contains("constant") || err.msg.contains("expected"), "got: {}", err.msg);
}
