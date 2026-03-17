//! Virtual Machine abstraction and lifecycle management

use alloc::string::String;
use alloc::vec::Vec;
use crate::vmm::vcpu::VirtualCpu;
use crate::vmm::vmbus::VmBus;
use crate::vmm::mapper::ResourceMapper;

/// Virtual Machine state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum VmState {
    Created,
    Running,
    Paused,
    Stopped,
    Failed,
    Decommissioned, // Added for Autolytic Protocol
}

impl core::fmt::Display for VmState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            VmState::Created => write!(f, "Created"),
            VmState::Running => write!(f, "Running"),
            VmState::Paused => write!(f, "Paused"),
            VmState::Stopped => write!(f, "Stopped"),
            VmState::Failed => write!(f, "Failed"),
            VmState::Decommissioned => write!(f, "Decommissioned"),
        }
    }
}

/// Virtual Machine instance
pub struct VirtualMachine {
    pub id: u32,
    pub name: String,
    pub state: VmState,
    pub memory_mb: u32,
    pub vcpu_count: u32,
    pub vcpus: Vec<VirtualCpu>,
    pub guest_memory_base: Option<usize>,
    pub vmbus: VmBus,            // New: Communication bus
    pub mapper: ResourceMapper, // New: Memory/Disk mapping
}

#[allow(dead_code)]
impl VirtualMachine {
    /// Create a new virtual machine
    pub fn new(id: u32, name: &str, memory_mb: u32, vcpu_count: u32) -> Result<Self, &'static str> {
        if memory_mb == 0 || vcpu_count == 0 {
            return Err("Memory and vCPU count must be > 0");
        }

        let mut vcpus = Vec::new();
        for i in 0..vcpu_count {
            vcpus.push(VirtualCpu::new(i));
        }

        Ok(Self {
            id,
            name: String::from(name),
            state: VmState::Created,
            memory_mb,
            vcpu_count,
            vcpus,
            guest_memory_base: None,
            vmbus: VmBus::new(id),
            mapper: ResourceMapper::new(id),
        })
    }

    /// Allocate guest memory for this VM
    pub fn allocate_memory(&mut self, base_addr: usize) -> Result<(), &'static str> {
        if self.guest_memory_base.is_some() {
            return Err("Memory already allocated");
        }

        // Validate that requested memory is available
        let memory_bytes = (self.memory_mb as usize) * 1024 * 1024;
        if memory_bytes == 0 {
            return Err("Invalid memory size");
        }

        self.guest_memory_base = Some(base_addr);
        Ok(())
    }

    /// Get guest memory base address
    pub fn get_memory_base(&self) -> Option<usize> {
        self.guest_memory_base
    }

    /// Get guest memory size in bytes
    pub fn get_memory_size(&self) -> usize {
        (self.memory_mb as usize) * 1024 * 1024
    }

    /// Add a vCPU to this VM
    pub fn add_vcpu(&mut self) -> u32 {
        let vcpu_id = self.vcpu_count;
        self.vcpus.push(VirtualCpu::new(vcpu_id));
        self.vcpu_count += 1;
        vcpu_id
    }

    /// Get a specific vCPU
    pub fn get_vcpu(&self, vcpu_id: u32) -> Option<&VirtualCpu> {
        self.vcpus.get(vcpu_id as usize)
    }

    /// Get mutable vCPU
    pub fn get_vcpu_mut(&mut self, vcpu_id: u32) -> Option<&mut VirtualCpu> {
        self.vcpus.get_mut(vcpu_id as usize)
    }

    /// Resume execution
    pub fn resume(&mut self) -> Result<(), &'static str> {
        if self.state == VmState::Paused {
            self.state = VmState::Running;
            Ok(())
        } else {
            Err("VM must be paused to resume")
        }
    }

    /// Pause execution
    pub fn pause(&mut self) -> Result<(), &'static str> {
        if self.state == VmState::Running {
            self.state = VmState::Paused;
            Ok(())
        } else {
            Err("VM must be running to pause")
        }
    }
}