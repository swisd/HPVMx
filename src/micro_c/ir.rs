use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub enum IRInst {
    LoadConst(String, i64),
    LoadVar(String, String),
    StoreVar(String, String),

    Add(String, String, String),
    Sub(String, String, String),
    Mul(String, String, String),
    Div(String, String, String),

    Eq(String, String, String),
    Neq(String, String, String),
    Lt(String, String, String),
    Gt(String, String, String),
    LtEq(String, String, String),
    GtEq(String, String, String),

    Label(String),
    Jump(String),
    JumpIfZero(String, String),

    Call(String, String, Vec<String>),
    Return(String),
}