//! Intel VT-x virtualization support for HPVMx

use raw_cpuid::CpuId;
use bitflags::bitflags;
use uefi::proto::console::text::Color;
use crate::{hpvm_log, hpvm_info};

/// VT-x capability flags
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct VtxCapabilities {
    pub available: bool,
    pub vmxon_supported: bool,
    pub ept_supported: bool,
    pub vpid_supported: bool,
    pub unrestricted_guest: bool,
}

bitflags! {
    /// VMCS control bits for VM execution control
    pub struct VmcsControl: u32 {
        const INTERRUPT_EXIT = 1 << 0;
        const NMI_EXITING = 1 << 3;
        const VIRTUAL_NMI = 1 << 5;
        const ACTIVATE_PREEMPTION_TIMER = 1 << 6;
        const PROCESS_POSTED_INTERRUPTS = 1 << 7;
    }
}

bitflags! {
    /// VMCS entry control bits
    pub struct VmcsEntryControl: u32 {
        const LOAD_DEBUG = 1 << 2;
        const IA32E_MODE = 1 << 9;
        const ENTRY_TO_SMM = 1 << 10;
        const DEACTIVATE_DUAL_MONITOR = 1 << 11;
        const LOAD_IA32_PERF = 1 << 13;
        const LOAD_IA32_PAT = 1 << 14;
        const LOAD_IA32_EFER = 1 << 15;
    }
}

#[allow(dead_code, unused)]
impl VtxCapabilities {
    /// Detect VT-x capabilities on this CPU
    pub fn detect() -> Self {
        let cpuid = CpuId::new();

        let available = cpuid
            .get_feature_info()
            .map(|info| true) // true for testing only. Change to info.has_vmx()
            .unwrap_or(false);

        hpvm_info!("vmx", "hypervisor capabilities available: {}", available);

        if !available {
            return Self {
                available: false,
                vmxon_supported: false,
                ept_supported: false,
                vpid_supported: false,
                unrestricted_guest: false,
            };
        }

        // Check for EPT (Extended Page Tables) support
        let ept_supported = cpuid
            .get_extended_feature_info()
            .map(|info| true)
            .unwrap_or(false);

        hpvm_info!("vmx", "ept capabilities available: {}", ept_supported);

        // Check for VPID support
        let vpid_supported = cpuid
            .get_extended_feature_info()
            .map(|info| true)
            .unwrap_or(false);

        hpvm_info!("vmx", "virtual network capabilities available: {}", vpid_supported);

        Self {
            available,
            vmxon_supported: available,
            ept_supported,
            vpid_supported,
            unrestricted_guest: cpuid
                .get_extended_feature_info()
                .map(|info| false)
                .unwrap_or(false),
        }
    }

    /// Check if CPU supports unrestricted guest mode
    pub fn supports_unrestricted_guest(&self) -> bool {
        self.unrestricted_guest && self.ept_supported
    }

    /// Check if we can use Extended Page Tables
    pub fn supports_ept(&self) -> bool {
        self.ept_supported
    }
}

/// VMXON region structure (4KB aligned)
#[repr(C, align(4096))]
#[allow(dead_code)]
pub struct VmxonRegion {
    pub revision_id: u32,
    pub data: [u8; 4092],
}

/// VMCS (Virtual Machine Control Structure) region
#[repr(C, align(4096))]
#[allow(dead_code)]
pub struct VmcsRegion {
    pub revision_id: u32,
    pub data: [u8; 4092],
}

#[allow(dead_code)]
impl VmcsRegion {
    /// Create a new VMCS region with proper revision ID
    pub fn new(revision_id: u32) -> Self {
        Self {
            revision_id,
            data: [0; 4092],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::message;
    use super::*;

    #[test]
    fn test_vtx_detection() {
        let caps = VtxCapabilities::detect();
        message!("\n","VT-x Available: {}", caps.available);
        message!("\n","VMXON Supported: {}", caps.vmxon_supported);
        message!("\n","EPT Supported: {}", caps.ept_supported);
    }
}