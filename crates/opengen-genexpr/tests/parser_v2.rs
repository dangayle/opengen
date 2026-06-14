//! Parser v2 tests: full expression grammar, declarations, control-flow statements.
//!
//! Each test parses a GenExpr fragment and asserts the AST shape. Sad paths included.
//! These test the *parser only* — lowering is tested separately in lower_v2.rs.

use opengen_genexpr::{
    parse,
    ast::{DeclType, StatementKind, Expr, BinOpKind, UnaryOp},
};

// ─── Expressions ──────────────────────────────────────────────────────

#[test]
fn ternary_expression() {
    let ast = parse("out1 = 1 ? 2 : 3;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { expr, .. } => {
            match expr {
                Expr::Ternary { cond, true_expr, false_expr } => {
                    assert!(matches!(**cond, Expr::Number(1.0)));
                    assert!(matches!(**true_expr, Expr::Number(2.0)));
                    assert!(matches!(**false_expr, Expr::Number(3.0)));
                }
                _ => panic!("expected ternary"),
            }
        }
        _ => panic!("expected assign"),
    }
}

#[test]
fn ternary_right_assoc() {
    let ast = parse("out1 = a ? b : c ? d : e;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { expr, .. } => {
            // Should be Ternary(a, b, Ternary(c, d, e))
            match expr {
                Expr::Ternary { cond, true_expr, false_expr } => {
                    assert!(matches!(**cond, Expr::Ident(_)));
                    assert!(matches!(**true_expr, Expr::Ident(_)));
                    assert!(matches!(**false_expr, Expr::Ternary { .. }),
                        "expected nested ternary on false branch");
                }
                _ => panic!("expected ternary at top"),
            }
        }
        _ => panic!("expected assign"),
    }
}

#[test]
fn logical_or() {
    let ast = parse("out1 = 1 || 0;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { expr, .. } => {
            match expr {
                Expr::BinOp { op: BinOpKind::LogicalOr, .. } => {}
                _ => panic!("expected LogicalOr"),
            }
        }
        _ => panic!("expected assign"),
    }
}

#[test]
fn logical_and() {
    let ast = parse("out1 = 1 && 0;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { expr, .. } => {
            match expr {
                Expr::BinOp { op: BinOpKind::LogicalAnd, .. } => {}
                _ => panic!("expected LogicalAnd"),
            }
        }
        _ => panic!("expected assign"),
    }
}

#[test]
fn logical_xor() {
    let ast = parse("out1 = 1 ^^ 0;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { expr, .. } => {
            match expr {
                Expr::BinOp { op: BinOpKind::LogicalXor, .. } => {}
                _ => panic!("expected LogicalXor"),
            }
        }
        _ => panic!("expected assign"),
    }
}

#[test]
fn caretcaret_binds_between_oror_and_andand() {
    // Vendor PEP precedence ladder (low→high; gen~ only, no bitwise):
    // ?: → || → ^^ → && → equality → relational → ...
    // ^^ is between || (looser) and && (tighter).
    // Verified against live C74 docs 2026-06-13: bitwise `&`/`|`/`^` are not
    // gen~ operators; `&` and `^` now produce lexer errors.

    // 1 & 2 ^^ 3 — `&` is a lexer error (not a gen~ operator)
    assert!(parse("out1 = 1 & 2 ^^ 3;").is_err());
    // 1 ^ 2 ^^ 3 — `^` is a lexer error
    assert!(parse("out1 = 1 ^ 2 ^^ 3;").is_err());

    // 1 && 2 ^^ 3  =>  XorLogical( And(1,2), 3 )  — && is tighter than ^^, so (1&&2) ^^ 3
    let ast = parse("out1 = 1 && 2 ^^ 3;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { expr, .. } => {
            match expr {
                Expr::BinOp { op: BinOpKind::LogicalXor, left, right } => {
                    assert!(matches!(**left, Expr::BinOp { op: BinOpKind::LogicalAnd, .. }));
                    assert!(matches!(**right, Expr::Number(3.0)));
                }
                other => panic!("expected LogicalXor at top for '1 && 2 ^^ 3', got {other:?}"),
            }
        }
        _ => panic!("expected assign"),
    }

    // 1 || 2 ^^ 3  =>  Or( 1, XorLogical(2,3) )  — || is looser than ^^, so 1 || (2^^3)
    let ast = parse("out1 = 1 || 2 ^^ 3;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { expr, .. } => {
            match expr {
                Expr::BinOp { op: BinOpKind::LogicalOr, left, right } => {
                    assert!(matches!(**left, Expr::Number(1.0)));
                    assert!(matches!(**right, Expr::BinOp { op: BinOpKind::LogicalXor, .. }));
                }
                other => panic!("expected LogicalOr at top for '1 || 2 ^^ 3', got {other:?}"),
            }
        }
        _ => panic!("expected assign"),
    }
}

