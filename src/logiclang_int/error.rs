//! Error types for LogicLang

use alloc::format;
use alloc::string::{String, ToString};

#[derive(Debug, Clone)]
pub enum LogicError {
    SyntaxError(String),
    RuntimeError(String),
    UndefinedVariable(String),
    TypeError(String),
    UnificationFailed(String),
    InvalidRule(String),
    StackOverflow,
    NoSolution,
    ExecutionError(String),
    Error(String),
}

impl LogicError {
    pub fn message(&self) -> String {
        match self {
            LogicError::SyntaxError(msg) => format!("SyntaxError: {}", msg),
            LogicError::RuntimeError(msg) => format!("RuntimeError: {}", msg),
            LogicError::UndefinedVariable(var) => format!("UndefinedVariable: {}", var),
            LogicError::TypeError(msg) => format!("TypeError: {}", msg),
            LogicError::UnificationFailed(msg) => format!("UnificationFailed: {}", msg),
            LogicError::InvalidRule(msg) => format!("InvalidRule: {}", msg),
            LogicError::StackOverflow => "StackOverflow".to_string(),
            LogicError::NoSolution => "NoSolution".to_string(),
            LogicError::ExecutionError(msg) => format!("ExecutionError: {}", msg),
            LogicError::Error(msg) => format!("Error: {}", msg),
        }
    }

    pub fn string_to_logicerror(error: String) -> LogicError {
        LogicError::Error(error)
    }
}