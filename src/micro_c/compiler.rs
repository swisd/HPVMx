//! Top-level compiler interface.
//!
//! This module provides the high-level [`compile`] function which orchestrates
//! the entire compilation pipeline from source code to assembly.

use alloc::string::{String, ToString};
use crate::arch::Architecture;
use crate::arch::win64::WIN64Backend;
use crate::arch::arm64::ARM64Backend;
use crate::arch::x86_64_raw::X86_64RawBackend;
use crate::arch::bytecode64::Bytecode64Backend;
use crate::codegen_ir::IRGenerator;
use crate::error::error;
use crate::lexer::Lexer;
use crate::parser::Parser;

/// Compiles the given Micro-C source code for the specified architecture.
///
/// Supported architectures: "win64", "arm64", "x86_64", "x86_64_hsis", "bytecode64".
///
/// Returns the generated assembly code as a String.
pub fn compile(source: &str, arch: &str) -> String {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let ast = parser.parse_program();

    let mut irgen = IRGenerator::new();
    irgen.gen_program(&ast);

    match arch {
        "win64" => {
            let mut backend = WIN64Backend::new(irgen.function_params);
            backend.emit_program(&irgen.code)
        }

        "arm64" => {
            let mut backend = ARM64Backend::new();
            backend.emit_program(&irgen.code)
        }

        "x86_64" => {
            let mut backend = X86_64RawBackend::new(irgen.function_params);
            backend.emit_program(&irgen.code)
        },
        "x86_64_hsis" => {
            let mut backend = X86_64RawBackend::new(irgen.function_params);
            backend.emit_program(&irgen.code)
        },
        "bytecode64" => {
            let mut backend = Bytecode64Backend::new(irgen.function_params);
            backend.emit_program(&irgen.code)
        },

        _ => {
            error("Unsupported architecture");
            "".to_string()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::compile;

    #[test]
    fn emits_extern_function_declaration_and_call() {
        let asm = compile(
            r#"
extern fn host_add(a, b);

export fn main() {
    return host_add(2, 3);
}
"#,
            "x86_64",
        );

        assert!(asm.contains("extern host_add"));
        assert!(asm.contains("call host_add"));
    }

    #[test]
    fn imports_builtin_sys_declarations() {
        let asm = compile(
            r#"
#include <Sys>

export fn main() {
    return malloc(8);
}
"#,
            "x86_64",
        );

        assert!(asm.contains("extern malloc"));
        assert!(asm.contains("extern free"));
        assert!(asm.contains("call malloc"));
    }

    #[test]
    fn compiles_memory_and_index_operations() {
        let asm = compile(
            r#"
export fn main() {
    let ptr = 0x1000;
    ptr[1] = 7;
    poke(ptr, 42);
    return peek(ptr) + ptr[1];
}
"#,
            "x86_64",
        );

        assert!(asm.contains("mov ["));
        assert!(asm.contains("mov "));
    }

    #[test]
    fn compiles_struct_field_operations() {
        let asm = compile(
            r#"
struct Point {
    x: i64;
    y: i64;
}

export fn main() {
    let p = alloc_struct(Point);
    p.x = 10;
    p.y = 20;
    return p.x + p.y;
}
"#,
            "x86_64",
        );

        assert!(asm.contains("sub rsp, 16"));
        assert!(asm.contains("mov ["));
    }

    #[test]
    fn compiles_loop_break_and_continue() {
        let asm = compile(
            r#"
export fn main() {
    loop {
        continue;
        break;
    }

    return 0;
}
"#,
            "x86_64",
        );

        assert!(asm.contains("jmp L"));
        assert!(asm.contains("L"));
    }

    #[test]
    fn compiles_bytecode64_basic_function() {
        let bc = compile(
            r#"
extern fn host_calc(x, y);

export fn add(a, b) {
    let sum = a + b;
    return sum;
}

export fn main() {
    let result = add(10, 20);
    return host_calc(result, 42);
}
"#,
            "bytecode64",
        );

        assert!(bc.contains("; ARCH bytecode64"));
        assert!(bc.contains(".extern host_calc"));
        assert!(bc.contains("add:"));
        assert!(bc.contains("add64"));
        assert!(bc.contains("ret64"));
        assert!(bc.contains("main:"));
        assert!(bc.contains("call64"));
    }

    #[test]
    fn compiles_bytecode64_structs_memory_and_loops() {
        let bc = compile(
            r#"
struct Vector {
    x: i64;
    y: i64;
}

export fn main() {
    let v = alloc_struct(Vector);
    v.x = 15;
    v.y = 25;
    let ptr = 0x2000;
    poke(ptr, v.x + v.y);

    loop {
        if (peek(ptr) == 40) {
            break;
        }
    }

    return peek(ptr);
}
"#,
            "bytecode64",
        );

        assert!(bc.contains("alloca64"));
        assert!(bc.contains("store64"));
        assert!(bc.contains("load64"));
        assert!(bc.contains("eq64"));
        assert!(bc.contains("jz"));
        assert!(bc.contains("jmp"));
    }
}
