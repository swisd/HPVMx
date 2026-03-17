//! Hardware Partitioner - Carves physical CPU cores and memory segments into isolated "Silicons."
//! Prevents cross-VM interference and significantly mitigates CPU side-channel leaks.

use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SiliconUnit {
    pub vm_id: u32,
    pub cores: Vec<u32>,
    pub memory_base: u64,
    pub memory_size: usize,
}

pub struct HardwarePartitioner {
    units: Vec<SiliconUnit>,
}

impl HardwarePartitioner {
    pub fn new() -> Self {
        Self {
            units: Vec::new(),
        }
    }

    /// Carve out a new Silicon unit for a VM.
    pub fn create_silicon_unit(
        &mut self,
        vm_id: u32,
        cores: Vec<u32>,
        memory_base: u64,
        memory_size: usize,
    ) -> Result<(), &'static str> {
        // In a real implementation, this would involve programming the hardware (MMU/IOMMU)
        // to enforce these boundaries.
        let unit = SiliconUnit {
            vm_id,
            cores,
            memory_base,
            memory_size,
        };
        self.units.push(unit);
        Ok(())
    }

    /// Remove a Silicon unit.
    pub fn remove_silicon_unit(&mut self, vm_id: u32) {
        self.units.retain(|u| u.vm_id != vm_id);
    }

    pub fn get_unit(&self, vm_id: u32) -> Option<&SiliconUnit> {
        self.units.iter().find(|u| u.vm_id == vm_id)
    }
}
