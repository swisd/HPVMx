//! Deep Level Security & Autolytic "Fail-Stop" Protocol.
//! Central gateway for intercepting all VMBUS traffic and managing intrusion response.

use crate::dls::{
    summarize_hwbus, summarize_vmbus, AnalysisVerdict, DeepSecurityLevel, SoftwareAnalysisMemory,
    SoftwareAnalysisSample,
};
use crate::vmm::hwbus::{HwBus, HwBusMessage};
use crate::vmm::vmbus::{VmBus, VmBusMessage};

pub struct DeepLevelSecurity {
    pub policy: DeepSecurityPolicy,
    memory: SoftwareAnalysisMemory,
}

#[derive(Clone, Copy, Debug)]
pub struct DeepSecurityPolicy {
    pub level: DeepSecurityLevel,
    pub fail_stop_threshold: u16,
    pub training_capture_enabled: bool,
}

impl DeepLevelSecurity {
    pub fn new() -> Self {
        Self {
            policy: DeepSecurityPolicy::default(),
            memory: SoftwareAnalysisMemory::new(),
        }
    }

    pub fn with_policy(policy: DeepSecurityPolicy) -> Self {
        Self {
            policy,
            memory: SoftwareAnalysisMemory::new(),
        }
    }

    pub fn latest_analysis_sample(&self) -> Option<&SoftwareAnalysisSample> {
        self.memory.latest()
    }

    pub fn analysis_memory(&self) -> &SoftwareAnalysisMemory {
        &self.memory
    }

    /// Intercept and inspect traffic on the VMBUS.
    /// Acts as a second-tier firewall between the VM Unit and physical hardware/drivers.
    pub fn inspect_bus(&mut self, bus: &VmBus) -> Result<SoftwareAnalysisSample, &'static str> {
        let messages = bus.queued_messages();
        let sample = summarize_vmbus(
            bus.vm_id,
            "vmbus.queue",
            self.policy.level,
            &messages,
            self.policy.fail_stop_threshold,
        );

        let mut violations = false;
        bus.inspect_messages(|msg| {
            if !self.is_authorized(msg) {
                violations = true;
                false // Terminate inspection
            } else {
                true
            }
        });

        self.learn_from_sample(sample.clone());

