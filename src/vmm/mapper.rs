//! VDISK and VMEM Mapper.
//! Translates virtual addresses/sectors to physical ones.
//! Abstraction layer that prevents a VM from seeing or touching the host's actual memory or disk structure.

use alloc::vec::Vec;

pub struct MemoryMapping {
    pub gpa: u64,
    pub hpa: u64,
    pub size: usize,
}

pub struct DiskMapping {
    pub guest_sector: u64,
    pub host_vhd_offset: u64,
    pub size_sectors: u64,
}

pub struct ResourceMapper {
    pub vm_id: u32,
    memory_mappings: Vec<MemoryMapping>,
    disk_mappings: Vec<DiskMapping>,
}

impl ResourceMapper {
    pub fn new(vm_id: u32) -> Self {
        Self {
            vm_id,
            memory_mappings: Vec::new(),
            disk_mappings: Vec::new(),
        }
    }

    pub fn add_memory_mapping(&mut self, gpa: u64, hpa: u64, size: usize) {
        self.memory_mappings.push(MemoryMapping { gpa, hpa, size });
    }

    pub fn add_disk_mapping(&mut self, guest_sector: u64, host_vhd_offset: u64, size_sectors: u64) {
        self.disk_mappings.push(DiskMapping { guest_sector, host_vhd_offset, size_sectors });
    }

    pub fn get_memory_mappings(&self) -> &[MemoryMapping] {
        &self.memory_mappings
    }

    /// Translate GPA to HPA.
    pub fn translate_gpa(&self, gpa: u64) -> Option<u64> {
        for m in &self.memory_mappings {
            if gpa >= m.gpa && gpa < m.gpa + m.size as u64 {
                return Some(m.hpa + (gpa - m.gpa));
            }
        }
        None
    }

    /// Translate guest sector to host VHD offset.
    pub fn translate_sector(&self, sector: u64) -> Option<u64> {
        for d in &self.disk_mappings {
            if sector >= d.guest_sector && sector < d.guest_sector + d.size_sectors {
                return Some(d.host_vhd_offset + (sector - d.guest_sector) * 512);
            }
        }
        None
    }
}
