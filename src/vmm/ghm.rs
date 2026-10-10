//! Global Hardware Manager - assigns actual UEFI-allocated pages and CPU cores.

use crate::{hardware, vdebug_autoprefix};
use alloc::vec;
use alloc::vec::Vec;

const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalResourceId {
    CpuCore(u32),
    MemorySegment(u64, usize),
}

pub struct ResourceAssignment {
    pub vm_id: u32,
    pub resource_id: PhysicalResourceId,
}

pub struct GlobalHardwareManager {
    assignments: Vec<ResourceAssignment>,
    available_cores: Vec<u32>,
    reserved_cores: Vec<u32>,
    threads: Vec<u32>,
}

impl GlobalHardwareManager {
    pub fn new(total_cores: u32, total_memory_mb: usize) -> Self {
        vdebug_autoprefix!(
            "GHM: initializing with {} cores, {} MB requested-memory budget",
            total_cores,
            total_memory_mb
        );
        let available_cores = (1..total_cores).collect();

        Self {
            assignments: Vec::new(),
            available_cores,
            reserved_cores: vec![0],
            threads: vec![1],
        }
    }

    /// Push an assignment to a VM. VMs cannot directly request host resources.
    fn push_assignment(&mut self, vm_id: u32, resource: PhysicalResourceId) {
        vdebug_autoprefix!("GHM: pushing resource assignment to VM {}", vm_id);
        self.assignments.push(ResourceAssignment {
            vm_id,
            resource_id: resource,
        });
    }

    pub fn allocate_core_to_vm(&mut self, vm_id: u32) -> Result<u32, &'static str> {
        if let Some(core) = self.available_cores.pop() {
            vdebug_autoprefix!("GHM: allocated core {} to VM {}", core, vm_id);
            self.push_assignment(vm_id, PhysicalResourceId::CpuCore(core));
            Ok(core)
        } else {
            Err("No available CPU cores")
        }
    }

    /// Allocate zeroed, page-backed memory that is addressable by the host.
    pub fn allocate_memory_to_vm(
        &mut self,
        vm_id: u32,
        size_bytes: usize,
    ) -> Result<u64, &'static str> {
        let pages = pages_for_size(size_bytes)?;
        let allocated_size = pages
            .checked_mul(PAGE_SIZE)
            .ok_or("memory allocation size overflow")?;
        let address = uefi::boot::allocate_pages(
            uefi::boot::AllocateType::AnyPages,
            uefi::boot::MemoryType::LOADER_DATA,
            pages,
        )
        .map_err(|_| "UEFI could not allocate guest memory")?;

        unsafe {
            core::ptr::write_bytes(address.as_ptr(), 0, allocated_size);
        }
        let base = address.as_ptr() as u64;
        vdebug_autoprefix!(
            "GHM: allocated and zeroed {} bytes at 0x{:x} for VM {}",
            allocated_size,
            base,
            vm_id
        );
        self.push_assignment(
            vm_id,
            PhysicalResourceId::MemorySegment(base, allocated_size),
        );
        Ok(base)
    }

    /// Clear all currently assigned guest pages before they can be reused.
    pub fn zero_memory_for_vm(&self, vm_id: u32) {
        for assignment in &self.assignments {
            if assignment.vm_id == vm_id {
                if let PhysicalResourceId::MemorySegment(base, size) = assignment.resource_id {
                    unsafe {
                        core::ptr::write_bytes(base as *mut u8, 0, size);
                    }
                }
            }
        }
    }

    /// Release a VM's assignments. Failed page frees remain tracked for retry.
    pub fn revoke_assignments(&mut self, vm_id: u32) -> Result<(), &'static str> {
        vdebug_autoprefix!("GHM: revoking all assignments for VM {}", vm_id);
        // Prevent stale guest contents from being exposed if firmware reuses the pages.
        self.zero_memory_for_vm(vm_id);

        let mut remaining = Vec::new();
        let mut release_failed = false;
        for assignment in core::mem::take(&mut self.assignments) {
            if assignment.vm_id != vm_id {
                remaining.push(assignment);
                continue;
            }

            match assignment.resource_id {
                PhysicalResourceId::CpuCore(core) => self.available_cores.push(core),
                PhysicalResourceId::MemorySegment(base, size) => {
                    let Some(address) = core::ptr::NonNull::new(base as *mut u8) else {
                        remaining.push(assignment);
                        release_failed = true;
                        continue;
                    };
                    let pages = size / PAGE_SIZE;
                    if size % PAGE_SIZE != 0 || pages == 0 {
                        remaining.push(assignment);
                        release_failed = true;
                        continue;
                    }
                    if unsafe { uefi::boot::free_pages(address, pages) }.is_err() {
                        log::error!(
                            "failed to release {} guest-memory pages for VM {}",
                            pages,
                            vm_id
                        );
                        remaining.push(assignment);
                        release_failed = true;
                    }
                }
            }
        }
        self.assignments = remaining;

        if release_failed {
            Err("one or more VM resource assignments could not be released")
        } else {
            Ok(())
        }
    }

    pub fn set_cpu_props(&mut self, cpu_info: &hardware::cpu::CpuInfo) {
        self.available_cores = (1..cpu_info.cores)
            .filter(|core| {
                !self
                    .assignments
                    .iter()
                    .any(|assignment| assignment.resource_id == PhysicalResourceId::CpuCore(*core))
            })
            .collect();
        self.reserved_cores = vec![0];
        self.threads = (1..cpu_info.threads).collect();
    }
}

fn pages_for_size(size_bytes: usize) -> Result<usize, &'static str> {
    if size_bytes == 0 {
        return Err("guest memory size must be non-zero");
    }
    size_bytes
        .checked_add(PAGE_SIZE - 1)
        .map(|rounded| rounded / PAGE_SIZE)
        .ok_or("memory allocation size overflow")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guest_memory_sizes_round_up_to_pages() {
        assert_eq!(pages_for_size(1), Ok(1));
        assert_eq!(pages_for_size(PAGE_SIZE), Ok(1));
        assert_eq!(pages_for_size(PAGE_SIZE + 1), Ok(2));
    }

    #[test]
    fn zero_sized_or_overflowing_allocations_are_rejected() {
        assert_eq!(pages_for_size(0), Err("guest memory size must be non-zero"));
        assert_eq!(
            pages_for_size(usize::MAX),
            Err("memory allocation size overflow")
        );
    }
}
