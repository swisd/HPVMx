//! Intel VT-x capability discovery and host-mode primitives.

use bitflags::bitflags;
use core::marker::PhantomData;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, Ordering};
use raw_cpuid::CpuId;
use uefi::boot;

const IA32_FEATURE_CONTROL: u32 = 0x3A;
const IA32_VMX_BASIC: u32 = 0x480;
const IA32_VMX_PINBASED_CTLS: u32 = 0x481;
const IA32_VMX_PROCBASED_CTLS: u32 = 0x482;
const IA32_VMX_EXIT_CTLS: u32 = 0x483;
const IA32_VMX_ENTRY_CTLS: u32 = 0x484;
const IA32_VMX_EPT_VPID_CAP: u32 = 0x48C;
const IA32_VMX_PROCBASED_CTLS2: u32 = 0x48B;
const IA32_VMX_CR0_FIXED0: u32 = 0x486;
const IA32_VMX_CR0_FIXED1: u32 = 0x487;
const IA32_VMX_CR4_FIXED0: u32 = 0x488;
const IA32_VMX_CR4_FIXED1: u32 = 0x489;
const FEATURE_CONTROL_LOCK: u64 = 1 << 0;
const FEATURE_CONTROL_VMX_OUTSIDE_SMX: u64 = 1 << 2;
const CR4_VMXE: u64 = 1 << 13;
const VMX_REGION_BYTES: usize = 4096;
const VMX_REGION_MAX_ADDRESS: u64 = 0xFFFF_FFFF;
const VM_EXIT_STACK_PAGES: usize = 16;
const MAX_PINNED_CPUS: usize = 256;
pub const EXIT_RESUME: u64 = 0;
pub const EXIT_HALT: u64 = 1;
pub const EXIT_FATAL: u64 = 2;
pub const EXIT_YIELD: u64 = 4;
const ENTRY_FAILED: u64 = 3;

static PINNED_CPUS: [AtomicBool; MAX_PINNED_CPUS] =
    [const { AtomicBool::new(false) }; MAX_PINNED_CPUS];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct GuestRegisters {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

#[repr(C, align(16))]
#[derive(Clone, Debug)]
pub struct VmxGuestFpuState {
    bytes: [u8; 512],
}

impl VmxGuestFpuState {
    pub fn new() -> Self {
        let mut host = FxSaveArea([0; 512]);
        let mut guest = FxSaveArea([0; 512]);
        let default_mxcsr = 0x1f80u32;
        unsafe {
            core::arch::asm!(
                "fxsave [{host}]",
                host = in(reg) host.0.as_mut_ptr(),
                options(nostack)
            );
            core::arch::asm!(
                "fninit",
                "ldmxcsr [{mxcsr}]",
                "fxsave [{guest}]",
                "fxrstor [{host}]",
                mxcsr = in(reg) &default_mxcsr,
                guest = in(reg) guest.0.as_mut_ptr(),
                host = in(reg) host.0.as_ptr(),
                options(nostack)
            );
        }
        Self { bytes: guest.0 }
    }
}

impl Default for VmxGuestFpuState {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C, align(16))]
struct FxSaveArea([u8; 512]);

pub type VmExitHandler =
    unsafe extern "efiapi" fn(*mut VmxRunContext, *mut core::ffi::c_void) -> u64;

#[repr(C, align(16))]
pub struct VmxRunContext {
    pub registers: GuestRegisters,
    handler: VmExitHandler,
    opaque: *mut core::ffi::c_void,
    host_rsp: u64,
    host_rbx: u64,
    host_rbp: u64,
    host_rsi: u64,
    host_rdi: u64,
    host_r12: u64,
    host_r13: u64,
    host_r14: u64,
    host_r15: u64,
    outcome: u64,
    exit_stack_top: u64,
    resume: u64,
    host_fpu: FxSaveArea,
    guest_fpu: FxSaveArea,
}

const _: () = {
    assert!(core::mem::offset_of!(VmxRunContext, handler) == 120);
    assert!(core::mem::offset_of!(VmxRunContext, opaque) == 128);
    assert!(core::mem::offset_of!(VmxRunContext, host_rsp) == 136);
    assert!(core::mem::offset_of!(VmxRunContext, outcome) == 208);
    assert!(core::mem::offset_of!(VmxRunContext, exit_stack_top) == 216);
    assert!(core::mem::offset_of!(VmxRunContext, resume) == 224);
    assert!(core::mem::offset_of!(VmxRunContext, host_fpu) == 240);
    assert!(core::mem::offset_of!(VmxRunContext, guest_fpu) == 752);
};

