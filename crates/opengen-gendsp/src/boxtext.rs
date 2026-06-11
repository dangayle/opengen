//! Box-text tokeniser and classifier.
//!
//! Processes the `text` field of `newobj` boxes, tokenising on whitespace and
//! classifying the box's role (operator, I/O port, param, history, delay, etc.).
//!
//! # Provenance
//!
//! Tokenisation rules per `@rnbo/xam/lib/parser.js` (box text tokenizer).
//! Classification table derived from `docs/research/gen_docs/gendsp_ebnf.md`
//! Section 3 (special object text forms) and Section 2 (operator names).
//! Cross-checked against `reference/gen/examples/*.gendsp` (cite paths only).

use opengen_genexpr::{Expr, parse_expression};

/// Classification of a box's role based on its text content.
#[derive(Debug, Clone, PartialEq)]
pub enum BoxKind {
    /// `in N [name…] [@comment c]` — patcher inlet (1-based N)
    Inlet(u16),
    /// `out N [name…]` — patcher outlet (1-based N)
    Outlet(u16),
    /// `param NAME [default…] [@min e] [@max e]` — parameter
    /// The f64 is the default value parsed from the box text (defaults to 0.0 if missing).
    Param(String, f64),
    /// `setparam NAME` — D13 rewiring
    SetParam(String),
    /// Constant value: `f V` or bare numeric text (`0.5`, `75`)
    Constant(f64),
    /// `send NAME` / `s NAME` — signal bus send
    Send(String),
    /// `receive NAME` / `r NAME` — signal bus receive
    Receive(String),
    /// `history [NAME]` — history node, optional binding name
    History(Option<String>),
    /// `delay SIZE [TAPS]` — delay line (TAPS > 1 → clear error, M3)
    Delay(u32, u32), // (size, taps)
    /// `data NAME SIZE…` / `buffer NAME…` — data region (D10)
    Data(String),
    /// Subpatcher/abstraction: `gen @file NAME`, `gen @gen NAME`, bare `NAME` (unknown op)
    Subpatcher(String),
    /// `expr EXPRESSION…` — expression box with GenExpr expression
    Expr(Expr),
    /// `mc_channel` — constant 1.0 (D14)
    McChannel,
    /// Named operator (in the op registry). Positional args fill trailing inlets.
    /// `args` are numeric literals; `expr_args` are non-numeric arg tokens (e.g. `twopi/samplerate`,
    /// a param name) that must be parsed as GenExpr expressions at build time.
    Operator {
        name: String,
        args: Vec<f64>,
        /// Raw text of non-numeric positional args (expression args, param-name args)
        expr_args: Vec<String>,
        attrs: Vec<(String, String)>,
    },
}

