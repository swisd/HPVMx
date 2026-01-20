use x86_64::{
    structures::paging::{PageTable, OffsetPageTable, Page, Size4KiB, Mapper, FrameAllocator},
    registers::control::Cr3,
    VirtAddr, PhysAddr,
};

pub struct PagingManager;

impl PagingManager {
    #[allow(dead_code)]
    #[allow(unsafe_code)]
    /// Returns a mapper for the current active level 4 page table.
    /// In UEFI, the physical memory is often identity mapped or at a known offset.
    pub unsafe fn get_active_mapper(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
        let (level_4_table_frame, _) = Cr3::read();
        let phys = level_4_table_frame.start_address();
        let virt = physical_memory_offset + phys.as_u64(); // Simple offset mapping
        let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

        OffsetPageTable::new(&mut *page_table_ptr, physical_memory_offset)
    }

    /// Example: Map a specific virtual address to a physical address
    pub fn map_address(
        virt: u64,
        phys: u64,
        flags: x86_64::structures::paging::PageTableFlags,
        mapper: &mut OffsetPageTable,
        frame_allocator: &mut impl FrameAllocator<Size4KiB>
    ) -> Result<(), &'static str> {
        let page: x86_64::structures::paging::Page = Page::containing_address(VirtAddr::new(virt));
        let frame = x86_64::structures::paging::PhysFrame::containing_address(PhysAddr::new(phys));

        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)
                .map_err(|_| "Mapping failed")?
                .flush();
        }
        Ok(())
    }
}