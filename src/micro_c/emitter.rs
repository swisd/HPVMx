use alloc::vec::Vec;
use crate::micro_c::parser::Node;

pub struct Emitter {
    pub code: Vec<u8>,
}

impl Emitter {
    pub fn new() -> Self { Self { code: Vec::new() } }

    fn emit(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
    }

    // Example: MOV RAX, Immediate64
    fn mov_rax_imm(&mut self, val: u64) {
        self.emit(&[0x48, 0xB8]); // REX.W + B8
        self.emit(&val.to_le_bytes());
    }

    // Example: RET
    fn ret(&mut self) {
        self.emit(&[0xC3]);
    }
}


impl Emitter {
    pub fn compile_node(&mut self, node: &Node) {
        match node {
            Node::Number(n) => self.mov_rax_imm(*n),
            Node::List(list) => {
                let cmd = match &list[0] {
                    Node::Symbol(s) => s.as_str(),
                    _ => return,
                };

                match cmd {
                    "poke" => {
                        // (poke addr value)
                        self.compile_node(&list[2]); // Calculate value -> RAX
                        self.emit(&[0x50]);          // PUSH RAX
                        self.compile_node(&list[1]); // Calculate addr -> RAX
                        self.emit(&[0x5B]);          // POP RBX (value)
                        self.emit(&[0x48, 0x89, 0x18]); // MOV [RAX], RBX
                    }
                    "+" => {
                        // ( + a b)
                        self.compile_node(&list[1]); // a -> RAX
                        self.emit(&[0x50]);          // PUSH RAX
                        self.compile_node(&list[2]); // b -> RAX
                        self.emit(&[0x5B]);          // POP RBX (a)
                        self.emit(&[0x48, 0x01, 0xD8]); // ADD RAX, RBX
                    }
                    "if" => self.compile_if(list),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn compile_if(&mut self, list: &Vec<Node>) {
        // (if cond true-block false-block)
        self.compile_node(&list[1]); // Condition result in RAX
        self.emit(&[0x48, 0x85, 0xC0]); // TEST RAX, RAX

        // Placeholder for JZ (Jump if Zero) to false-block
        let jz_index = self.code.len();
        self.emit(&[0x0F, 0x84, 0x00, 0x00, 0x00, 0x00]);

        self.compile_node(&list[2]); // True block

        // Placeholder for JMP to end
        let jmp_index = self.code.len();
        self.emit(&[0xE9, 0x00, 0x00, 0x00, 0x00]);

        // Fix up JZ offset
        let false_offset = (self.code.len() - (jz_index + 6)) as i32;
        self.code[jz_index + 2..jz_index + 6].copy_from_slice(&false_offset.to_le_bytes());

        if list.len() > 3 {
            self.compile_node(&list[3]); // False block
        }

        // Fix up JMP offset
        let end_offset = (self.code.len() - (jmp_index + 5)) as i32;
        self.code[jmp_index + 1..jmp_index + 5].copy_from_slice(&end_offset.to_le_bytes());
    }
}