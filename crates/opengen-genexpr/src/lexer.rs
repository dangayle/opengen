//! Hand-written lexer for GenExpr

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
    // End of input
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) {
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

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        let ch = match self.current() {
            Some(c) => c,
            None => return Ok(Token::Eof),
        };

        // Numbers
        if ch.is_ascii_digit() || (ch == '.' && self.peek_is_digit()) {
            return self.read_number();
        }

        // Identifiers and keywords
        if ch.is_alphabetic() || ch == '_' {
            return self.read_ident_or_keyword();
        }

        // Punctuation
        self.advance();
        match ch {
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            ';' => Ok(Token::Semicolon),
            ',' => Ok(Token::Comma),
            '=' => Ok(Token::Equals),
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Star),
            '/' => Ok(Token::Slash),
            _ => Err(format!("unexpected character: '{}'", ch)),
        }
    }

    fn peek_is_digit(&self) -> bool {
        self.input.get(self.pos + 1).map_or(false, |c| c.is_ascii_digit())
    }

    fn read_number(&mut self) -> Result<Token, String> {
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
        num_str.parse::<f64>()
            .map(Token::Number)
            .map_err(|e| format!("invalid number: {}", e))
    }

    fn read_ident_or_keyword(&mut self) -> Result<Token, String> {
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
        match ident.as_str() {
            "Param" => Ok(Token::Param),
            _ => Ok(Token::Ident(ident)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_numbers() {
        let mut lex = Lexer::new("42 3.14 0.5");
        assert_eq!(lex.next_token().unwrap(), Token::Number(42.0));
        assert_eq!(lex.next_token().unwrap(), Token::Number(3.14));
        assert_eq!(lex.next_token().unwrap(), Token::Number(0.5));
        assert_eq!(lex.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn tokenizes_identifiers_and_keywords() {
        let mut lex = Lexer::new("Param freq out1");
        assert_eq!(lex.next_token().unwrap(), Token::Param);
        assert_eq!(lex.next_token().unwrap(), Token::Ident("freq".to_string()));
        assert_eq!(lex.next_token().unwrap(), Token::Ident("out1".to_string()));
    }

    #[test]
    fn tokenizes_punctuation() {
        let mut lex = Lexer::new("( ) ; , = + - * /");
        assert_eq!(lex.next_token().unwrap(), Token::LParen);
        assert_eq!(lex.next_token().unwrap(), Token::RParen);
        assert_eq!(lex.next_token().unwrap(), Token::Semicolon);
        assert_eq!(lex.next_token().unwrap(), Token::Comma);
        assert_eq!(lex.next_token().unwrap(), Token::Equals);
        assert_eq!(lex.next_token().unwrap(), Token::Plus);
        assert_eq!(lex.next_token().unwrap(), Token::Minus);
        assert_eq!(lex.next_token().unwrap(), Token::Star);
        assert_eq!(lex.next_token().unwrap(), Token::Slash);
    }
}
