mod emitter;
mod parser;

use alloc::vec::Vec;
use parser::*;
use emitter::*;
use crate::filesystem::FileSystem;

pub fn load_package(path: &str) -> Vec<Node> {
    // 1. Read file from UEFI FAT32
    let content = FileSystem::read_file_to_string(path).expect("Failed to read package");

    // 2. Run the parser
    let mut parser = Parser::new(&content);
    let ast = parser.parse_all().expect("Syntax Error in Micro-C");

    // 3. Handle (require "name") - Simple dependency resolution
    let mut expanded_ast = Vec::new();
    for node in ast {
        if let Node::List(ref l) = node {
            if l.get(0) == Some(&Node::Symbol("require".into())) {
                if let Some(Node::Symbol(dep_path)) = l.get(1) {
                    expanded_ast.extend(load_package(dep_path));
                    continue;
                }
            }
        }
        expanded_ast.push(node);
    }
    expanded_ast
}