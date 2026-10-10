//! Four-level, 4 KiB-page Extended Page Tables for Intel VT-x.

use alloc::vec::Vec;
use bitflags::bitflags;
use core::ptr::NonNull;

const PAGE_SIZE: u64 = 4096;
const PAGE_MASK: u64 = PAGE_SIZE - 1;
const ADDRESS_MASK: u64 = 0x000F_FFFF_FFFF_F000;
const EPT_MEMORY_TYPE_WB: u64 = 6;
const EPT_LEAF_MEMORY_TYPE_WB: u64 = 6 << 3;
const EPT_WALK_LENGTH_4: u64 = 3 << 3;
const EPTP_ACCESSED_DIRTY: u64 = 1 << 6;

bitflags! {
    /// Permissions applied to a guest-physical mapping.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct EptPermissions: u64 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const EXECUTE = 1 << 2;
    }
}

/// Owns the page-table pages backing an EPT address space.
pub struct Ept {
    root: NonNull<u64>,
    tables: Vec<NonNull<u64>>,
    mapped_ranges: Vec<(u64, u64)>,
    accessed_dirty: bool,
}

impl Ept {
    /// Allocate a four-level EPT root. UEFI Boot Services must still be active.
    pub fn new(accessed_dirty: bool) -> Result<Self, &'static str> {
        let root = allocate_table()?;
        Ok(Self {
            root,
            tables: alloc::vec![root],
            mapped_ranges: Vec::new(),
            accessed_dirty,
        })
    }

    /// Physical address and EPTP value to place in VMCS field EPT_POINTER.
    pub fn ept_pointer(&self) -> u64 {
        self.root.as_ptr() as u64
            | EPT_MEMORY_TYPE_WB
            | EPT_WALK_LENGTH_4
            | if self.accessed_dirty {
                EPTP_ACCESSED_DIRTY
            } else {
                0
            }
    }

    /// Map a page-aligned guest physical range to host physical memory.
    pub fn map_range(
        &mut self,
        guest_start: u64,
        host_start: u64,
        size: u64,
        permissions: EptPermissions,
    ) -> Result<(), &'static str> {
        validate_mapping(guest_start, host_start, size, permissions)?;
        let guest_end = guest_start + size;
        if self
            .mapped_ranges
            .iter()
            .any(|(start, end)| guest_start < *end && *start < guest_end)
        {
            return Err("EPT guest range overlaps an existing mapping");
        }
        let mut offset = 0u64;
        while offset < size {
            if let Err(error) =
                self.map_page(guest_start + offset, host_start + offset, permissions)
            {
                let mut rollback = 0;
                while rollback < offset {
                    self.unmap_page(guest_start + rollback);
                    rollback += PAGE_SIZE;
                }
                return Err(error);
            }
            offset += PAGE_SIZE;
        }
        self.mapped_ranges.push((guest_start, guest_end));
        Ok(())
    }

    fn unmap_page(&mut self, guest: u64) {
        let indices = [
            ((guest >> 39) & 0x1ff) as usize,
            ((guest >> 30) & 0x1ff) as usize,
            ((guest >> 21) & 0x1ff) as usize,
            ((guest >> 12) & 0x1ff) as usize,
        ];
        let mut table = self.root;
        for index in indices[..3].iter().copied() {
            let value = unsafe { table.as_ptr().add(index).read() };
            let Some(child) = NonNull::new((value & ADDRESS_MASK) as *mut u64) else {
                return;
            };
            table = child;
        }
        unsafe {
            table.as_ptr().add(indices[3]).write(0);
        }
    }

    fn map_page(
        &mut self,
        guest: u64,
        host: u64,
        permissions: EptPermissions,
    ) -> Result<(), &'static str> {
        let indices = [
            ((guest >> 39) & 0x1ff) as usize,
            ((guest >> 30) & 0x1ff) as usize,
            ((guest >> 21) & 0x1ff) as usize,
            ((guest >> 12) & 0x1ff) as usize,
        ];
        let mut table = self.root;

        for index in indices[..3].iter().copied() {
            let entry = unsafe { table.as_ptr().add(index) };
            let value = unsafe { entry.read() };
            if value & 0x7 == 0 {
                let child = allocate_table()?;
                self.tables.push(child);
                unsafe {
                    entry.write(child.as_ptr() as u64 | 0x7);
                }
                table = child;
            } else {
                if value & (1 << 7) != 0 {
                    return Err("EPT mapping conflicts with an existing large page");
                }
                let child = NonNull::new((value & ADDRESS_MASK) as *mut u64)
                    .ok_or("EPT contains a null intermediate table")?;
                table = child;
            }
        }

        let leaf = unsafe { table.as_ptr().add(indices[3]) };
        if unsafe { leaf.read() } & 0x7 != 0 {
            return Err("EPT guest page is already mapped");
        }
        unsafe {
            leaf.write((host & ADDRESS_MASK) | permissions.bits() | EPT_LEAF_MEMORY_TYPE_WB);
        }
        Ok(())
    }
}

impl Drop for Ept {
    fn drop(&mut self) {
        for table in self.tables.drain(..).rev() {
            if let Err(error) = unsafe { uefi::boot::free_pages(table.cast(), 1) } {
                log::error!("failed to free EPT page table: {}", error);
            }
        }
    }
}

fn allocate_table() -> Result<NonNull<u64>, &'static str> {
    let address = uefi::boot::allocate_pages(
        uefi::boot::AllocateType::MaxAddress(0xFFFF_FFFF),
        uefi::boot::MemoryType::LOADER_DATA,
        1,
    )
    .map_err(|_| "UEFI could not allocate an EPT page table")?;
    let table = address.cast::<u64>();
    unsafe {
        core::ptr::write_bytes(table.as_ptr(), 0, (PAGE_SIZE / 8) as usize);
    }
    Ok(table)
}

fn validate_mapping(
    guest_start: u64,
    host_start: u64,
    size: u64,
    permissions: EptPermissions,
) -> Result<(), &'static str> {
    if size == 0
        || guest_start & PAGE_MASK != 0
        || host_start & PAGE_MASK != 0
        || size & PAGE_MASK != 0
    {
        return Err("EPT mappings must be non-empty and page aligned");
    }
    if permissions.is_empty() {
        return Err("EPT mappings require at least one permission");
    }
    let guest_end = guest_start
        .checked_add(size)
        .ok_or("EPT guest range overflows")?;
    let host_end = host_start
        .checked_add(size)
        .ok_or("EPT host range overflows")?;
    if guest_end > (1u64 << 48) || host_end > (1u64 << 52) {
        return Err("EPT range exceeds the supported physical-address width");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ept_mapping_inputs_are_checked_before_page_table_mutation() {
        assert_eq!(
            validate_mapping(
                0,
                0x2000,
                0x1000,
                EptPermissions::READ | EptPermissions::WRITE
            ),
            Ok(())
        );
        assert_eq!(
            validate_mapping(1, 0x2000, 0x1000, EptPermissions::READ),
            Err("EPT mappings must be non-empty and page aligned")
        );
        assert_eq!(
            validate_mapping(0, 0, 0, EptPermissions::READ),
            Err("EPT mappings must be non-empty and page aligned")
        );
        assert_eq!(
            validate_mapping(0, 0, 0x1000, EptPermissions::empty()),
            Err("EPT mappings require at least one permission")
        );
        assert_eq!(
            validate_mapping(u64::MAX & !PAGE_MASK, 0, PAGE_SIZE, EptPermissions::READ),
            Err("EPT guest range overflows")
        );
    }
}
