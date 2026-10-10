//! Typed guest/host execution state and VMCS field programming.

use crate::hardware::cpu::vmx::{Vmcs, VmxHostSession};

mod field {
    pub const PIN_BASED_CONTROLS: u32 = 0x4000;
    pub const PRIMARY_PROC_CONTROLS: u32 = 0x4002;
    pub const VM_EXIT_CONTROLS: u32 = 0x400c;
    pub const VM_ENTRY_CONTROLS: u32 = 0x4012;
    pub const SECONDARY_PROC_CONTROLS: u32 = 0x401e;
    pub const VM_EXIT_REASON: u32 = 0x4402;
    pub const EXIT_INSTRUCTION_LENGTH: u32 = 0x440c;
    pub const VMX_PREEMPTION_TIMER: u32 = 0x482e;
    pub const GUEST_ES_SELECTOR: u32 = 0x0800;
    pub const GUEST_CS_SELECTOR: u32 = 0x0802;
    pub const GUEST_SS_SELECTOR: u32 = 0x0804;
    pub const GUEST_DS_SELECTOR: u32 = 0x0806;
    pub const GUEST_FS_SELECTOR: u32 = 0x0808;
    pub const GUEST_GS_SELECTOR: u32 = 0x080a;
    pub const GUEST_LDTR_SELECTOR: u32 = 0x080c;
    pub const GUEST_TR_SELECTOR: u32 = 0x080e;
    pub const HOST_ES_SELECTOR: u32 = 0x0c00;
    pub const HOST_CS_SELECTOR: u32 = 0x0c02;
    pub const HOST_SS_SELECTOR: u32 = 0x0c04;
    pub const HOST_DS_SELECTOR: u32 = 0x0c06;
    pub const HOST_FS_SELECTOR: u32 = 0x0c08;
    pub const HOST_GS_SELECTOR: u32 = 0x0c0a;
    pub const HOST_TR_SELECTOR: u32 = 0x0c0c;
    pub const IO_BITMAP_A: u32 = 0x2000;
    pub const IO_BITMAP_B: u32 = 0x2002;
    pub const EPT_POINTER: u32 = 0x201a;
    pub const GUEST_PHYSICAL_ADDRESS: u32 = 0x2400;
    pub const GUEST_LINK_POINTER: u32 = 0x2800;
    pub const HOST_PAT: u32 = 0x2c00;
    pub const HOST_EFER: u32 = 0x2c02;
    pub const GUEST_DEBUGCTL: u32 = 0x2802;
    pub const GUEST_PAT: u32 = 0x2804;
    pub const GUEST_EFER: u32 = 0x2806;
    pub const GUEST_CR0: u32 = 0x6800;
    pub const GUEST_CR3: u32 = 0x6802;
    pub const GUEST_CR4: u32 = 0x6804;
    pub const GUEST_ES_BASE: u32 = 0x6806;
    pub const GUEST_CS_BASE: u32 = 0x6808;
    pub const GUEST_SS_BASE: u32 = 0x680a;
    pub const GUEST_DS_BASE: u32 = 0x680c;
    pub const GUEST_FS_BASE: u32 = 0x680e;
    pub const GUEST_GS_BASE: u32 = 0x6810;
    pub const GUEST_LDTR_BASE: u32 = 0x6812;
    pub const GUEST_TR_BASE: u32 = 0x6814;
    pub const GUEST_GDTR_BASE: u32 = 0x6816;
    pub const GUEST_IDTR_BASE: u32 = 0x6818;
    pub const GUEST_DR7: u32 = 0x681a;
    pub const GUEST_RSP: u32 = 0x681c;
    pub const GUEST_RIP: u32 = 0x681e;
    pub const GUEST_RFLAGS: u32 = 0x6820;
    pub const GUEST_PENDING_DEBUG: u32 = 0x6822;
    pub const GUEST_SYSENTER_ESP: u32 = 0x6824;
    pub const GUEST_SYSENTER_EIP: u32 = 0x6826;
    pub const HOST_CR0: u32 = 0x6c00;
    pub const HOST_CR3: u32 = 0x6c02;
    pub const HOST_CR4: u32 = 0x6c04;
    pub const HOST_FS_BASE: u32 = 0x6c06;
    pub const HOST_GS_BASE: u32 = 0x6c08;
    pub const HOST_TR_BASE: u32 = 0x6c0a;
    pub const HOST_GDTR_BASE: u32 = 0x6c0c;
    pub const HOST_IDTR_BASE: u32 = 0x6c0e;
    pub const HOST_SYSENTER_ESP: u32 = 0x6c10;
    pub const HOST_SYSENTER_EIP: u32 = 0x6c12;
    pub const HOST_RSP: u32 = 0x6c14;
    pub const HOST_RIP: u32 = 0x6c16;
    pub const GUEST_ES_LIMIT: u32 = 0x4800;
    pub const GUEST_CS_LIMIT: u32 = 0x4802;
    pub const GUEST_SS_LIMIT: u32 = 0x4804;
    pub const GUEST_DS_LIMIT: u32 = 0x4806;
    pub const GUEST_FS_LIMIT: u32 = 0x4808;
    pub const GUEST_GS_LIMIT: u32 = 0x480a;
    pub const GUEST_LDTR_LIMIT: u32 = 0x480c;
    pub const GUEST_TR_LIMIT: u32 = 0x480e;
    pub const GUEST_GDTR_LIMIT: u32 = 0x4810;
    pub const GUEST_IDTR_LIMIT: u32 = 0x4812;
    pub const GUEST_ES_AR: u32 = 0x4814;
    pub const GUEST_CS_AR: u32 = 0x4816;
    pub const GUEST_SS_AR: u32 = 0x4818;
    pub const GUEST_DS_AR: u32 = 0x481a;
    pub const GUEST_FS_AR: u32 = 0x481c;
    pub const GUEST_GS_AR: u32 = 0x481e;
    pub const GUEST_LDTR_AR: u32 = 0x4820;
    pub const GUEST_TR_AR: u32 = 0x4822;
    pub const GUEST_INTERRUPTIBILITY: u32 = 0x4824;
    pub const GUEST_ACTIVITY_STATE: u32 = 0x4826;
    pub const GUEST_SYSENTER_CS: u32 = 0x482a;
    pub const VM_EXIT_QUALIFICATION: u32 = 0x6400;
    pub const HOST_SYSENTER_CS: u32 = 0x4c00;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentState {
    pub selector: u16,
    pub base: u64,
    pub limit: u32,
    pub access_rights: u32,
}

impl SegmentState {
    pub const fn null() -> Self {
        Self {
            selector: 0,
            base: 0,
            limit: 0,
            access_rights: 1 << 16,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GuestState {
    pub es: SegmentState,
    pub cs: SegmentState,
    pub ss: SegmentState,
    pub ds: SegmentState,
    pub fs: SegmentState,
    pub gs: SegmentState,
    pub ldtr: SegmentState,
    pub tr: SegmentState,
    pub gdtr_base: u64,
    pub gdtr_limit: u32,
    pub idtr_base: u64,
    pub idtr_limit: u32,
    pub cr0: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub dr7: u64,
    pub rsp: u64,
    pub rip: u64,
    pub rflags: u64,
    pub pending_debug: u64,
    pub interruptibility: u32,
    pub activity_state: u32,
    pub sysenter_cs: u32,
    pub sysenter_esp: u64,
    pub sysenter_eip: u64,
    pub debugctl: u64,
    pub pat: u64,
    pub efer: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct HostState {
    pub es_selector: u16,
    pub cs_selector: u16,
    pub ss_selector: u16,
    pub ds_selector: u16,
    pub fs_selector: u16,
    pub gs_selector: u16,
    pub tr_selector: u16,
    pub cr0: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub fs_base: u64,
    pub gs_base: u64,
    pub tr_base: u64,
    pub gdtr_base: u64,
    pub idtr_base: u64,
    pub sysenter_cs: u32,
    pub sysenter_esp: u64,
    pub sysenter_eip: u64,
    pub pat: u64,
    pub efer: u64,
    pub rsp: u64,
    pub rip: u64,
}

impl GuestState {
    pub fn ia32e_mode(&self) -> bool {
        self.efer & (1 << 10) != 0
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.cr4 & (1 << 18) != 0 {
            return Err(
                "guest OSXSAVE must be disabled until XSAVE state switching is implemented",
            );
        }
        if self.rflags & 2 == 0 {
            return Err("guest RFLAGS bit 1 must be set");
        }
        if self.activity_state > 3 {
            return Err("guest activity state is invalid");
        }
        if self.interruptibility & !0x1f != 0 {
            return Err("guest interruptibility state contains reserved bits");
        }
        if self.tr.access_rights & (1 << 16) != 0 {
            return Err("guest task register must be usable");
        }
        if self.efer & (1 << 10) != 0 && self.efer & (1 << 8) == 0 {
            return Err("guest EFER.LMA requires EFER.LME");
        }
        if self.ia32e_mode()
            && (self.cr0 & (1 << 31) == 0
                || self.cr4 & (1 << 5) == 0
                || self.cs.access_rights & (1 << 13) == 0
                || self.cs.access_rights & (1 << 14) != 0)
        {
            return Err("IA-32e guest state requires paging, PAE, and a long code segment");
        }
        for segment in [
            self.es, self.cs, self.ss, self.ds, self.fs, self.gs, self.ldtr, self.tr,
        ] {
            if segment.access_rights & 0xfffe_0f00 != 0 {
                return Err("guest segment access rights contain reserved bits");
            }
        }
        Ok(())
    }

    pub fn write_to_vmcs(&self, vmcs: &mut Vmcs<'_>) -> Result<(), &'static str> {
        self.validate()?;
        for (segment, selector, base, limit, ar) in [
            (
                self.es,
                field::GUEST_ES_SELECTOR,
                field::GUEST_ES_BASE,
                field::GUEST_ES_LIMIT,
                field::GUEST_ES_AR,
            ),
            (
                self.cs,
                field::GUEST_CS_SELECTOR,
                field::GUEST_CS_BASE,
                field::GUEST_CS_LIMIT,
                field::GUEST_CS_AR,
            ),
            (
                self.ss,
                field::GUEST_SS_SELECTOR,
                field::GUEST_SS_BASE,
                field::GUEST_SS_LIMIT,
                field::GUEST_SS_AR,
            ),
            (
                self.ds,
                field::GUEST_DS_SELECTOR,
                field::GUEST_DS_BASE,
                field::GUEST_DS_LIMIT,
                field::GUEST_DS_AR,
            ),
            (
                self.fs,
                field::GUEST_FS_SELECTOR,
                field::GUEST_FS_BASE,
                field::GUEST_FS_LIMIT,
                field::GUEST_FS_AR,
            ),
            (
                self.gs,
                field::GUEST_GS_SELECTOR,
                field::GUEST_GS_BASE,
                field::GUEST_GS_LIMIT,
                field::GUEST_GS_AR,
            ),
            (
                self.ldtr,
                field::GUEST_LDTR_SELECTOR,
                field::GUEST_LDTR_BASE,
                field::GUEST_LDTR_LIMIT,
                field::GUEST_LDTR_AR,
            ),
            (
                self.tr,
                field::GUEST_TR_SELECTOR,
                field::GUEST_TR_BASE,
                field::GUEST_TR_LIMIT,
                field::GUEST_TR_AR,
            ),
        ] {
            write(vmcs, selector, segment.selector as u64)?;
            write(vmcs, base, segment.base)?;
            write(vmcs, limit, segment.limit as u64)?;
            write(vmcs, ar, segment.access_rights as u64)?;
        }
        for (encoding, value) in [
            (field::GUEST_GDTR_BASE, self.gdtr_base),
            (field::GUEST_IDTR_BASE, self.idtr_base),
            (field::GUEST_CR0, self.cr0),
            (field::GUEST_CR3, self.cr3),
            (field::GUEST_CR4, self.cr4),
            (field::GUEST_DR7, self.dr7),
            (field::GUEST_RSP, self.rsp),
            (field::GUEST_RIP, self.rip),
            (field::GUEST_RFLAGS, self.rflags),
            (field::GUEST_PENDING_DEBUG, self.pending_debug),
            (field::GUEST_SYSENTER_ESP, self.sysenter_esp),
            (field::GUEST_SYSENTER_EIP, self.sysenter_eip),
            (field::GUEST_DEBUGCTL, self.debugctl),
            (field::GUEST_PAT, self.pat),
            (field::GUEST_EFER, self.efer),
            (field::GUEST_GDTR_LIMIT, self.gdtr_limit as u64),
            (field::GUEST_IDTR_LIMIT, self.idtr_limit as u64),
            (field::GUEST_INTERRUPTIBILITY, self.interruptibility as u64),
            (field::GUEST_ACTIVITY_STATE, self.activity_state as u64),
            (field::GUEST_SYSENTER_CS, self.sysenter_cs as u64),
        ] {
            write(vmcs, encoding, value)?;
        }
        Ok(())
    }
}

impl HostState {
    /// Capture the current processor's host state for VM-exit loading.
    ///
    /// # Safety
    /// Must run at CPL 0 with valid current descriptor tables. `exit_stack` and
    /// `exit_handler` must be canonical addresses, and the handler must be a
    /// VM-exit trampoline that saves guest GPRs before calling Rust code.
    pub unsafe fn capture_current(
        exit_stack: u64,
        exit_handler: u64,
    ) -> Result<Self, &'static str> {
        #[repr(C, packed)]
        struct DescriptorTableRegister {
            limit: u16,
            base: u64,
        }

        let mut gdtr = DescriptorTableRegister { limit: 0, base: 0 };
        let mut idtr = DescriptorTableRegister { limit: 0, base: 0 };
        let es: u16;
        let cs: u16;
        let ss: u16;
        let ds: u16;
        let fs: u16;
        let gs: u16;
        let tr: u16;
        unsafe {
            core::arch::asm!("sgdt [{}]", in(reg) &mut gdtr, options(nostack, preserves_flags));
            core::arch::asm!("sidt [{}]", in(reg) &mut idtr, options(nostack, preserves_flags));
            core::arch::asm!("mov {0:x}, es", out(reg) es, options(nomem, nostack, preserves_flags));
            core::arch::asm!("mov {0:x}, cs", out(reg) cs, options(nomem, nostack, preserves_flags));
            core::arch::asm!("mov {0:x}, ss", out(reg) ss, options(nomem, nostack, preserves_flags));
            core::arch::asm!("mov {0:x}, ds", out(reg) ds, options(nomem, nostack, preserves_flags));
            core::arch::asm!("mov {0:x}, fs", out(reg) fs, options(nomem, nostack, preserves_flags));
            core::arch::asm!("mov {0:x}, gs", out(reg) gs, options(nomem, nostack, preserves_flags));
            core::arch::asm!("str {0:x}", out(reg) tr, options(nomem, nostack, preserves_flags));
        }

        let gdtr_base = unsafe { core::ptr::addr_of!(gdtr.base).read_unaligned() };
        let gdtr_limit = unsafe { core::ptr::addr_of!(gdtr.limit).read_unaligned() };
        let idtr_base = unsafe { core::ptr::addr_of!(idtr.base).read_unaligned() };
        let idtr_limit = unsafe { core::ptr::addr_of!(idtr.limit).read_unaligned() };
        let tr_base = unsafe { read_tss_base(gdtr_base, gdtr_limit, tr)? };
        let cr0: u64;
        let cr3: u64;
        let cr4: u64;
        unsafe {
            core::arch::asm!("mov {}, cr0", out(reg) cr0, options(nomem, nostack, preserves_flags));
            core::arch::asm!("mov {}, cr3", out(reg) cr3, options(nomem, nostack, preserves_flags));
            core::arch::asm!("mov {}, cr4", out(reg) cr4, options(nomem, nostack, preserves_flags));
        }
        let captured = Self {
            es_selector: es,
            cs_selector: cs,
            ss_selector: ss,
            ds_selector: ds,
            fs_selector: fs,
            gs_selector: gs,
            tr_selector: tr,
            cr0,
            cr3,
            cr4,
            fs_base: unsafe { read_msr(0xc000_0100) },
            gs_base: unsafe { read_msr(0xc000_0101) },
            tr_base,
            gdtr_base,
            idtr_base,
            sysenter_cs: unsafe { read_msr(0x174) as u32 },
            sysenter_esp: unsafe { read_msr(0x175) },
            sysenter_eip: unsafe { read_msr(0x176) },
            pat: unsafe { read_msr(0x277) },
            efer: unsafe { read_msr(0xc000_0080) },
            rsp: exit_stack,
            rip: exit_handler,
        };
        captured.validate()?;
        Ok(captured)
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.cs_selector & 7 != 0 || self.tr_selector & 7 != 0 {
            return Err("host CS and TR selectors must reference GDT entries");
        }

        for address in [
            self.fs_base,
            self.gs_base,
            self.tr_base,
            self.gdtr_base,
            self.idtr_base,
            self.sysenter_esp,
            self.sysenter_eip,
            self.rsp,
            self.rip,
        ] {
            if !is_canonical(address) {
                return Err("host VMCS state contains a non-canonical address");
            }
        }
        if self.rsp == 0 || self.rip == 0 {
            return Err("host VM-exit stack and entry point must be non-zero");
        }
        if self.efer & (1 << 10) == 0 {
            return Err("host processor must be in IA-32e mode");
        }
        Ok(())
    }

    pub fn write_to_vmcs(&self, vmcs: &mut Vmcs<'_>) -> Result<(), &'static str> {
        self.validate()?;
        for (encoding, value) in [
            (field::HOST_ES_SELECTOR, self.es_selector & !7),
            (field::HOST_CS_SELECTOR, self.cs_selector & !7),
            (field::HOST_SS_SELECTOR, self.ss_selector & !7),
            (field::HOST_DS_SELECTOR, self.ds_selector & !7),
            (field::HOST_FS_SELECTOR, self.fs_selector & !7),
            (field::HOST_GS_SELECTOR, self.gs_selector & !7),
            (field::HOST_TR_SELECTOR, self.tr_selector & !7),
        ] {
            write(vmcs, encoding, value as u64)?;
        }
        for (encoding, value) in [
            (field::HOST_CR0, self.cr0),
            (field::HOST_CR3, self.cr3),
            (field::HOST_CR4, self.cr4),
            (field::HOST_PAT, self.pat),
            (field::HOST_EFER, self.efer),
            (field::HOST_FS_BASE, self.fs_base),
            (field::HOST_GS_BASE, self.gs_base),
            (field::HOST_TR_BASE, self.tr_base),
            (field::HOST_GDTR_BASE, self.gdtr_base),
            (field::HOST_IDTR_BASE, self.idtr_base),
            (field::HOST_SYSENTER_ESP, self.sysenter_esp),
            (field::HOST_SYSENTER_EIP, self.sysenter_eip),
            (field::HOST_RSP, self.rsp),
            (field::HOST_RIP, self.rip),
            (field::HOST_SYSENTER_CS, self.sysenter_cs as u64),
        ] {
            write(vmcs, encoding, value)?;
        }
        Ok(())
    }
}

unsafe fn read_tss_base(
    gdtr_base: u64,
    gdtr_limit: u16,
    selector: u16,
) -> Result<u64, &'static str> {
    let descriptor_offset = (selector & !7) as u64;
    if selector == 0
        || selector & 4 != 0
        || descriptor_offset
            .checked_add(15)
            .ok_or("TSS descriptor offset overflows")?
            > gdtr_limit as u64
    {
        return Err("host task register does not reference a full GDT TSS descriptor");
    }
    let descriptor_address = gdtr_base
        .checked_add(descriptor_offset)
        .ok_or("TSS descriptor address overflows")?;
    let descriptor = descriptor_address as *const u64;
    let low = unsafe { descriptor.read_unaligned() };
    let high = unsafe { descriptor.add(1).read_unaligned() } as u32;
    let access_type = (low >> 40) & 0xf;
    if access_type != 9 && access_type != 11 {
        return Err("host task register descriptor is not a TSS");
    }
    let base = ((low >> 16) & 0xffff)
        | ((low >> 32) & 0xff) << 16
        | ((low >> 56) & 0xff) << 24
        | ((high as u64) << 32);
    Ok(base)
}

unsafe fn read_msr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        core::arch::asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") low,
            out("edx") high,
            options(nomem, nostack, preserves_flags)
        );
    }
    ((high as u64) << 32) | low as u64
}

/// Populate mandatory VMCS control fields, enabling secondary controls and EPT.
pub fn configure_controls(
    vmcs: &mut Vmcs<'_>,
    primary_controls_msr: u64,
    pin_controls_msr: u64,
    exit_controls_msr: u64,
    entry_controls_msr: u64,
    secondary_controls_msr: u64,
    ept_pointer: u64,
    unrestricted_guest: bool,
    guest_ia32e: bool,
    preemption_timer: Option<u32>,
) -> Result<(), &'static str> {
    const ACTIVATE_SECONDARY: u32 = 1 << 31;
    const ENABLE_EPT: u32 = 1 << 1;
    const UNRESTRICTED_GUEST: u32 = 1 << 7;