/// Parse a box `text` field into its classification and parsed arguments.
///
/// Tokenisation: whitespace-delimited tokens. Tokens from the first
/// `@`-prefixed token onward are attribute pairs `(@attr value…)`.
/// The leading token is the operator/class name. Remaining tokens before
/// `@` are positional args.
pub fn classify_box_text(text: &str) -> BoxKind {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    if tokens.is_empty() {
        // Empty text → treat as constant 0.0? Or just a passthrough?
        return BoxKind::Operator { name: String::new(), args: vec![], expr_args: vec![], attrs: vec![] };
    }

    let (cmd, rest) = {
        let cmd = tokens[0];
        let rest = &tokens[1..];
        (cmd, rest)
    };

    // ── I/O ports ──────────────────────────────────────────────────
    if let Some(idx) = try_parse_io(cmd, "in", rest) {
        return BoxKind::Inlet(idx);
    }
    if let Some(idx) = try_parse_io(cmd, "out", rest) {
        return BoxKind::Outlet(idx);
    }

    // ── Param ──────────────────────────────────────────────────────
    if cmd == "param" && !rest.is_empty() {
        let name = rest[0];
        // Parse default value if present (2nd positional arg)
        let default = if rest.len() > 1 {
            let raw_default = rest[1];
            if raw_default.starts_with('@') {
                0.0
            } else {
                raw_default.parse::<f64>().unwrap_or(0.0)
            }
        } else {
            0.0
        };
        return BoxKind::Param(name.to_string(), default);
    }

    // ── setparam ───────────────────────────────────────────────────
    if cmd == "setparam" && rest.len() >= 1 {
        return BoxKind::SetParam(rest[0].to_string());
    }

    // ── Constant: f V or bare number ────────────────────────────────
    if cmd == "f" && rest.len() >= 1 {
        if let Ok(n) = parse_f64_token(rest[0]) {
            return BoxKind::Constant(n);
        }
    }
    if let Ok(n) = parse_f64_token(cmd) {
        return BoxKind::Constant(n);
    }

    // ── Send / Receive ─────────────────────────────────────────────
    if (cmd == "send" || cmd == "s") && rest.len() >= 1 {
        return BoxKind::Send(rest[0].to_string());
    }
    if (cmd == "receive" || cmd == "r") && rest.len() >= 1 {
        return BoxKind::Receive(rest[0].to_string());
    }

    // ── History ────────────────────────────────────────────────────
    if cmd == "history" {
        let name = if rest.is_empty() { None } else { Some(rest[0].to_string()) };
        return BoxKind::History(name);
    }

    // ── Delay ──────────────────────────────────────────────────────
    if cmd == "delay" {
        let size = if rest.len() >= 1 {
            rest[0].parse::<u32>().unwrap_or(48000)
        } else {
            48000 // default = samplerate per gen~ convention
        };
        let taps = if rest.len() > 1 {
            rest[1].parse::<u32>().unwrap_or(1)
        } else {
            1
        };
        return BoxKind::Delay(size, taps);
    }

    // ── Data / Buffer ──────────────────────────────────────────────
    if (cmd == "data" || cmd == "buffer" || cmd == "buffer~") && rest.len() >= 1 {
        return BoxKind::Data(rest[0].to_string());
    }

    // ── mc_channel ─────────────────────────────────────────────────
    if cmd == "mc_channel" {
        return BoxKind::McChannel;
    }

    // ── gen subpatcher ─────────────────────────────────────────────
    if cmd == "gen" && rest.len() >= 2 {
        // gen @file NAME or gen @gen NAME
        let attr = rest[0];
        if attr == "@file" || attr == "@gen" {
            return BoxKind::Subpatcher(rest[1].to_string());
        }
    }

    // ── Expr box ───────────────────────────────────────────────────
    if cmd == "expr" && !rest.is_empty() {
        let expr_text = rest.join(" ");
        if let Ok(expr) = parse_expression(&expr_text) {
            return BoxKind::Expr(expr);
        }
    }

    // ── Operator (default) ─────────────────────────────────────────
    // Split positional args from @-prefixed attribute tokens
    let mut args = Vec::new();
    let mut expr_args = Vec::new();
    let mut attrs = Vec::new();
    let mut i = 0;
    while i < rest.len() {
        if rest[i].starts_with('@') {
            // Attribute pair: @name value
            let attr_name = rest[i].to_string();
            i += 1;
            let attr_val = if i < rest.len() && !rest[i].starts_with('@') {
                let v = rest[i].to_string();
                i += 1;
                v
            } else {
                String::new()
            };
            attrs.push((attr_name, attr_val));
        } else {
            // Positional arg — try to parse as f64
            if let Ok(n) = parse_f64_token(rest[i]) {
                args.push(n);
            } else {
                // Non-numeric arg — keep as expression string
                expr_args.push(rest[i].to_string());
            }
            i += 1;
        }
    }

    BoxKind::Operator { name: cmd.to_string(), args, expr_args, attrs }
}

/// Try to parse an I/O port: `in N [name…]` or `out N [name…]`.
fn try_parse_io(cmd: &str, expected: &str, rest: &[&str]) -> Option<u16> {
    if cmd != expected {
        return None;
    }
    if rest.is_empty() {
        return None;
    }
    rest[0].parse::<u16>().ok().filter(|&n| n >= 1)
}

/// Parse a token as f64, supporting scientific notation and decimals.
fn parse_f64_token(s: &str) -> Result<f64, ()> {
    s.parse::<f64>().map_err(|_| ())
}

