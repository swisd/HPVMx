//! Hardware abstraction layer with virtualization support

pub mod cpu;
//mod vmx;

#[cfg(target_arch = "x86_64")]
pub use cpu::vmx;