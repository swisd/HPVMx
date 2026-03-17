//! Global Hardware Manager (GHM) - A "Push-Only" allocator that assigns physical resources to VM Units.
//! Zero-Request Model: The GHM accepts zero incoming requests from VMs, eliminating hypercall-based privilege escalation.

use alloc::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub enum PhysicalResourceId {
    CpuCore(u32),
    MemorySegment(u64, usize), // Base address and size
}

pub struct ResourceAssignment {
    pub vm_id: u32,
    pub resource_id: PhysicalResourceId,
}

pub struct GlobalHardwareManager {
    assignments: Vec<ResourceAssignment>,
    available_cores: Vec<u32>,
    available_memory: Vec<(u64, usize)>,
}

impl GlobalHardwareManager {
    pub fn new(total_cores: u32, total_memory_mb: usize) -> Self {
        let mut available_cores = Vec::new();
        for i in 0..total_cores {
            available_cores.push(i);
        }

        let mut available_memory = Vec::new();
        // Simplified: single large block of memory
        available_memory.push((0x100000000, total_memory_mb * 1024 * 1024));

        Self {
            assignments: Vec::new(),
            available_cores,
            available_memory,
        }
    }

    /// Push an assignment to a VM. This is the only way resources are allocated.
    /// There is no "request_resource" method accessible to VMs.
    pub fn push_assignment(&mut self, vm_id: u32, resource: PhysicalResourceId) {
        self.assignments.push(ResourceAssignment {
            vm_id,
            resource_id: resource,
        });
    }

    pub fn allocate_core_to_vm(&mut self, vm_id: u32) -> Result<u32, &'static str> {
        if let Some(core) = self.available_cores.pop() {
            self.push_assignment(vm_id, PhysicalResourceId::CpuCore(core));
            Ok(core)
        } else {
            Err("No available CPU cores")
        }
    }

    pub fn allocate_memory_to_vm(&mut self, vm_id: u32, size_bytes: usize) -> Result<u64, &'static str> {
        for i in 0..self.available_memory.len() {
            let (base, size) = self.available_memory[i];
            if size >= size_bytes {
                let allocated_base = base;
                if size == size_bytes {
                    self.available_memory.remove(i);
                } else {
                    self.available_memory[i] = (base + size_bytes as u64, size - size_bytes);
                }
                self.push_assignment(vm_id, PhysicalResourceId::MemorySegment(allocated_base, size_bytes));
                return Ok(allocated_base);
            }
        }
        Err("No available memory segments of requested size")
    }

    /// Revoke all assignments for a specific VM (used during decommissioning)
    pub fn revoke_assignments(&mut self, vm_id: u32) {
        self.assignments.retain(|a| {
            if a.vm_id == vm_id {
                match a.resource_id {
                    PhysicalResourceId::CpuCore(core) => self.available_cores.push(core),
                    PhysicalResourceId::MemorySegment(base, size) => self.available_memory.push((base, size)),
                }
                false
            } else {
                true
            }
        });
        // Coalesce memory fragments (simplified)
        self.available_memory.sort_by_key(|&(base, _)| base);
    }
}