#[test]
fn bitwise_ops_rejected() {
    // gen~ does not support bitwise operators (verified against live C74
    // docs, gen_common_operators reference, and in-Max codebox compiler,
    // 2026-06-13). `&`/`|`/`^` are lexer errors.
    assert!(parse("out1 = 5 & 3;").is_err());
    assert!(parse("out1 = 5 | 1;").is_err());
    assert!(parse("out1 = 5 ^ 2;").is_err());
    // << >> are not operators — they lex as two separate tokens and fail to parse
    assert!(parse("out1 = 1 << 2;").is_err());
    assert!(parse("out1 = 1 >> 2;").is_err());
}

#[test]
fn unary_not() {
    let ast = parse("out1 = !1;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { expr, .. } => {
            match expr {
                Expr::Unary(UnaryOp::Not, e) => {
                    assert!(matches!(**e, Expr::Number(1.0)));
                }
                _ => panic!("expected Unary(Not)"),
            }
        }
        _ => panic!("expected assign"),
    }
}

#[test]
fn unary_not_binds_tighter_than_binary() {
    let ast = parse("out1 = !1 + !2;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { expr, .. } => {
            // Should be Add(Not(1), Not(2))
            match expr {
                Expr::BinOp { op: BinOpKind::Add, left, right } => {
                    assert!(matches!(**left, Expr::Unary(UnaryOp::Not, _)));
                    assert!(matches!(**right, Expr::Unary(UnaryOp::Not, _)));
                }
                _ => panic!("expected Add at top"),
            }
        }
        _ => panic!("expected assign"),
    }
}

#[test]
fn member_call_parses() {
    let ast = parse("out1 = d.read(100);").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { expr, .. } => {
            match expr {
                Expr::MemberCall { object, method, args, .. } => {
                    assert!(matches!(**object, Expr::Ident(ref s) if s == "d"));
                    assert_eq!(method, "read");
                    assert_eq!(args.len(), 1);
                    assert!(matches!(args[0], Expr::Number(100.0)));
                }
                _ => panic!("expected MemberCall"),
            }
        }
        _ => panic!("expected assign"),
    }
}

#[test]
fn member_call_with_named_args() {
    let ast = parse(r#"out1 = d.read(100, interp="linear");"#).unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { expr, .. } => {
            match expr {
                Expr::MemberCall { object, method, args, named_args } => {
                    assert!(matches!(**object, Expr::Ident(ref s) if s == "d"));
                    assert_eq!(method, "read");
                    assert_eq!(args.len(), 1);
                    assert_eq!(named_args.len(), 1);
                    assert_eq!(named_args[0].0, "interp");
                    assert!(matches!(named_args[0].1, Expr::Str(ref s) if s == "linear"));
                }
                _ => panic!("expected MemberCall"),
            }
        }
        _ => panic!("expected assign"),
    }
}

// ─── Declarations ────────────────────────────────────────────────────

#[test]
fn typed_declaration_history() {
    let ast = parse("History h(0);").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Decl { ty, items } => {
            assert_eq!(*ty, DeclType::History);
            assert_eq!(items.len(), 1);
            assert_eq!(items[0].name, "h");
            assert_eq!(items[0].args.len(), 1);
            assert!(matches!(items[0].args[0], Expr::Number(0.0)));
        }
        _ => panic!("expected Decl"),
    }
}

#[test]
fn typed_declaration_multiple_items() {
    let ast = parse("History a(0), b(1);").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Decl { ty, items } => {
            assert_eq!(*ty, DeclType::History);
            assert_eq!(items.len(), 2);
            assert_eq!(items[0].name, "a");
            assert_eq!(items[1].name, "b");
        }
        _ => panic!("expected Decl with multiple items"),
    }
}

