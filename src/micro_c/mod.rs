//! MicroC Compiler.
//!
//! A lightweight C compiler targeting multiple architectures.
//! This module includes lexing, parsing, code generation,
//! and architecture-specific backends.

use crate::Color;
use crate::hpvm_log;
use alloc::string::String;
use crate::hpvm_error;
use crate::micro_c::compiler::compile;

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod interpreter;
pub mod ir;
pub mod codegen_ir;
pub mod backend;
pub mod regalloc;
pub mod emitter;
pub mod compiler;
pub mod arch;
pub mod stackframe;
pub mod error;
pub mod fs;

/// Compiles a MicroC source file to assembly.
pub fn compile_from_file_to_asm(srcpath: String) -> String {
    if let Ok(source) = crate::FileSystem::read_file_to_string(&*srcpath) {
        let asm = compile(&*source, "x86_64");
        asm
    } else {
        hpvm_error!("micro-c", "could not open file");
        " ".parse().unwrap()
    }
}

// --- Rust Bridge for Micro-C ---

#[repr(C)]
pub struct CVM {
    pub id: i64,
    pub state: i64,
    pub memory_mb: i64,
    pub vcpu_count: i64,
}

static mut SHADOW_VM: CVM = CVM {
    id: 0,
    state: 0,
    memory_mb: 0,
    vcpu_count: 0,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_draw_pixel(x: i64, y: i64, color: i64) {
    if let Some(mut pg) = crate::ui::pixel_graphics::PixelGraphics::new() {
        pg.draw_pixel(x as usize, y as usize, color as u32);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_fill_rect(x: i64, y: i64, w: i64, h: i64, color: i64) {
    if let Some(mut pg) = crate::ui::pixel_graphics::PixelGraphics::new() {
        pg.fill_rect(x as usize, y as usize, w as usize, h as usize, color as u32);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_draw_line(x1: i64, y1: i64, x2: i64, y2: i64, color: i64) {
    if let Some(mut pg) = crate::ui::pixel_graphics::PixelGraphics::new() {
        pg.draw_line(x1 as usize, y1 as usize, x2 as usize, y2 as usize, color as u32);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_draw_text(x: i64, y: i64, text_ptr: *const u8, color: i64) {
    if text_ptr.is_null() { return; }
    // Basic C-string to Rust string conversion (assuming UTF-8/ASCII)
    let mut len = 0;
    unsafe {
        while *text_ptr.add(len) != 0 {
            len += 1;
        }
        let slice = core::slice::from_raw_parts(text_ptr, len);
        if let Ok(s) = core::str::from_utf8(slice) {
            if let Some(mut pg) = crate::ui::pixel_graphics::PixelGraphics::new() {
                pg.draw_text(x as usize, y as usize, s, color as u32);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_clear(color: i64) {
    if let Some(mut pg) = crate::ui::pixel_graphics::PixelGraphics::new() {
        pg.clear(color as u32);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_get_resolution_x() -> i64 {
    if let Some(pg) = crate::ui::pixel_graphics::PixelGraphics::new() {
        pg.resolution().0 as i64
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ui_get_resolution_y() -> i64 {
    if let Some(pg) = crate::ui::pixel_graphics::PixelGraphics::new() {
        pg.resolution().1 as i64
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn os_get_vm(id: i64) -> *mut CVM {
    unsafe {
        if let Some(ref mut hv) = crate::HYPERVISOR {
            if let Some(vm) = hv.get_vm(id as u32) {
                SHADOW_VM.id = vm.id as i64;
                SHADOW_VM.state = match vm.state {
                    crate::vmm::vm::VmState::Created => 0,
                    crate::vmm::vm::VmState::Running => 1,
                    crate::vmm::vm::VmState::Paused => 2,
                    crate::vmm::vm::VmState::Stopped => 3,
                    crate::vmm::vm::VmState::Failed => 4,
                    crate::vmm::vm::VmState::Decommissioned => 5,
                };
                SHADOW_VM.memory_mb = vm.memory_mb as i64;
                SHADOW_VM.vcpu_count = vm.vcpu_count as i64;
                return &mut SHADOW_VM;
            }
        }
    }
    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn os_save_vm(cvm_ptr: *mut CVM) {
    if cvm_ptr.is_null() { return; }
    unsafe {
        let cvm = &*cvm_ptr;
        if let Some(ref mut hv) = crate::HYPERVISOR {
            if let Some(vm) = hv.get_vm_mut(cvm.id as u32) {
                vm.state = match cvm.state {
                    0 => crate::vmm::vm::VmState::Created,
                    1 => crate::vmm::vm::VmState::Running,
                    2 => crate::vmm::vm::VmState::Paused,
                    3 => crate::vmm::vm::VmState::Stopped,
                    4 => crate::vmm::vm::VmState::Failed,
                    _ => crate::vmm::vm::VmState::Decommissioned,
                };
                vm.memory_mb = cvm.memory_mb as u32;
                vm.vcpu_count = cvm.vcpu_count as u32;
            }
        }
    }
}