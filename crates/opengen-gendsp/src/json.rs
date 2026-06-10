//! Minimal zero-dependency JSON parser.
//!
//! Supports the full JSON grammar including `\uXXXX` escapes (surrogate pairs),
//! scientific-notation numbers (f64), and trailing-garbage tolerance for
//! embedded JSON inside binary `.amxd` containers.
//!
//! # Provenance
//!
//! Written from scratch. Grammar per ECMA-404 / RFC 8259.

use std::fmt;

/// A JSON value.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    /// Look up a key in a JSON object by first exact match.
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(kvs) => kvs.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// Access as f64 if this is a Num variant.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Json::Num(n) => Some(*n),
            _ => None,
        }
    }

    /// Access as &str if this is a Str variant.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Access as &[Json] if this is an Arr variant.
    pub fn as_arr(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(arr) => Some(arr),
            _ => None,
        }
    }
}

/// A JSON parse error.
#[derive(Debug, Clone, PartialEq)]
pub struct JsonError {
    pub msg: String,
    pub offset: usize,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JSON error at offset {}: {}", self.offset, self.msg)
    }
}

impl std::error::Error for JsonError {}

// ─── Parser ───────────────────────────────────────────────────────────────────

struct Parser<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(src: &'a [u8]) -> Self {
        Parser { src, pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let b = self.src.get(self.pos).copied()?;
        self.pos += 1;
        Some(b)
    }

    fn error(&self, msg: impl Into<String>) -> JsonError {
        JsonError { msg: msg.into(), offset: self.pos }
    }

    fn skip_whitespace(&mut self) {
        while let Some(b) = self.peek() {
            if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn expect_byte(&mut self, expected: u8) -> Result<(), JsonError> {
        match self.advance() {
            Some(b) if b == expected => Ok(()),
            Some(b) => Err(self.error(format!("expected '{}', got '{}'", expected as char, b as char))),
            None => Err(self.error(format!("expected '{}', got EOF", expected as char))),
        }
    }

    fn parse_value(&mut self) -> Result<Json, JsonError> {
        self.skip_whitespace();
        match self.peek() {
            Some(b'n') => self.parse_literal("null", Json::Null),
            Some(b't') => self.parse_literal("true", Json::Bool(true)),
            Some(b'f') => self.parse_literal("false", Json::Bool(false)),
            Some(b'"') => self.parse_string().map(Json::Str),
            Some(b'[') => self.parse_array(),
            Some(b'{') => self.parse_object(),
            Some(b) if b == b'-' || b.is_ascii_digit() => self.parse_number(),
            Some(b) => Err(self.error(format!("unexpected character '{}'", b as char))),
            None => Err(self.error("unexpected EOF")),
        }
    }

    fn parse_literal(&mut self, expected: &str, value: Json) -> Result<Json, JsonError> {
        for b in expected.bytes() {
            if self.advance() != Some(b) {
                return Err(self.error(format!("expected '{}'", expected)));
            }
        }
        Ok(value)
    }

    fn parse_string(&mut self) -> Result<String, JsonError> {
        self.expect_byte(b'"')?;
        let mut s = String::new();
        loop {
            match self.advance() {
                None => return Err(self.error("unterminated string")),
                Some(b'"') => return Ok(s),
                Some(b'\\') => {
                    match self.advance() {
                        None => return Err(self.error("unterminated escape in string")),
                        Some(b'"') => s.push('"'),
                        Some(b'\\') => s.push('\\'),
                        Some(b'/') => s.push('/'),
                        Some(b'b') => s.push('\u{0008}'),
                        Some(b'f') => s.push('\u{000c}'),
                        Some(b'n') => s.push('\n'),
                        Some(b'r') => s.push('\r'),
                        Some(b't') => s.push('\t'),
                        Some(b'u') => {
                            let code = self.parse_hex4()?;
                            // Handle surrogate pairs
                            if (0xD800..=0xDBFF).contains(&code) {
                                // High surrogate — expect low surrogate
                                if self.advance() != Some(b'\\') || self.advance() != Some(b'u') {
                                    return Err(self.error("expected low surrogate after high surrogate"));
                                }
                                let low = self.parse_hex4()?;
                                if !(0xDC00..=0xDFFF).contains(&low) {
                                    return Err(self.error("invalid low surrogate"));
                                }
                                let cp = 0x10000 + (code - 0xD800) * 0x400 + (low - 0xDC00);
                                match char::from_u32(cp) {
                                    Some(c) => s.push(c),
                                    None => return Err(self.error("invalid surrogate pair")),
                                }
                            } else {
                                match char::from_u32(code) {
                                    Some(c) => s.push(c),
                                    None => return Err(self.error("invalid unicode escape")),
                                }
                            }
                        }
                        Some(b) => return Err(self.error(format!("invalid escape '\\{}'", b as char))),
                    }
                }
                Some(b) => s.push(b as char),
            }
        }
    }

    fn parse_hex4(&mut self) -> Result<u32, JsonError> {
        let mut code = 0u32;
        for _ in 0..4 {
            match self.advance() {
                Some(b @ b'0'..=b'9') => code = code * 16 + (b - b'0') as u32,
                Some(b @ b'a'..=b'f') => code = code * 16 + (b - b'a' + 10) as u32,
                Some(b @ b'A'..=b'F') => code = code * 16 + (b - b'A' + 10) as u32,
                Some(b) => return Err(self.error(format!("invalid hex digit '{}'", b as char))),
                None => return Err(self.error("unexpected EOF in \\u escape")),
            }
        }
        Ok(code)
    }

    fn parse_number(&mut self) -> Result<Json, JsonError> {
        let start = self.pos;

        // Optional minus
        if self.peek() == Some(b'-') {
            self.advance();
        }

        // Integer part
        match self.peek() {
            Some(b'0') => { self.advance(); }
            Some(b'1'..=b'9') => {
                while let Some(b) = self.peek() {
                    if b.is_ascii_digit() {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            Some(b'.') => { /* fractional-only like .5 — valid */ }
            Some(b) => return Err(self.error(format!("unexpected character '{}' in number", b as char))),
            None => return Err(self.error("unexpected EOF in number")),
        }

        // Fractional part
        if self.peek() == Some(b'.') {
            self.advance();
            // Require at least one digit after decimal point
            match self.peek() {
                Some(b) if b.is_ascii_digit() => {
                    while let Some(b) = self.peek() {
                        if b.is_ascii_digit() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                _ => return Err(self.error("expected digit after decimal point")),
            }
        }

        // Exponent
        if self.peek() == Some(b'e') || self.peek() == Some(b'E') {
            self.advance();
            // Optional sign
            if self.peek() == Some(b'+') || self.peek() == Some(b'-') {
                self.advance();
            }
            match self.peek() {
                Some(b) if b.is_ascii_digit() => {
                    while let Some(b) = self.peek() {
                        if b.is_ascii_digit() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                _ => return Err(self.error("expected digit in exponent")),
            }
        }

        let slice = std::str::from_utf8(&self.src[start..self.pos])
            .map_err(|_| self.error("invalid UTF-8 in number"))?;
        let n: f64 = slice.parse().map_err(|_| self.error(format!("invalid number '{}'", slice)))?;
        Ok(Json::Num(n))
    }

    fn parse_array(&mut self) -> Result<Json, JsonError> {
        self.expect_byte(b'[')?;
        let mut arr = Vec::new();
        self.skip_whitespace();
        if self.peek() == Some(b']') {
            self.advance();
            return Ok(Json::Arr(arr));
        }
        loop {
            arr.push(self.parse_value()?);
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => { self.advance(); self.skip_whitespace(); }
                Some(b']') => { self.advance(); return Ok(Json::Arr(arr)); }
                Some(b) => return Err(self.error(format!("expected ',' or ']' in array, got '{}'", b as char))),
                None => return Err(self.error("unexpected EOF in array")),
            }
        }
    }

    fn parse_object(&mut self) -> Result<Json, JsonError> {
        self.expect_byte(b'{')?;
        let mut kvs = Vec::new();
        self.skip_whitespace();
        if self.peek() == Some(b'}') {
            self.advance();
            return Ok(Json::Obj(kvs));
        }
        loop {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect_byte(b':')?;
            self.skip_whitespace();
            let value = self.parse_value()?;
            kvs.push((key, value));
            self.skip_whitespace();
            match self.peek() {
                Some(b',') => { self.advance(); }
                Some(b'}') => { self.advance(); return Ok(Json::Obj(kvs)); }
                Some(b) => return Err(self.error(format!("expected ',' or '}}' in object, got '{}'", b as char))),
                None => return Err(self.error("unexpected EOF in object")),
            }
        }
    }
}

/// Parse a JSON string into a `Json` value.
pub fn parse(src: &str) -> Result<Json, JsonError> {
    let mut p = Parser::new(src.as_bytes());
    let value = p.parse_value()?;
    p.skip_whitespace();
    if p.peek().is_some() {
        return Err(p.error("trailing characters after JSON value"));
    }
    Ok(value)
}

/// Parse the first JSON document in a possibly binary-wrapped buffer.
///
/// `.amxd` containers wrap JSON in binary headers. This function seeks to the
/// first `{`, decodes the JSON from there, and ignores trailing bytes.
pub fn parse_embedded(bytes: &[u8]) -> Result<Json, JsonError> {
    // Find the first '{' in the buffer
    let start = bytes.iter().position(|&b| b == b'{')
        .ok_or_else(|| JsonError {
            msg: "no JSON object found in buffer".to_string(),
            offset: 0,
        })?;

    let mut p = Parser::new(&bytes[start..]);
    let value = p.parse_value()?;
    // Don't check for trailing data — embedded JSON may have trailing binary bytes
    Ok(value)
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Scalars ───────────────────────────────────────────────────────

    #[test]
    fn parse_null() {
        assert_eq!(parse("null").unwrap(), Json::Null);
    }

    #[test]
    fn parse_bool() {
        assert_eq!(parse("true").unwrap(), Json::Bool(true));
        assert_eq!(parse("false").unwrap(), Json::Bool(false));
    }

    #[test]
    fn parse_integer() {
        assert_eq!(parse("42").unwrap(), Json::Num(42.0));
        assert_eq!(parse("0").unwrap(), Json::Num(0.0));
        assert_eq!(parse("-1").unwrap(), Json::Num(-1.0));
    }

    #[test]
    fn parse_float() {
        assert_eq!(parse("3.14").unwrap(), Json::Num(3.14));
        assert_eq!(parse("-0.5").unwrap(), Json::Num(-0.5));
    }

    #[test]
    fn parse_scientific() {
        assert_eq!(parse("1e-5").unwrap(), Json::Num(1e-5));
        assert_eq!(parse("1.5E3").unwrap(), Json::Num(1500.0));
        assert_eq!(parse("2.5e+2").unwrap(), Json::Num(250.0));
    }

    // ── Strings and escapes ───────────────────────────────────────────

    #[test]
    fn parse_simple_string() {
        assert_eq!(parse(r#""hello""#).unwrap(), Json::Str("hello".to_string()));
    }

    #[test]
    fn parse_unicode_escape() {
        assert_eq!(parse(r#""\u00e4""#).unwrap(), Json::Str("ä".to_string()));
    }

    #[test]
    fn parse_surrogate_pair() {
        // U+1F600 = grinning face emoji = D83D DE00
        assert_eq!(parse(r#""\uD83D\uDE00""#).unwrap(), Json::Str("😀".to_string()));
    }

    #[test]
    fn parse_control_escapes() {
        assert_eq!(parse(r#""\n\t\r\\\"""#).unwrap(), Json::Str("\n\t\r\\\"".to_string()));
    }

    // ── Nesting ───────────────────────────────────────────────────────

    #[test]
    fn parse_empty_array() {
        assert_eq!(parse("[]").unwrap(), Json::Arr(vec![]));
    }

    #[test]
    fn parse_array() {
        let v = parse("[1, 2, 3]").unwrap();
        assert_eq!(v, Json::Arr(vec![Json::Num(1.0), Json::Num(2.0), Json::Num(3.0)]));
    }

    #[test]
    fn parse_empty_object() {
        assert_eq!(parse("{}").unwrap(), Json::Obj(vec![]));
    }

    #[test]
    fn parse_nested_object() {
        let src = r#"{"a": {"b": [1, 2]}}"#;
        let v = parse(src).unwrap();
        let inner = v.get("a").unwrap();
        let arr = inner.get("b").unwrap().as_arr().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn parse_object_with_multiple_keys() {
        let src = r#"{"x": 1, "y": 2}"#;
        let v = parse(src).unwrap();
        assert_eq!(v.get("x").unwrap().as_f64(), Some(1.0));
        assert_eq!(v.get("y").unwrap().as_f64(), Some(2.0));
        assert_eq!(v.get("z"), None);
    }

    // ── Typed accessors ───────────────────────────────────────────────

    #[test]
    fn json_accessors() {
        let v = Json::Num(3.14);
        assert_eq!(v.as_f64(), Some(3.14));
        assert_eq!(v.as_str(), None);
        assert_eq!(v.as_arr(), None);

        let v = Json::Str("hi".into());
        assert_eq!(v.as_str(), Some("hi"));

        let v = Json::Arr(vec![]);
        assert_eq!(v.as_arr(), Some(&[][..]));
    }

    // ── Error offsets on malformed input ─────────────────────────────

    #[test]
    fn error_on_unterminated_string() {
        let err = parse(r#""unterminated"#).unwrap_err();
        assert!(err.offset > 0);
        assert!(err.msg.contains("unterminated"));
    }

    #[test]
    fn error_on_bad_token() {
        let err = parse("?").unwrap_err();
        assert!(err.msg.contains("unexpected"));
    }

    #[test]
    fn error_on_trailing_garbage() {
        let err = parse("1 abc").unwrap_err();
        assert!(err.msg.contains("trailing"));
    }

    #[test]
    fn error_on_invalid_escape() {
        let err = parse(r#""\x""#).unwrap_err();
        assert!(err.msg.contains("invalid escape"));
    }

    // ── Embedded / amxd-aware ─────────────────────────────────────────

    #[test]
    fn parse_embedded_ignores_prefix_bytes() {
        let buf = [0x00, 0x01, 0x02, b'{', b'"', b'a', b'"', b':', b'1', b'}', 0xFF, 0xFE];
        let v = parse_embedded(&buf).unwrap();
        assert_eq!(v.get("a").unwrap().as_f64(), Some(1.0));
    }

    #[test]
    fn parse_embedded_ignores_trailing_bytes() {
        let buf = b"{}trailing";
        let v = parse_embedded(buf).unwrap();
        assert_eq!(v, Json::Obj(vec![]));
    }

    #[test]
    fn parse_embedded_no_json_object() {
        let buf = b"no braces here";
        let err = parse_embedded(buf).unwrap_err();
        assert!(err.msg.contains("no JSON object"));
    }

    // ── Conformance: real .gendsp files when reference/ exists ────────

    #[test]
    fn parse_all_gendsp_examples() {
        let root = std::path::Path::new("reference/gen/examples");
        if !root.exists() {
            eprintln!("skipping: reference/ directory not available");
            return;
        }

        let mut count = 0;
        for entry in std::fs::read_dir(root).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("gendsp") {
                continue;
            }
            let content = std::fs::read(&path).unwrap();
            let json = parse_embedded(&content)
                .unwrap_or_else(|e| panic!("{}: parse error: {}", path.display(), e));

            // Assert patcher.boxes is an array
            let patcher = json.get("patcher")
                .unwrap_or_else(|| panic!("{}: missing 'patcher' key", path.display()));
            let boxes = patcher.get("boxes")
                .unwrap_or_else(|| panic!("{}: missing 'boxes' key", path.display()));
            assert!(boxes.as_arr().is_some(), "{}: 'boxes' is not an array", path.display());
            count += 1;
        }

        assert!(count > 0, "no .gendsp files found in reference/gen/examples");
    }
}