#[test]
fn typed_declaration_param_with_named_args() {
    let ast = parse("Param freq(440, min=20, max=20000);").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Decl { ty, items } => {
            assert_eq!(*ty, DeclType::Param);
            assert_eq!(items.len(), 1);
            assert_eq!(items[0].name, "freq");
            assert_eq!(items[0].args.len(), 1);
            assert!(matches!(items[0].args[0], Expr::Number(440.0)));
            assert_eq!(items[0].named_args.len(), 2);
            assert_eq!(items[0].named_args[0].0, "min");
            assert_eq!(items[0].named_args[1].0, "max");
        }
        _ => panic!("expected Decl"),
    }
}

#[test]
fn typed_declaration_data() {
    let ast = parse("Data buf(512);").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Decl { ty, .. } => {
            assert_eq!(*ty, DeclType::Data);
        }
        _ => panic!("expected Decl"),
    }
}

#[test]
fn typed_declaration_delay() {
    let ast = parse("Delay d(1024);").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Decl { ty, .. } => {
            assert_eq!(*ty, DeclType::Delay);
        }
        _ => panic!("expected Decl"),
    }
}

#[test]
fn typed_declaration_buffer() {
    let ast = parse(r#"Buffer b("myBuffer");"#).unwrap();
    match &ast.statements[0].kind {
        StatementKind::Decl { ty, .. } => {
            assert_eq!(*ty, DeclType::Buffer);
        }
        _ => panic!("expected Decl"),
    }
}

// ─── Statement types ──────────────────────────────────────────────────

#[test]
fn expression_statement() {
    let ast = parse("d.write(x);").unwrap();
    match &ast.statements[0].kind {
        StatementKind::ExprStmt(Expr::MemberCall { object, method, args, .. }) => {
            assert!(matches!(**object, Expr::Ident(ref s) if s == "d"));
            assert_eq!(method, "write");
            assert_eq!(args.len(), 1);
        }
        other => panic!("expected ExprStmt(MemberCall), got {other:?}"),
    }
}

#[test]
fn block_statement() {
    let ast = parse("{ out1 = 1; out2 = 2; }").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Block(stmts) => {
            assert_eq!(stmts.len(), 2);
        }
        _ => panic!("expected Block"),
    }
}

#[test]
fn if_statement() {
    let ast = parse("if (x > 0) { out1 = x; }").unwrap();
    match &ast.statements[0].kind {
        StatementKind::If { cond, then_branch: _, else_branch } => {
            assert!(matches!(cond, Expr::BinOp { op: BinOpKind::Gt, .. }));
            assert!(matches!(else_branch, None));
        }
        _ => panic!("expected If"),
    }
}

#[test]
fn if_else_statement() {
    let ast = parse("if (1) out1 = 1; else out1 = 2;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::If { cond, then_branch: _, else_branch } => {
            assert!(matches!(cond, Expr::Number(1.0)));
            let else_stmt = else_branch.as_ref().expect("else branch should exist");
            // Single-statement else body is a bare Assign (not ExprStmt with semicolon)
            assert!(matches!(else_stmt.kind, StatementKind::Assign { .. }),
                "else branch should be Assign, got {:?}", else_stmt.kind);
        }
        _ => panic!("expected If"),
    }
}

#[test]
fn if_else_if_else() {
    let s = "if (a) out1 = 1; else if (b) out1 = 2; else out1 = 3;";
    let ast = parse(s).unwrap();
    match &ast.statements[0].kind {
        StatementKind::If { cond, then_branch: _, else_branch } => {
            assert!(matches!(cond, Expr::Ident(s) if s == "a"));
            let else_stmt = else_branch.as_ref().expect("else");
            // else branch should itself be an If (else-if chain)
            assert!(matches!(else_stmt.kind, StatementKind::If { .. }),
                "else-if chains should be nested: got {:?}", else_stmt.kind);
        }
        other => panic!("expected If, got {other:?}"),
    }
}

#[test]
fn while_statement() {
    let ast = parse("while (x < 10) { x = x + 1; }").unwrap();
    match &ast.statements[0].kind {
        StatementKind::While { cond, body } => {
            assert!(matches!(cond, Expr::BinOp { op: BinOpKind::Lt, .. }));
            assert!(matches!(body.kind, StatementKind::Block(_)));
        }
        _ => panic!("expected While"),
    }
}

