//! Comparison operators.
use crate::registry::OpDef;
use opengen_ir::StateDecl;

/// Greater than: `out = a > b`.
///
/// # Definition
/// Returns exactly 1.0 if a > b, otherwise 0.0. IEEE-754 f64 comparison.
/// NaN comparisons always return 0.0.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_gt.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = gt(2.0, 1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = gt(1.0, 2.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// let out3 = render("out1 = gt(1.0, 1.0);", 48000.0, 1);
/// assert_eq!(out3.ch(0)[0], 0.0);
/// ```
pub fn gt(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] > inputs[1] { 1.0 } else { 0.0 }
}

/// Greater than or equal: `out = a >= b`.
///
/// # Definition
/// Returns exactly 1.0 if a >= b, otherwise 0.0. IEEE-754 f64 comparison.
/// NaN comparisons always return 0.0.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_gte.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = gte(2.0, 1.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = gte(1.0, 1.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 1.0);
/// let out3 = render("out1 = gte(1.0, 2.0);", 48000.0, 1);
/// assert_eq!(out3.ch(0)[0], 0.0);
/// ```
pub fn gte(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] >= inputs[1] { 1.0 } else { 0.0 }
}

/// Less than: `out = a < b`.
///
/// # Definition
/// Returns exactly 1.0 if a < b, otherwise 0.0. IEEE-754 f64 comparison.
/// NaN comparisons always return 0.0.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_lt.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = lt(1.0, 2.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = lt(2.0, 1.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// let out3 = render("out1 = lt(1.0, 1.0);", 48000.0, 1);
/// assert_eq!(out3.ch(0)[0], 0.0);
/// ```
pub fn lt(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] < inputs[1] { 1.0 } else { 0.0 }
}

/// Less than or equal: `out = a <= b`.
///
/// # Definition
/// Returns exactly 1.0 if a <= b, otherwise 0.0. IEEE-754 f64 comparison.
/// NaN comparisons always return 0.0.
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_lte.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = lte(1.0, 2.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = lte(1.0, 1.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 1.0);
/// let out3 = render("out1 = lte(2.0, 1.0);", 48000.0, 1);
/// assert_eq!(out3.ch(0)[0], 0.0);
/// ```
pub fn lte(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] <= inputs[1] { 1.0 } else { 0.0 }
}

/// Equal: `out = a == b`.
///
/// # Definition
/// Returns exactly 1.0 if a == b, otherwise 0.0. IEEE-754 f64 comparison.
/// NaN comparisons always return 0.0 (NaN != NaN per IEEE-754).
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_eq.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = eq(1.5, 1.5);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = eq(1.5, 2.0);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// ```
pub fn eq(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] == inputs[1] { 1.0 } else { 0.0 }
}

/// Not equal: `out = a != b`.
///
/// # Definition
/// Returns exactly 1.0 if a != b, otherwise 0.0. IEEE-754 f64 comparison.
/// NaN comparisons always return 1.0 (NaN != NaN per IEEE-754).
///
/// # Documented
/// `reference/gen/refpages/common/gen_common_neq.maxref.xml`
///
/// ```
/// use opengen_testkit::render;
/// let out = render("out1 = neq(1.5, 2.0);", 48000.0, 1);
/// assert_eq!(out.ch(0)[0], 1.0);
/// let out2 = render("out1 = neq(1.5, 1.5);", 48000.0, 1);
/// assert_eq!(out2.ch(0)[0], 0.0);
/// ```
pub fn neq(inputs: &[f64], _state: &mut [f64], _sr: f64) -> f64 {
    if inputs[0] != inputs[1] { 1.0 } else { 0.0 }
}

pub fn defs() -> Vec<OpDef> {
    vec![
        OpDef { name: "gt", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: gt },
        OpDef { name: "gte", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: gte },
        OpDef { name: "lt", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: lt },
        OpDef { name: "lte", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: lte },
        OpDef { name: "eq", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: eq },
        OpDef { name: "neq", arity: 2, state: StateDecl::None, deferred_ports: &[], update: None, init: None, kernel: neq },
    ]
}
