use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::scripting::error::{ScriptError, ScriptErrorKind, ScriptResult};

/// Token types recognized by the scripting engine lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    KeywordIf,
    KeywordElse,
    KeywordElif,
    KeywordFor,
    KeywordWhile,
    KeywordBreak,
    KeywordContinue,
    KeywordSet,
    KeywordEcho,
    KeywordExit,
    KeywordReturn,
    KeywordFn,
    KeywordEnd,

    // Operators
    Equal,           // =
    NotEqual,        // !=
    LessThan,        // <
    GreaterThan,     // >
    LessEqual,       // <=
    GreaterEqual,    // >=
    Plus,            // +
    Minus,           // -
    Increment,       // ++
    Decrement,       // --
    Star,            // *
    Slash,           // /
    Percent,         // %
    And,             // &&
    Or,              // ||
    Comma,           // ,
    Semicolon,       // ;
    Colon,           // :
    At,              // @

    // Delimiters
    LParen,          // (
    RParen,          // )
    LBrace,          // {
    RBrace,          // }
    LBracket,        // [
    RBracket,        // ]
    Newline,         // \n
    Eof,

    // Literals
    Identifier(String),
    Number(i64),
    StringLit(String),
    
    // Special: %%N argument references
    ArgRef(usize),
}

/// A single token with its position in the source.
#[derive(Debug, Clone)]
pub struct TokenWithPos {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

/// Lexer state for tokenizing script source.
pub struct Lexer {
    source: String,
    chars: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
    file: String,
}

impl Lexer {
    pub fn new(source: &str, file: &str) -> Self {
        Self {
            source: source.to_string(),
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
            file: file.to_string(),
        }
    }

