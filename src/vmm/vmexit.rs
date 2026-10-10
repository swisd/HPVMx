//! Decoding and emulation helpers for the VM-exit dispatcher.

pub use crate::hardware::cpu::vmx::GuestRegisters;
use crate::hardware::cpu::vmx::{
    self, Vmcs, VmxExitStack, VmxGuestFpuState, VmxRunContext, VmxRunOutcome,
};
use crate::vmm::io::VirtualDevices;
use crate::vmm::vmcs::{IoExitQualification, VmExitReason};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VmExit {
    pub reason: VmExitReason,
    pub entry_failure: bool,
    pub qualification: u64,
    pub instruction_length: u32,
    pub guest_physical_address: u64,
}

impl VmExit {
    pub fn from_vmcs(
        raw_reason: u32,
        qualification: u64,
        instruction_length: u32,
        guest_physical_address: u64,
    ) -> Self {
        Self {
            reason: VmExitReason::from_vmcs(raw_reason),
            entry_failure: raw_reason & (1 << 31) != 0,
            qualification,
            instruction_length,
            guest_physical_address,
        }
    }

    pub fn read_from_vmcs(vmcs: &mut Vmcs<'_>) -> Result<Self, &'static str> {
        let reason = vmcs.read_field(0x4402)? as u32;
        if reason & (1 << 31) != 0 {
            return Ok(Self::from_vmcs(reason, 0, 0, 0));
        }
        let qualification = vmcs.read_field(0x6400)?;
        let instruction_length = vmcs.read_field(0x440c)? as u32;
        let guest_physical_address = vmcs.read_field(0x2400)?;
        Ok(Self::from_vmcs(
            reason,
            qualification,
            instruction_length,
            guest_physical_address,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitDisposition {
    ResumeGuest { advance_rip_by: u32 },
    YieldGuest,
    HaltGuest,
    Fatal(&'static str),
}

/// Handles the initial non-string port-I/O/CPUID/HLT exit set. Unsupported
/// instructions terminate the guest path explicitly instead of silently
/// advancing guest state.
pub struct VmExitDispatcher;

impl VmExitDispatcher {
    pub fn dispatch_vmcs(
        vmcs: &mut Vmcs<'_>,
        exit: VmExit,
        registers: &mut GuestRegisters,
        devices: &mut VirtualDevices,
    ) -> ExitDisposition {
        let disposition = Self::dispatch(exit, registers, devices);
        let advance_rip_by = match disposition {
            ExitDisposition::ResumeGuest { advance_rip_by } => Some(advance_rip_by),
            ExitDisposition::HaltGuest if exit.instruction_length != 0 => {
                Some(exit.instruction_length)
            }
            ExitDisposition::HaltGuest => return ExitDisposition::Fatal("HLT exit length is zero"),
            ExitDisposition::YieldGuest | ExitDisposition::Fatal(_) => None,
        };
        if let Some(advance_rip_by) = advance_rip_by {
            let guest_rip = match vmcs.read_field(0x681e) {
                Ok(rip) => rip,
                Err(error) => return ExitDisposition::Fatal(error),
            };
            let Some(next_rip) = guest_rip.checked_add(advance_rip_by as u64) else {
                return ExitDisposition::Fatal("guest instruction pointer overflows");
            };
            if let Err(error) = vmcs.write_field(0x681e, next_rip) {
                return ExitDisposition::Fatal(error);
            }
        }
        disposition
    }

    pub fn dispatch(
        exit: VmExit,
        registers: &mut GuestRegisters,
        devices: &mut VirtualDevices,
    ) -> ExitDisposition {
        if exit.entry_failure {
            return ExitDisposition::Fatal("VM entry failed; inspect VM-instruction error field");
        }
        match exit.reason {
            VmExitReason::Cpuid => {
                emulate_cpuid(registers);
                Self::resume_after(exit)
            }
            VmExitReason::IoInstruction => Self::dispatch_io(exit, registers, devices),
            VmExitReason::Hlt => ExitDisposition::HaltGuest,
            VmExitReason::PreemptionTimer => ExitDisposition::YieldGuest,
            VmExitReason::EptViolation => {
                ExitDisposition::Fatal("guest accessed an unmapped EPT page")
            }
            VmExitReason::EptMisconfiguration => {
                ExitDisposition::Fatal("EPT contains a misconfigured entry")
            }
            VmExitReason::ExceptionOrNmi => {
                ExitDisposition::Fatal("guest exception or NMI requires interrupt injection")
            }
            VmExitReason::TripleFault => ExitDisposition::Fatal("guest triple fault"),
            VmExitReason::ExternalInterrupt => {
                ExitDisposition::Fatal("external interrupt routing is not configured")
            }
            VmExitReason::Other(_) => ExitDisposition::Fatal("VM-exit reason is not implemented"),
        }
    }

    fn dispatch_io(
        exit: VmExit,
        registers: &mut GuestRegisters,
        devices: &mut VirtualDevices,
    ) -> ExitDisposition {
        let qualification = match IoExitQualification::decode(exit.qualification) {
            Ok(value) => value,
            Err(error) => return ExitDisposition::Fatal(error),
        };
        if qualification.string || qualification.repeat {
            return ExitDisposition::Fatal(
                "string and repeated port-I/O instructions are not implemented",
            );
        }
        if (0x3f8..=0x3ff).contains(&qualification.port) && qualification.width != 1 {
            return ExitDisposition::Fatal("virtual UART only supports byte-sized I/O");
        }
        if (0x3d0..=0x3d1).contains(&qualification.port) && qualification.width != 1 {
            return ExitDisposition::Fatal("virtual display only supports byte-sized I/O");
        }
        if !(0x3f8..=0x3ff).contains(&qualification.port)
            && !(0x1f0..=0x1f7).contains(&qualification.port)
            && !(0x3d0..=0x3d1).contains(&qualification.port)
        {
            return ExitDisposition::Fatal("guest accessed an unassigned I/O port");
        }

        let io_result = if qualification.input {
            devices
                .read_io(qualification.port, qualification.width)
                .map(|value| {
                    let mask = match qualification.width {
                        1 => 0xff,
                        2 => 0xffff,
                        _ => u32::MAX,
                    };
                    registers.rax = (registers.rax & !(mask as u64)) | (value as u64 & mask as u64);
                })
        } else {
            devices.write_io(
                qualification.port,
                qualification.width,
                registers.rax as u32,
            )
        };
        match io_result {
            Ok(()) => Self::resume_after(exit),
            Err(error) => ExitDisposition::Fatal(error),
        }
    }

    fn resume_after(exit: VmExit) -> ExitDisposition {
        if exit.instruction_length == 0 {
            ExitDisposition::Fatal("VM-exit instruction length is zero")
        } else {
            ExitDisposition::ResumeGuest {
                advance_rip_by: exit.instruction_length,
            }
        }
    }
}

struct VmExitRuntime<'a> {
    vmcs: *mut Vmcs<'a>,
    devices: *mut VirtualDevices,
    preemption_timer: Option<u32>,
    fatal: Option<&'static str>,
}

/// Run one guest quantum. A supported VMX preemption timer yields back to the
/// caller; a guest HLT also returns, while unsupported exits are fatal.
///
/// # Safety
/// Guest memory and all VMCS fields must already be validated. The caller must
/// keep this VMCS current and must not access it concurrently while the guest
/// is running. The exit stack and devices must remain valid for the call.
pub unsafe fn run_guest(
    vmcs: *mut Vmcs<'_>,
    stack: &VmxExitStack,
    registers: &mut GuestRegisters,
    fpu_state: &mut VmxGuestFpuState,
    devices: &mut VirtualDevices,
) -> Result<(VmxRunOutcome, Option<&'static str>), &'static str> {
    if vmcs.is_null() {
        return Err("VMCS pointer is null");
    }
    let mut runtime = VmExitRuntime {
        vmcs,
        devices,
        preemption_timer: unsafe { (*vmcs).preemption_timer() },
        fatal: None,
    };
    let mut context = VmxRunContext::new(
        *registers,
        fpu_state,
        dispatch_hardware_exit,
        (&mut runtime as *mut VmExitRuntime<'_>).cast(),
    );
    let outcome = unsafe {
        Vmcs::run_raw(
            vmcs,
            stack,
            &mut context,
            dispatch_hardware_exit,
            (&mut runtime as *mut VmExitRuntime<'_>).cast(),
        )?
    };
    *registers = context.registers;
    context.save_guest_fpu(fpu_state);
    Ok((outcome, runtime.fatal))
}

unsafe extern "efiapi" fn dispatch_hardware_exit(
    context: *mut VmxRunContext,
    opaque: *mut core::ffi::c_void,
) -> u64 {
    if context.is_null() || opaque.is_null() {
        return vmx::EXIT_FATAL;
    }
    let context = unsafe { &mut *context };
    let runtime = unsafe { &mut *opaque.cast::<VmExitRuntime<'_>>() };
    let vmcs = unsafe { &mut *runtime.vmcs };
    let devices = unsafe { &mut *runtime.devices };
    let exit = match VmExit::read_from_vmcs(vmcs) {
        Ok(exit) => exit,
        Err(error) => {
            runtime.fatal = Some(error);
            return vmx::EXIT_FATAL;
        }
    };
    match VmExitDispatcher::dispatch_vmcs(vmcs, exit, &mut context.registers, devices) {
        ExitDisposition::ResumeGuest { .. } => vmx::EXIT_RESUME,
        ExitDisposition::YieldGuest => match runtime.preemption_timer {
            Some(timer) => match vmcs.write_field(0x482e, timer as u64) {
                Ok(()) => vmx::EXIT_YIELD,
                Err(error) => {
                    runtime.fatal = Some(error);
                    vmx::EXIT_FATAL
                }
            },
            None => {
                runtime.fatal = Some("VM-exit requested scheduling without a preemption timer");
                vmx::EXIT_FATAL
            }
        },
        ExitDisposition::HaltGuest => vmx::EXIT_HALT,
        ExitDisposition::Fatal(error) => {
            runtime.fatal = Some(error);
            vmx::EXIT_FATAL
        }
    }
}

fn emulate_cpuid(registers: &mut GuestRegisters) {
    let leaf = registers.rax as u32;
    let subleaf = registers.rcx as u32;
    match (leaf, subleaf) {
        (0, _) => {
            registers.rax = 1;
            registers.rbx = u32::from_le_bytes(*b"HPVM") as u64;
            registers.rdx = u32::from_le_bytes(*b"xGue") as u64;
            registers.rcx = u32::from_le_bytes(*b"st  ") as u64;
        }
        (1, _) => {
            registers.rax = 0;
            registers.rbx = 0;
            registers.rcx = 1 << 31;
            registers.rdx = 0;
        }
        _ => {
            registers.rax = 0;
            registers.rbx = 0;
            registers.rcx = 0;
            registers.rdx = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vmm::io::VirtualDevices;

    #[test]
    fn cpuid_and_halt_exits_have_explicit_dispatch_results() {
        let mut devices = VirtualDevices::new(512, 2, 2).unwrap();
        let mut registers = GuestRegisters {
            rax: 0,
            rcx: 0,
            ..GuestRegisters::default()
        };
        assert_eq!(
            VmExitDispatcher::dispatch(
                VmExit {
                    reason: VmExitReason::Cpuid,
                    entry_failure: false,
                    qualification: 0,
                    instruction_length: 2,
                    guest_physical_address: 0,
                },
                &mut registers,
                &mut devices,
            ),
            ExitDisposition::ResumeGuest { advance_rip_by: 2 }
        );
        assert_eq!(registers.rax, 1);
        assert_eq!(
            VmExitDispatcher::dispatch(
                VmExit {
                    reason: VmExitReason::Hlt,
                    entry_failure: false,
                    qualification: 0,
                    instruction_length: 1,
                    guest_physical_address: 0,
                },
                &mut registers,
                &mut devices,
            ),
            ExitDisposition::HaltGuest
        );
    }

    #[test]
    fn preemption_timer_exit_yields_without_advancing_guest_rip() {
        let mut devices = VirtualDevices::new(512, 2, 2).unwrap();
        let mut registers = GuestRegisters::default();
        assert_eq!(
            VmExitDispatcher::dispatch(
                VmExit {
                    reason: VmExitReason::PreemptionTimer,
                    entry_failure: false,
                    qualification: 0,
                    instruction_length: 0,
                    guest_physical_address: 0,
                },
                &mut registers,
                &mut devices,
            ),
            ExitDisposition::YieldGuest
        );
    }

    #[test]
    fn uart_io_exits_route_to_isolated_guest_console() {
        let mut devices = VirtualDevices::new(512, 2, 2).unwrap();
        let mut registers = GuestRegisters {
            rax: b'Z' as u64,
            ..GuestRegisters::default()
        };
        let output_exit = VmExit {
            reason: VmExitReason::IoInstruction,
            entry_failure: false,
            qualification: 0x3f8u64 << 16,
            instruction_length: 1,
            guest_physical_address: 0,
        };
        assert_eq!(
            VmExitDispatcher::dispatch(output_exit, &mut registers, &mut devices),
            ExitDisposition::ResumeGuest { advance_rip_by: 1 }
        );
        assert_eq!(devices.console.drain_output(1), b"Z");
    }

    #[test]
    fn display_io_exits_update_the_guest_framebuffer() {
        let mut devices = VirtualDevices::new(512, 2, 2).unwrap();
        let mut registers = GuestRegisters::default();
        for (register, value) in [(0u8, 1u8), (2, 1), (4, 0x56), (5, 0x34), (6, 0x12), (7, 1)] {
            let select = VmExit {
                reason: VmExitReason::IoInstruction,
                entry_failure: false,
                qualification: (0x3d0u64 << 16),
                instruction_length: 1,
                guest_physical_address: 0,
            };
            registers.rax = register as u64;
            assert_eq!(
                VmExitDispatcher::dispatch(select, &mut registers, &mut devices),
                ExitDisposition::ResumeGuest { advance_rip_by: 1 }
            );
            let write = VmExit {
                qualification: 0x3d1u64 << 16,
                ..select
            };
            registers.rax = value as u64;
            assert_eq!(
                VmExitDispatcher::dispatch(write, &mut registers, &mut devices),
                ExitDisposition::ResumeGuest { advance_rip_by: 1 }
            );
        }
        assert_eq!(devices.display.pixels()[3], 0x0012_3456);
    }

    #[test]
    fn unsupported_guest_io_is_not_silently_consumed() {
        let mut devices = VirtualDevices::new(512, 2, 2).unwrap();
        let mut registers = GuestRegisters::default();
        let exit = VmExit {
            reason: VmExitReason::IoInstruction,
            entry_failure: false,
            qualification: (0x80u64 << 16) | 1,
            instruction_length: 1,
            guest_physical_address: 0,
        };
        assert_eq!(
            VmExitDispatcher::dispatch(exit, &mut registers, &mut devices),
            ExitDisposition::Fatal("guest accessed an unassigned I/O port")
        );
    }
}
