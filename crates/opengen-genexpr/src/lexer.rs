//! Hand-written lexer for GenExpr
//!
//! Numeric form disambiguation (per genexpr.pegjs):
//! - Digits then dot with no following digit → float literal (`1.` → `1.0`)
//! - Dot then digit → float literal (`.5` → `0.5`)
//! - Lone dot between identifiers → Dot token (member access)
//! - Scientific notation: `[eE][+-]?digits` after a number
//!
//! Maximal munch: two-char operators (`&&`, `^^`, `<<`, `>>`, `+=`, etc.)
//! are checked before their single-char counterparts.

use crate::ast::SourceLoc;

/// A token with its source location.
#[derive(Debug, Clone, PartialEq)]
pub struct Spanned {
    pub tok: Token,
    pub loc: SourceLoc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),
    // Keywords
    Param,
    Require,
    Return,
    Break,
    Continue,
    If,
    Else,
    While,
    Do,
    For,
    // Punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Comma,
    Dot,
    Equals,
    Question,
    Colon,
    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    // Bitwise operators
    Amp,       // &
    Pipe,      // |
    Caret,     // ^
    CaretCaret,// ^^
    Shl,       // <<
    Shr,       // >>
    // Logical operators
    AndAnd,    // &&
    OrOr,      // ||
    // Unary
    Bang,      // !
    // Comparison operators
    Gt,         // >
    Gte,        // >=
    Lt,         // <
    Lte,        // <=
    EqualEqual, // ==
    BangEqual,  // !=
    // String literal
    Str(String),
    // End of input
    Eof,
}

