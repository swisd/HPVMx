use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use core::ptr::addr_of_mut;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

static mut TSS: TaskStateSegment = TaskStateSegment::new();
static mut GDT: (GlobalDescriptorTable, Selectors) = (GlobalDescriptorTable::new(), Selectors {
    code_selector: SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0),
    tss_selector: SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0),
});

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    use x86_64::instructions::segmentation::{CS, Segment};
    use x86_64::instructions::tables::load_tss;

    unsafe {
        let tss_ptr = addr_of_mut!(TSS);
        (*tss_ptr).interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(addr_of_mut!(STACK));
            let stack_end = stack_start + STACK_SIZE as u64;
            stack_end
        };

        let gdt_ptr = addr_of_mut!(GDT);
        let gdt = &mut (*gdt_ptr).0;
        let selectors = &mut (*gdt_ptr).1;

        selectors.code_selector = gdt.append(Descriptor::kernel_code_segment());
        selectors.tss_selector = gdt.append(Descriptor::tss_segment(&*tss_ptr));

        gdt.load();
        CS::set_reg(selectors.code_selector);
        load_tss(selectors.tss_selector);
    }
}
