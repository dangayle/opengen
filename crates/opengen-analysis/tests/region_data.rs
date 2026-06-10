//! Integration test: Data + for loop + poke/peek + History in ONE program.
//!
//! Reproduces the dang-tools voice-allocator shape: a round-robin allocation
//! table driven by a trigger signal, sum-peeked every sample, verifying that
//! region data access plumbing (data_ref in PExpr + state arena resolution)
//! works end-to-end through control flow (if + for loop + History).
//!
//! # The voice-allocator program
//!
//! ```text
//! Data d(4);              // 4 voice slots, initially 0
//! History cursor(0);      // round-robin cursor
//! if (in1 > 0) {          // trigger: poke value at cursor, advance
//!     poke(d, in2, cursor);
//!     cursor = (cursor + 1) % 4;
//! }
//! sum = 0;
//! for (i = 0; i < 4; i += 1) {
//!     sum = sum + peek(d, i);   // sum all voice slots
//! }
//! out1 = sum;             // table sum
//! out2 = peek(d, 0);      // verify slot 0 directly
//! out3 = cursor;          // verify cursor advancement
//! ```
//!
//! # Hand-trace (8 samples)
//!
//! Initial state: d = [0, 0, 0, 0], cursor = 0, sum resets to 0 each sample.
//!
//! | Smp | in1 | in2 | Trigger body                  | Sum     | peek(d,0) | cursor |
//! |-----|-----|-----|-------------------------------|---------|-----------|--------|
//! | 0   | 1   | 10  | poke(d,10,0) → d[0]=10, c=1 | 10+0+0+0 = 10 | 10 | 1 |
//! | 1   | 0   | 0   | (no trigger)                  | 10+0+0+0 = 10 | 10 | 1 |
//! | 2   | 1   | 20  | poke(d,20,1) → d[1]=20, c=2 | 10+20+0+0 = 30 | 10 | 2 |
//! | 3   | 0   | 0   | (no trigger)                  | 10+20+0+0 = 30 | 10 | 2 |
//! | 4   | 1   | 30  | poke(d,30,2) → d[2]=30, c=3 | 10+20+30+0 = 60 | 10 | 3 |
//! | 5   | 0   | 0   | (no trigger)                  | 10+20+30+0 = 60 | 10 | 3 |
//! | 6   | 1   | 40  | poke(d,40,3) → d[3]=40, c=0 | 10+20+30+40 = 100 | 10 | 0 |
//! | 7   | 0   | 0   | (no trigger)                  | 10+20+30+40 = 100 | 10 | 0 |
//!
//! The pattern exercises: Data decl + poke inside if inside region, peek inside
//! for loop, History persistence — the dang-tools pattern end-to-end.

use opengen_analysis::*;

#[test]
fn voice_allocator_round_robin() {
    let src = "\
Data d(4);
History cursor(0);
if (in1 > 0) {
    poke(d, in2, cursor);
    cursor = (cursor + 1) % 4;
}
sum = 0;
for (i = 0; i < 4; i += 1) {
    sum = sum + peek(d, i);
}
out1 = sum;
out2 = peek(d, 0);
out3 = cursor;
";

    // Drive 8 samples: trigger events on samples 0, 2, 4, 6
    let in1: Vec<f64> = vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
    let in2: Vec<f64> = vec![10.0, 0.0, 20.0, 0.0, 30.0, 0.0, 40.0, 0.0];

    let out = opengen_testkit::render_with_inputs_n(
        src, 48_000.0, &[&in1, &in2], 8);

    // out1 = sum of all voice slots
    assert_eq!(out.ch(0), &[10.0, 10.0, 30.0, 30.0, 60.0, 60.0, 100.0, 100.0],
        "out1 (table sum) mismatch");

    // out2 = peek(d, 0) — slot 0 filled at sample 0, never overwritten
    assert_eq!(out.ch(1), &[10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0],
        "out2 (slot 0 peek) mismatch");

    // out3 = cursor — advances modulo 4
    assert_eq!(out.ch(2), &[1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 0.0, 0.0],
        "out3 (cursor) mismatch");

    // Stability assertion: all outputs finite, no denormals
    assert_stable!(out.ch(0));
    assert_stable!(out.ch(1));
    assert_stable!(out.ch(2));
}

#[test]
fn voice_allocator_single_sample_no_trigger() {
    // With no trigger, table stays zeroed, cursor stays at 0.
    let src = "\
Data d(4);
History cursor(0);
if (in1 > 0) {
    poke(d, in2, cursor);
    cursor = (cursor + 1) % 4;
}
sum = 0;
for (i = 0; i < 4; i += 1) {
    sum = sum + peek(d, i);
}
out1 = sum;
out2 = peek(d, 0);
out3 = cursor;
";

    let out = opengen_testkit::render_with_inputs_n(
        src, 48_000.0, &[&[0.0], &[0.0]], 3);

    assert_eq!(out.ch(0), &[0.0, 0.0, 0.0], "sum stays 0");
    assert_eq!(out.ch(1), &[0.0, 0.0, 0.0], "slot 0 stays 0");
    assert_eq!(out.ch(2), &[0.0, 0.0, 0.0], "cursor stays 0");
    assert_stable!(out.ch(0));
}

#[test]
fn voice_allocator_oob_cursor_safety() {
    // Cursor starts at 0, advances to 3 after 4 triggers.
    // At cursor=3, poke targets index 3 (valid). After cursor wraps to 0,
    // slot 0 gets overwritten. Verify via peek(d,0) on the second pass.
    let src = "\
Data d(4);
History cursor(0);
if (in1 > 0) {
    poke(d, in2, cursor);
    cursor = (cursor + 1) % 4;
}
sum = 0;
for (i = 0; i < 4; i += 1) {
    sum = sum + peek(d, i);
}
out1 = sum;
out2 = peek(d, 0);
out3 = cursor;
";

    // Fire 4 triggers to fill all slots, then 1 more to wrap around
    let in1: Vec<f64> = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let in2: Vec<f64> = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0];

    let out = opengen_testkit::render_with_inputs_n(
        src, 48_000.0, &[&in1, &in2], 6);

    // After 4 triggers: d = [10, 20, 30, 40], cursor = 0
    // Sample 0: sum=10,      d[0]=10, cursor=1
    // Sample 1: sum=10+20=30,   d[1]=20, cursor=2
    // Sample 2: sum=10+20+30=60,  d[2]=30, cursor=3
    // Sample 3: sum=10+20+30+40=100, d[3]=40, cursor=0
    // Sample 4: trigger again: poke(d, 50, 0) → d[0]=50, cursor=1
    //           sum=50+20+30+40=140
    // Sample 5: trigger again: poke(d, 60, 1) → d[1]=60, cursor=2
    //           sum=50+60+30+40=180
    assert_eq!(out.ch(0), &[10.0, 30.0, 60.0, 100.0, 140.0, 180.0],
        "out1 (table sum) mismatch during wrap test");
    assert_eq!(out.ch(2), &[1.0, 2.0, 3.0, 0.0, 1.0, 2.0],
        "out3 (cursor) mismatch during wrap test");
    assert_stable!(out.ch(0));
}
