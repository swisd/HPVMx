use alloc::boxed::Box;
use alloc::{format, vec};
use alloc::string::ToString;
use alloc::vec::Vec;
use crate::ast::*;
use crate::error::error;
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current: Token,
    next: Token,
    position: u64,
    condition: u8,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let next = lexer.next_token();
        Self { lexer, current, next, position:0, condition:0, }
    }

    fn advance(&mut self) {
        self.current = core::mem::replace(&mut self.next, self.lexer.next_token());
        self.position += 1;
    }

    fn expect(&mut self, t: Token) {
        if self.current == t {
            self.advance();
        } else {
            error(&format!("(@{:#X}), Expected {:?}, got {:?}\n {} {}\n ^", self.position, t, self.current, self.current, self.next));
            self.advance();
            return;
        }
    }

    pub fn parse_program(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];

        while self.current != Token::EOF {
            stmts.push(self.parse_stmt());
        }

        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.current {
            Token::Let => self.parse_let(),
            Token::Struct => self.parse_struct(),
            Token::Fn | Token::Export => self.parse_fn(),
            Token::If => self.parse_if(),
            Token::Loop => self.parse_loop(),
            Token::Return => self.parse_return(),

            Token::Break => {
                self.advance();
                self.expect(Token::Semicolon);
                Stmt::Break
            }

            Token::Continue => {
                self.advance();
                self.expect(Token::Semicolon);
                Stmt::Continue
            }

            Token::Ident(_) if self.next == Token::Assign => self.parse_assign(),

            _ => {
                let expr = self.parse_expr();

                // p.x = ...
                if let Expr::Field(base, field) = expr.clone() {
                    if self.current == Token::Assign {
                        self.advance();
                        let value = self.parse_expr();
                        self.expect(Token::Semicolon);

                        return Stmt::AssignField {
                            base: *base,
                            field,
                            value,
                        };
                    }
                }

                // arr[i] = ...
                if let Expr::Index(base, index) = expr.clone() {
                    if self.current == Token::Assign {
                        self.advance();
                        let value = self.parse_expr();
                        self.expect(Token::Semicolon);

                        return Stmt::AssignIndex {
                            base: *base,
                            index: *index,
                            value,
                        };
                    }
                }

                self.expect(Token::Semicolon);

                if let Expr::Call(name, args) = expr.clone() {
                    if name == "poke" {
                        return Stmt::Poke(args[0].clone(), args[1].clone());
                    }
                }

                Stmt::Expr(expr)
            }
        }
    }

    fn parse_let(&mut self) -> Stmt {
        self.advance();

        let name = match self.current.clone() {
            Token::Ident(s) => s,
            _ => {error("Expected identifier");
                self.advance();
                return Stmt::None},
        };
        self.advance();

        let ty = if self.current == Token::Colon {
            self.advance();

            match self.current.clone() {
                Token::Ident(t) => {
                    self.advance();
                    Some(match t.as_str() {
                        "i64" => Type::I64,
                        "bool" => Type::Bool,
                        "ptr" => Type::Ptr(Box::new(Type::I64)),
                        _ => Type::Struct(t),
                    })
                }
                _ => {
                    error("Expected type");
                    self.advance();
                    return Stmt::None
                },
            }
        } else {
            None
        };

        self.expect(Token::Assign);
        let value = self.parse_expr();
        self.expect(Token::Semicolon);

        Stmt::Let { name, ty, value }
    }

    fn parse_assign(&mut self) -> Stmt {
        let name = match self.current.clone() {
            Token::Ident(s) => s,
            _ => {
                error("");
                return Stmt::None
            },
        };
        self.advance();

        self.expect(Token::Assign);
        let value = self.parse_expr();
        self.expect(Token::Semicolon);

        Stmt::Assign(name, value)
    }

    fn parse_struct(&mut self) -> Stmt {
        self.advance();

        let name = match self.current.clone() {
            Token::Ident(s) => s,
            _ => {
                error("Expected struct name");
                self.advance();
                return Stmt::None
            },
        };
        self.advance();

        self.expect(Token::LBrace);

        let mut fields = vec![];

        while self.current != Token::RBrace {
            let field = match self.current.clone() {
                Token::Ident(s) => s,
                _ => {error("Expected field name");
                    self.advance();
                    return Stmt::None},
            };
            self.advance();

            self.expect(Token::Colon);

            let ty = match self.current.clone() {
                Token::Ident(t) => {
                    self.advance();
                    match t.as_str() {
                        "i64" => Type::I64,
                        "bool" => Type::Bool,
                        _ => Type::Struct(t),
                    }
                }
                _ => {error("Expected type");
                    self.advance();
                    Type::I64},
            };

            self.expect(Token::Semicolon);

            fields.push((field, ty));
        }

        self.expect(Token::RBrace);

        Stmt::Struct { name, fields }
    }

    fn parse_fn(&mut self) -> Stmt {
        let export = if self.current == Token::Export {
            self.advance();
            true
        } else {
            false
        };

        self.expect(Token::Fn);

        let name = match self.current.clone() {
            Token::Ident(s) => s,
            _ => {
                error("Expected function name");
                self.advance();
                return Stmt::None
            },
        };
        self.advance();

        self.expect(Token::LParen);

        let mut params = vec![];

        while self.current != Token::RParen {
            if let Token::Ident(s) = self.current.clone() {
                params.push(s);
                self.advance();
            }

            if self.current == Token::Comma {
                self.advance();
            }
        }

        self.expect(Token::RParen);

        let body = self.parse_block();

        Stmt::Function { name, params, body, export }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.expect(Token::LBrace);

        let mut stmts = vec![];

        while self.current != Token::RBrace {
            stmts.push(self.parse_stmt());
        }

        self.expect(Token::RBrace);

        stmts
    }

    fn parse_if(&mut self) -> Stmt {
        self.advance();

        self.expect(Token::LParen);
        let cond = self.parse_expr();
        self.expect(Token::RParen);

        let then_branch = self.parse_block();

        let mut elif = vec![];

        while self.current == Token::Elif {
            self.advance();

            self.expect(Token::LParen);
            let c = self.parse_expr();
            self.expect(Token::RParen);

            let b = self.parse_block();
            elif.push((c, b));
        }

        let else_branch = if self.current == Token::Else {
            self.advance();
            Some(self.parse_block())
        } else {
            None
        };

        Stmt::If { cond, then_branch, elif, else_branch }
    }

    fn parse_loop(&mut self) -> Stmt {
        self.advance();
        Stmt::Loop(self.parse_block())
    }

    fn parse_return(&mut self) -> Stmt {
        self.advance();
        let e = self.parse_expr();
        self.expect(Token::Semicolon);
        Stmt::Return(e)
    }


    fn parse_expr(&mut self) -> Expr {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();

        while matches!(self.current, Token::EqEq | Token::NotEq) {
            let op = if self.current == Token::EqEq {
                Op::Eq
            } else {
                Op::Neq
            };

            self.advance();
            let right = self.parse_comparison();

            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();

        while matches!(self.current, Token::Lt | Token::Gt | Token::LtEq | Token::GtEq) {
            let op = match self.current {
                Token::Lt => Op::Lt,
                Token::Gt => Op::Gt,
                Token::LtEq => Op::LtEq,
                Token::GtEq => Op::GtEq,
                _ => unreachable!(),
            };

            self.advance();
            let right = self.parse_term();

            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while matches!(self.current, Token::Plus | Token::Minus) {
            let op = if self.current == Token::Plus {
                Op::Add
            } else {
                Op::Sub
            };

            self.advance();
            let right = self.parse_factor();

            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        while matches!(self.current, Token::Star | Token::Slash) {
            let op = if self.current == Token::Star {
                Op::Mul
            } else {
                Op::Div
            };

            self.advance();
            let right = self.parse_primary();

            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current.clone() {
            Token::Number(n) => {
                self.advance();
                Expr::Number(n)
            }

            Token::Ident(name) => {
                self.advance();

                let mut expr = Expr::Variable(name.clone());

                // function call
                if self.current == Token::LParen {
                    self.advance();

                    let mut args = vec![];

                    while self.current != Token::RParen {
                        args.push(self.parse_expr());

                        if self.current == Token::Comma {
                            self.advance();
                        }
                    }

                    self.expect(Token::RParen);

                    expr = if name == "peek" {
                        Expr::Peek(Box::new(args[0].clone()))
                    } else {
                        Expr::Call(name, args)
                    };
                }

                // indexing + field chaining
                loop {
                    match self.current {
                        Token::LBracket => {
                            self.advance();
                            let idx = self.parse_expr();
                            self.expect(Token::RBracket);

                            expr = Expr::Index(Box::new(expr), Box::new(idx));
                        }

                        Token::Dot => {
                            self.advance();

                            let field = match self.current.clone() {
                                Token::Ident(s) => s,
                                _ => {
                                    error(&format!("(@{:#X}) Expected field name\n {} {}\n ^", self.position, self.current, self.next));
                                    self.advance();
                                    return Expr::Number(0)},
                            };
                            self.advance();

                            expr = Expr::Field(Box::new(expr), field);
                        }

                        _ => break,
                    }
                }

                expr
            }

            Token::LParen => {
                self.advance();
                let expr = self.parse_expr();
                self.expect(Token::RParen);
                expr
            }

            _ => {
                error(&format!("(@{:#X}) Unexpected token: {}\n {} {}\n ^", self.position, self.current, self.current, self.next));
                self.advance();
                Expr::Number(0)},
        }
    }
}