impl VmxRunContext {
    pub fn new(
        registers: GuestRegisters,
        guest_fpu: &VmxGuestFpuState,
        handler: VmExitHandler,
        opaque: *mut core::ffi::c_void,
    ) -> Self {
        Self {
            registers,
            handler,
            opaque,
            host_rsp: 0,
            host_rbx: 0,
            host_rbp: 0,
            host_rsi: 0,
            host_rdi: 0,
            host_r12: 0,
            host_r13: 0,
            host_r14: 0,
            host_r15: 0,
            outcome: ENTRY_FAILED,
            exit_stack_top: 0,
            resume: 0,
            host_fpu: FxSaveArea([0; 512]),
            guest_fpu: FxSaveArea(guest_fpu.bytes),
        }
    }

    pub fn save_guest_fpu(&self, state: &mut VmxGuestFpuState) {
        state.bytes.copy_from_slice(&self.guest_fpu.0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmxRunOutcome {
    Yielded,
    Halted,
    Fatal,
    EntryFailed,
}

/// Owns the VM-exit stack for one synchronous run on one pinned processor.
pub struct VmxExitStack {
    address: NonNull<u8>,
}

impl VmxExitStack {
    pub fn allocate() -> Result<Self, &'static str> {
        let address = boot::allocate_pages(
            boot::AllocateType::AnyPages,
            boot::MemoryType::LOADER_DATA,
            VM_EXIT_STACK_PAGES,
        )
        .map_err(|_| "UEFI could not allocate the VM-exit stack")?;
        unsafe {
            core::ptr::write_bytes(address.as_ptr(), 0, VM_EXIT_STACK_PAGES * VMX_REGION_BYTES);
        }
        Ok(Self { address })
    }

    pub fn host_stack_pointer(&self) -> u64 {
        self.address.as_ptr() as u64 + (VM_EXIT_STACK_PAGES * VMX_REGION_BYTES) as u64 - 24
    }

    pub fn vm_exit_trampoline_address() -> u64 {
        hpvmx_vmxexit_trampoline as *const () as u64
    }

    fn install_context(&self, context: &mut VmxRunContext) {
        let top = self.host_stack_pointer() + 8;
        unsafe {
            (top as *mut u64).write(context as *mut VmxRunContext as u64);
        }
        context.exit_stack_top = top;
    }
}

impl Drop for VmxExitStack {
    fn drop(&mut self) {
        if let Err(error) = unsafe { boot::free_pages(self.address, VM_EXIT_STACK_PAGES) } {
            log::error!("failed to release VM-exit stack: {}", error);
        }
    }
}

/// Prevents concurrent VMX use on a logical processor and verifies the caller
/// remains on that processor for the full synchronous run.
pub struct VmxCpuPin {
    processor: usize,
    _not_send: PhantomData<*mut ()>,
}

impl VmxCpuPin {
    pub fn acquire() -> Result<Self, &'static str> {
        let processor = crate::hardware::cpu::mp::MpManager::who_am_i()
            .ok_or("UEFI MP Services WhoAmI is required to pin VMX execution")?;
        let slot = PINNED_CPUS
            .get(processor)
            .ok_or("logical processor index exceeds the VMX pin table")?;
        slot.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .map_err(|_| "logical processor already owns a VMX execution context")?;
        Ok(Self {
            processor,
            _not_send: PhantomData,
        })
    }

    pub fn verify_current(&self) -> Result<(), &'static str> {
        if crate::hardware::cpu::mp::MpManager::who_am_i() == Some(self.processor) {
            Ok(())
        } else {
            Err("VMX execution migrated away from its pinned logical processor")
        }
    }

    pub fn processor_index(&self) -> usize {
        self.processor
    }
}

impl Drop for VmxCpuPin {
    fn drop(&mut self) {
        PINNED_CPUS[self.processor].store(false, Ordering::Release);
    }
}

bitflags! {
    /// VMCS execution control bits.
    pub struct VmcsControl: u32 {
        const INTERRUPT_EXIT = 1 << 0;
        const NMI_EXITING = 1 << 3;
        const VIRTUAL_NMI = 1 << 5;
        const ACTIVATE_PREEMPTION_TIMER = 1 << 6;
        const PROCESS_POSTED_INTERRUPTS = 1 << 7;
    }
}

bitflags! {
    /// VMCS entry control bits.
    pub struct VmcsEntryControl: u32 {
        const LOAD_DEBUG = 1 << 2;
        const IA32E_MODE = 1 << 9;
        const ENTRY_TO_SMM = 1 << 10;
        const DEACTIVATE_DUAL_MONITOR = 1 << 11;
        const LOAD_IA32_PERF = 1 << 13;
        const LOAD_IA32_PAT = 1 << 14;
        const LOAD_IA32_EFER = 1 << 15;
    }
}

