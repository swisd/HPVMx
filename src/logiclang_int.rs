//! LogicLang Interpreter - A logic programming language for HPVMx
//!
//! This is a Rust port of the Python LogicLang interpreter
//! It supports custom command execution for the HPVMx OS

pub mod lexer;
pub mod parser;
pub mod interpreter;
pub mod builtins;
pub mod error;

pub use error::LogicError;
pub use lexer::Lexer;
pub use parser::Parser;

