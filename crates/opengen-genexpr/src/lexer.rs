//! Hand-written lexer for GenExpr

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
    // Punctuation
    LParen,
    RParen,
    Semicolon,
    Comma,
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    // Comparison operators
    Gt,         // >
    Gte,        // >=
    Lt,         // <
    Lte,        // <=
    EqualEqual, // ==
    BangEqual,  // !=
    // End of input
    Eof,
}

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

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Spanned, String> {
        self.skip_whitespace();
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

        // Multi-character operators
        if ch == '>' {
            self.advance();
            if self.current() == Some('=') {
                self.advance();
                return Ok(Spanned { tok: Token::Gte, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Gt, loc: start_loc });
        }
        if ch == '<' {
            self.advance();
            if self.current() == Some('=') {
                self.advance();
                return Ok(Spanned { tok: Token::Lte, loc: start_loc });
            }
            return Ok(Spanned { tok: Token::Lt, loc: start_loc });
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
            return Err("unexpected '!' (did you mean '!='?)".to_string());
        }

        // Single-character punctuation
        self.advance();
        let tok = match ch {
            '(' => Token::LParen,
            ')' => Token::RParen,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '%' => Token::Percent,
            _ => return Err(format!("unexpected character: '{}'", ch)),
        };
        Ok(Spanned { tok, loc: start_loc })
    }

    fn peek_is_digit(&self) -> bool {
        self.input.get(self.pos + 1).map_or(false, |c| c.is_ascii_digit())
    }

    fn read_number_with_loc(&mut self, start_loc: SourceLoc) -> Result<Spanned, String> {
        let start = self.pos;
        
        // Integer part
        while let Some(ch) = self.current() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Decimal part
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

        let num_str: String = self.input[start..self.pos].iter().collect();
        let tok = num_str.parse::<f64>()
            .map(Token::Number)
            .map_err(|e| format!("invalid number: {}", e))?;
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
            _ => Token::Ident(ident),
        };
        Ok(Spanned { tok, loc: start_loc })
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
}
