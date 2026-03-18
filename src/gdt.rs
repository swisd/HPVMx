use crate::{hpvm_error, hpvm_info, hpvm_log, hpvm_warn};
use uefi::proto::console::text::Color;
use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use core::ptr::addr_of_mut;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 1;

static mut TSS: TaskStateSegment = TaskStateSegment::new();
static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

pub struct Selectors {
    pub code_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}

static mut SELECTORS: Option<Selectors> = None;

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};

    unsafe {
        let tss = addr_of_mut!(TSS);
        (*tss).interrupt_stack_table[(DOUBLE_FAULT_IST_INDEX - 1) as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(addr_of_mut!(STACK));
            let stack_end = stack_start + STACK_SIZE as u64;
            stack_end
        };

        let gdt = addr_of_mut!(GDT);
        // Using append for compatibility with x86_64 0.15.4
        let code_selector = (*gdt).append(Descriptor::kernel_code_segment());
        hpvm_info!("GDT", "gdt append code selector {:#?}", code_selector);
        let data_selector = (*gdt).append(Descriptor::kernel_data_segment());
        hpvm_info!("GDT", "gdt append data_selector {:#?}", data_selector);
        let tss_selector = (*gdt).append(Descriptor::tss_segment(&*tss));
        hpvm_info!("GDT", "tss append selector {:#?}", tss_selector);

        hpvm_info!("GDT", "start gdt load");
        hpvm_error!("GDT", "no gdt loading");
        //(*gdt).load();
        //hpvm_info!("GDT", "gdt load ok");

        //hpvm_info!("GDT", "set reg DS ES SS to 0");
        use x86_64::instructions::segmentation::{DS, ES, SS, Segment};
        //DS::set_reg(SegmentSelector(0));
        //ES::set_reg(SegmentSelector(0));
        //SS::set_reg(SegmentSelector(0));
        // hpvm_info!("GDT", "set reg DS ES SS ok");
        //
        // hpvm_info!("GDT", "set reg CS");
        // //CS::set_reg(code_selector);
        // hpvm_info!("GDT", "set reg CS ok with {:#?}", code_selector);
        //
        // hpvm_info!("GDT", "load tss {:#?}", tss_selector);
        // //load_tss(tss_selector);
        // hpvm_info!("GDT", "load tss ok");
        //
        // hpvm_info!("GDT", "GDT and TSS loaded");
        hpvm_warn!("GDT", "gdt may be invalid due to misload of _gdt");

        SELECTORS = Some(Selectors {
            code_selector,
            tss_selector,
        });
    }
}