#[test]
fn do_while_statement() {
    let ast = parse("do { x = x + 1; } while (x < 10);").unwrap();
    match &ast.statements[0].kind {
        StatementKind::DoWhile { body, cond } => {
            assert!(matches!(body.kind, StatementKind::Block(_)));
            assert!(matches!(cond, Expr::BinOp { op: BinOpKind::Lt, .. }));
        }
        _ => panic!("expected DoWhile"),
    }
}

#[test]
fn for_statement() {
    let ast = parse("for (i = 0; i < 10; i += 1) { out1 = i; }").unwrap();
    match &ast.statements[0].kind {
        StatementKind::For { init, cond, step, body } => {
            // Init: i = 0 should be an assignment
            let init_stmt = init.as_ref().expect("for init");
            assert!(matches!(init_stmt.kind, StatementKind::Assign { .. }),
                "for init should be Assign, got {:?}", init_stmt.kind);
            assert!(matches!(cond, Some(Expr::BinOp { op: BinOpKind::Lt, .. })));
            // Step: i += 1 desugars to BinOp(Add, Ident(i), 1)
            assert!(matches!(step, Some(Expr::BinOp { op: BinOpKind::Add, .. })),
                "step should be BinOp(Add), got {:?}", step);
            assert!(matches!(body.kind, StatementKind::Block(_)));
        }
        _ => panic!("expected For"),
    }
}

#[test]
fn break_statement() {
    let ast = parse("while (1) { break; }").unwrap();
    match &ast.statements[0].kind {
        StatementKind::While { body, .. } => {
            match &body.kind {
                StatementKind::Block(stmts) => {
                    assert!(matches!(stmts[0].kind, StatementKind::Break));
                }
                _ => panic!("expected Block in while body"),
            }
        }
        _ => panic!("expected While"),
    }
}

#[test]
fn continue_statement() {
    let ast = parse("while (1) { continue; }").unwrap();
    match &ast.statements[0].kind {
        StatementKind::While { body, .. } => {
            match &body.kind {
                StatementKind::Block(stmts) => {
                    assert!(matches!(stmts[0].kind, StatementKind::Continue));
                }
                _ => panic!("expected Block in while body"),
            }
        }
        _ => panic!("expected While"),
    }
}

#[test]
fn return_statement() {
    let ast = parse("return 42;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Return(vals) => {
            assert_eq!(vals.len(), 1);
            assert!(matches!(vals[0], Expr::Number(42.0)));
        }
        _ => panic!("expected Return"),
    }
}

#[test]
fn return_multiple_values() {
    let ast = parse("return a, b, c;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Return(vals) => {
            assert_eq!(vals.len(), 3);
        }
        _ => panic!("expected Return with 3 values"),
    }
}

#[test]
fn multi_assign() {
    let ast = parse("a, b = f(x);").unwrap();
    match &ast.statements[0].kind {
        StatementKind::MultiAssign { names, expr } => {
            assert_eq!(names.len(), 2);
            assert_eq!(names[0], "a");
            assert_eq!(names[1], "b");
            assert!(matches!(expr, Expr::Call { name, .. } if name == "f"));
        }
        _ => panic!("expected MultiAssign"),
    }
}

#[test]
fn function_declaration() {
    let src = "myFunc(a, b) { return a + b; }";
    let ast = parse(src).unwrap();
    match &ast.statements[0].kind {
        StatementKind::FuncDecl { name, params, body } => {
            assert_eq!(name, "myFunc");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], "a");
            assert_eq!(params[1], "b");
            assert_eq!(body.len(), 1);
            assert!(matches!(body[0].kind, StatementKind::Return(_)));
        }
        other => panic!("expected FuncDecl, got {other:?}"),
    }
}

