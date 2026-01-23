//! Lexer for LogicLang - tokenizes input

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Identifiers and literals
    Atom(String),
    Variable(String),
    Integer(i64),
    String(String),

    // Operators
    ColonDash,           // :-
    Dot,                 // .
    Comma,               // ,
    Pipe,                // |
    LeftParen,           // (
    RightParen,          // )
    LeftBracket,         // [
    RightBracket,        // ]
    LeftBrace,           // {
    RightBrace,          // }

    // Comparison operators
    Equals,              // =
    NotEquals,           // \=
    LessThan,            // <
    GreaterThan,         // >
    LessThanEq,          // =<
    GreaterThanEq,       // >=
    Is,                  // is

    // Arithmetic operators
    Plus,                // +
    Minus,               // -
    Star,                // *
    Slash,               // /
    Mod,                 // mod

    // Special
    Cut,                 // !
    Underscore,          // _
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            self.skip_whitespace_and_comments();

            if self.position >= self.input.len() {
                break;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, String> {
        let ch = self.current_char();

        match ch {
            '(' => {
                self.position += 1;
                Ok(Token::LeftParen)
            }
            ')' => {
                self.position += 1;
                Ok(Token::RightParen)
            }
            '[' => {
                self.position += 1;
                Ok(Token::LeftBracket)
            }
            ']' => {
                self.position += 1;
                Ok(Token::RightBracket)
            }
            '{' => {
                self.position += 1;
                Ok(Token::LeftBrace)
            }
            '}' => {
                self.position += 1;
                Ok(Token::RightBrace)
            }
            ',' => {
                self.position += 1;
                Ok(Token::Comma)
            }
            '|' => {
                self.position += 1;
                Ok(Token::Pipe)
            }
            '!' => {
                self.position += 1;
                Ok(Token::Cut)
            }
            '.' => {
                self.position += 1;
                Ok(Token::Dot)
            }
            '+' => {
                self.position += 1;
                Ok(Token::Plus)
            }
            '*' => {
                self.position += 1;
                Ok(Token::Star)
            }
            '/' => {
                self.position += 1;
                Ok(Token::Slash)
            }
            '=' => {
                self.position += 1;
                if self.current_char() == '<' {
                    self.position += 1;
                    Ok(Token::LessThanEq)
                } else if self.current_char() == ':' && self.peek_char() == Some('-') {
                    self.position += 2;
                    Ok(Token::ColonDash)
                } else {
                    Ok(Token::Equals)
                }
            }
            '<' => {
                self.position += 1;
                Ok(Token::LessThan)
            }
            '>' => {
                self.position += 1;
                if self.current_char() == '=' {
                    self.position += 1;
                    Ok(Token::GreaterThanEq)
                } else {
                    Ok(Token::GreaterThan)
                }
            }
            ':' => {
                self.position += 1;
                if self.current_char() == '-' {
                    self.position += 1;
                    Ok(Token::ColonDash)
                } else {
                    Err("unexpected character after ':'".to_string())
                }
            }
            '\\' => {
                self.position += 1;
                if self.current_char() == '=' {
                    self.position += 1;
                    Ok(Token::NotEquals)
                } else {
                    Err("unexpected character after '\\'".to_string())
                }
            }
            '-' => {
                self.position += 1;
                if self.current_char().is_ascii_digit() {
                    self.position -= 1;
                    self.read_number()
                } else {
                    Ok(Token::Minus)
                }
            }
            '_' if !self.peek_char().map_or(false, |c| c.is_alphanumeric()) => {
                self.position += 1;
                Ok(Token::Underscore)
            }
            '"' => self.read_string(),
            '\'' => self.read_quoted_atom(),
            _ if ch.is_ascii_uppercase() || ch == '_' => self.read_variable(),
            _ if ch.is_ascii_lowercase() => self.read_atom_or_keyword(),
            _ if ch.is_ascii_digit() => self.read_number(),
            _ => Err(format!("unexpected character: {}", ch)),
        }
    }

    fn read_atom_or_keyword(&mut self) -> Result<Token, String> {
        let atom = self.read_identifier();

        let token = match atom.as_str() {
            "is" => Token::Is,
            "mod" => Token::Mod,
            _ => Token::Atom(atom),
        };

        Ok(token)
    }

    fn read_variable(&mut self) -> Result<Token, String> {
        Ok(Token::Variable(self.read_identifier()))
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.position += 1; // Skip opening quote
        let mut string = String::new();

        while self.position < self.input.len() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                self.position += 1;
                match self.current_char() {
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    'r' => string.push('\r'),
                    '"' => string.push('"'),
                    '\\' => string.push('\\'),
                    _ => string.push(self.current_char()),
                }
            } else {
                string.push(self.current_char());
            }
            self.position += 1;
        }

        if self.position >= self.input.len() {
            return Err("unterminated string".to_string());
        }

        self.position += 1; // Skip closing quote
        Ok(Token::String(string))
    }

    fn read_quoted_atom(&mut self) -> Result<Token, String> {
        self.position += 1; // Skip opening quote
        let mut atom = String::new();

        while self.position < self.input.len() && self.current_char() != '\'' {
            atom.push(self.current_char());
            self.position += 1;
        }

        if self.position >= self.input.len() {
            return Err("unterminated atom".to_string());
        }

        self.position += 1; // Skip closing quote
        Ok(Token::Atom(atom))
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();

        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.position += 1;
            } else {
                break;
            }
        }

        ident
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut number = String::new();

        if self.current_char() == '-' {
            number.push('-');
            self.position += 1;
        }

        while self.position < self.input.len() && self.current_char().is_ascii_digit() {
            number.push(self.current_char());
            self.position += 1;
        }

        number
            .parse::<i64>()
            .map(Token::Integer)
            .map_err(|_| "invalid number".to_string())
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.position < self.input.len() {
            let ch = self.current_char();

            if ch.is_whitespace() {
                self.position += 1;
            } else if ch == '%' {
                // Line comment
                while self.position < self.input.len() && self.current_char() != '\n' {
                    self.position += 1;
                }
            } else if ch == '/' && self.peek_char() == Some('*') {
                // Block comment
                self.position += 2;
                while self.position + 1 < self.input.len() {
                    if self.current_char() == '*' && self.peek_char() == Some('/') {
                        self.position += 2;
                        break;
                    }
                    self.position += 1;
                }
            } else {
                break;
            }
        }
    }

    fn current_char(&self) -> char {
        self.input
            .get(self.position)
            .copied()
            .unwrap_or('\0')
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }
}