    let primary_requested = ACTIVATE_SECONDARY | (1 << 24) | (1 << 7);
    let primary = adjust_control(primary_controls_msr, primary_requested)?;
    let secondary_requested = ENABLE_EPT
        | if unrestricted_guest {
            UNRESTRICTED_GUEST
        } else {
            0
        };
    let secondary = adjust_control(secondary_controls_msr, secondary_requested)?;
    if secondary & ENABLE_EPT == 0 {
        return Err("processor does not permit EPT in secondary execution controls");
    }
    if ept_pointer & 0x7 != 6
        || ((ept_pointer >> 3) & 0x7) != 3
        || ept_pointer & 0x0000_0000_0000_0f80 != 0
    {
        return Err("EPT pointer must specify write-back memory and a four-level walk");
    }
    let pin_requested = if preemption_timer.is_some() {
        1 << 6
    } else {
        0
    };
    let pin = adjust_control(pin_controls_msr, pin_requested)?;
    if pin & (1 << 6) != 0 && preemption_timer.is_none() {
        return Err("VMX preemption timer is required by processor pin-based controls");
    }
    let exit = adjust_control(exit_controls_msr, (1 << 9) | (1 << 19) | (1 << 21))?;
    let entry_requested = (1 << 14) | (1 << 15) | if guest_ia32e { 1 << 9 } else { 0 };
    let entry = adjust_control(entry_controls_msr, entry_requested)?;