#[test]
fn require_statement() {
    let ast = parse(r#"require "msplib_biquad";"#).unwrap();
    match &ast.statements[0].kind {
        StatementKind::Require(path) => {
            assert_eq!(path, "msplib_biquad");
        }
        _ => panic!("expected Require"),
    }
}

#[test]
fn bare_final_expression_sugar() {
    let ast = parse("in1 * 0.5").unwrap();
    // Should desugar to out1 = in1 * 0.5;
    match &ast.statements[0].kind {
        StatementKind::Assign { name, expr } => {
            assert_eq!(name, "out1");
            assert!(matches!(expr, Expr::BinOp {
                op: BinOpKind::Mul, ..
            }));
        }
        other => panic!("expected Assign for bare expression, got {other:?}"),
    }
}

#[test]
fn bare_expr_sugar_for_non_ident_expressions() {
    // Bare expressions that don't start with an ident (e.g. parenthesized, unary)
    // should also get out1 = sugar applied.
    let ast = parse("(in1 + in2) * 0.5").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { name, expr } => {
            assert_eq!(name, "out1");
            assert!(matches!(expr, Expr::BinOp {
                op: BinOpKind::Mul, ..
            }), "expected Mul at top, got {expr:?}");
        }
        other => panic!("expected Assign for bare non-ident expression, got {other:?}"),
    }

    // Unary negation as bare expression
    let ast = parse("!x").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { name, expr } => {
            assert_eq!(name, "out1");
            assert!(matches!(expr, Expr::Unary(UnaryOp::Not, _)));
        }
        other => panic!("expected Assign for bare '!x', got {other:?}"),
    }

    // Numeric literal as bare expression
    let ast = parse("42").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { name, expr } => {
            assert_eq!(name, "out1");
            assert!(matches!(expr, Expr::Number(42.0)));
        }
        other => panic!("expected Assign for bare '42', got {other:?}"),
    }
}

#[test]
fn compound_assignment_desugars_to_binop() {
    let ast = parse("x += 2;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { name, expr } => {
            assert_eq!(name, "x");
            match expr {
                Expr::BinOp { op: BinOpKind::Add, left, right } => {
                    assert!(matches!(**left, Expr::Ident(ref s) if s == "x"));
                    assert!(matches!(**right, Expr::Number(2.0)));
                }
                _ => panic!("expected BinOp(Add) for +=, got {expr:?}"),
            }
        }
        other => panic!("expected Assign for x += 2, got {other:?}"),
    }
}

// ─── Sad paths ────────────────────────────────────────────────────────

#[test]
fn if_missing_body_is_parse_error() {
    let err = parse("if (1)").unwrap_err();
    assert!(err.msg.contains("expected") || err.msg.contains("body") || err.msg.contains("statement"),
        "error msg: {}", err.msg);
}

#[test]
fn invalid_operator_is_parse_error() {
    let err = parse("x +== 1;").unwrap_err();
    assert!(err.to_string().contains("unexpected") || err.to_string().contains("error"),
        "error msg: {}", err);
}

#[test]
fn empty_return_parses() {
    let ast = parse("return;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Return(v) => {
            assert!(v.is_empty(), "expected empty return value vec");
        }
        other => panic!("expected Return, got {other:?}"),
    }
}

#[test]
fn chained_member_calls_nest() {
    let ast = parse("x = a.b(1).d(2);").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Assign { expr, .. } => {
            match expr {
                Expr::MemberCall { object, method: outer_method, args: outer_args, .. } => {
                    assert_eq!(outer_method, "d");
                    assert_eq!(outer_args.len(), 1);
                    assert!(matches!(outer_args[0], Expr::Number(2.0)));
                    match &**object {
                        Expr::MemberCall { object: inner_obj, method: inner_method, args: inner_args, .. } => {
                            assert_eq!(inner_method, "b");
                            assert_eq!(inner_args.len(), 1);
                            assert!(matches!(inner_args[0], Expr::Number(1.0)));
                            assert!(matches!(**inner_obj, Expr::Ident(ref s) if s == "a"));
                        }
                        other => panic!("expected inner MemberCall, got {other:?}"),
                    }
                }
                other => panic!("expected MemberCall at top, got {other:?}"),
            }
        }
        other => panic!("expected Assign, got {other:?}"),
    }
}

#[test]
fn return_outside_function_parses_ok_lowering_rejects() {
    // Return is syntactically valid anywhere — lowering rejects it
    let ast = parse("return 1;").unwrap();
    match &ast.statements[0].kind {
        StatementKind::Return(_) => {} // OK at parse time
        other => panic!("expected Return, got {other:?}"),
    }
}

#[test]
fn unclosed_string_errors() {
    let err = parse(r#"require "unclosed;"#).unwrap_err();
    assert!(err.to_string().contains("unterminated") || err.to_string().contains("string"),
        "error msg: {}", err);
}
