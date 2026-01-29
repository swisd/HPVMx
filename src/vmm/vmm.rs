//! HPVMx Virtual Machine Monitor - Core hypervisor coordinator

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::vmm::vm::{VirtualMachine, VmState};

static VM_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// HPVMx Virtual Machine Monitor
pub struct HypervisorManager {
    /// Map of VM ID to Virtual Machine instance
    vms: BTreeMap<u32, VirtualMachine>,
    /// Hypervisor capabilities
    pub is_initialized: bool,
    pub vm_count: u32,
}

#[allow(dead_code)]
impl HypervisorManager {
    /// Create a new hypervisor manager instance
    pub fn new() -> Self {
        Self {
            vms: BTreeMap::new(),
            is_initialized: false,
            vm_count: 0,
        }
    }

    /// Initialize the hypervisor
    pub fn initialize(&mut self) -> Result<(), &'static str> {
        if self.is_initialized {
            return Err("Hypervisor already initialized");
        }

        #[cfg(target_arch = "x86_64")]
        {
            use crate::hardware::vmx::VtxCapabilities;
            let vtx = VtxCapabilities::detect();
            if !vtx.available {
                return Err("VT-x not available on this CPU");
            }
        }

        self.is_initialized = true;
        Ok(())
    }

    /// Create a new virtual machine
    pub fn create_vm(&mut self, name: &str, memory_mb: u32, vcpu_count: u32) -> Result<u32, &'static str> {
        if !self.is_initialized {
            return Err("Hypervisor not initialized");
        }

        let vm_id = VM_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

        let vm = VirtualMachine::new(vm_id, name, memory_mb, vcpu_count)?;
        self.vms.insert(vm_id, vm);
        self.vm_count += 1;

        Ok(vm_id)
    }

    /// Start a virtual machine
    pub fn start_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        match self.vms.get_mut(&vm_id) {
            Some(vm) => {
                vm.state = VmState::Running;
                Ok(())
            }
            None => Err("VM not found"),
        }
    }

    /// Stop a virtual machine
    pub fn stop_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        match self.vms.get_mut(&vm_id) {
            Some(vm) => {
                vm.state = VmState::Stopped;
                Ok(())
            }
            None => Err("VM not found"),
        }
    }

    /// Get VM by ID
    pub fn get_vm(&self, vm_id: u32) -> Option<&VirtualMachine> {
        self.vms.get(&vm_id)
    }

    /// Get mutable VM by ID
    pub fn get_vm_mut(&mut self, vm_id: u32) -> Option<&mut VirtualMachine> {
        self.vms.get_mut(&vm_id)
    }

    /// List all VMs
    pub fn list_vms(&self) -> Vec<(u32, String, VmState)> {
        self.vms
            .iter()
            .map(|(id, vm)| (*id, vm.name.clone(), vm.state))
            .collect()
    }

    /// Delete a VM
    pub fn delete_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        match self.vms.remove(&vm_id) {
            Some(_) => {
                self.vm_count = self.vm_count.saturating_sub(1);
                Ok(())
            }
            None => Err("VM not found"),
        }
    }

    /// Get hypervisor statistics
    pub fn get_stats(&self) -> HypervisorStats {
        let running_vms = self
            .vms
            .iter()
            .filter(|(_, vm)| matches!(vm.state, VmState::Running))
            .count() as u32;

        let total_memory = self.vms.iter().map(|(_, vm)| vm.memory_mb).sum();

        HypervisorStats {
            initialized: self.is_initialized,
            total_vms: self.vm_count,
            running_vms,
            total_memory_mb: total_memory,
        }
    }

    pub fn get_stats_advanced(&self) -> (HypervisorStats, String) {
        let running_vms = self
            .vms
            .iter()
            .filter(|(_, vm)| matches!(vm.state, VmState::Running))
            .count() as u32;

        let total_memory = self.vms.iter().map(|(_, vm)| vm.memory_mb).sum();

        let vms = self.vms.iter();

        let mut list_str = String::new();

        for vm in vms {
            if !list_str.is_empty() {
                list_str.push('\n');
            }
            list_str.push_str(&format!("  INT {}  ID {}  NAME {}  STATE {}  MEM {} #CPU {}",
                                       vm.0, vm.1.id, vm.1.name, vm.1.state, vm.1.memory_mb, vm.1.vcpu_count));
        }

        (HypervisorStats {
            initialized: self.is_initialized,
            total_vms: self.vm_count,
            running_vms,
            total_memory_mb: total_memory,
        }, list_str)
    }

    pub fn boot_vm_with_media(&mut self, _vm_id: u32, _media_data: &[u8]) -> Result<(), &str> {
        // Find the VM
        // let vm = self.vms.iter_mut().find(|v| v.id == vm_id)
        //     .ok_or("VM not found")?;

        // Set the VM's boot media
        // In a real implementation, this would:
        // 1. Map the media data into guest memory
        // 2. Set up boot parameters
        // 3. Jump to the bootloader entry point

        //vm.state = crate::vmm::vm::VmState::Running;

        Ok(())
    }
}

/// Hypervisor statistics
#[derive(Debug, Clone)]
pub struct HypervisorStats {
    pub initialized: bool,
    pub total_vms: u32,
    pub running_vms: u32,
    pub total_memory_mb: u32,
}