core::arch::global_asm!(
    r#"
    .text
    .globl hpvmx_vmx_enter
hpvmx_vmx_enter:
    fxsave [rcx + 240]
    cmp qword ptr [rcx + 224], 0
    jne hpvmx_vmx_resume
    mov qword ptr [rcx + 136], rsp
    mov qword ptr [rcx + 144], rbx
    mov qword ptr [rcx + 152], rbp
    mov qword ptr [rcx + 160], rsi
    mov qword ptr [rcx + 168], rdi
    mov qword ptr [rcx + 176], r12
    mov qword ptr [rcx + 184], r13
    mov qword ptr [rcx + 192], r14
    mov qword ptr [rcx + 200], r15
    push rcx
    mov rax, rcx
    mov rbx, qword ptr [rax + 8]
    mov rcx, qword ptr [rax + 16]
    mov rdx, qword ptr [rax + 24]
    mov rsi, qword ptr [rax + 32]
    mov rdi, qword ptr [rax + 40]
    mov rbp, qword ptr [rax + 48]
    mov r8, qword ptr [rax + 56]
    mov r9, qword ptr [rax + 64]
    mov r10, qword ptr [rax + 72]
    mov r11, qword ptr [rax + 80]
    mov r12, qword ptr [rax + 88]
    mov r13, qword ptr [rax + 96]
    mov r14, qword ptr [rax + 104]
    mov r15, qword ptr [rax + 112]
    fxrstor [rax + 752]
    mov rax, qword ptr [rax]
    vmlaunch
    jmp hpvmx_vmx_entry_failed
hpvmx_vmx_resume:
    mov qword ptr [rcx + 136], rsp
    mov qword ptr [rcx + 144], rbx
    mov qword ptr [rcx + 152], rbp
    mov qword ptr [rcx + 160], rsi
    mov qword ptr [rcx + 168], rdi
    mov qword ptr [rcx + 176], r12
    mov qword ptr [rcx + 184], r13
    mov qword ptr [rcx + 192], r14
    mov qword ptr [rcx + 200], r15
    push rcx
    mov rax, rcx
    mov rbx, qword ptr [rax + 8]
    mov rcx, qword ptr [rax + 16]
    mov rdx, qword ptr [rax + 24]
    mov rsi, qword ptr [rax + 32]
    mov rdi, qword ptr [rax + 40]
    mov rbp, qword ptr [rax + 48]
    mov r8, qword ptr [rax + 56]
    mov r9, qword ptr [rax + 64]
    mov r10, qword ptr [rax + 72]
    mov r11, qword ptr [rax + 80]
    mov r12, qword ptr [rax + 88]
    mov r13, qword ptr [rax + 96]
    mov r14, qword ptr [rax + 104]
    mov r15, qword ptr [rax + 112]
    fxrstor [rax + 752]
    mov rax, qword ptr [rax]
    vmresume
hpvmx_vmx_entry_failed:
    setc r10b
    setz r11b
    pop rcx
    mov qword ptr [rcx + 208], 3
    jmp hpvmx_vmx_restore_host

    .globl hpvmx_vmxexit_trampoline
hpvmx_vmxexit_trampoline:
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15
    mov rax, qword ptr [rsp + 128]
    mov r10, qword ptr [rsp + 112]
    mov qword ptr [rax], r10
    mov r10, qword ptr [rsp + 104]
    mov qword ptr [rax + 8], r10
    mov r10, qword ptr [rsp + 96]
    mov qword ptr [rax + 16], r10
    mov r10, qword ptr [rsp + 88]
    mov qword ptr [rax + 24], r10
    mov r10, qword ptr [rsp + 80]
    mov qword ptr [rax + 32], r10
    mov r10, qword ptr [rsp + 72]
    mov qword ptr [rax + 40], r10
    mov r10, qword ptr [rsp + 64]
    mov qword ptr [rax + 48], r10
    mov r10, qword ptr [rsp + 56]
    mov qword ptr [rax + 56], r10
    mov r10, qword ptr [rsp + 48]
    mov qword ptr [rax + 64], r10
    mov r10, qword ptr [rsp + 40]
    mov qword ptr [rax + 72], r10
    mov r10, qword ptr [rsp + 32]
    mov qword ptr [rax + 80], r10
    mov r10, qword ptr [rsp + 24]
    mov qword ptr [rax + 88], r10
    mov r10, qword ptr [rsp + 16]
    mov qword ptr [rax + 96], r10
    mov r10, qword ptr [rsp + 8]
    mov qword ptr [rax + 104], r10
    mov r10, qword ptr [rsp]
    mov qword ptr [rax + 112], r10
    fxsave [rax + 752]
    fxrstor [rax + 240]
    mov rcx, rax
    mov rdx, qword ptr [rax + 128]
    mov rax, qword ptr [rax + 120]
    sub rsp, 32
    call rax
    add rsp, 32
    mov rcx, qword ptr [rsp + 128]
    mov qword ptr [rcx + 208], rax
    cmp rax, 0
    jne hpvmx_vmxexit_leave
    add rsp, 120
    mov rax, qword ptr [rsp + 8]
    fxrstor [rax + 752]
    mov rbx, qword ptr [rax + 8]
    mov rcx, qword ptr [rax + 16]
    mov rdx, qword ptr [rax + 24]
    mov rsi, qword ptr [rax + 32]
    mov rdi, qword ptr [rax + 40]
    mov rbp, qword ptr [rax + 48]
    mov r8, qword ptr [rax + 56]
    mov r9, qword ptr [rax + 64]
    mov r10, qword ptr [rax + 72]
    mov r11, qword ptr [rax + 80]
    mov r12, qword ptr [rax + 88]
    mov r13, qword ptr [rax + 96]
    mov r14, qword ptr [rax + 104]
    mov r15, qword ptr [rax + 112]
    mov rax, qword ptr [rax]
    vmresume
    mov rcx, qword ptr [rsp + 8]
    mov qword ptr [rcx + 208], 3
    jmp hpvmx_vmx_restore_host

hpvmx_vmxexit_leave:
    add rsp, 120
    mov rcx, qword ptr [rsp + 8]
hpvmx_vmx_restore_host:
    fxrstor [rcx + 240]
    mov rbx, qword ptr [rcx + 144]
    mov rbp, qword ptr [rcx + 152]
    mov rsi, qword ptr [rcx + 160]
    mov rdi, qword ptr [rcx + 168]
    mov r12, qword ptr [rcx + 176]
    mov r13, qword ptr [rcx + 184]
    mov r14, qword ptr [rcx + 192]
    mov r15, qword ptr [rcx + 200]
    mov rsp, qword ptr [rcx + 136]
    mov rax, qword ptr [rcx + 208]
    ret
"#
);

