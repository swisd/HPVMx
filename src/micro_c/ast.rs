//! Abstract Syntax Tree (AST) definitions.
//!
//! This module defines the structure of the Micro-C language after parsing,
//! consisting of [`Type`]s, [`Expr`]essions, and [`Stmt`]atements.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

/// Represents a type in Micro-C.
#[derive(Debug, Clone)]
pub enum Type {
    I64,
    Bool,
    Ptr(Box<Type>),
    Struct(String),
}

/// Represents an expression that evaluates to a value.
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Variable(String),

    Binary(Box<Expr>, Op, Box<Expr>),

    Call(String, Vec<Expr>),

    Peek(Box<Expr>),
    Index(Box<Expr>, Box<Expr>),

    Field(Box<Expr>, String),
}

/// Represents a statement that performs an action.
#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        ty: Option<Type>,
        value: Expr,
    },

    Assign(String, Expr),

    AssignIndex {
        base: Expr,
        index: Expr,
        value: Expr,
    },

    AssignField {
        base: Expr,
        field: String,
        value: Expr,
    },

    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },

    Return(Expr),
    Expr(Expr),

    Poke(Expr, Expr),

    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        elif: Vec<(Expr, Vec<Stmt>)>,
        else_branch: Option<Vec<Stmt>>,
    },

    Loop(Vec<Stmt>),

    Break,
    Continue,

    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        export: bool,
    },
    ExternFunction {
        name: String,
        params: Vec<String>,
    },
    Import {
        name: String,
    },
    None
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add, Sub, Mul, Div,
    Eq, Neq,
    Lt, Gt, LtEq, GtEq,
}