/// Get a structured representation of the tokens split into
/// [classname, positional_args, attributes].
pub fn tokenise(text: &str) -> (String, Vec<String>, Vec<(String, String)>) {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    if tokens.is_empty() {
        return (String::new(), vec![], vec![]);
    }

    let classname = tokens[0].to_string();
    let mut positional = Vec::new();
    let mut attrs = Vec::new();
    let mut in_attrs = false;

    for &token in &tokens[1..] {
        if token.starts_with('@') {
            in_attrs = true;
            let attr_name = token.to_string();
            attrs.push((attr_name, String::new()));
        } else if in_attrs {
            // Assign to the most recent attribute
            if let Some(last) = attrs.last_mut() {
                if last.1.is_empty() {
                    last.1 = token.to_string();
                    continue;
                }
            }
            // If no pending attribute, add as positional
            positional.push(token.to_string());
        } else {
            positional.push(token.to_string());
        }
    }

    (classname, positional, attrs)
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── I/O ports ──────────────────────────────────────────────────

    #[test]
    fn classify_inlet() {
        assert_eq!(classify_box_text("in 1"), BoxKind::Inlet(1));
        assert_eq!(classify_box_text("in 3 @comment fb"), BoxKind::Inlet(3));
        assert_eq!(classify_box_text("in 2 left"), BoxKind::Inlet(2));
        assert_eq!(classify_box_text("out 1"), BoxKind::Outlet(1));
        assert_eq!(classify_box_text("out 2 @comment bridge"), BoxKind::Outlet(2));
    }

    #[test]
    fn classify_in_requires_number() {
        // "in" alone without number should not be an Inlet
        let kind = classify_box_text("in");
        assert!(!matches!(kind, BoxKind::Inlet(_)));
    }

    // ── Param ──────────────────────────────────────────────────────

    #[test]
    fn classify_param() {
        assert_eq!(classify_box_text("param freq 600 @min 1 @max samplerate/2"),
                   BoxKind::Param("freq".to_string(), 600.0));
        assert_eq!(classify_box_text("param spread 0 @min 0 @max 400"),
                   BoxKind::Param("spread".to_string(), 0.0));
        assert_eq!(classify_box_text("param dampen 0.25"),
                   BoxKind::Param("dampen".to_string(), 0.25));
        assert_eq!(classify_box_text("param pitch"),
                   BoxKind::Param("pitch".to_string(), 0.0));
    }

    // ── setparam ───────────────────────────────────────────────────

    #[test]
    fn classify_setparam() {
        let kind = classify_box_text("setparam gain");
        assert_eq!(kind, BoxKind::SetParam("gain".to_string()));
    }

    // ── Constant ───────────────────────────────────────────────────

    #[test]
    fn classify_constant_f() {
        assert_eq!(classify_box_text("f 0.5"), BoxKind::Constant(0.5));
        assert_eq!(classify_box_text("f 75"), BoxKind::Constant(75.0));
    }

    #[test]
    fn classify_constant_bare_number() {
        assert_eq!(classify_box_text("0.5"), BoxKind::Constant(0.5));
        assert_eq!(classify_box_text("75"), BoxKind::Constant(75.0));
        assert_eq!(classify_box_text("-3.14"), BoxKind::Constant(-3.14));
    }

    // ── Send / Receive ─────────────────────────────────────────────

    #[test]
    fn classify_send_receive() {
        assert_eq!(classify_box_text("send mybus"), BoxKind::Send("mybus".to_string()));
        assert_eq!(classify_box_text("s mybus"), BoxKind::Send("mybus".to_string()));
        assert_eq!(classify_box_text("receive mybus"), BoxKind::Receive("mybus".to_string()));
        assert_eq!(classify_box_text("r mybus"), BoxKind::Receive("mybus".to_string()));
    }

    // ── History ────────────────────────────────────────────────────

    #[test]
    fn classify_history() {
        assert_eq!(classify_box_text("history"), BoxKind::History(None));
        assert_eq!(classify_box_text("history y1"), BoxKind::History(Some("y1".to_string())));
        assert_eq!(classify_box_text("history y2"), BoxKind::History(Some("y2".to_string())));
    }

    // ── Delay ──────────────────────────────────────────────────────

    #[test]
    fn classify_delay() {
        assert_eq!(classify_box_text("delay 2000"), BoxKind::Delay(2000, 1));
        assert_eq!(classify_box_text("delay 44100"), BoxKind::Delay(44100, 1));
        // delay with no size defaults to samplerate (48000)
        assert_eq!(classify_box_text("delay"), BoxKind::Delay(48000, 1));
    }

    // ── Data / Buffer ──────────────────────────────────────────────

    #[test]
    fn classify_data() {
        let kind = classify_box_text("data mydata 512");
        assert_eq!(kind, BoxKind::Data("mydata".to_string()));
    }

    #[test]
    fn classify_buffer() {
        assert_eq!(classify_box_text("buffer mybuf"), BoxKind::Data("mybuf".to_string()));
        assert_eq!(classify_box_text("buffer~ mybuf"), BoxKind::Data("mybuf".to_string()));
    }

    // ── mc_channel ─────────────────────────────────────────────────

    #[test]
    fn classify_mc_channel() {
        assert_eq!(classify_box_text("mc_channel"), BoxKind::McChannel);
    }

    // ── Expr ───────────────────────────────────────────────────────

    #[test]
    fn classify_expr_simple() {
        let kind = classify_box_text("expr in1 * 2");
        if let BoxKind::Expr(expr) = kind {
            // Should parse as multiplication: in1 * 2
            match &expr {
                Expr::BinOp { op: opengen_genexpr::BinOpKind::Mul, .. } => {}
                other => panic!("expected BinOp::Mul, got {:?}", other),
            }
        } else {
            panic!("expected BoxKind::Expr");
        }
    }

    #[test]
    fn classify_expr_function_call() {
        let kind = classify_box_text("expr cycle(freq)");
        if let BoxKind::Expr(expr) = kind {
            match &expr {
                Expr::Call { name, .. } => assert_eq!(name, "cycle"),
                other => panic!("expected Call, got {:?}", other),
            }
        } else {
            panic!("expected BoxKind::Expr");
        }
    }

    // ── Subpatcher ─────────────────────────────────────────────────

    #[test]
    fn classify_subpatcher_gen_file() {
        let kind = classify_box_text("gen @file freeverb_allpass");
        assert_eq!(kind, BoxKind::Subpatcher("freeverb_allpass".to_string()));
    }

    #[test]
    fn classify_subpatcher_gen_gen() {
        let kind = classify_box_text("gen @gen mypatch");
        assert_eq!(kind, BoxKind::Subpatcher("mypatch".to_string()));
    }

    // ── Operators (default) ────────────────────────────────────────

    #[test]
    fn classify_operator_add() {
        let kind = classify_box_text("+");
        assert_eq!(kind, BoxKind::Operator { name: "+".to_string(), args: vec![], expr_args: vec![], attrs: vec![] });
    }

    #[test]
    fn classify_operator_slide() {
        let kind = classify_box_text("slide 200 200");
        match kind {
            BoxKind::Operator { name, args, expr_args, .. } => {
                assert_eq!(name, "slide");
                assert_eq!(args, vec![200.0, 200.0]);
                assert!(expr_args.is_empty());
            }
            other => panic!("expected Operator, got {:?}", other),
        }
    }

    #[test]
    fn classify_operator_with_attrs() {
        let kind = classify_box_text("lookup @interp linear");
        match kind {
            BoxKind::Operator { name, args, expr_args, attrs } => {
                assert_eq!(name, "lookup");
                assert!(args.is_empty());
                assert!(expr_args.is_empty());
                assert_eq!(attrs, vec![("@interp".to_string(), "linear".to_string())]);
            }
            other => panic!("expected Operator, got {:?}", other),
        }
    }

    #[test]
    fn classify_operator_subtract_with_constant() {
        let kind = classify_box_text("!- 1");
        match kind {
            BoxKind::Operator { name, args, expr_args, .. } => {
                assert_eq!(name, "!-");
                assert_eq!(args, vec![1.0]);
                assert!(expr_args.is_empty());
            }
            other => panic!("expected Operator, got {:?}", other),
        }
    }

    // ── Tokenise helper ────────────────────────────────────────────

    #[test]
    fn tokenise_simple() {
        let (name, pos, attrs) = tokenise("slide 200 200");
        assert_eq!(name, "slide");
        assert_eq!(pos, vec!["200", "200"]);
        assert!(attrs.is_empty());
    }

    #[test]
    fn tokenise_with_attrs() {
        let (name, pos, attrs) = tokenise("param freq 600 @min 1 @max samplerate/2");
        assert_eq!(name, "param");
        assert_eq!(pos, vec!["freq", "600"]);
        assert_eq!(attrs, vec![
            ("@min".to_string(), "1".to_string()),
            ("@max".to_string(), "samplerate/2".to_string()),
        ]);
    }

    #[test]
    fn tokenise_inlet_with_comment() {
        let (name, pos, attrs) = tokenise("in 3 @comment fb");
        assert_eq!(name, "in");
        assert_eq!(pos, vec!["3"]);
        assert_eq!(attrs, vec![("@comment".to_string(), "fb".to_string())]);
    }

    // ── Conformance: classify boxes from real .gendsp files ────────

    #[test]
    fn classify_all_example_boxes() {
        let root = std::path::Path::new("reference/gen/examples");
        if !root.exists() {
            eprintln!("skipping: reference/ directory not available");
            return;
        }

        use crate::json;
        use crate::model::Patcher;

        let mut total_boxes = 0;
        for entry in std::fs::read_dir(root).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("gendsp") {
                continue;
            }
            let content = std::fs::read(&path).unwrap();
            let j = json::parse_embedded(&content).unwrap();
            let patcher = Patcher::from_json(&j).unwrap_or_else(|e| panic!("{}: {}", path.display(), e));

            for bx in &patcher.boxes {
                if bx.maxclass == "newobj" && !bx.text.is_empty() {
                    let kind = classify_box_text(&bx.text);
                    // Every newobj with text should produce a meaningful BoxKind
                    match &kind {
                        BoxKind::Operator { name, expr_args: _, .. } => {
                            assert!(!name.is_empty(), "{}: empty operator name in '{}'",
                                    path.display(), bx.text);
                        }
                        // All special kinds are valid
                        _ => {}
                    }
                    total_boxes += 1;
                }
            }
        }

        assert!(total_boxes > 0, "no newobj boxes found");
        eprintln!("classified {} newobj boxes from .gendsp examples", total_boxes);
    }
}
