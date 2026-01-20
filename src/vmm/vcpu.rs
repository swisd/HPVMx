//! Virtual CPU management

use bitflags::bitflags;

bitflags! {
    /// vCPU execution flags
    #[derive(Clone, Copy, Debug)]
    pub struct VCpuFlags: u32 {
        const HALTED = 1 << 0;
        const INTERRUPT_PENDING = 1 << 1;
        const NMI_PENDING = 1 << 2;
        const PREEMPTED = 1 << 3;
    }
}

/// Virtual CPU state
#[derive(Debug, Clone)]
pub struct VirtualCpu {
    pub id: u32,
    pub flags: VCpuFlags,
    pub rip: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub interrupt_pending: bool,
}

impl VirtualCpu {
    /// Create a new virtual CPU
    pub fn new(id: u32) -> Self {
        Self {
            id,
            flags: VCpuFlags::empty(),
            rip: 0,
            rsp: 0,
            rbp: 0,
            interrupt_pending: false,
        }
    }

    /// Set instruction pointer
    pub fn set_instruction_pointer(&mut self, rip: u64) {
        self.rip = rip;
    }

    /// Set stack pointer
    pub fn set_stack_pointer(&mut self, rsp: u64) {
        self.rsp = rsp;
    }

    /// Halt the vCPU
    pub fn halt(&mut self) {
        self.flags.insert(VCpuFlags::HALTED);
    }

    /// Resume the vCPU
    pub fn resume(&mut self) {
        self.flags.remove(VCpuFlags::HALTED);
    }

    /// Is vCPU halted?
    pub fn is_halted(&self) -> bool {
        self.flags.contains(VCpuFlags::HALTED)
    }

    /// Inject an interrupt
    pub fn inject_interrupt(&mut self) {
        self.interrupt_pending = true;
        self.flags.insert(VCpuFlags::INTERRUPT_PENDING);
    }

    /// Get guest register state (simplified)
    pub fn get_registers(&self) -> VCpuRegisters {
        VCpuRegisters {
            rip: self.rip,
            rsp: self.rsp,
            rbp: self.rbp,
        }
    }
}

/// Simplified vCPU register state
#[derive(Debug, Clone)]
pub struct VCpuRegisters {
    pub rip: u64,
    pub rsp: u64,
    pub rbp: u64,
}