//! Guest memory management and isolation

use alloc::vec::Vec;

/// Guest memory page entry
#[derive(Debug, Clone)]
pub struct MemoryPage {
    pub gpa: u64,           // Guest physical address
    pub hpa: u64,           // Host physical address
    pub size: usize,        // Page size in bytes
    pub writable: bool,
    pub executable: bool,
}

/// Guest memory manager
pub struct MemoryManager {
    pages: Vec<MemoryPage>,
    total_size: usize,
}

impl MemoryManager {
    /// Create a new memory manager for guest memory
    pub fn new(total_size: usize) -> Self {
        Self {
            pages: Vec::new(),
            total_size,
        }
    }

    /// Map guest physical address to host physical address
    pub fn map_page(
        &mut self,
        gpa: u64,
        hpa: u64,
        size: usize,
        writable: bool,
        executable: bool,
    ) -> Result<(), &'static str> {
        if size == 0 {
            return Err("Page size must be > 0");
        }

        let page = MemoryPage {
            gpa,
            hpa,
            size,
            writable,
            executable,
        };

        self.pages.push(page);
        Ok(())
    }

    /// Lookup GPA to HPA translation
    pub fn translate_gpa(&self, gpa: u64) -> Option<u64> {
        for page in &self.pages {
            if gpa >= page.gpa && gpa < (page.gpa + page.size as u64) {
                let offset = gpa - page.gpa;
                return Some(page.hpa + offset);
            }
        }
        None
    }

    /// Get page count
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Get total allocated memory
    pub fn total_size(&self) -> usize {
        self.total_size
    }
}

/// Extended Page Tables (EPT) support for VT-x
#[cfg(target_arch = "x86_64")]
pub mod ept {
    use bitflags::bitflags;

    bitflags! {
        /// EPT page table entry flags
        pub struct EptFlags: u64 {
            const READ = 1 << 0;
            const WRITE = 1 << 1;
            const EXECUTE = 1 << 2;
            const MEMORY_TYPE_MASK = 0x7 << 3;
            const IGNORE_PAT = 1 << 6;
            const LARGE_PAGE = 1 << 7;
        }
    }

    /// EPT page table entry
    #[repr(transparent)]
    pub struct EptEntry(u64);

    impl EptEntry {
        pub fn new(physical_addr: u64, flags: EptFlags) -> Self {
            Self((physical_addr & 0xFFFFFFFFFFFFF000) | flags.bits())
        }

        pub fn flags(&self) -> EptFlags {
            EptFlags::from_bits_truncate(self.0)
        }

        pub fn physical_address(&self) -> u64 {
            self.0 & 0xFFFFFFFFFFFFF000
        }
    }
}