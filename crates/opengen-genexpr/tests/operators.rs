// Integration tests for operators via rendering
use opengen_testkit::render;

#[test]
fn test_arithmetic_operators_render() {
    // Test subtraction
    let out = render("out1 = 5.0 - 2.0;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 3.0);
    
    // Test division
    let out = render("out1 = 10.0 / 4.0;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 2.5);
    
    // Test modulo via % operator
    let out = render("out1 = 5.5 % 2.0;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 1.5);
}

#[test]
fn test_comparison_operators_render() {
    // Greater than
    let out = render("out1 = 2.0 > 1.0;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 1.0);
    
    let out = render("out1 = 1.0 > 2.0;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 0.0);
    
    // Less than
    let out = render("out1 = 1.0 < 2.0;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 1.0);
    
    // Equal
    let out = render("out1 = 1.5 == 1.5;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 1.0);
    
    // Not equal
    let out = render("out1 = 1.0 != 2.0;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 1.0);
}

#[test]
fn test_unary_minus_with_sub() {
    // Unary minus now works via sub operator
    let out = render("out1 = -0.5 + 1.0;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 0.5);
}
