//! Integration tests for Data / Buffer / peek / poke.
//!
//! These tests exercise the full pipeline: parse → lower → compile → process.
//! Lower-level kernel and compile tests live in `opengen-ops/src/memory.rs`.

use opengen_testkit::render;

// ═══════════════════════════════════════════════════════════════════
//  Data decl + node output
// ═══════════════════════════════════════════════════════════════════

#[test]
fn data_node_output_is_size() {
    let out = render("Data d(4); out1 = d;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 4.0);
}

#[test]
fn data_default_size_is_512() {
    let out = render("Data d; out1 = d;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 512.0);
}

#[test]
fn buffer_is_alias_for_data() {
    let out = render("Buffer b(8); out1 = b;", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 8.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Basic peek / poke round-trips
// ═══════════════════════════════════════════════════════════════════

#[test]
fn peek_poke_round_trip() {
    // poke(d, value, index); peek(d, index); → value
    let out = render(
        "Data d(4); poke(d, 42.0, 1); out1 = peek(d, 1);",
        48000.0, 1,
    );
    assert_eq!(out.ch(0)[0], 42.0);
}

#[test]
fn peek_reads_zero_before_poke() {
    // Data is zero-initialized
    let out = render("Data d(4); out1 = peek(d, 2);", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 0.0);
}

#[test]
fn peek_poke_multiple_positions() {
    let out = render(
        "Data d(8); poke(d, 10.0, 0); poke(d, 20.0, 4); poke(d, 30.0, 7); \
         out1 = peek(d, 0); out2 = peek(d, 4); out3 = peek(d, 7);",
        48000.0, 1,
    );
    assert_eq!(out.ch(0)[0], 10.0);
    assert_eq!(out.ch(1)[0], 20.0);
    assert_eq!(out.ch(2)[0], 30.0);
}

#[test]
fn replace_write_same_index() {
    // Last poke wins (replace-write)
    let out = render(
        "Data d(4); poke(d, 1.0, 0); poke(d, 2.0, 0); out1 = peek(d, 0);",
        48000.0, 1,
    );
    assert_eq!(out.ch(0)[0], 2.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Out-of-range peek → 0
// ═══════════════════════════════════════════════════════════════════

#[test]
fn peek_oob_positive() {
    // index >= size → 0
    let out = render("Data d(4); out1 = peek(d, 9);", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 0.0);
}

#[test]
fn peek_oob_at_size_boundary() {
    // index == size → 0
    let out = render("Data d(4); out1 = peek(d, 4);", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 0.0);
}

#[test]
fn peek_oob_negative() {
    let out = render("Data d(4); out1 = peek(d, -1);", 48000.0, 1);
    assert_eq!(out.ch(0)[0], 0.0);
}

#[test]
fn peek_float_index_truncates() {
    // Float index truncates toward zero
    let out = render(
        "Data d(4); poke(d, 99.0, 1); out1 = peek(d, 1.5);",
        48000.0, 1,
    );
    assert_eq!(out.ch(0)[0], 99.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Out-of-range poke → no write
// ═══════════════════════════════════════════════════════════════════

#[test]
fn poke_oob_negative_index() {
    let out = render(
        "Data d(4); poke(d, 99.0, -1); out1 = peek(d, 0);",
        48000.0, 1,
    );
    assert_eq!(out.ch(0)[0], 0.0);
}

#[test]
fn poke_oob_beyond_size() {
    let out = render(
        "Data d(4); poke(d, 99.0, 4); out1 = peek(d, 3);",
        48000.0, 1,
    );
    assert_eq!(out.ch(0)[0], 0.0);
}

#[test]
fn poke_oob_beyond_size_does_not_corrupt() {
    // Verify state is unchanged after OOB poke
    let out = render(
        "Data d(4); poke(d, 10.0, 0); poke(d, 99.0, 9); out1 = peek(d, 0);",
        48000.0, 1,
    );
    assert_eq!(out.ch(0)[0], 10.0);
}

// ═══════════════════════════════════════════════════════════════════
//  peek/poke with expression statements
// ═══════════════════════════════════════════════════════════════════

#[test]
fn poke_as_expression_statement() {
    // Poke used as a standalone statement (side-effect only)
    let out = render(
        "Data d(4); poke(d, 42.0, 2); out1 = peek(d, 2);",
        48000.0, 1,
    );
    assert_eq!(out.ch(0)[0], 42.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Multiple data buffers
// ═══════════════════════════════════════════════════════════════════

#[test]
fn two_data_buffers_independent() {
    let out = render(
        "Data a(4); Data b(4); \
         poke(a, 10.0, 0); poke(b, 20.0, 0); \
         out1 = peek(a, 0); out2 = peek(b, 0);",
        48000.0, 1,
    );
    assert_eq!(out.ch(0)[0], 10.0);
    assert_eq!(out.ch(1)[0], 20.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Read ordering: same-sample poke before peek
// ═══════════════════════════════════════════════════════════════════

#[test]
fn poke_before_peek_same_sample() {
    // poke is scheduled before peek in topo order (poke has arity 0 read inputs,
    // peek has arity 1 read, but poke comes before peek because of declaration order)
    let out = render(
        "Data d(4); \
         poke(d, 42.0, 0); \t// poke node\n\
         out1 = peek(d, 0); \t// peek node\n",
        48000.0, 1,
    );
    assert_eq!(out.ch(0)[0], 42.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Error cases (lowering)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn peek_unknown_data_errors() {
    let err = opengen_genexpr::parse_and_lower("out1 = peek(unknown, 0);").unwrap_err();
    assert!(
        err.to_string().contains("unknown data buffer"),
        "got: {}",
        err.to_string()
    );
}

#[test]
fn poke_unknown_data_errors() {
    let err = opengen_genexpr::parse_and_lower("poke(unknown, 1.0, 0); out1 = 0;").unwrap_err();
    assert!(
        err.to_string().contains("unknown data buffer"),
        "got: {}",
        err.to_string()
    );
}

// ═══════════════════════════════════════════════════════════════════
//  peek/poke inside region (control flow)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn peek_poke_inside_if_round_trip() {
    // The dang-tools pattern: poke inside if
    let src = "Data d(4); if (in1 > 0) { poke(d, 42.0, 0); } out1 = peek(d, 0);";
    let out = render(src, 48000.0, 1);
    // With in1 = 0.0 (default), condition is false → poke doesn't execute
    assert_eq!(out.ch(0)[0], 0.0);
}

#[test]
fn peek_poke_inside_if_with_true_cond() {
    let src = "Data d(4); Param c(1); if (c) { poke(d, 42.0, 0); } out1 = peek(d, 0);";
    let out = render(src, 48000.0, 1);
    assert_eq!(out.ch(0)[0], 42.0);
}

// ═══════════════════════════════════════════════════════════════════
//  Determinism: multi-sample test
// ═══════════════════════════════════════════════════════════════════

#[test]
fn data_persists_across_samples() {
    // Data retains values between samples.
    // Use render_with_inputs to explicitly control in1 per sample:
    //   Sample 0: in1=5.0 -> poke(d, 5.0, 0)
    //   Sample 1: in1=0.0 -> no poke, peek returns 5.0 (persisted)
    //   Sample 2: in1=0.0 -> no poke, peek returns 5.0 (persisted)
    //   Sample 3: in1=3.0 -> poke(d, 3.0, 0), peek returns 3.0 (overwritten)
    let src = "Data d(4); \
               if (in1 > 0) { poke(d, in1, 0); } \
               out1 = peek(d, 0);";
    let out = opengen_testkit::render_with_inputs(src, 48000.0, &[&[5.0, 0.0, 0.0, 3.0]]);
    // Sample 0: in1=5, poke writes 5, peek returns 5
    assert_eq!(out.ch(0)[0], 5.0, "sample 0: poke then peek should return 5");
    // Sample 1: in1=0, no poke, peek returns persisted 5
    assert_eq!(out.ch(0)[1], 5.0, "sample 1: value persisted from sample 0");
    // Sample 2: in1=0, no poke, peek returns persisted 5
    assert_eq!(out.ch(0)[2], 5.0, "sample 2: value persisted from sample 0");
    // Sample 3: in1=3, poke overwrites to 3, peek returns 3
    assert_eq!(out.ch(0)[3], 3.0, "sample 3: poke overwrites to 3");
}

// ═══════════════════════════════════════════════════════════════════
//  Region-local variable as peek/poke data (must error)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn region_local_as_peek_data_errors() {
    // A region-local variable (assigned inside the region) is NOT a data buffer.
    // peek(x, 0) where x is a local should error at lowering.
    let err = opengen_genexpr::parse_and_lower(
        r"if (1) { x = 1; out1 = peek(x, 0); }"
    ).unwrap_err();
    assert!(
        err.to_string().contains("unknown data buffer"),
        "expected 'unknown data buffer' error, got: {}",
        err.to_string()
    );
}

#[test]
fn region_local_as_poke_data_errors() {
    // Same for poke: a region-local variable is not a valid data buffer.
    let err = opengen_genexpr::parse_and_lower(
        r"if (1) { x = 1; poke(x, 5.0, 0); }"
    ).unwrap_err();
    assert!(
        err.to_string().contains("unknown data buffer"),
        "expected 'unknown data buffer' error, got: {}",
        err.to_string()
    );
}

// ═══════════════════════════════════════════════════════════════════
//  Data name shadowed by region-local still resolves to Data
// ═══════════════════════════════════════════════════════════════════

#[test]
fn data_not_shadowed_by_region_local_of_same_name() {
    // A Data named 'd' at graph level AND a local 'd' inside the region:
    // peek(d, ...) resolves to the Data buffer, not the local.
    let src = "Data d(4); \
               if (1) { \
                 d = 0; /* region-local named d */ \
                 poke(d, 42.0, 0); /* writes to Data buffer d */ \
                 out1 = peek(d, 0); /* reads from Data buffer d */ \
               }";
    let out = render(src, 48000.0, 1);
    assert_eq!(out.ch(0)[0], 42.0);
}

// ═══════════════════════════════════════════════════════════════════
//  String-arg rejection (no external buffer~ in M2)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn buffer_string_arg_rejected() {
    let err = opengen_genexpr::parse_and_lower(r#"Buffer b("ext"); out1 = peek(b, 0);"#).unwrap_err();
    assert!(
        err.to_string().contains("unsupported in M2"),
        "expected error about M2 limitation, got: {}",
        err.to_string()
    );
}