unsafe extern "efiapi" {
    fn hpvmx_vmx_enter(context: *mut VmxRunContext) -> u64;
    fn hpvmx_vmxexit_trampoline();
}

/// VT-x capabilities exposed by the CPU and permitted by firmware.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VtxCapabilities {
    /// VMX is present and firmware has enabled VMX operation outside SMX.
    pub available: bool,
    /// The CPU implements VMX instructions (regardless of firmware policy).
    pub vmxon_supported: bool,
    pub ept_supported: bool,
    pub ept_write_back: bool,
    pub ept_four_level_walk: bool,
    pub vpid_supported: bool,
    pub unrestricted_guest: bool,
}

impl VtxCapabilities {
    /// Detect VT-x support without changing processor or firmware state.
    ///
    /// # Safety
    /// Must be called at CPL 0 because capability discovery reads privileged MSRs.
    pub unsafe fn detect() -> Self {
        let cpuid = CpuId::new();
        let vmx_supported = cpuid
            .get_feature_info()
            .map(|info| info.has_vmx())
            .unwrap_or(false);
        if !vmx_supported {
            return Self::from_registers(false, 0, 0, 0);
        }

        // These architectural capability MSRs are present whenever CPUID.VMX is set.
        let feature_control = unsafe { read_msr(IA32_FEATURE_CONTROL) };
        let vmx_basic = unsafe { read_msr(IA32_VMX_BASIC) };
        let secondary_controls_available = vmx_basic & (1 << 55) != 0;
        let secondary_controls = if secondary_controls_available {
            unsafe { read_msr(IA32_VMX_PROCBASED_CTLS2) }
        } else {
            0
        };
        let ept_vpid_capabilities = if secondary_controls_available {
            unsafe { read_msr(IA32_VMX_EPT_VPID_CAP) }
        } else {
            0
        };
        Self::from_registers(
            vmx_supported,
            feature_control,
            secondary_controls,
            ept_vpid_capabilities,
        )
    }

    fn from_registers(
        vmx_supported: bool,
        feature_control: u64,
        secondary_controls: u64,
        ept_vpid_capabilities: u64,
    ) -> Self {
        let firmware_enabled = feature_control & FEATURE_CONTROL_LOCK != 0
            && feature_control & FEATURE_CONTROL_VMX_OUTSIDE_SMX != 0;
        let secondary_allowed = (secondary_controls >> 32) as u32;
        Self {
            available: vmx_supported && firmware_enabled,
            vmxon_supported: vmx_supported,
            ept_supported: vmx_supported
                && secondary_allowed & (1 << 1) != 0
                && ept_vpid_capabilities & (1 << 6) != 0
                && ept_vpid_capabilities & (1 << 14) != 0,
            ept_write_back: vmx_supported && ept_vpid_capabilities & (1 << 6) != 0,
            ept_four_level_walk: vmx_supported && ept_vpid_capabilities & (1 << 14) != 0,
            vpid_supported: vmx_supported && secondary_allowed & (1 << 5) != 0,
            unrestricted_guest: vmx_supported && secondary_allowed & (1 << 7) != 0,
        }
    }

