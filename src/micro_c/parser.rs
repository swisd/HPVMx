extern crate alloc;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::boxed::Box;
use crate::message;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Symbol(String),
    Number(u64),
    List(Vec<Node>),
    Bool(bool),
}


fn tokenize(input: &str) -> Vec<String> {
    input
        .replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub struct Parser {
    tokens: Vec<String>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self {
            tokens: tokenize(input),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Node, String> {
        let token = self.tokens.get(self.pos)
            .ok_or_else(|| "Unexpected end of input".to_string())?;

        match token.as_str() {
            "(" => {
                self.pos += 1; // Consume '('
                let mut list = Vec::new();
                while self.tokens.get(self.pos).map(|s| s.as_str()) != Some(")") {
                    list.push(self.parse()?);
                }
                self.pos += 1; // Consume ')'
                Ok(Node::List(list))
            }
            ")" => Err("Unexpected closing parenthesis".to_string()),
            "true" => { self.pos += 1; Ok(Node::Bool(true)) }
            "false" => { self.pos += 1; Ok(Node::Bool(false)) }
            _ => {
                self.pos += 1;
                // Try to parse as a number (hex or dec)
                if token.starts_with("0x") {
                    u64::from_str_radix(&token[2..], 16)
                        .map(Node::Number)
                        .or(Ok(Node::Symbol(token.clone())))
                } else if let Ok(num) = token.parse::<u64>() {
                    Ok(Node::Number(num))
                } else {
                    Ok(Node::Symbol(token.clone()))
                }
            }
        }
    }

    /// Helper to parse a whole file containing multiple top-level S-expressions
    pub fn parse_all(&mut self, print: bool) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        while self.pos < self.tokens.len() {
            let node = self.parse()?;
            // if print {
            //     message!("\n", "{:?}", node.clone())
            // }
            nodes.push(node);

        }
        if print {
            message!("\n", "{:?}", nodes.clone())
        }
        Ok(nodes)
    }
}