#[derive(Clone)]
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: u32,
    col: u32,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) {
        match self.current() {
            Some('\n') => {
                self.line += 1;
                self.col = 1;
            }
            Some('\r') => {
                // Treat CR as a newline; tabs count as one column (no tab-stop expansion)
                self.line += 1;
                self.col = 1;
                // If followed by LF, consume it to treat \r\n as a single line break
                if self.input.get(self.pos + 1) == Some(&'\n') {
                    self.pos += 1;
                }
            }
            Some(_) => {
                // Tabs count as one column (no tab-stop expansion)
                self.col += 1;
            }
            None => {}
        }
        self.pos += 1;
    }

    /// Skip whitespace and comments. Returns an error if a block comment is unterminated.
    fn skip_whitespace_and_comments(&mut self) -> Result<(), String> {
        loop {
            // Skip whitespace
            while let Some(ch) = self.current() {
                if ch.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }

            // Skip line comments: // ... \n
            if self.current() == Some('/') && self.input.get(self.pos + 1) == Some(&'/') {
                while let Some(ch) = self.current() {
                    if ch == '\n' || ch == '\r' {
                        break;
                    }
                    self.advance();
                }
                continue;
            }

            // Skip block comments: /* ... */
            if self.current() == Some('/') && self.input.get(self.pos + 1) == Some(&'*') {
                // Capture start location before consuming /*
                let start_line = self.line;
                let start_col = self.col;
                // consume /*
                self.advance();
                self.advance();
                loop {
                    match self.current() {
                        Some('*') if self.input.get(self.pos + 1) == Some(&'/') => {
                            // consume */
                            self.advance();
                            self.advance();
                            break;
                        }
                        Some(_) => {
                            self.advance();
                        }
                        None => {
                            return Err(format!(
                                "unterminated block comment starting at line {}, col {}",
                                start_line, start_col
                            ));
                        }
                    }
                }
                continue;
            }

            break;
        }
        Ok(())
    }

    pub fn next_token(&mut self) -> Result<Spanned, String> {
        self.skip_whitespace_and_comments()?;
        let start_loc = SourceLoc { line: self.line, col: self.col };

        let ch = match self.current() {
            Some(c) => c,
            None => return Ok(Spanned { tok: Token::Eof, loc: start_loc }),
        };

        // Numbers
        if ch.is_ascii_digit() || (ch == '.' && self.peek_is_digit()) {
            return self.read_number_with_loc(start_loc);
        }

        // Identifiers and keywords
        if ch.is_alphabetic() || ch == '_' {
            return self.read_ident_or_keyword_with_loc(start_loc);
        }

        // Multi-character operators (maximal munch)
        if ch == '&' {
            self.advance();
            if self.current() == Some('&') {
                self.advance();
                return Ok(Spanned { tok: Token::AndAnd, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Amp, loc: start_loc });
        }
        if ch == '|' {
            self.advance();
            if self.current() == Some('|') {
                self.advance();
                return Ok(Spanned { tok: Token::OrOr, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Pipe, loc: start_loc });
        }
        if ch == '^' {
            self.advance();
            if self.current() == Some('^') {
                self.advance();
                return Ok(Spanned { tok: Token::CaretCaret, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Caret, loc: start_loc });
        }
        if ch == '<' {
            self.advance();
            if self.current() == Some('<') {
                self.advance();
                return Ok(Spanned { tok: Token::Shl, loc: start_loc });
            }
            if self.current() == Some('=') {
                self.advance();
                return Ok(Spanned { tok: Token::Lte, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Lt, loc: start_loc });
        }
        if ch == '>' {
            self.advance();
            if self.current() == Some('>') {
                self.advance();
                return Ok(Spanned { tok: Token::Shr, loc: start_loc });
            }
            if self.current() == Some('=') {
                self.advance();
                return Ok(Spanned { tok: Token::Gte, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Gt, loc: start_loc });
        }
        if ch == '=' {
            self.advance();
            if self.current() == Some('=') {
                self.advance();
                return Ok(Spanned { tok: Token::EqualEqual, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Equals, loc: start_loc });
        }
        if ch == '!' {
            self.advance();
            if self.current() == Some('=') {
                self.advance();
                return Ok(Spanned { tok: Token::BangEqual, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Bang, loc: start_loc });
        }
        if ch == '+' {
            self.advance();
            if self.current() == Some('=') {
                self.advance();
                return Ok(Spanned { tok: Token::PlusEq, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Plus, loc: start_loc });
        }
        if ch == '-' {
            self.advance();
            if self.current() == Some('=') {
                self.advance();
                return Ok(Spanned { tok: Token::MinusEq, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Minus, loc: start_loc });
        }
        if ch == '*' {
            self.advance();
            if self.current() == Some('=') {
                self.advance();
                return Ok(Spanned { tok: Token::StarEq, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Star, loc: start_loc });
        }
        if ch == '/' {
            self.advance();
            if self.current() == Some('=') {
                self.advance();
                return Ok(Spanned { tok: Token::SlashEq, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Slash, loc: start_loc });
        }
        if ch == '%' {
            self.advance();
            if self.current() == Some('=') {
                self.advance();
                return Ok(Spanned { tok: Token::PercentEq, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Percent, loc: start_loc });
        }

        // String literals: "..." (double-quoted, no escape sequences)
        if ch == '"' {
            self.advance();
            return self.read_string_with_loc(start_loc);
        }

        // Single-character punctuation
        self.advance();
        let tok = match ch {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '?' => Token::Question,
            ':' => Token::Colon,
            _ => return Err(format!("unexpected character: '{}'", ch)),
        };
        Ok(Spanned { tok, loc: start_loc })
    }

    fn peek_is_digit(&self) -> bool {
        self.input.get(self.pos + 1).map_or(false, |c| c.is_ascii_digit())
    }

    fn read_number_with_loc(&mut self, start_loc: SourceLoc) -> Result<Spanned, String> {
        let start = self.pos;

        // Integer part (may be empty for .5 style floats)
        while let Some(ch) = self.current() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Decimal part: digits then dot (1.) or digits then dot then digits (1.5)
        // or just dot then digits (.5 — handled by peek_is_digit in next_token)
        if self.current() == Some('.') {
            self.advance();
            while let Some(ch) = self.current() {
                if ch.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Exponent part: [eE][+-]?digits
        if self.current() == Some('e') || self.current() == Some('E') {
            self.advance();
            // Optional sign
            if self.current() == Some('+') || self.current() == Some('-') {
                self.advance();
            }
            // At least one digit required after exponent
            let has_digit = self.current().map_or(false, |c| c.is_ascii_digit());
            if !has_digit {
                let num_str: String = self.input[start..self.pos].iter().collect();
                return Err(format!("invalid number: '{}' missing exponent digits", num_str));
            }
            while let Some(ch) = self.current() {
                if ch.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let num_str: String = self.input[start..self.pos].iter().collect();
        let tok = num_str.parse::<f64>()
            .map(Token::Number)
            .map_err(|e| format!("invalid number '{}': {}", num_str, e))?;
        Ok(Spanned { tok, loc: start_loc })
    }

    fn read_ident_or_keyword_with_loc(&mut self, start_loc: SourceLoc) -> Result<Spanned, String> {
        let start = self.pos;

        while let Some(ch) = self.current() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let ident: String = self.input[start..self.pos].iter().collect();

        // Check for keywords
        let tok = match ident.as_str() {
            "Param" => Token::Param,
            "require" => Token::Require,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "do" => Token::Do,
            "for" => Token::For,
            _ => Token::Ident(ident),
        };
        Ok(Spanned { tok, loc: start_loc })
    }
    fn read_string_with_loc(&mut self, start_loc: SourceLoc) -> Result<Spanned, String> {
        // Opening quote consumed by the caller (next_token); read until closing double-quote
        let mut value = String::new();
        loop {
            match self.current() {
                Some('"') => {
                    self.advance();
                    break;
                }
                Some(ch) => {
                    value.push(ch);
                    self.advance();
                }
                None => {
                    return Err(format!(
                        "unterminated string literal starting at line {}, col {}",
                        start_loc.line, start_loc.col
                    ));
                }
            }
        }
        Ok(Spanned { tok: Token::Str(value), loc: start_loc })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_numbers() {
        let mut lex = Lexer::new("42 3.14 0.5");
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(42.0));
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(3.14));
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(0.5));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);
    }

    #[test]
    fn tokenizes_identifiers_and_keywords() {
        let mut lex = Lexer::new("Param freq out1");
        assert_eq!(lex.next_token().unwrap().tok, Token::Param);
        assert_eq!(lex.next_token().unwrap().tok, Token::Ident("freq".to_string()));
        assert_eq!(lex.next_token().unwrap().tok, Token::Ident("out1".to_string()));
    }

    #[test]
    fn tokenizes_punctuation() {
        let mut lex = Lexer::new("( ) ; , = + - * /");
        assert_eq!(lex.next_token().unwrap().tok, Token::LParen);
        assert_eq!(lex.next_token().unwrap().tok, Token::RParen);
        assert_eq!(lex.next_token().unwrap().tok, Token::Semicolon);
        assert_eq!(lex.next_token().unwrap().tok, Token::Comma);
        assert_eq!(lex.next_token().unwrap().tok, Token::Equals);
        assert_eq!(lex.next_token().unwrap().tok, Token::Plus);
        assert_eq!(lex.next_token().unwrap().tok, Token::Minus);
        assert_eq!(lex.next_token().unwrap().tok, Token::Star);
        assert_eq!(lex.next_token().unwrap().tok, Token::Slash);
    }

    #[test]
    fn tokenizes_with_correct_locations() {
        // Single token on line 1 col 1
        let mut lex = Lexer::new("a");
        let t = lex.next_token().unwrap();
        assert_eq!(t.tok, Token::Ident("a".into()));
        assert_eq!(t.loc, SourceLoc { line: 1, col: 1 });

        // Second token after space on same line
        let mut lex = Lexer::new("a b");
        let t1 = lex.next_token().unwrap();
        assert_eq!(t1.tok, Token::Ident("a".into()));
        assert_eq!(t1.loc, SourceLoc { line: 1, col: 1 });
        let t2 = lex.next_token().unwrap();
        assert_eq!(t2.tok, Token::Ident("b".into()));
        assert_eq!(t2.loc, SourceLoc { line: 1, col: 3 });

        // After newline: col resets to 1, line increments
        let mut lex = Lexer::new("a\nb");
        let t1 = lex.next_token().unwrap();
        assert_eq!(t1.tok, Token::Ident("a".into()));
        assert_eq!(t1.loc, SourceLoc { line: 1, col: 1 });
        let t2 = lex.next_token().unwrap();
        assert_eq!(t2.tok, Token::Ident("b".into()));
        assert_eq!(t2.loc, SourceLoc { line: 2, col: 1 });

        // Tab counted as 1 column (no tab-stop expansion)
        let mut lex = Lexer::new("a\tb");
        let t1 = lex.next_token().unwrap();
        assert_eq!(t1.tok, Token::Ident("a".into()));
        assert_eq!(t1.loc, SourceLoc { line: 1, col: 1 });
        let t2 = lex.next_token().unwrap();
        assert_eq!(t2.tok, Token::Ident("b".into()));
        assert_eq!(t2.loc, SourceLoc { line: 1, col: 3 });

        // Multi-line input
        let mut lex = Lexer::new("a\nbc\ndef");
        let t1 = lex.next_token().unwrap();
        assert_eq!((t1.tok, t1.loc), (Token::Ident("a".into()), SourceLoc { line: 1, col: 1 }));
        let t2 = lex.next_token().unwrap();
        assert_eq!((t2.tok, t2.loc), (Token::Ident("bc".into()), SourceLoc { line: 2, col: 1 }));
        let t3 = lex.next_token().unwrap();
        assert_eq!((t3.tok, t3.loc), (Token::Ident("def".into()), SourceLoc { line: 3, col: 1 }));

        // CRLF: b at line 2 col 1
        let mut lex = Lexer::new("a\r\nb");
        let t1 = lex.next_token().unwrap();
        assert_eq!(t1.tok, Token::Ident("a".into()));
        assert_eq!(t1.loc, SourceLoc { line: 1, col: 1 });
        let t2 = lex.next_token().unwrap();
        assert_eq!(t2.tok, Token::Ident("b".into()));
        assert_eq!(t2.loc, SourceLoc { line: 2, col: 1 });
    }

    #[test]
    fn new_operator_tokens_lex_successfully() {
        // Logical and comparison operators
        let mut lex = Lexer::new("&& || ^^ ! ? :");
        assert_eq!(lex.next_token().unwrap().tok, Token::AndAnd);
        assert_eq!(lex.next_token().unwrap().tok, Token::OrOr);
        assert_eq!(lex.next_token().unwrap().tok, Token::CaretCaret);
        assert_eq!(lex.next_token().unwrap().tok, Token::Bang);
        assert_eq!(lex.next_token().unwrap().tok, Token::Question);
        assert_eq!(lex.next_token().unwrap().tok, Token::Colon);
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // Bitwise operators (single and compound)
        let mut lex = Lexer::new("& | ^ << >>");
        assert_eq!(lex.next_token().unwrap().tok, Token::Amp);
        assert_eq!(lex.next_token().unwrap().tok, Token::Pipe);
        assert_eq!(lex.next_token().unwrap().tok, Token::Caret);
        assert_eq!(lex.next_token().unwrap().tok, Token::Shl);
        assert_eq!(lex.next_token().unwrap().tok, Token::Shr);
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // Compound assignment operators
        let mut lex = Lexer::new("+= -= *= /= %=");
        assert_eq!(lex.next_token().unwrap().tok, Token::PlusEq);
        assert_eq!(lex.next_token().unwrap().tok, Token::MinusEq);
        assert_eq!(lex.next_token().unwrap().tok, Token::StarEq);
        assert_eq!(lex.next_token().unwrap().tok, Token::SlashEq);
        assert_eq!(lex.next_token().unwrap().tok, Token::PercentEq);
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // Dot and braces
        let mut lex = Lexer::new(". { }");
        assert_eq!(lex.next_token().unwrap().tok, Token::Dot);
        assert_eq!(lex.next_token().unwrap().tok, Token::LBrace);
        assert_eq!(lex.next_token().unwrap().tok, Token::RBrace);
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // Maximal munch: && before &, ^^ before ^, << before <, >> before >
        let mut lex = Lexer::new("&&& ^^^ <<< >>>");
        assert_eq!(lex.next_token().unwrap().tok, Token::AndAnd);
        assert_eq!(lex.next_token().unwrap().tok, Token::Amp);
        assert_eq!(lex.next_token().unwrap().tok, Token::CaretCaret);
        assert_eq!(lex.next_token().unwrap().tok, Token::Caret);
        assert_eq!(lex.next_token().unwrap().tok, Token::Shl);
        assert_eq!(lex.next_token().unwrap().tok, Token::Lt);
        assert_eq!(lex.next_token().unwrap().tok, Token::Shr);
        assert_eq!(lex.next_token().unwrap().tok, Token::Gt);
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);
    }

    #[test]
    fn numeric_forms_lex_correctly() {
        // Trailing-dot float: 1.
        let mut lex = Lexer::new("1.");
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(1.0));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // Leading-dot float: .5
        let mut lex = Lexer::new(".5");
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(0.5));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // Scientific notation
        let mut lex = Lexer::new("1e-3");
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(0.001));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        let mut lex = Lexer::new("2.5E+2");
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(250.0));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // Dot disambiguation: 1.foo = Number(1.0) then Ident("foo")
        // The `.` is consumed as part of the trailing-dot float `1.`
        // (use r#"..."# to avoid Rust 2021 float literal suffix warnings)
        let mut lex = Lexer::new(r#"1.foo"#);
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(1.0));
        assert_eq!(lex.next_token().unwrap().tok, Token::Ident("foo".into()));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // Dot disambiguation: .foo = Dot then Ident("foo")
        let mut lex = Lexer::new(r#".foo"#);
        assert_eq!(lex.next_token().unwrap().tok, Token::Dot);
        assert_eq!(lex.next_token().unwrap().tok, Token::Ident("foo".into()));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // .5.foo = Number(0.5) then Dot then Ident("foo")
        let mut lex = Lexer::new(r#".5.foo"#);
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(0.5));
        assert_eq!(lex.next_token().unwrap().tok, Token::Dot);
        assert_eq!(lex.next_token().unwrap().tok, Token::Ident("foo".into()));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // 1..5 = Number(1.0) then Number(0.5)
        let mut lex = Lexer::new("1..5");
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(1.0));
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(0.5));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);
    }

    #[test]
    fn comments_lex_correctly() {
        // Line comment
        let mut lex = Lexer::new("// line comment\n42");
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(42.0));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // Block comment
        let mut lex = Lexer::new("/* block comment */42");
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(42.0));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // Multi-line block comment
        let mut lex = Lexer::new("/* block\ncomment */42");
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(42.0));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);

        // Comments with surrounding whitespace
        let mut lex = Lexer::new("1 /* comment */ + 2");
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(1.0));
        assert_eq!(lex.next_token().unwrap().tok, Token::Plus);
        assert_eq!(lex.next_token().unwrap().tok, Token::Number(2.0));
        assert_eq!(lex.next_token().unwrap().tok, Token::Eof);
    }

    #[test]
    fn unterminated_block_comment_errors_in_lexer() {
        let mut lex = Lexer::new("/* unterminated");
        let err = lex.next_token().unwrap_err();
        assert!(
            err.contains("unterminated"),
            "error should mention unterminated: {}",
            err
        );
    }
}