    pub fn supports_unrestricted_guest(&self) -> bool {
        self.unrestricted_guest && self.ept_supported
    }

    pub fn supports_ept(&self) -> bool {
        self.ept_supported
    }
}

/// VMXON region. Hardware requires a page-aligned, write-back memory region.
#[repr(C, align(4096))]
pub struct VmxonRegion {
    pub revision_id: u32,
    pub data: [u8; 4092],
}

/// VMCS region. Hardware requires a page-aligned, write-back memory region.
#[repr(C, align(4096))]
pub struct VmcsRegion {
    pub revision_id: u32,
    pub data: [u8; 4092],
}

impl VmcsRegion {
    pub fn new(revision_id: u32) -> Self {
        Self {
            revision_id: revision_id & 0x7fff_ffff,
            data: [0; 4092],
        }
    }
}

struct VmxPage {
    address: NonNull<u8>,
}

impl VmxPage {
    fn allocate(revision_id: u32) -> Result<Self, &'static str> {
        let address = uefi::boot::allocate_pages(
            uefi::boot::AllocateType::MaxAddress(VMX_REGION_MAX_ADDRESS),
            uefi::boot::MemoryType::LOADER_DATA,
            1,
        )
        .map_err(|_| "could not allocate a low-memory VMX region")?;

        unsafe {
            core::ptr::write_bytes(address.as_ptr(), 0, VMX_REGION_BYTES);
            address
                .as_ptr()
                .cast::<u32>()
                .write(revision_id & 0x7fff_ffff);
        }
        Ok(Self { address })
    }

    fn physical_address(&self) -> u64 {
        self.address.as_ptr() as u64
    }
}

impl Drop for VmxPage {
    fn drop(&mut self) {
        if let Err(error) = unsafe { uefi::boot::free_pages(self.address, 1) } {
            log::error!("failed to release VMX region: {}", error);
        }
    }
}

/// An active VMX-root session on the current processor.
///
/// Dropping the session executes VMXOFF and restores the host CR0/CR4 values.
pub struct VmxHostSession {
    _vmxon_region: VmxPage,
    cpu_pin: VmxCpuPin,
    original_cr0: u64,
    original_cr4: u64,
    active: bool,
    _not_send: PhantomData<*mut ()>,
}

impl VmxHostSession {
    /// Enter VMX root operation on the current processor.
    ///
    /// # Safety
    /// The caller must run at CPL 0 on a processor that is not already in VMX
    /// operation, and must keep this session on that same processor until it is
    /// dropped. The caller must not migrate or context-switch this session, and
    /// UEFI Boot Services must remain available until the session is dropped.
    pub unsafe fn enter_on_current_cpu() -> Result<Self, &'static str> {
        let cpu_pin = VmxCpuPin::acquire()?;
        let cpuid = CpuId::new();
        if !cpuid
            .get_feature_info()
            .map(|info| info.has_vmx())
            .unwrap_or(false)
        {
            return Err("processor does not implement VT-x");
        }

        let feature_control = unsafe { read_msr(IA32_FEATURE_CONTROL) };
        if feature_control & FEATURE_CONTROL_LOCK == 0 {
            return Err(
                "firmware has not locked IA32_FEATURE_CONTROL; refusing to change firmware policy",
            );
        }
        if feature_control & FEATURE_CONTROL_VMX_OUTSIDE_SMX == 0 {
            return Err("firmware disabled VT-x outside SMX");
        }

        let vmx_basic = unsafe { read_msr(IA32_VMX_BASIC) };
        let region_size = ((vmx_basic >> 32) & 0x1fff) as usize;
        let memory_type = (vmx_basic >> 50) & 0xf;
        if region_size == 0 || region_size > VMX_REGION_BYTES {
            return Err("unsupported VMX region size");
        }
        if memory_type != 6 {
            return Err("VMX regions require write-back memory on this backend");
        }

        let revision_id = (vmx_basic as u32) & 0x7fff_ffff;
        let vmxon_region = VmxPage::allocate(revision_id)?;