    write(vmcs, field::PIN_BASED_CONTROLS, pin as u64)?;
    write(vmcs, field::PRIMARY_PROC_CONTROLS, primary as u64)?;
    write(vmcs, field::SECONDARY_PROC_CONTROLS, secondary as u64)?;
    write(vmcs, field::VM_EXIT_CONTROLS, exit as u64)?;
    write(vmcs, field::VM_ENTRY_CONTROLS, entry as u64)?;
    write(vmcs, field::EPT_POINTER, ept_pointer)?;
    write(vmcs, field::GUEST_LINK_POINTER, u64::MAX)?;
    if let Some(timer) = preemption_timer {
        write(vmcs, field::VMX_PREEMPTION_TIMER, timer as u64)?;
    }
    Ok(())
}

/// Allocate a current VMCS and populate controls plus both architectural states.
///
/// This prepares hardware state. The synchronous VMX trampoline and
/// preemption-timer scheduler enter the resulting VMCS separately.
pub fn prepare_vmcs<'session>(
    session: &'session VmxHostSession,
    ept_pointer: u64,
    unrestricted_guest: bool,
    guest: &GuestState,
    host: &HostState,
) -> Result<Vmcs<'session>, &'static str> {
    if !guest.ia32e_mode() && !unrestricted_guest {
        return Err("real-mode guests require processor unrestricted-guest support");
    }
    let (cr0_fixed0, cr0_fixed1, cr4_fixed0, cr4_fixed1) = session.guest_control_register_masks();
    let unrestricted_cr0_mask = if unrestricted_guest {
        cr0_fixed0 & !((1 << 0) | (1 << 31))
    } else {
        cr0_fixed0
    };
    if guest.cr0 & unrestricted_cr0_mask != unrestricted_cr0_mask || guest.cr0 & !cr0_fixed1 != 0 {
        return Err("guest CR0 violates the processor VMX fixed-bit requirements");
    }
    if guest.cr4 & cr4_fixed0 != cr4_fixed0 || guest.cr4 & !cr4_fixed1 != 0 {
        return Err("guest CR4 violates the processor VMX fixed-bit requirements");
    }
    let (primary, pin, exit, entry, secondary) = session.control_capability_msrs();
    let preemption_timer = Some(
        session
            .preemption_timer_value()
            .ok_or("reliable VMX preemption timer is required for scheduled guest entry")?,
    );
    let mut vmcs = session.create_vmcs()?;
    configure_controls(
        &mut vmcs,
        primary,
        pin,
        exit,
        entry,
        secondary,
        ept_pointer,
        unrestricted_guest,
        guest.ia32e_mode(),
        preemption_timer,
    )?;
    vmcs.set_preemption_timer(preemption_timer);
    guest.write_to_vmcs(&mut vmcs)?;
    host.write_to_vmcs(&mut vmcs)?;
    Ok(vmcs)
}

