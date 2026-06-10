//! User-defined function inlining tests (Task 16).
//!
//! TDD: all tests fail first (function inlining not yet implemented),
//! then pass after implementing inline.rs + updating lower.rs / lib.rs.

use opengen_testkit::render;

// ═══════════════════════════════════════════════════════════════════
//  Basic function inlining
// ═══════════════════════════════════════════════════════════════════

#[test]
fn function_inlines_per_call_site() {
    let src =
        "double(x) { return x * 2; }\
         out1 = double(3) + double(4);";
    assert_eq!(render(src, 48_000.0, 1).ch(0)[0], 14.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Multi-return destructuring
// ═══════════════════════════════════════════════════════════════════

#[test]
fn multi_return_destructures() {
    let src =
        "mm(a, b) { return min(a, b), max(a, b); }\
         lo, hi = mm(7, 3);\
         out1 = lo; out2 = hi;";
    let out = render(src, 48_000.0, 1);
    assert_eq!((out.ch(0)[0], out.ch(1)[0]), (3.0, 7.0));
}

// ═══════════════════════════════════════════════════════════════════
//  Per-call-site state (History inside function)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn function_state_is_per_call_site() {
    // History inside a function: each call site gets independent state (user-docs fact)
    let src =
        "count() { History h(0); h = h + 1; return h; }\
         out1 = count(); out2 = count();";
    let out = render(src, 48_000.0, 3);
    assert_eq!(out.ch(0), out.ch(1)); // both independent counters: 1,2,3 each
}

// ═══════════════════════════════════════════════════════════════════
//  Recursion is rejected
// ═══════════════════════════════════════════════════════════════════

#[test]
fn recursion_is_rejected() {
    let err = opengen_genexpr::parse_and_lower("f(x) { return f(x); }\nout1 = f(1);").unwrap_err();
    assert!(
        err.contains("recursi") || err.contains("cycle"),
        "expected error about recursion, got: {err}"
    );
}

#[test]
fn mutual_recursion_is_rejected() {
    let err = opengen_genexpr::parse_and_lower(
        "f(x) { return g(x); } g(x) { return f(x); }\nout1 = f(1);",
    )
    .unwrap_err();
    assert!(
        err.contains("recursi") || err.contains("cycle"),
        "expected error about recursion, got: {err}"
    );
}

// ═══════════════════════════════════════════════════════════════════
//  Nested function calls (double(double(2)))
// ═══════════════════════════════════════════════════════════════════

#[test]
fn nested_function_calls() {
    let src =
        "double(x) { return x * 2; }\
         out1 = double(double(2));";
    assert_eq!(render(src, 48_000.0, 1).ch(0)[0], 8.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Extra LHS vars filled with 0 (multi-assign destructuring rules §7)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn extra_lhs_vars_get_zero() {
    // single(c) = c; called as a, b = single(7) → a=7, b=0
    let src =
        "single(c) { return c; }\
         a, b = single(7);\
         out1 = a; out2 = b;";
    let out = render(src, 48_000.0, 1);
    assert_eq!((out.ch(0)[0], out.ch(1)[0]), (7.0, 0.0));
}

// ═══════════════════════════════════════════════════════════════════
//  Extra RHS values ignored (multi-assign destructuring rules §7)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn extra_rhs_values_ignored() {
    // triple(a,b,c) { return a,b,c; } called as x, y = triple(1,2,3) → x=1, y=2, c ignored
    let src =
        "triple(a, b, c) { return a, b, c; }\
         x, y = triple(1, 2, 3);\
         out1 = x; out2 = y;";
    let out = render(src, 48_000.0, 1);
    assert_eq!((out.ch(0)[0], out.ch(1)[0]), (1.0, 2.0));
}

// ═══════════════════════════════════════════════════════════════════
//  Function with control flow → correctly inlines into region path
// ═══════════════════════════════════════════════════════════════════

#[test]
fn function_with_control_flow_inlines_into_region() {
    let src =
        "abs_diff(a, b) { if (a > b) { return a - b; } else { return b - a; } }\
         out1 = abs_diff(5, 3) + abs_diff(2, 7);";
    let out = render(src, 48_000.0, 1);
    assert_eq!(out.ch(0)[0], 7.0); // 2 + 5 = 7
}

// ═══════════════════════════════════════════════════════════════════
//  Function call in a multi-assign from a multi-return function
//  where the function is called in a loop
// ═══════════════════════════════════════════════════════════════════

#[test]
fn multi_assign_from_func_with_loop() {
    let src =
        "acc(n) { s = 0; for (i = 0; i < n; i += 1) { s = s + 1; } return s; }\
         out1 = acc(3);";
    assert_eq!(render(src, 48_000.0, 1).ch(0)[0], 3.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Multi-assign from non-function call → error
// ═══════════════════════════════════════════════════════════════════

#[test]
fn multi_assign_from_non_function_errors() {
    let err =
        opengen_genexpr::parse_and_lower("x, y = 42;\nout1 = x;").unwrap_err();
    assert!(
        err.contains("multi-assign"),
        "expected 'multi-assign' in error, got: {err}"
    );
}

// ═══════════════════════════════════════════════════════════════════
//  Return outside function → error
// ═══════════════════════════════════════════════════════════════════

#[test]
fn return_outside_function_errors() {
    let err =
        opengen_genexpr::parse_and_lower("return 42;\nout1 = 1;").unwrap_err();
    assert!(
        err.contains("return"),
        "expected error about return, got: {err}"
    );
}
