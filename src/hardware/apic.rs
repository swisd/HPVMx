//! Deep Level Security analysis records.
//!
//! These structs are intentionally serializable and compact so they can be
//! persisted as JSON/Core JSON for offline software analysis or fed back into
//! a future deep-level analysis model.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::vmm::hwbus::HwBusMessage;
use crate::vmm::vmbus::VmBusMessage;

pub const DLS_SCHEMA_VERSION: u16 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DeepSecurityLevel {
    Standard,
    Paranoid,
    Lab,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AnalysisVerdict {
    Allow,
    Observe,
    Quarantine,
    FailStop,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TrainingLabel {
    Benign,
    Suspicious,
    Malicious,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AnalysisSurface {
    VmBus,
    HwBus,
    BootMedia,
    Package,
    Kernel,
    Driver,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SecuritySignalKind {
    RestrictedIoPort,
    PrivilegedInterrupt,
    LargeStorageWrite,
    PayloadWrite,
    SuspiciousInstruction,
    RestrictedCall,
    HardwareBoundaryViolation,
    HighRiskScore,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecuritySignal {
    pub kind: SecuritySignalKind,
    pub code: u32,
    pub severity: u8,
    pub description: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SoftwareFeatureVector {
    pub message_count: u32,
    pub hwbus_messages: u32,
    pub io_reads: u32,
    pub io_writes: u32,
    pub memory_reads: u32,
    pub memory_writes: u32,
    pub pci_config_reads: u32,
    pub pci_config_writes: u32,
    pub dma_requests: u32,
    pub storage_reads: u32,
    pub storage_writes: u32,
    pub interrupts: u32,
    pub instructions: u32,
    pub calls: u32,
    pub restricted_io_hits: u32,
    pub privileged_interrupt_hits: u32,
    pub suspicious_instruction_hits: u32,
    pub restricted_call_hits: u32,
    pub write_payload_bytes: u64,
    pub max_storage_span_sectors: u32,
    pub risk_score: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoftwareAnalysisSample {
    pub schema_version: u16,
    pub vm_id: u32,
    pub artifact_name: String,
    pub surface: AnalysisSurface,
    pub security_level: DeepSecurityLevel,
    pub features: SoftwareFeatureVector,
    pub signals: Vec<SecuritySignal>,
    pub verdict: AnalysisVerdict,
    pub label: TrainingLabel,
    pub remediation_hint: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DeepLearningStats {
    pub samples_seen: u32,
    pub benign_samples: u32,
    pub suspicious_samples: u32,
    pub malicious_samples: u32,
    pub cumulative_risk_score: u64,
    pub last_risk_score: u16,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SoftwareAnalysisMemory {
    pub schema_version: u16,
    pub stats: DeepLearningStats,
    pub samples: Vec<SoftwareAnalysisSample>,
}

impl SoftwareAnalysisSample {
    pub fn new(vm_id: u32, artifact_name: &str, security_level: DeepSecurityLevel) -> Self {
        Self {
            schema_version: DLS_SCHEMA_VERSION,
            vm_id,
            artifact_name: artifact_name.to_string(),
            surface: AnalysisSurface::VmBus,
            security_level,
            features: SoftwareFeatureVector::default(),
            signals: Vec::new(),
            verdict: AnalysisVerdict::Allow,
            label: TrainingLabel::Unknown,
            remediation_hint: None,
        }
    }

    pub fn add_signal(
        &mut self,
        kind: SecuritySignalKind,
        code: u32,
        severity: u8,
        description: &str,
    ) {
        self.signals.push(SecuritySignal {
            kind,
            code,
            severity,
            description: description.to_string(),
        });
        self.features.risk_score = self.features.risk_score.saturating_add(severity as u16);
    }

    pub fn finalize(&mut self, fail_stop_threshold: u16) {
        self.verdict = if self.features.risk_score >= fail_stop_threshold {
            AnalysisVerdict::FailStop
        } else if self.features.risk_score >= fail_stop_threshold / 2 {
            AnalysisVerdict::Quarantine
        } else if self.features.risk_score > 0 {
            AnalysisVerdict::Observe
        } else {
            AnalysisVerdict::Allow
        };

        self.label = match self.verdict {
            AnalysisVerdict::Allow => TrainingLabel::Benign,
            AnalysisVerdict::Observe | AnalysisVerdict::Quarantine => TrainingLabel::Suspicious,
            AnalysisVerdict::FailStop => TrainingLabel::Malicious,
        };

        if matches!(self.verdict, AnalysisVerdict::Quarantine | AnalysisVerdict::FailStop) {
            self.remediation_hint = Some("isolate VM, revoke mapped hardware, and preserve this sample for model retraining".to_string());
        }
    }
}

impl SoftwareAnalysisMemory {
    pub fn new() -> Self {
        Self {
            schema_version: DLS_SCHEMA_VERSION,
            stats: DeepLearningStats::default(),
            samples: Vec::new(),
        }
    }

    pub fn learn_from(&mut self, sample: SoftwareAnalysisSample) {
        self.stats.samples_seen = self.stats.samples_seen.saturating_add(1);
        self.stats.cumulative_risk_score = self.stats.cumulative_risk_score
            .saturating_add(sample.features.risk_score as u64);
        self.stats.last_risk_score = sample.features.risk_score;

        match sample.label {
            TrainingLabel::Benign => self.stats.benign_samples = self.stats.benign_samples.saturating_add(1),
            TrainingLabel::Suspicious | TrainingLabel::Unknown => {
                self.stats.suspicious_samples = self.stats.suspicious_samples.saturating_add(1)
            }
            TrainingLabel::Malicious => self.stats.malicious_samples = self.stats.malicious_samples.saturating_add(1),
        }

        self.samples.push(sample);
    }

    pub fn latest(&self) -> Option<&SoftwareAnalysisSample> {
        self.samples.last()
    }
}

pub fn summarize_vmbus(
    vm_id: u32,
    artifact_name: &str,
    level: DeepSecurityLevel,
    messages: &[VmBusMessage],
    fail_stop_threshold: u16,
) -> SoftwareAnalysisSample {
    let mut sample = SoftwareAnalysisSample::new(vm_id, artifact_name, level);

    for message in messages {
        sample.features.message_count = sample.features.message_count.saturating_add(1);
        match message {
            VmBusMessage::IoRequest { address, write, data, .. } => {
                if *write {
                    sample.features.io_writes = sample.features.io_writes.saturating_add(1);
                    sample.features.write_payload_bytes = sample.features.write_payload_bytes
                        .saturating_add(data.as_ref().map(|d| d.len() as u64).unwrap_or(0));
                } else {
                    sample.features.io_reads = sample.features.io_reads.saturating_add(1);
                }

                if is_restricted_io_port(*address) {
                    sample.features.restricted_io_hits = sample.features.restricted_io_hits.saturating_add(1);
                    sample.add_signal(
                        SecuritySignalKind::RestrictedIoPort,
                        *address as u32,
                        40,
                        "access to restricted platform control IO port",
                    );
                }

                if *write && data.as_ref().map(|d| !d.is_empty()).unwrap_or(false) {
                    sample.add_signal(
                        SecuritySignalKind::PayloadWrite,
                        *address as u32,
                        5,
                        "guest attempted an IO write carrying payload bytes",
                    );
                }
            }
            VmBusMessage::Interrupt { vector } => {
                sample.features.interrupts = sample.features.interrupts.saturating_add(1);
                if is_privileged_interrupt(*vector) {
                    sample.features.privileged_interrupt_hits = sample.features.privileged_interrupt_hits.saturating_add(1);
                    sample.add_signal(
                        SecuritySignalKind::PrivilegedInterrupt,
                        *vector as u32,
                        35,
                        "guest attempted to signal a privileged interrupt vector",
                    );
                }
            }
            VmBusMessage::StorageRequest { count, write, data, .. } => {
                sample.features.max_storage_span_sectors = sample.features.max_storage_span_sectors.max(*count);
                if *write {
                    sample.features.storage_writes = sample.features.storage_writes.saturating_add(1);
                    sample.features.write_payload_bytes = sample.features.write_payload_bytes
                        .saturating_add(data.as_ref().map(|d| d.len() as u64).unwrap_or(0));
                    if *count > 2048 {
                        sample.add_signal(
                            SecuritySignalKind::LargeStorageWrite,
                            *count,
                            25,
                            "large storage write exceeds deep-scan threshold",
                        );
                    }
                } else {
                    sample.features.storage_reads = sample.features.storage_reads.saturating_add(1);
                }
            }
            VmBusMessage::InstructionTrace { rip, opcode, mnemonic, .. } => {
                sample.features.instructions = sample.features.instructions.saturating_add(1);
                if is_suspicious_instruction(*opcode, mnemonic) {
                    sample.features.suspicious_instruction_hits = sample.features.suspicious_instruction_hits.saturating_add(1);
                    sample.add_signal(
                        SecuritySignalKind::SuspiciousInstruction,
                        (*rip & 0xFFFF_FFFF) as u32,
                        30,
                        "guest instruction trace matched a privileged or high-risk operation",
                    );
                }
            }
            VmBusMessage::Call { from, to, target_name } => {
                sample.features.calls = sample.features.calls.saturating_add(1);
                if is_restricted_call(*to, target_name.as_deref()) {
                    sample.features.restricted_call_hits = sample.features.restricted_call_hits.saturating_add(1);
                    sample.add_signal(
                        SecuritySignalKind::RestrictedCall,
                        (*from & 0xFFFF_FFFF) as u32,
                        30,
                        "guest call target crosses into a restricted service boundary",
                    );
                }
            }
        }
    }

    sample.finalize(fail_stop_threshold);
    sample
}

pub fn summarize_hwbus(
    vm_id: u32,
    artifact_name: &str,
    level: DeepSecurityLevel,
    messages: &[HwBusMessage],
    fail_stop_threshold: u16,
) -> SoftwareAnalysisSample {
    let mut sample = SoftwareAnalysisSample::new(vm_id, artifact_name, level);
    sample.surface = AnalysisSurface::HwBus;

    for message in messages {
        sample.features.message_count = sample.features.message_count.saturating_add(1);
        sample.features.hwbus_messages = sample.features.hwbus_messages.saturating_add(1);

        match message {
            HwBusMessage::MemoryAccess { write, .. } => {
                if *write {
                    sample.features.memory_writes = sample.features.memory_writes.saturating_add(1);
                } else {
                    sample.features.memory_reads = sample.features.memory_reads.saturating_add(1);
                }
            }
            HwBusMessage::PciConfig { bus, device, function, write, .. } => {
                if *write {
                    sample.features.pci_config_writes = sample.features.pci_config_writes.saturating_add(1);
                    sample.add_signal(
                        SecuritySignalKind::HardwareBoundaryViolation,
                        ((*bus as u32) << 16) | ((*device as u32) << 8) | *function as u32,
                        25,
                        "guest attempted to write PCI configuration through HWBUS",
                    );
                } else {
                    sample.features.pci_config_reads = sample.features.pci_config_reads.saturating_add(1);
                }
            }
            HwBusMessage::DevicePort { port, write, data, .. } => {
                if *write {
                    sample.features.io_writes = sample.features.io_writes.saturating_add(1);
                    sample.features.write_payload_bytes = sample.features.write_payload_bytes
                        .saturating_add(data.as_ref().map(|d| d.len() as u64).unwrap_or(0));
                } else {
                    sample.features.io_reads = sample.features.io_reads.saturating_add(1);
                }

                if is_restricted_io_port(*port as u64) {
                    sample.features.restricted_io_hits = sample.features.restricted_io_hits.saturating_add(1);
                    sample.add_signal(
                        SecuritySignalKind::RestrictedIoPort,
                        *port as u32,
                        40,
                        "HWBUS device-port access targets restricted platform IO",
                    );
                }
            }
            HwBusMessage::DmaRequest { bytes, write, .. } => {
                sample.features.dma_requests = sample.features.dma_requests.saturating_add(1);
                if *write {
                    sample.features.memory_writes = sample.features.memory_writes.saturating_add(1);
                    sample.features.write_payload_bytes = sample.features.write_payload_bytes.saturating_add(*bytes as u64);
                }
                if *bytes > 1024 * 1024 {
                    sample.add_signal(
                        SecuritySignalKind::HardwareBoundaryViolation,
                        (*bytes).min(u32::MAX as usize) as u32,
                        30,
                        "large DMA request exceeds deep hardware boundary threshold",
                    );
                }
            }
            HwBusMessage::InstructionTrace { rip, opcode, mnemonic, .. } => {
                sample.features.instructions = sample.features.instructions.saturating_add(1);
                if is_suspicious_instruction(*opcode, mnemonic) {
                    sample.features.suspicious_instruction_hits = sample.features.suspicious_instruction_hits.saturating_add(1);
                    sample.add_signal(
                        SecuritySignalKind::SuspiciousInstruction,
                        (*rip & 0xFFFF_FFFF) as u32,
                        35,
                        "hardware-manager instruction trace matched a privileged operation",
                    );
                }
            }
            HwBusMessage::Call { from, to, target_name } => {
                sample.features.calls = sample.features.calls.saturating_add(1);
                if is_restricted_call(*to, target_name.as_deref()) {
                    sample.features.restricted_call_hits = sample.features.restricted_call_hits.saturating_add(1);
                    sample.add_signal(
                        SecuritySignalKind::RestrictedCall,
                        (*from & 0xFFFF_FFFF) as u32,
                        35,
                        "hardware-manager call target crosses into a restricted service boundary",
                    );
                }
            }
        }
    }

    sample.finalize(fail_stop_threshold);
    sample
}

pub fn is_restricted_io_port(address: u64) -> bool {
    matches!(address, 0x20..=0x21 | 0x40..=0x43 | 0x70..=0x71 | 0xCF8..=0xCFF)
}

pub fn is_privileged_interrupt(vector: u8) -> bool {
    vector < 32 || vector == 0x80
}

pub fn is_suspicious_instruction(opcode: u16, mnemonic: &str) -> bool {
    matches!(opcode, 0x0F01 | 0x0F30 | 0x0F32 | 0x0F34 | 0x0F35)
        || matches!(mnemonic, "vmcall" | "vmlaunch" | "vmresume" | "wrmsr" | "rdmsr" | "sysenter" | "sysexit")
}

pub fn is_restricted_call(target: u64, target_name: Option<&str>) -> bool {
    target < 0x1000
        || target_name
            .map(|name| matches!(name, "ghm.revoke_assignments" | "partitioner.remove_silicon_unit" | "runtime.reset"))
            .unwrap_or(false)
}
