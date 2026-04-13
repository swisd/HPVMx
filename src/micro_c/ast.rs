use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub enum Type {
    I64,
    Bool,
    Ptr(Box<Type>),
    Struct(String),
}

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
    None
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add, Sub, Mul, Div,
    Eq, Neq,
    Lt, Gt, LtEq, GtEq,
}