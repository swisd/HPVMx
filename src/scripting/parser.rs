use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::scripting::error::{ScriptError, ScriptErrorKind, ScriptResult};
use crate::scripting::lexer::{Token, TokenWithPos};
use crate::scripting::ast::{Expr, Stmt, Script, BinOp};

/// Recursive descent parser for the scripting language.
pub struct Parser {
    tokens: Vec<TokenWithPos>,
    pos: usize,
    file: String,
    source_lines: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithPos>, file: &str, source: &str) -> Self {
        let source_lines: Vec<String> = source.lines().map(|l| l.to_string()).collect();
        Self {
            tokens,
            pos: 0,
            file: file.to_string(),
            source_lines,
        }
    }

    fn current(&self) -> &TokenWithPos {
        &self.tokens[self.pos]
    }

    fn peek_token(&self) -> &Token {
        &self.tokens[self.pos].token
    }

    fn advance(&mut self) -> TokenWithPos {
        let tok = self.tokens[self.pos].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        tok
    }

    fn expect(&mut self, expected: &str) -> ScriptResult<TokenWithPos> {
        let tok = self.advance();
        match &tok.token {
            Token::KeywordIf if expected == "if" => Ok(tok),
            Token::KeywordElse if expected == "else" => Ok(tok),
            Token::KeywordElif if expected == "elif" => Ok(tok),
            Token::KeywordFor if expected == "for" => Ok(tok),
            Token::KeywordWhile if expected == "while" => Ok(tok),
            Token::KeywordEnd if expected == "end" => Ok(tok),
            Token::KeywordSet if expected == "set" => Ok(tok),
            Token::KeywordEcho if expected == "echo" => Ok(tok),
            Token::KeywordExit if expected == "exit" => Ok(tok),
            Token::KeywordBreak if expected == "break" => Ok(tok),
            Token::KeywordContinue if expected == "continue" => Ok(tok),
            Token::LParen if expected == "(" => Ok(tok),
            Token::RParen if expected == ")" => Ok(tok),
            Token::LBrace if expected == "{" => Ok(tok),
            Token::RBrace if expected == "}" => Ok(tok),
            Token::Semicolon if expected == ";" => Ok(tok),
            Token::Equal if expected == "=" => Ok(tok),
            Token::Identifier(_) if expected == "identifier" => Ok(tok),
            _ => {
                let src = self.source_line(tok.line);
                Err(ScriptError::new(
                    ScriptErrorKind::SyntaxError,
                    alloc::format!("Expected '{}' but got '{}'", expected, self.token_name(&tok.token)),
                    &self.file,
                    tok.line,
                    tok.column,
                ).with_source_line(src).with_hint(alloc::format!("Missing '{}' at this position", expected)))
            }
        }
    }

    fn check(&self, token: &Token) -> bool {
        self.peek_token() == token
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn source_line(&self, line: usize) -> String {
        if line > 0 && line <= self.source_lines.len() {
            self.source_lines[line - 1].clone()
        } else {
            String::new()
        }
    }

    fn token_name(&self, token: &Token) -> String {
        match token {
            Token::KeywordIf => "if".into(),
            Token::KeywordElse => "else".into(),
            Token::KeywordElif => "elif".into(),
            Token::KeywordFor => "for".into(),
            Token::KeywordWhile => "while".into(),
            Token::KeywordBreak => "break".into(),
            Token::KeywordContinue => "continue".into(),
            Token::KeywordSet => "set".into(),
            Token::KeywordEcho => "echo".into(),
            Token::KeywordExit => "exit".into(),
            Token::KeywordEnd => "end".into(),
            Token::KeywordReturn => "return".into(),
            Token::KeywordFn => "fn".into(),
            Token::Equal => "=".into(),
            Token::NotEqual => "!=".into(),
            Token::LessThan => "<".into(),
            Token::GreaterThan => ">".into(),
            Token::LessEqual => "<=".into(),
            Token::GreaterEqual => ">=".into(),
            Token::Plus => "+".into(),
            Token::Minus => "-".into(),
            Token::Increment => "++".into(),
            Token::Decrement => "--".into(),
            Token::Star => "*".into(),
            Token::Slash => "/".into(),
            Token::Percent => "%".into(),
            Token::And => "&&".into(),
            Token::Or => "||".into(),
            Token::LParen => "(".into(),
            Token::RParen => ")".into(),
            Token::LBrace => "{".into(),
            Token::RBrace => "}".into(),
            Token::Semicolon => ";".into(),
            Token::Newline => "newline".into(),
            Token::Eof => "EOF".into(),
            Token::Identifier(name) => alloc::format!("'{}'", name),
            Token::Number(n) => alloc::format!("{}", n),
            Token::StringLit(s) => alloc::format!("string literal"),
            Token::ArgRef(n) => alloc::format!("%%{}", n),
            _ => "unknown".into(),
        }
    }
    
    pub fn parse(mut self) -> ScriptResult<Script> {
        let mut statements = Vec::new();

        while !self.check(&Token::Eof) {
            if self.match_token(&Token::Newline) {
                continue;
            }
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        Ok(Script { statements })
    }

    fn parse_statement(&mut self) -> ScriptResult<Stmt> {
        let tok = self.current().clone();

        match &tok.token {
            Token::KeywordIf => self.parse_if(),
            Token::KeywordFor => self.parse_for(),
            Token::KeywordWhile => self.parse_while(),
            Token::KeywordSet => self.parse_set(),
            Token::KeywordEcho => self.parse_echo(),
            Token::KeywordExit => self.parse_exit(),
            Token::KeywordBreak => {
                self.advance();
                Ok(Stmt::Break)
            }
            Token::KeywordContinue => {
                self.advance();
                Ok(Stmt::Continue)
            }
            Token::LBrace => self.parse_block(),
            Token::Identifier(_) => self.parse_command(),
            Token::Newline => {
                self.advance();
                // If we hit EOF after a newline, just return - don't error
                if self.check(&Token::Eof) {
                    return self.parse_statement();
                }
                self.parse_statement()
            }
            Token::Eof => Err(ScriptError::new(
                ScriptErrorKind::SyntaxError,
                String::from("Unexpected end of file — expected a statement"),
                &self.file,
                tok.line,
                tok.column,
            ).with_hint(String::from("Check for unclosed blocks or missing 'end' keywords"))),
            _ => Err(ScriptError::new(
                ScriptErrorKind::SyntaxError,
                alloc::format!("Unexpected token '{}' at line {}", self.token_name(&tok.token), tok.line),
                &self.file,
                tok.line,
                tok.column,
            ).with_source_line(self.source_line(tok.line)).with_hint(
                String::from("Expected a statement: if, for, while, set, echo, exit, or a command")
            )),
        }
    }

    fn parse_if(&mut self) -> ScriptResult<Stmt> {
        self.expect("if")?;
        self.expect("(")?;
        let condition = self.parse_expression()?;
        self.expect(")")?;
        let then_body = self.parse_block()?;

        let then_stmts = match then_stmts_extract(then_body)? {
            stmts => stmts,
        };

        let mut elif_branches = Vec::new();
        let mut else_body = None;

        while self.check(&Token::KeywordElif) {
            self.advance(); // consume elif
            self.expect("(")?;
            let elif_cond = self.parse_expression()?;
            self.expect(")")?;
            let elif_block = self.parse_block()?;
            let elif_stmts = match_stmts(elif_block)?;
            elif_branches.push((elif_cond, elif_stmts));
        }

        if self.check(&Token::KeywordElse) {
            self.advance(); // consume else
            let else_block = self.parse_block()?;
            let else_stmts = match_stmts(else_block)?;
            else_body = Some(else_stmts);
        }

        Ok(Stmt::If {
            condition: Box::new(condition),
            then_body: then_stmts,
            elif_branches,
            else_body,
        })
    }

    fn parse_for(&mut self) -> ScriptResult<Stmt> {
        self.expect("for")?;
        self.expect("(")?;

        // Parse: set var = expr
        self.expect("set")?;
        let var_tok = self.expect("identifier")?;
        let init_var = match &var_tok.token {
            Token::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };
        self.expect("=")?;
        let init_value = self.parse_expression()?;

        self.expect(";")?;

        // Condition: var < expr
        let condition = self.parse_expression()?;

        self.expect(";")?;

        // Increment: var++
        let increment = self.parse_expression()?;

        self.expect(")")?;

        let body_block = self.parse_block()?;
        let body = match_stmts(body_block)?;

        Ok(Stmt::For {
            init_var,
            init_value: Box::new(init_value),
            condition: Box::new(condition),
            increment: Box::new(increment),
            body,
        })
    }

    fn parse_while(&mut self) -> ScriptResult<Stmt> {
        self.expect("while")?;
        self.expect("(")?;
        let condition = self.parse_expression()?;
        self.expect(")")?;
        let body_block = self.parse_block()?;
        let body = match_stmts(body_block)?;

        Ok(Stmt::While {
            condition: Box::new(condition),
            body,
        })
    }

    fn parse_set(&mut self) -> ScriptResult<Stmt> {
        self.expect("set")?;
        let var_tok = self.expect("identifier")?;
        let name = match &var_tok.token {
            Token::Identifier(name) => name.clone(),
            _ => {
                let src = self.source_line(var_tok.line);
                return Err(ScriptError::new(
                    ScriptErrorKind::VariableError,
                    alloc::format!("Expected variable name, got '{}'", self.token_name(&var_tok.token)),
                    &self.file,
                    var_tok.line,
                    var_tok.column,
                ).with_source_line(src).with_hint(String::from("Use: set <name> = <value>")));
            }
        };
        self.expect("=")?;
        let value = self.parse_expression()?;
        Ok(Stmt::Set {
            name,
            value: Box::new(value),
        })
    }

    fn parse_echo(&mut self) -> ScriptResult<Stmt> {
        self.expect("echo")?;
        let mut args = Vec::new();
        args.push(self.parse_expression()?);
        // Allow comma-separated echo args
        while self.check(&Token::Comma) || self.check(&Token::Identifier(String::new())) {
            if self.match_token(&Token::Comma) {
                args.push(self.parse_expression()?);
            } else {
                break;
            }
        }
        Ok(Stmt::Echo(args))
    }

    fn parse_exit(&mut self) -> ScriptResult<Stmt> {
        self.expect("exit")?;
        // Optional exit code
        let code = if !self.check(&Token::Newline) && !self.check(&Token::Eof) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        Ok(Stmt::Exit(code))
    }

    fn parse_block(&mut self) -> ScriptResult<Stmt> {
        self.expect("{")?;
        let mut stmts = Vec::new();
        while !self.check(&Token::RBrace) && !self.check(&Token::Eof) {
            if self.match_token(&Token::Newline) {
                continue;
            }
            let stmt = self.parse_statement()?;
            stmts.push(stmt);
        }
        self.expect("}")?;
        Ok(Stmt::Block(stmts))
    }

    fn parse_command(&mut self) -> ScriptResult<Stmt> {
        // A command is an identifier followed by optional arguments
        let mut args = Vec::new();
        let cmd_tok = self.advance();
        let cmd_name = match &cmd_tok.token {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(ScriptError::new(
                    ScriptErrorKind::SyntaxError,
                    alloc::format!("Expected command name, got '{}'", self.token_name(&cmd_tok.token)),
                    &self.file,
                    cmd_tok.line,
                    cmd_tok.column,
                ));
            }
        };
        args.push(Expr::StringLit(cmd_name));

        // Collect arguments until newline or EOF
        while !self.check(&Token::Newline) && !self.check(&Token::Eof) {
            args.push(self.parse_expression()?);
        }

        Ok(Stmt::Command(args))
    }

    // --- Expression parsing (recursive descent) ---

    fn parse_expression(&mut self) -> ScriptResult<Expr> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> ScriptResult<Expr> {
        let mut left = self.parse_and()?;
        while self.check(&Token::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinOp::Or,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> ScriptResult<Expr> {
        let mut left = self.parse_comparison()?;
        while self.check(&Token::And) {
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::BinaryOp {
                left: Box::new(left),
                op: BinOp::And,
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> ScriptResult<Expr> {
        let mut left = self.parse_addition()?;
        loop {
            match self.peek_token() {
                Token::Equal => {
                    self.advance();
                    let right = self.parse_addition()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: BinOp::Eq,
                        right: Box::new(right),
                    };
                }
                Token::NotEqual => {
                    self.advance();
                    let right = self.parse_addition()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: BinOp::NotEq,
                        right: Box::new(right),
                    };
                }
                Token::LessThan => {
                    self.advance();
                    let right = self.parse_addition()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: BinOp::Lt,
                        right: Box::new(right),
                    };
                }
                Token::GreaterThan => {
                    self.advance();
                    let right = self.parse_addition()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: BinOp::Gt,
                        right: Box::new(right),
                    };
                }
                Token::LessEqual => {
                    self.advance();
                    let right = self.parse_addition()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: BinOp::Le,
                        right: Box::new(right),
                    };
                }
                Token::GreaterEqual => {
                    self.advance();
                    let right = self.parse_addition()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: BinOp::Ge,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_addition(&mut self) -> ScriptResult<Expr> {
        let mut left = self.parse_multiplication()?;
        loop {
            match self.peek_token() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: BinOp::Add,
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: BinOp::Sub,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> ScriptResult<Expr> {
        let mut left = self.parse_unary()?;
        loop {
            match self.peek_token() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: BinOp::Mul,
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: BinOp::Div,
                        right: Box::new(right),
                    };
                }
                Token::Percent => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: BinOp::Mod,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> ScriptResult<Expr> {
        if self.check(&Token::Minus) {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::UnaryMinus(Box::new(expr)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> ScriptResult<Expr> {
        let tok = self.current().clone();

        match &tok.token {
            Token::Number(n) => {
                self.advance();
                Ok(Expr::Number(*n))
            }
            Token::StringLit(s) => {
                self.advance();
                Ok(Expr::StringLit(s.clone()))
            }
            Token::Identifier(name) => {
                self.advance();
                // Check for postfix ++ or --
                match self.peek_token() {
                    Token::Increment => {
                        self.advance();
                        Ok(Expr::Increment(name.clone()))
                    }
                    Token::Decrement => {
                        self.advance();
                        Ok(Expr::Decrement(name.clone()))
                    }
                    _ => Ok(Expr::Variable(name.clone()))
                }
            }
            Token::ArgRef(n) => {
                self.advance();
                Ok(Expr::ArgRef(*n))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(")")?;
                Ok(expr)
            }
            _ => {
                let src = self.source_line(tok.line);
                Err(ScriptError::new(
                    ScriptErrorKind::SyntaxError,
                    alloc::format!("Unexpected token '{}' in expression at line {}", self.token_name(&tok.token), tok.line),
                    &self.file,
                    tok.line,
                    tok.column,
                ).with_source_line(src).with_hint(
                    String::from("Expected a value: number, string, variable, %%N, or parenthesized expression")
                ))
            }
        }
    }
}

// Helper to extract statements from a Block
fn match_stmts(block: Stmt) -> ScriptResult<Vec<Stmt>> {
    match block {
        Stmt::Block(stmts) => Ok(stmts),
        _ => Err(ScriptError::new(
            ScriptErrorKind::SyntaxError,
            String::from("Expected a block '{ ... }'"),
            "", 0, 0,
        ).with_hint(String::from("Wrap statement bodies in curly braces"))),
    }
}

fn then_stmts_extract(block: Stmt) -> ScriptResult<Vec<Stmt>> {
    match_stmts(block)
}