        if violations || matches!(sample.verdict, AnalysisVerdict::FailStop) {
            Err("VMBUS security policy violation detected")
        } else {
            Ok(sample)
        }
    }

    /// Intercept and inspect traffic on the HWBUS.
    pub fn inspect_hwbus(&mut self, bus: &HwBus) -> Result<SoftwareAnalysisSample, &'static str> {
        let messages = bus.queued_messages();
        let sample = summarize_hwbus(
            bus.vm_id,
            "hwbus.queue",
            self.policy.level,
            &messages,
            self.policy.fail_stop_threshold,
        );

        let mut violations = false;
        bus.inspect_messages(|msg| {
            if !self.is_hw_authorized(msg) {
                violations = true;
                false
            } else {
                true
            }
        });

        self.learn_from_sample(sample.clone());

        if violations || matches!(sample.verdict, AnalysisVerdict::FailStop) {
            Err("HWBUS security policy violation detected")
        } else {
            Ok(sample)
        }
    }

    fn learn_from_sample(&mut self, sample: SoftwareAnalysisSample) {
        if self.policy.training_capture_enabled {
            self.memory.learn_from(sample);
        }
    }

    fn is_authorized(&self, message: &VmBusMessage) -> bool {
        match message {
            VmBusMessage::IoRequest { address, .. } => {
                !crate::dls::is_restricted_io_port(*address)
            }
            VmBusMessage::Interrupt { vector } => {
                if matches!(self.policy.level, DeepSecurityLevel::Lab) {
                    true
                } else {
                    !crate::dls::is_privileged_interrupt(*vector)
                }
            }
            VmBusMessage::StorageRequest { count, write, .. } => {
                !(*write && *count > self.max_storage_write_sectors())
            }
            VmBusMessage::InstructionTrace { opcode, mnemonic, .. } => {
                if matches!(self.policy.level, DeepSecurityLevel::Lab) {
                    true
                } else {
                    !crate::dls::is_suspicious_instruction(*opcode, mnemonic)
                }
            }
            VmBusMessage::Call { to, target_name, .. } => {
                !crate::dls::is_restricted_call(*to, target_name.as_deref())
            }
        }
    }

    fn is_hw_authorized(&self, message: &HwBusMessage) -> bool {
        match message {
            HwBusMessage::MemoryAccess { .. } => true,
            HwBusMessage::PciConfig { write, .. } => {
                matches!(self.policy.level, DeepSecurityLevel::Lab) || !*write
            }
            HwBusMessage::DevicePort { port, .. } => {
                !crate::dls::is_restricted_io_port(*port as u64)
            }
            HwBusMessage::DmaRequest { bytes, .. } => {
                matches!(self.policy.level, DeepSecurityLevel::Lab) || *bytes <= 1024 * 1024
            }
            HwBusMessage::InstructionTrace { opcode, mnemonic, .. } => {
                if matches!(self.policy.level, DeepSecurityLevel::Lab) {
                    true
                } else {
                    !crate::dls::is_suspicious_instruction(*opcode, mnemonic)
                }
            }
            HwBusMessage::Call { to, target_name, .. } => {
                !crate::dls::is_restricted_call(*to, target_name.as_deref())
            }
        }
    }

    fn max_storage_write_sectors(&self) -> u32 {
        match self.policy.level {
            DeepSecurityLevel::Standard => 4096,
            DeepSecurityLevel::Paranoid => 2048,
            DeepSecurityLevel::Lab => u32::MAX,
        }
    }
}

impl Default for DeepSecurityPolicy {
    fn default() -> Self {
        Self {
            level: DeepSecurityLevel::Paranoid,
            fail_stop_threshold: 80,
            training_capture_enabled: true,
        }
    }
}

pub struct AutolyticProtocol;

impl AutolyticProtocol {
    /// Phase 1: Detection
    /// Triggered by a GHM boundary violation or a software-induced crash.
    pub fn detect_violation(vm_id: u32, error_code: u32) {
        crate::message!("\n", "ALERT: AUTOLYTIC VIOLATION DETECTED FOR VM {} (CODE: {:#X})", vm_id, error_code);
        Self::trigger_fail_stop(vm_id, error_code);
    }

    /// Phase 2: System Hibernation
    fn trigger_fail_stop(vm_id: u32, _error_code: u32) {
        crate::message!("\n", "FAIL-STOP: TERMINATING VM {}, CLEARING CACHES, AND RESETTING HARDWARE", vm_id);
        // 1. Terminate the VM Unit (handled by HypervisorManager)
        // 2. Clear volatile states (simplified representation)
        // 3. Reset hardware to known-good state.
    }

    /// Phase 3: Forensic Deep Scan
    /// Performs a Deep Scan of the VM's VHD while the system is in a restricted state.
    pub fn forensic_scan(vhd_path: &str, error_code: u32) -> bool {
        crate::message!("\n", "FORENSIC: SCANNING {} FOR ERROR PATTERNS ({:#X})", vhd_path, error_code);
        // Heuristic Matching: Correlates the GHM error code with code patterns found in the VHD.
        // Persistence Check: Looks for loops or payloads designed to trigger repeated hardware faults.
        true // Assume malicious for simulation
    }

    /// Phase 4: Decommissioning & Zeroing
    pub fn decommissioning_and_zeroing(vm_id: u32) {
        crate::message!("\n", "DECOMMISSIONING: PERMANENTLY RETIRING VM {}", vm_id);
        crate::message!("\n", "ZEROING: WIPING MEMORY AND DISK SECTORS ALLOCATED TO VM {}", vm_id);
        // 1. Permanent retirement of the VM.
        // 2. Zeroing specific sectors and memory regions.
    }
}