        let original_cr0 = unsafe { read_cr0() };
        let original_cr4 = unsafe { read_cr4() };
        let fixed_cr0 = fixed_register_value(
            original_cr0,
            unsafe { read_msr(IA32_VMX_CR0_FIXED0) },
            unsafe { read_msr(IA32_VMX_CR0_FIXED1) },
        )?;
        let cr4_fixed0 = unsafe { read_msr(IA32_VMX_CR4_FIXED0) };
        let cr4_fixed1 = unsafe { read_msr(IA32_VMX_CR4_FIXED1) };
        if cr4_fixed1 & CR4_VMXE == 0 {
            return Err("CR4.VMXE is not permitted by the processor");
        }
        let fixed_cr4 = fixed_register_value(original_cr4, cr4_fixed0, cr4_fixed1)? | CR4_VMXE;
        unsafe {
            write_cr0(fixed_cr0);
            write_cr4(fixed_cr4);
        }

        let physical_address = vmxon_region.physical_address();
        let mut carry = 0u8;
        let mut zero = 0u8;
        unsafe {
            core::arch::asm!(
                "vmxon [{operand}]",
                "setc {carry}",
                "setz {zero}",
                operand = in(reg) &physical_address,
                carry = lateout(reg_byte) carry,
                zero = lateout(reg_byte) zero,
                options(nostack)
            );
        }
        if carry != 0 || zero != 0 {
            unsafe {
                write_cr4(original_cr4);
                write_cr0(original_cr0);
            }
            return Err("VMXON instruction failed");
        }

        Ok(Self {
            _vmxon_region: vmxon_region,
            cpu_pin,
            original_cr0,
            original_cr4,
            active: true,
            _not_send: PhantomData,
        })
    }

    /// Allocate and make a fresh VMCS current on this processor.
    ///
    /// The returned handle borrows the host session so its VMCS cannot outlive
    /// the VMX-root operation.
    pub fn create_vmcs(&self) -> Result<Vmcs<'_>, &'static str> {
        self.cpu_pin.verify_current()?;
        let vmx_basic = unsafe { read_msr(IA32_VMX_BASIC) };
        let revision_id = (vmx_basic as u32) & 0x7fff_ffff;
        let region = VmxPage::allocate(revision_id)?;
        let physical_address = region.physical_address();
        let mut carry = 0u8;
        let mut zero = 0u8;
        unsafe {
            core::arch::asm!(
                "vmclear [{operand}]",
                "setc {carry}",
                "setz {zero}",
                operand = in(reg) &physical_address,
                carry = lateout(reg_byte) carry,
                zero = lateout(reg_byte) zero,
                options(nostack)
            );
        }
        if carry != 0 || zero != 0 {
            return Err("VMCLEAR instruction failed");
        }

        unsafe {
            core::arch::asm!(
                "vmptrld [{operand}]",
                "setc {carry}",
                "setz {zero}",
                operand = in(reg) &physical_address,
                carry = lateout(reg_byte) carry,
                zero = lateout(reg_byte) zero,
                options(nostack)
            );
        }
        if carry != 0 || zero != 0 {
            return Err("VMPTRLD instruction failed");
        }

        Ok(Vmcs {
            _region: region,
            processor: self.cpu_pin.processor_index(),
            launched: false,
            preemption_timer: None,
            _session: PhantomData,
        })
    }

    /// Read the processor capability MSRs used to construct VMCS controls.
    pub fn control_capability_msrs(&self) -> (u64, u64, u64, u64, u64) {
        let basic = unsafe { read_msr(IA32_VMX_BASIC) };
        unsafe {
            let secondary = if basic & (1 << 55) != 0 {
                read_msr(IA32_VMX_PROCBASED_CTLS2)
            } else {
                0
            };
            (
                read_msr(IA32_VMX_PROCBASED_CTLS),
                read_msr(IA32_VMX_PINBASED_CTLS),
                read_msr(IA32_VMX_EXIT_CTLS),
                read_msr(IA32_VMX_ENTRY_CTLS),
                secondary,
            )
        }
    }

    pub fn preemption_timer_value(&self) -> Option<u32> {
        let pin_controls = unsafe { read_msr(IA32_VMX_PINBASED_CTLS) };
        let may_be_one = (pin_controls >> 32) as u32;
        if may_be_one & (1 << 6) == 0 {
            return None;
        }
        let frequency = CpuId::new().get_tsc_info()?.tsc_frequency()?;
        let rate = (unsafe { read_msr(IA32_VMX_BASIC) } & 0x1f) as u32;
        preemption_timer_ticks(frequency, rate)
    }

    pub fn guest_control_register_masks(&self) -> (u64, u64, u64, u64) {
        unsafe {
            (
                read_msr(IA32_VMX_CR0_FIXED0),
                read_msr(IA32_VMX_CR0_FIXED1),
                read_msr(IA32_VMX_CR4_FIXED0),
                read_msr(IA32_VMX_CR4_FIXED1),
            )
        }
    }
}

