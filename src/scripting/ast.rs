use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    StringLit(String),
    Variable(String),
    ArgRef(usize),
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    UnaryMinus(Box<Expr>),
    Increment(String),
    Decrement(String),
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Eq,       // ==
    NotEq,    // !=
    Lt,       // <
    Gt,       // >
    Le,       // <=
    Ge,       // >=
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Mod,      // %
    And,      // &&
    Or,       // ||
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Set {
        name: String,
        value: Box<Expr>,
    },
    Echo(Vec<Expr>),
    If {
        condition: Box<Expr>,
        then_body: Vec<Stmt>,
        elif_branches: Vec<(Expr, Vec<Stmt>)>,
        else_body: Option<Vec<Stmt>>,
    },
    For {
        init_var: String,
        init_value: Box<Expr>,
        condition: Box<Expr>,
        increment: Box<Expr>,
        body: Vec<Stmt>,
    },
    While {
        condition: Box<Expr>,
        body: Vec<Stmt>,
    },
    Break,
    Continue,
    Exit(Option<Expr>),
    Command(Vec<Expr>),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub struct Script {
    pub statements: Vec<Stmt>,
}
