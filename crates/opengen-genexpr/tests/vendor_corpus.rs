//! Vendor genexpr corpus characterization.
//!
//! Runs the gen~ vendor `.genexpr` corpus (80 files at
//! `reference/gen_exprs/genexpr_js/genexprs/`) and reports parse
//! failures. Also contains minimal reproducing tests for each known
//! failure.
//!
//! The corpus directory is gitignored; tests silently pass when absent.

use opengen_genexpr;

fn run_vendor_corpus() -> Vec<(String, String)> {
    let dir = std::path::Path::new("reference/gen_exprs/genexpr_js/genexprs");
    if !dir.exists() {
        return vec![];
    }
    let mut failures = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("genexpr") {
            continue;
        }
        let src = std::fs::read_to_string(&path).unwrap();
        if let Err(e) = opengen_genexpr::parse(&src) {
            failures.push((
                path.file_name().unwrap().to_string_lossy().to_string(),
                e.to_string(),
            ));
        }
    }
    failures
}

#[test]
fn vendor_genexpr_corpus_report() {
    let failures = run_vendor_corpus();
    for (file, err) in &failures {
        eprintln!("FAIL {}: {}", file, err);
    }
    // Informational — no assertion yet. Once all 80 pass, change to assert 80/80.
    println!("vendor corpus: {}/80 pass", 80 - failures.len());
}

// --- Minimal reproducing tests for each known failure ---
// These tests FAIL now and will PASS after the parser fixes in Tasks 2–4.

#[test]
fn parse_failure_1_comma_in_for_init() {
    let src = "for(i=0, j=0; i<10; i+=1) { out1 = i; }";
    let result = opengen_genexpr::parse(src);
    assert!(
        result.is_ok(),
        "comma in for-init should parse: {}",
        result.unwrap_err()
    );
}

#[test]
fn parse_failure_2_named_argument_in_call() {
    // Parser already accepts named args; the gap is in lowering.
    let src = "out1 = foo(bar=in1);";
    let result = opengen_genexpr::parse_and_lower(src);
    assert!(
        result.is_err(),
        "named arg in call should fail lowering (not yet implemented)"
    );
}

// Placeholders for the 2 remaining failures (to be filled in once identified
// from corpus output).  These are marked #[ignore] until the failure is
// characterized.
#[test]
#[ignore = "characterize from corpus output"]
fn parse_failure_3_tbd() {
    // TODO: fill in once identified from corpus output
}

#[test]
#[ignore = "characterize from corpus output"]
fn parse_failure_4_tbd() {
    // TODO: fill in once identified from corpus output
}

// --- Assertion test: ratchet to 80/80 once all parse ---
#[test]
fn vendor_genexpr_corpus_all_parse() {
    // This test only asserts when the reference directory exists.
    let dir = std::path::Path::new("reference/gen_exprs/genexpr_js/genexprs");
    if !dir.exists() {
        eprintln!("SKIP: reference corpus directory not found");
        return;
    }
    let failures = run_vendor_corpus();
    // Ratchet: unpin to 80 when all 4 failures are fixed.
    assert_eq!(failures.len(), 4, "expected 4 remaining failures");
}