impl Drop for VmxHostSession {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        unsafe {
            core::arch::asm!("vmxoff", options(nostack));
            write_cr4(self.original_cr4);
            write_cr0(self.original_cr0);
        }
        self.active = false;
    }
}

/// A current, cleared VMCS. Guest and host state must be populated before launch.
pub struct Vmcs<'session> {
    _region: VmxPage,
    processor: usize,
    launched: bool,
    preemption_timer: Option<u32>,
    _session: PhantomData<&'session VmxHostSession>,
}

impl Vmcs<'_> {
    pub fn set_preemption_timer(&mut self, timer: Option<u32>) {
        self.preemption_timer = timer;
    }

    pub fn preemption_timer(&self) -> Option<u32> {
        self.preemption_timer
    }

    pub fn load_current(&mut self) -> Result<(), &'static str> {
        if crate::hardware::cpu::mp::MpManager::who_am_i() != Some(self.processor) {
            return Err("VMCS can only be loaded on its owning logical processor");
        }
        let physical_address = self._region.physical_address();
        let mut carry = 0u8;
        let mut zero = 0u8;
        unsafe {
            core::arch::asm!(
                "vmptrld [{operand}]",
                "setc {carry}",
                "setz {zero}",
                operand = in(reg) &physical_address,
                carry = lateout(reg_byte) carry,
                zero = lateout(reg_byte) zero,
                options(nostack)
            );
        }
        if carry != 0 || zero != 0 {
            return Err("VMPTRLD instruction failed");
        }
        Ok(())
    }

    /// Write a VMCS field by its Intel-defined encoding.
    pub fn write_field(&mut self, encoding: u32, value: u64) -> Result<(), &'static str> {
        let field_encoding = u64::from(encoding);
        let mut carry = 0u8;
        let mut zero = 0u8;
        unsafe {
            core::arch::asm!(
                "vmwrite {encoding}, {value}",
                "setc {carry}",
                "setz {zero}",
                encoding = in(reg) field_encoding,
                value = in(reg) value,
                carry = lateout(reg_byte) carry,
                zero = lateout(reg_byte) zero,
                options(nostack)
            );
        }
        if carry != 0 || zero != 0 {
            return Err("VMWRITE instruction failed");
        }
        Ok(())
    }

    /// Read a VMCS field by its Intel-defined encoding.
    pub fn read_field(&mut self, encoding: u32) -> Result<u64, &'static str> {
        let field_encoding = u64::from(encoding);
        let mut value: u64;
        let mut carry = 0u8;
        let mut zero = 0u8;
        unsafe {
            core::arch::asm!(
                "vmread {value}, {encoding}",
                "setc {carry}",
                "setz {zero}",
                value = lateout(reg) value,
                encoding = in(reg) field_encoding,
                carry = lateout(reg_byte) carry,
                zero = lateout(reg_byte) zero,
                options(nostack)
            );
        }
        if carry != 0 || zero != 0 {
            return Err("VMREAD instruction failed");
        }
        Ok(value)
    }

    /// Enter the guest and service exits synchronously on the pinned processor.
    ///
    /// # Safety
    /// The VMCS must contain validated guest state, EPT mappings, and a host
    /// state compatible with the current firmware context. The callback and
    /// opaque pointer must remain valid until this function returns. Guest
    /// execution is privileged and may access every mapping and device granted
    /// to the VM.
    pub unsafe fn run_raw(
        vmcs: *mut Self,
        stack: &VmxExitStack,
        context: &mut VmxRunContext,
        handler: VmExitHandler,
        opaque: *mut core::ffi::c_void,
    ) -> Result<VmxRunOutcome, &'static str> {
        if vmcs.is_null() {
            return Err("VMCS pointer is null");
        }
        let processor = unsafe { (*vmcs).processor };
        if crate::hardware::cpu::mp::MpManager::who_am_i() != Some(processor) {
            return Err("VMCS can only be entered on its owning logical processor");
        }
        context.handler = handler;
        context.opaque = opaque;
        context.outcome = ENTRY_FAILED;
        context.resume = u64::from(unsafe { (*vmcs).launched });
        stack.install_context(context);
        {
            let vmcs = unsafe { &mut *vmcs };
            vmcs.load_current()?;
            vmcs.write_field(0x6c14, stack.host_stack_pointer())?;
            vmcs.write_field(0x6c16, VmxExitStack::vm_exit_trampoline_address())?;
        }

        let result = unsafe { hpvmx_vmx_enter(context) };
        if crate::hardware::cpu::mp::MpManager::who_am_i() != Some(processor) {
            return Err("VMX execution returned on a different logical processor");
        }
        match result {
            EXIT_HALT => {
                unsafe { (*vmcs).launched = true };
                Ok(VmxRunOutcome::Halted)
            }
            EXIT_YIELD => {
                unsafe { (*vmcs).launched = true };
                Ok(VmxRunOutcome::Yielded)
            }
            EXIT_FATAL => {
                unsafe { (*vmcs).launched = true };
                Ok(VmxRunOutcome::Fatal)
            }
            ENTRY_FAILED => Ok(VmxRunOutcome::EntryFailed),
            _ => Err("VMX trampoline returned an invalid exit status"),
        }
    }
}