fn write(vmcs: &mut Vmcs<'_>, encoding: u32, value: u64) -> Result<(), &'static str> {
    vmcs.write_field(encoding, value)
}

fn adjust_control(capability_msr: u64, requested: u32) -> Result<u32, &'static str> {
    let must_be_one = capability_msr as u32;
    let may_be_one = (capability_msr >> 32) as u32;
    if must_be_one & !may_be_one != 0 || requested & !may_be_one != 0 {
        return Err("requested VMX control is not supported by processor");
    }
    Ok((requested | must_be_one) & may_be_one)
}

fn is_canonical(address: u64) -> bool {
    let upper = address >> 48;
    let sign = (address >> 47) & 1;
    upper == if sign == 0 { 0 } else { 0xffff }
}

/// Current VM-exit reason from VMCS field EXIT_REASON.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmExitReason {
    ExceptionOrNmi,
    ExternalInterrupt,
    TripleFault,
    Cpuid,
    Hlt,
    PreemptionTimer,
    IoInstruction,
    EptViolation,
    EptMisconfiguration,
    Other(u16),
}

impl VmExitReason {
    pub fn from_vmcs(raw: u32) -> Self {
        match (raw & 0xffff) as u16 {
            0 => Self::ExceptionOrNmi,
            1 => Self::ExternalInterrupt,
            2 => Self::TripleFault,
            10 => Self::Cpuid,
            12 => Self::Hlt,
            30 => Self::IoInstruction,
            48 => Self::EptViolation,
            49 => Self::EptMisconfiguration,
            52 => Self::PreemptionTimer,
            other => Self::Other(other),
        }
    }
}

