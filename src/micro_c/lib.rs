extern crate alloc;

mod lexer;
mod parser;
mod ast;
mod interpreter;
mod ir;
mod codegen_ir;
mod backend;
mod regalloc;
mod emitter;
mod compiler;
mod arch;
mod stackframe;
mod error;
mod fs;

use alloc::fmt::format;
use alloc::format;
// use std::{env, fs};
// use std::fs::read_to_string;
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use crate::compiler::compile;
use crate::error::print;


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn compile_from_web(source_code: &str, target_arch: &str) -> String {
    // Bridges the string from JavaScript into your internal no_std compiler pipeline
    match crate::compiler::compile(source_code, target_arch) {
        assembly => assembly
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_errors() {
    crate::error::init_errors();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn get_errors() -> Vec<String> {
    crate::error::get_errors()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn clear_errors() {
    crate::error::clear_errors();
}