// src/main.rs
#![allow(unused_features)]
#![feature(abi_x86_interrupt)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points


use uefi::*;
extern crate alloc;

// No panic handler needed since we are importing uefi

static HELLO: &[u8] = b"Hello World!";

#[entry]
fn main() -> Status {
    let vga_buffer = 0xb8000 as *mut u8;

    #[allow(unsafe_code)]
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}