    pub fn file(&self) -> &str {
        &self.file
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied();
        if let Some(c) = ch {
            self.pos += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        // Skip # comments to EOL
        if self.peek() == Some('#') {
            while let Some(ch) = self.advance() {
                if ch == '\n' {
                    break;
                }
            }
        }
    }
    
    pub fn tokenize(mut self) -> ScriptResult<Vec<TokenWithPos>> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();
            self.skip_comment();

            let (line, column) = (self.line, self.column);

            match self.peek() {
                None => {
                    tokens.push(TokenWithPos {
                        token: Token::Eof,
                        line,
                        column,
                    });
                    break;
                }
                Some('\n') => {
                    self.advance();
                    tokens.push(TokenWithPos {
                        token: Token::Newline,
                        line,
                        column,
                    });
                }
                Some('%') => {
                    // Check for %%N argument reference
                    self.advance(); // consume first %
                    if self.peek() == Some('%') {
                        self.advance(); // consume second %
                        // Read the number
                        let start_col = self.column;
                        let mut num_str = String::new();
                        while let Some(ch) = self.peek() {
                            if ch.is_ascii_digit() {
                                num_str.push(ch);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        if num_str.is_empty() {
                            return Err(ScriptError::new(
                                ScriptErrorKind::ArgumentError,
                                alloc::format!("Expected argument index after '%%', got nothing"),
                                &self.file,
                                line,
                                start_col,
                            ).with_hint(String::from("Use %%0 for script name, %%1-%%9 for positional arguments")));
                        }
                        let arg_idx: usize = num_str.parse().map_err(|_| {
                            ScriptError::new(
                                ScriptErrorKind::ArgumentError,
                                alloc::format!("Invalid argument index '{}'", num_str),
                                &self.file,
                                line,
                                start_col,
                            ).with_hint(String::from("Argument indices must be non-negative integers"))
                        })?;
                        let stored_idx = if arg_idx == 0 { 0 } else { arg_idx - 1 };
                        tokens.push(TokenWithPos {
                            token: Token::ArgRef(stored_idx),
                            line,
                            column: start_col,
                        });
                    } else {
                        tokens.push(TokenWithPos { token: Token::Percent, line, column });
                    }
                }
                Some('"') => {
                    self.advance(); // consume opening quote
                    let s = self.read_string()?;
                    tokens.push(TokenWithPos {
                        token: Token::StringLit(s),
                        line,
                        column,
                    });
                }
                Some('A'..='Z') | Some('a'..='z') | Some('_') => {
                    let ident = self.read_identifier();
                    let token = match ident.as_str() {
                        "if" => Token::KeywordIf,
                        "else" => Token::KeywordElse,
                        "elif" => Token::KeywordElif,
                        "for" => Token::KeywordFor,
                        "while" => Token::KeywordWhile,
                        "break" => Token::KeywordBreak,
                        "continue" => Token::KeywordContinue,
                        "set" => Token::KeywordSet,
                        "echo" => Token::KeywordEcho,
                        "exit" => Token::KeywordExit,
                        "return" => Token::KeywordReturn,
                        "fn" => Token::KeywordFn,
                        "end" => Token::KeywordEnd,
                        _ => Token::Identifier(ident),
                    };
                    tokens.push(TokenWithPos {
                        token,
                        line,
                        column,
                    });
                }
                Some('0'..='9') => {
                    let num = self.read_number()?;
                    tokens.push(TokenWithPos {
                        token: Token::Number(num),
                        line,
                        column,
                    });
                }
                Some('=') => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(TokenWithPos { token: Token::Equal, line, column }); // We use Equal for == in conditions
                    } else {
                        tokens.push(TokenWithPos { token: Token::Equal, line, column });
                    }
                }
                Some('!') => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(TokenWithPos { token: Token::NotEqual, line, column });
                    } else {
                        return Err(ScriptError::new(
                            ScriptErrorKind::SyntaxError,
                            alloc::format!("Unexpected '!' at column {}", column),
                            &self.file,
                            line,
                            column,
                        ).with_hint(String::from("Use '!=' for not-equal comparison")));
                    }
                }
                Some('<') => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(TokenWithPos { token: Token::LessEqual, line, column });
                    } else {
                        tokens.push(TokenWithPos { token: Token::LessThan, line, column });
                    }
                }
                Some('>') => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(TokenWithPos { token: Token::GreaterEqual, line, column });
                    } else {
                        tokens.push(TokenWithPos { token: Token::GreaterThan, line, column });
                    }
                }
                Some('+') => {
                    self.advance();
                    if self.peek() == Some('+') {
                        self.advance();
                        tokens.push(TokenWithPos { token: Token::Increment, line, column });
                    } else {
                        tokens.push(TokenWithPos { token: Token::Plus, line, column });
                    }
                }
                Some('-') => {
                    self.advance();
                    if self.peek() == Some('-') {
                        self.advance();
                        tokens.push(TokenWithPos { token: Token::Decrement, line, column });
                    } else {
                        tokens.push(TokenWithPos { token: Token::Minus, line, column });
                    }
                }
                Some('*') => { self.advance(); tokens.push(TokenWithPos { token: Token::Star, line, column }); }
                Some('/') => { self.advance(); tokens.push(TokenWithPos { token: Token::Slash, line, column }); }
                Some('&') => {
                    self.advance();
                    if self.peek() == Some('&') {
                        self.advance();
                        tokens.push(TokenWithPos { token: Token::And, line, column });
                    } else {
                        return Err(ScriptError::new(
                            ScriptErrorKind::SyntaxError,
                            String::from("Expected '&&' for logical AND"),
                            &self.file, line, column,
                        ).with_hint(String::from("Use '&&' instead of single '&'")));
                    }
                }
                Some('|') => {
                    self.advance();
                    if self.peek() == Some('|') {
                        self.advance();
                        tokens.push(TokenWithPos { token: Token::Or, line, column });
                    } else {
                        return Err(ScriptError::new(
                            ScriptErrorKind::SyntaxError,
                            String::from("Expected '||' for logical OR"),
                            &self.file, line, column,
                        ).with_hint(String::from("Use '||' instead of single '|'")));
                    }
                }
                Some(',') => { self.advance(); tokens.push(TokenWithPos { token: Token::Comma, line, column }); }
                Some(';') => { self.advance(); tokens.push(TokenWithPos { token: Token::Semicolon, line, column }); }
                Some(':') => { self.advance(); tokens.push(TokenWithPos { token: Token::Colon, line, column }); }
                Some('@') => { self.advance(); tokens.push(TokenWithPos { token: Token::At, line, column }); }
                Some('(') => { self.advance(); tokens.push(TokenWithPos { token: Token::LParen, line, column }); }
                Some(')') => { self.advance(); tokens.push(TokenWithPos { token: Token::RParen, line, column }); }
                Some('{') => { self.advance(); tokens.push(TokenWithPos { token: Token::LBrace, line, column }); }
                Some('}') => { self.advance(); tokens.push(TokenWithPos { token: Token::RBrace, line, column }); }
                Some('[') => { self.advance(); tokens.push(TokenWithPos { token: Token::LBracket, line, column }); }
                Some(']') => { self.advance(); tokens.push(TokenWithPos { token: Token::RBracket, line, column }); }
                Some('#') => {
                    while let Some(ch) = self.advance() {
                        if ch == '\n' {
                            break;
                        }
                    }
                }
                Some(ch) => {
                    return Err(ScriptError::new(
                        ScriptErrorKind::SyntaxError,
                        alloc::format!("Unexpected character '{}' at line {}, column {}", ch, line, column),
                        &self.file,
                        line,
                        column,
                    ).with_hint(String::from("Check for typos or unsupported characters")));
                }
            }
        }

        Ok(tokens)
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self) -> ScriptResult<i64> {
        let start_col = self.column;
        let mut num_str = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        num_str.parse::<i64>().map_err(|_| {
            ScriptError::new(
                ScriptErrorKind::ParseError,
                alloc::format!("Number '{}' too large or malformed", num_str),
                &self.file,
                self.line,
                start_col,
            ).with_hint(String::from("Use smaller integer values"))
        })
    }

    fn read_string(&mut self) -> ScriptResult<String> {
        let start_col = self.column;
        let mut s = String::new();
        loop {
            match self.advance() {
                Some('"') => break,
                Some('\\') => {
                    match self.advance() {
                        Some('n') => s.push('\n'),
                        Some('t') => s.push('\t'),
                        Some('\\') => s.push('\\'),
                        Some('"') => s.push('"'),
                        Some('%') => s.push('%'),
                        Some(ch) => {
                            return Err(ScriptError::new(
                                ScriptErrorKind::ParseError,
                                alloc::format!("Unknown escape sequence '\\{}'", ch),
                                &self.file,
                                self.line,
                                start_col,
                            ).with_hint(String::from("Supported escapes: \\n \\t \\\\ \\\" \\%")));
                        }
                        None => {
                            return Err(ScriptError::new(
                                ScriptErrorKind::ParseError,
                                String::from("Unterminated string — escape at end of file"),
                                &self.file,
                                self.line,
                                start_col,
                            ));
                        }
                    }
                }
                Some(ch) => s.push(ch),
                None => {
                    return Err(ScriptError::new(
                        ScriptErrorKind::ParseError,
                        alloc::format!("Unterminated string starting at column {}", start_col),
                        &self.file,
                        self.line,
                        start_col,
                    ).with_hint(String::from("Close the string with a matching double quote")));
                }
            }
        }
        Ok(s)
    }
}
