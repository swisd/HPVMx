//! Hardware abstraction layer with virtualization support

pub mod cpu;
pub mod pci;
pub mod sysinfo;
pub use cpu::{ap_count, core_count, CpuInfo};
pub use cpu::mp;
pub use sysinfo::SystemInformation;
//mod vmx;

#[cfg(target_arch = "x86_64")]
pub use cpu::vmx;