/// I/O qualification decoded from a VM-exit qualification field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoExitQualification {
    pub width: u8,
    pub input: bool,
    pub string: bool,
    pub repeat: bool,
    pub port: u16,
}

impl IoExitQualification {
    pub fn decode(value: u64) -> Result<Self, &'static str> {
        let width = match value & 7 {
            0 => 1,
            1 => 2,
            3 => 4,
            _ => return Err("unsupported VM-exit I/O operand width"),
        };
        Ok(Self {
            width,
            input: value & (1 << 3) != 0,
            string: value & (1 << 4) != 0,
            repeat: value & (1 << 5) != 0,
            port: ((value >> 16) & 0xffff) as u16,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vmx_controls_respect_must_be_one_and_allowed_masks() {
        let caps = (0b0010u64 << 32) | 0b0001;
        assert_eq!(adjust_control(caps, 0b0010), Ok(0b0011));
        assert_eq!(
            adjust_control(caps, 0b0100),
            Err("requested VMX control is not supported by processor")
        );
        assert_eq!(
            adjust_control(0b0010, 0),
            Err("requested VMX control is not supported by processor")
        );
    }

    #[test]
    fn primary_controls_require_dispatchable_guest_exits() {
        let required = (1 << 31) | (1 << 24) | (1 << 7);
        let capability = (required as u64) << 32;
        assert_eq!(adjust_control(capability, required), Ok(required));
        assert_eq!(
            adjust_control(capability & !((1u64 << 32) << 24), required),
            Err("requested VMX control is not supported by processor")
        );
    }

    #[test]
    fn guest_and_host_state_reject_invalid_architectural_values() {
        let mut guest = GuestState {
            es: SegmentState::null(),
            cs: SegmentState::null(),
            ss: SegmentState::null(),
            ds: SegmentState::null(),
            fs: SegmentState::null(),
            gs: SegmentState::null(),
            ldtr: SegmentState::null(),
            tr: SegmentState {
                access_rights: 0x8b,
                ..SegmentState::null()
            },
            gdtr_base: 0,
            gdtr_limit: 0,
            idtr_base: 0,
            idtr_limit: 0,
            cr0: 0,
            cr3: 0,
            cr4: 0,
            dr7: 0x400,
            rsp: 0,
            rip: 0,
            rflags: 2,
            pending_debug: 0,
            interruptibility: 0,
            activity_state: 0,
            sysenter_cs: 0,
            sysenter_esp: 0,
            sysenter_eip: 0,
            debugctl: 0,
            pat: 0x0007_0406_0007_0406,
            efer: 0,
        };
        assert_eq!(guest.validate(), Ok(()));
        guest.efer = (1 << 10) | (1 << 8);
        assert_eq!(
            guest.validate(),
            Err("IA-32e guest state requires paging, PAE, and a long code segment")
        );
        guest.efer = 0;
        guest.cr4 = 1 << 18;
        assert_eq!(
            guest.validate(),
            Err("guest OSXSAVE must be disabled until XSAVE state switching is implemented")
        );
        assert_eq!(
            HostState {
                es_selector: 0,
                cs_selector: 3,
                ss_selector: 0,
                ds_selector: 0,
                fs_selector: 0,
                gs_selector: 0,
                tr_selector: 0,
                cr0: 0,
                cr3: 0,
                cr4: 0,
                pat: 0,
                efer: 1 << 10,
                fs_base: 0,
                gs_base: 0,
                tr_base: 0,
                gdtr_base: 0,
                idtr_base: 0,
                sysenter_cs: 0,
                sysenter_esp: 0,
                sysenter_eip: 0,
                rsp: 1,
                rip: 1,
            }
            .validate(),
            Err("host CS and TR selectors must reference GDT entries")
        );
    }

    #[test]
    fn exit_reason_and_io_qualification_are_decoded() {
        assert_eq!(VmExitReason::from_vmcs(30), VmExitReason::IoInstruction);
        assert_eq!(
            VmExitReason::from_vmcs((1 << 31) | 30),
            VmExitReason::IoInstruction
        );
        let io = IoExitQualification::decode((0x3f8u64 << 16) | 1 | (1 << 3)).unwrap();
        assert_eq!(io.width, 2);
        assert!(io.input);
        assert_eq!(io.port, 0x3f8);
    }
}