impl Drop for Vmcs<'_> {
    fn drop(&mut self) {
        let physical_address = self._region.physical_address();
        let mut carry = 0u8;
        let mut zero = 0u8;
        unsafe {
            core::arch::asm!(
                "vmclear [{operand}]",
                "setc {carry}",
                "setz {zero}",
                operand = in(reg) &physical_address,
                carry = lateout(reg_byte) carry,
                zero = lateout(reg_byte) zero,
                options(nostack)
            );
        }
        if carry != 0 || zero != 0 {
            log::error!("VMCLEAR failed while releasing VMCS region");
        }
    }
}

fn fixed_register_value(current: u64, fixed0: u64, fixed1: u64) -> Result<u64, &'static str> {
    if fixed0 & !fixed1 != 0 {
        return Err("processor reports conflicting VMX fixed-register masks");
    }
    Ok((current | fixed0) & fixed1)
}

fn preemption_timer_ticks(tsc_hz: u64, decrement_rate: u32) -> Option<u32> {
    if decrement_rate >= 64 {
        return None;
    }
    let interval = tsc_hz.checked_div(100)? >> decrement_rate;
    Some(interval.clamp(1, u32::MAX as u64) as u32)
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

unsafe fn read_cr0() -> u64 {
    let value: u64;
    unsafe {
        core::arch::asm!("mov {}, cr0", out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

unsafe fn write_cr0(value: u64) {
    unsafe {
        core::arch::asm!("mov cr0, {}", in(reg) value, options(nomem, nostack, preserves_flags));
    }
}

unsafe fn read_cr4() -> u64 {
    let value: u64;
    unsafe {
        core::arch::asm!("mov {}, cr4", out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

unsafe fn write_cr4(value: u64) {
    unsafe {
        core::arch::asm!("mov cr4, {}", in(reg) value, options(nomem, nostack, preserves_flags));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn firmware_must_enable_and_lock_vmx() {
        let allowed_controls = (1u64 << 32) | (1u64 << 33) | (1u64 << 37) | (1u64 << 39);
        let ept_caps = (1 << 6) | (1 << 14);
        let unlocked = VtxCapabilities::from_registers(
            true,
            FEATURE_CONTROL_VMX_OUTSIDE_SMX,
            allowed_controls,
            ept_caps,
        );
        let locked_off =
            VtxCapabilities::from_registers(true, FEATURE_CONTROL_LOCK, allowed_controls, ept_caps);
        let locked_on = VtxCapabilities::from_registers(
            true,
            FEATURE_CONTROL_LOCK | FEATURE_CONTROL_VMX_OUTSIDE_SMX,
            allowed_controls,
            ept_caps,
        );

        assert!(!unlocked.available);
        assert!(!locked_off.available);
        assert!(locked_on.available);
        assert!(locked_on.ept_supported);
        assert!(locked_on.vpid_supported);
        assert!(locked_on.unrestricted_guest);
    }

    #[test]
    fn vmx_features_are_absent_without_vmx_cpuid_bit() {
        let capabilities = VtxCapabilities::from_registers(
            false,
            FEATURE_CONTROL_LOCK | FEATURE_CONTROL_VMX_OUTSIDE_SMX,
            u64::MAX,
            u64::MAX,
        );

        assert!(!capabilities.available);
        assert!(!capabilities.vmxon_supported);
        assert!(!capabilities.ept_supported);
        assert!(!capabilities.vpid_supported);
        assert!(!capabilities.unrestricted_guest);
    }

    #[test]
    fn fixed_register_masks_are_applied_and_validated() {
        assert_eq!(fixed_register_value(0b0010, 0b0100, 0b1101), Ok(0b0100));
        assert_eq!(
            fixed_register_value(0, 0b0010, 0b0001),
            Err("processor reports conflicting VMX fixed-register masks")
        );
    }

    #[test]
    fn preemption_timer_uses_a_bounded_ten_millisecond_slice() {
        assert_eq!(preemption_timer_ticks(3_200_000_000, 5), Some(1_000_000));
        assert_eq!(preemption_timer_ticks(1, 31), Some(1));
        assert_eq!(preemption_timer_ticks(u64::MAX, 0), Some(u32::MAX));
        assert_eq!(preemption_timer_ticks(3_200_000_000, 64), None);
    }
}
