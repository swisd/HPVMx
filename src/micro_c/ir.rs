//! Intermediate Representation (IR) instructions.
//!
//! This module defines the flat, three-address-style instructions used
//! between the frontend and the backends.

use alloc::string::String;
use alloc::vec::Vec;

/// A single instruction in the Intermediate Representation.
#[derive(Debug, Clone)]
pub enum IRInst {
    Extern(String),
    /// Loads a constant integer into a temporary.
    LoadConst(String, i64),
    /// Loads a variable's value into a temporary.
    LoadVar(String, String),
    /// Stores a temporary's value into a variable.
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
