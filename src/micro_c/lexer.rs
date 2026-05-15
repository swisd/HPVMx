//! Lexical analyzer for the Micro-C language.
//!
//! This module contains the [`Lexer`] which converts a raw string of source code
//! into a stream of [`Token`]s.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use crate::error::error;

/// Represents a single unit of the Micro-C language.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let, Fn, Return,
    If, Elif, Else,
    Loop, Break, Continue,
    Export,
    Struct,

    Ident(String),
    Number(i64),

    LParen, RParen,
    LBrace, RBrace,
    LBracket, RBracket,

    Comma, Semicolon,
    Colon, Dot,

    Plus, Minus, Star, Slash,

    EqEq, NotEq,
    Lt, Gt, LtEq, GtEq,

    Assign,

    EOF,
    Arrow,
    Include(String),
    None,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Let => write!(f, "let"),
            Token::Fn => write!(f, "fn"),
            Token::Return => write!(f, "return"),
            Token::If => write!(f, "if"),
            Token::Elif => write!(f, "elif"),
            Token::Else => write!(f, "else"),
            Token::Loop => write!(f, "loop"),
            Token::Break => write!(f, "break"),
            Token::Continue => write!(f, "continue"),
            Token::Export => write!(f, "export"),
            Token::Struct => write!(f, "struct"),

            Token::Ident(name) => write!(f, "{}", name),
            Token::Number(n) => write!(f, "{}", n),

            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),

            Token::Assign => write!(f, "="),
            Token::EqEq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Lt => write!(f, "<"),
            Token::LtEq => write!(f, "<="),
            Token::Gt => write!(f, ">"),
            Token::GtEq => write!(f, ">="),

            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),

            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::Dot => write!(f, "."),

            Token::EOF => write!(f, "<EOF>"),
            Token::Include(name) => write!(f, "#include <{}>", name),

            //fallback for future tokens
            other => write!(f, "{:?}", other),
        }
    }
}

/// Lexer state for converting source code into tokens.
pub struct Lexer {
    pub input: Vec<char>,
    pos: usize,
}

impl Lexer {
    /// Creates a new Lexer from the given source string.
    pub fn new(input: &str) -> Self {
        Self { input: input.chars().collect(), pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek();
        self.pos += 1;
        c
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.next();
        }
    }

    fn ident(&mut self) -> Token {
        let mut s = String::new();
        while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c == '_') {
            s.push(self.next().unwrap());
        }

        match s.as_str() {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "loop" => Token::Loop,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "export" => Token::Export,
            "struct" => Token::Struct,
            _ => Token::Ident(s),
        }
    }

    fn number(&mut self) -> Token {
        let mut s = String::new();

        if self.peek() == Some('0') {
            s.push(self.next().unwrap());

            match self.peek() {
                Some('x') => {
                    self.next();
                    let mut hex = String::new();
                    while matches!(self.peek(), Some(c) if c.is_ascii_hexdigit()) {
                        hex.push(self.next().unwrap());
                    }
                    return Token::Number(i64::from_str_radix(&hex, 16).unwrap());
                }
                Some('b') => {
                    self.next();
                    let mut bin = String::new();
                    while matches!(self.peek(), Some(c) if c == '0' || c == '1') {
                        bin.push(self.next().unwrap());
                    }
                    return Token::Number(i64::from_str_radix(&bin, 2).unwrap());
                }
                _ => {}
            }
        }

        while matches!(self.peek(), Some(c) if c.is_numeric()) {
            s.push(self.next().unwrap());
        }

        Token::Number(s.parse().unwrap())
    }

    /// Scans and returns the next token from the input.
    pub fn next_token(&mut self) -> Token {
        self.skip_ws();

        match self.next() {
            Some('/') => match self.peek() {
                Some('/') => {
                    while let Some(c) = self.peek() {
                        if c == '\n' { break; }
                        self.next();
                    }
                    self.next_token()
                }
                Some('*') => {
                    self.next();
                    while let Some(c) = self.next() {
                        if c == '*' && self.peek() == Some('/') {
                            self.next();
                            break;
                        }
                    }
                    self.next_token()
                }
                _ => Token::Slash,
            },

            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some('[') => Token::LBracket,
            Some(']') => Token::RBracket,

            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some(':') => Token::Colon,
            Some('.') => Token::Dot,

            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Star,

            Some('=') => {
                if self.peek() == Some('=') {
                    self.next();
                    Token::EqEq
                } else {
                    Token::Assign
                }
            }

            Some('!') => {
                if self.peek() == Some('=') {
                    self.next();
                    Token::NotEq
                } else {
                    {error("Unexpected !"); Token::None}
                }
            }

            Some('<') => {
                if self.peek() == Some('=') {
                    self.next();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }

            Some('>') => {
                if self.peek() == Some('=') {
                    self.next();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }

            Some(c) if c.is_alphabetic() || c == '_' => {
                self.pos -= 1;
                self.ident()
            }

            Some(c) if c.is_numeric() => {
                self.pos -= 1;
                self.number()
            }

            Some('#') => {
                let mut directive = String::new();
                // Read the directive name (e.g., "include")
                while matches!(self.peek(), Some(c) if c.is_alphabetic()) {
                    directive.push(self.next().unwrap());
                }

                if directive == "include" {
                    self.skip_ws();
                    if self.next() == Some('<') {
                        let mut module_name = String::new();
                        // Lex until the closing '>'
                        while let Some(c) = self.peek() {
                            if c == '>' {
                                self.next(); // consume '>'
                                return Token::Include(module_name);
                            }
                            module_name.push(self.next().unwrap());
                        }
                        error("Unterminated include directive: expected '>'");
                        Token::None
                    } else {
                        error("Expected '<' after #include");
                        Token::None
                    }
                } else {
                    error(&format!("Unknown directive #{}", directive));
                    Token::None
                }
            }

            None => Token::EOF,

            Some(c) => {error(&format!("Unexpected char {}", c)); Token::None}
        }
    }
}