
//! Parser for LogicLang - converts tokens to AST

use alloc::{format, vec};
use super::lexer::Token;
use super::error::LogicError;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

#[derive(Debug, Clone)]
pub enum Term {
    Atom(String),
    Variable(String),
    Integer(i64),
    String(String),
    Compound(String, Vec<Term>),
    List(Vec<Term>),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Clause {
    pub head: Term,
    pub body: Vec<Goal>,
}

#[derive(Debug, Clone)]
pub enum Goal {
    Predicate(Term),
    Comparison(CompOp, Term, Term),
    Cut,
    Call(String),  // For custom commands
}

#[derive(Debug, Clone)]
pub enum CompOp {
    Unify,           // =
    NotUnify,        // \=
    LessThan,        // <
    GreaterThan,     // >
    LessThanEq,      // =<
    GreaterThanEq,   // >=
    Is,              // is
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Clause>, LogicError> {
        let mut clauses = Vec::new();

        while !self.is_at_end() {
            clauses.push(self.parse_clause()?);
        }

        Ok(clauses)
    }

    fn parse_clause(&mut self) -> Result<Clause, LogicError> {
        let head = self.parse_term()?;

        let body = if self.match_token(&Token::ColonDash) {
            self.parse_goals()?
        } else {
            Vec::new()
        };

        self.expect_token(Token::Dot)?;

        Ok(Clause { head, body })
    }

    fn parse_goals(&mut self) -> Result<Vec<Goal>, LogicError> {
        let mut goals = Vec::new();

        loop {
            goals.push(self.parse_goal()?);

            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        Ok(goals)
    }

    fn parse_goal(&mut self) -> Result<Goal, LogicError> {
        if self.match_token(&Token::Cut) {
            return Ok(Goal::Cut);
        }

        let term = self.parse_term()?;

        // Check for comparison operators
        if self.current_token_matches(&Token::Equals) {
            self.position += 1;
            let right = self.parse_term()?;
            return Ok(Goal::Comparison(CompOp::Unify, term, right));
        }

        if self.current_token_matches(&Token::NotEquals) {
            self.position += 1;
            let right = self.parse_term()?;
            return Ok(Goal::Comparison(CompOp::NotUnify, term, right));
        }

        if self.current_token_matches(&Token::LessThan) {
            self.position += 1;
            let right = self.parse_term()?;
            return Ok(Goal::Comparison(CompOp::LessThan, term, right));
        }

        if self.current_token_matches(&Token::GreaterThan) {
            self.position += 1;
            let right = self.parse_term()?;
            return Ok(Goal::Comparison(CompOp::GreaterThan, term, right));
        }

        if self.current_token_matches(&Token::LessThanEq) {
            self.position += 1;
            let right = self.parse_term()?;
            return Ok(Goal::Comparison(CompOp::LessThanEq, term, right));
        }

        if self.current_token_matches(&Token::GreaterThanEq) {
            self.position += 1;
            let right = self.parse_term()?;
            return Ok(Goal::Comparison(CompOp::GreaterThanEq, term, right));
        }

        if self.current_token_matches(&Token::Is) {
            self.position += 1;
            let right = self.parse_term()?;
            return Ok(Goal::Comparison(CompOp::Is, term, right));
        }

        Ok(Goal::Predicate(term))
    }

    fn parse_term(&mut self) -> Result<Term, LogicError> {
        if let Token::Variable(name) = self.current_token() {
            let name_clone = name.clone();
            self.position += 1;
            return Ok(Term::Variable(name_clone));
        }

        if let Token::Integer(n) = self.current_token() {
            let n = *n;
            self.position += 1;
            return Ok(Term::Integer(n));
        }

        if let Token::String(s) = self.current_token() {
            let s = s.clone();
            self.position += 1;
            return Ok(Term::String(s));
        }

        if self.match_token(&Token::LeftBracket) {
            return self.parse_list();
        }

        if self.match_token(&Token::Underscore) {
            return Ok(Term::Variable("_".to_string()));
        }

        if let Token::Atom(name) = self.current_token() {
            let name = name.clone();
            self.position += 1;

            if self.match_token(&Token::LeftParen) {
                let args = self.parse_args()?;
                self.expect_token(Token::RightParen)?;
                Ok(Term::Compound(name, args))
            } else {
                Ok(Term::Atom(name))
            }
        } else {
            Err(LogicError::SyntaxError(
                "expected term".to_string(),
            ))
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Term>, LogicError> {
        let mut args = Vec::new();

        if self.current_token_matches(&Token::RightParen) {
            return Ok(args);
        }

        loop {
            args.push(self.parse_term()?);

            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        Ok(args)
    }

    fn parse_list(&mut self) -> Result<Term, LogicError> {
        let mut elements = Vec::new();

        if self.match_token(&Token::RightBracket) {
            return Ok(Term::Nil);
        }

        loop {
            elements.push(self.parse_term()?);

            if self.match_token(&Token::Pipe) {
                let tail = self.parse_term()?;
                self.expect_token(Token::RightBracket)?;

                // Build list with tail
                let mut result = tail;
                for elem in elements.into_iter().rev() {
                    result = Term::Compound(".".to_string(), vec![elem, result]);
                }
                return Ok(result);
            }

            if !self.match_token(&Token::Comma) {
                break;
            }
        }

        self.expect_token(Token::RightBracket)?;
        Ok(Term::List(elements))
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.current_token_matches(token) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn expect_token(&mut self, token: Token) -> Result<(), LogicError> {
        if self.current_token_matches(&token) {
            self.position += 1;
            Ok(())
        } else {
            Err(LogicError::SyntaxError(
                format!("expected {:?}, got {:?}", token, self.current_token()),
            ))
        }
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    fn current_token_matches(&self, token: &Token) -> bool {
        core::mem::discriminant(self.current_token()) == core::mem::discriminant(token)
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current_token(), Token::Eof)
    }
}