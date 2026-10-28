//! HPVMx Virtual Machine Monitor - Core hypervisor coordinator

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::dls::{SoftwareAnalysisMemory, SoftwareAnalysisSample};
use crate::filesystem::FileSystem;
use crate::hardware::cpu::vmx::{Vmcs, VmxExitStack, VmxHostSession, VmxRunOutcome};
use crate::vmm::ept::{Ept, EptPermissions};
use crate::vmm::ghm::GlobalHardwareManager;
use crate::vmm::io::VirtualDevices;
use crate::vmm::partitioner::HardwarePartitioner;
use crate::vmm::scheduler::VcpuScheduler;
use crate::vmm::security::{AutolyticProtocol, DeepLevelSecurity};
use crate::vmm::vm::{VirtualMachine, VmState};
use crate::vmm::vmcs::{self, GuestState, HostState};
use crate::vmm::vmexit;
use crate::{hardware, hpvm_error, hpvm_info, hpvm_log, vdebug, vdebug_autoprefix};
use uefi::mem::memory_map::MemoryMap;
use uefi::proto::console::text::Color;
use crate::micro_c::CVM;
use crate::vmm::container::Container;
use crate::vmm::cvm::ContainerVirtualMachine;

static VM_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootMediaKind {
    Iso,
    DiskImage,
    Efi,
    Unknown,
}

impl BootMediaKind {
    pub fn detect(path: &str) -> Self {
        let normalized = path.to_ascii_lowercase();

        if normalized.ends_with(".iso") {
            Self::Iso
        } else if normalized.ends_with(".efi") {
            Self::Efi
        } else if normalized.ends_with(".img")
            || normalized.ends_with(".vhd")
            || normalized.ends_with(".vhdx")
            || normalized.ends_with(".vdi")
            || normalized.ends_with(".vmdk")
            || normalized.ends_with(".qcow")
            || normalized.ends_with(".qcow2")
            || normalized.ends_with(".raw")
            || normalized.ends_with(".bin")
        {
            Self::DiskImage
        } else {
            Self::Unknown
        }
    }
}

const MIN_VM_MEMORY_MB: u32 = 64;
const MAX_VM_MEMORY_MB: u32 = 32768;
const MIN_VM_VCPUS: u32 = 1;
const MAX_VM_VCPUS: u32 = 64;

impl core::fmt::Display for BootMediaKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BootMediaKind::Iso => write!(f, "ISO"),
            BootMediaKind::DiskImage => write!(f, "Disk"),
            BootMediaKind::Efi => write!(f, "EFI"),
            BootMediaKind::Unknown => write!(f, "Unknown"),
        }
    }
}

fn validate_boot_media_path(media_path: &str) -> Result<BootMediaKind, &'static str> {
    if media_path.is_empty() {
        return Err("empty boot media path");
    }
    if media_path.contains("..") {
        return Err("path traversal in boot media path is not allowed");
    }

    let media_kind = BootMediaKind::detect(media_path);
    if media_kind == BootMediaKind::Unknown {
        return Err("unsupported boot media format");
    }

    Ok(media_kind)
}

fn validate_vm_profile(memory_mb: u32, vcpu_count: u32) -> Result<(), &'static str> {
    if !(MIN_VM_MEMORY_MB..=MAX_VM_MEMORY_MB).contains(&memory_mb) {
        return Err("memory size outside secure bounds");
    }
    if !(MIN_VM_VCPUS..=MAX_VM_VCPUS).contains(&vcpu_count) {
        return Err("vCPU count outside secure bounds");
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct BootedSystem {
    pub vm_id: u32,
    pub media_path: String,
    pub media_kind: BootMediaKind,
}

/// Manages the lifecycle and execution of all virtual machines.
pub struct HypervisorManager {
    /// Map of VM ID to Virtual Machine instance
    pub(crate) vms: BTreeMap<u32, VirtualMachine>,
    pub(crate) containers: BTreeMap<u32, Container>,
    pub(crate) cvms: BTreeMap<u32, ContainerVirtualMachine>,
    /// Hypervisor capabilities
    pub is_initialized: bool,
    pub vm_count: u32,

    // New core components
    pub ghm: GlobalHardwareManager,
    pub partitioner: HardwarePartitioner,
    pub security: DeepLevelSecurity,
}

#[allow(dead_code)]
impl HypervisorManager {
    /// Create a new hypervisor manager instance
    pub fn new() -> Self {
        Self {
            vms: BTreeMap::new(),
            containers: BTreeMap::new(),
            cvms: BTreeMap::new(),
            is_initialized: false,
            vm_count: 0,
            ghm: GlobalHardwareManager::new(8, 4096), // Example defaults
            partitioner: HardwarePartitioner::new(),
            security: DeepLevelSecurity::new(),
        }
    }

    pub fn cpu_props_from_info(&mut self, cpu_info: &hardware::cpu::CpuInfo) {
        self.ghm.set_cpu_props(cpu_info);
    }

    /// Initialize the hypervisor
    pub fn initialize(&mut self) -> Result<(), &'static str> {
        if self.is_initialized {
            return Err("Hypervisor already initialized");
        }

        #[cfg(target_arch = "x86_64")]
        {
            use crate::hardware::vmx::VtxCapabilities;
            let vtx = unsafe { VtxCapabilities::detect() };
            if !vtx.vmxon_supported {
                return Err("processor does not implement VT-x");
            }
            if !vtx.available {
                return Err("firmware has not enabled VT-x outside SMX");
            }
            if !vtx.ept_supported || !vtx.ept_write_back || !vtx.ept_four_level_walk {
                return Err("processor does not support write-back four-level EPT");
            }
        }

        self.is_initialized = true;
        Ok(())
    }

    /// Creates a new virtual machine.
    pub fn create_vm(
        &mut self,
        name: &str,
        memory_mb: u32,
        vcpu_count: u32,
    ) -> Result<u32, &'static str> {
        if !self.is_initialized {
            return Err("Hypervisor not initialized");
        }
        validate_vm_profile(memory_mb, vcpu_count)?;

        let vm_id = VM_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

        let mut vm = VirtualMachine::new(vm_id, name, memory_mb, vcpu_count)?;

        let mut allocated_cores = Vec::new();
        for _ in 0..vcpu_count {
            match self.ghm.allocate_core_to_vm(vm_id) {
                Ok(core) => allocated_cores.push(core),
                Err(error) => {
                    self.ghm.revoke_assignments(vm_id)?;
                    return Err(error);
                }
            }
        }
        let memory_size = (memory_mb as usize) * 1024 * 1024;
        let hpa = match self.ghm.allocate_memory_to_vm(vm_id, memory_size) {
            Ok(address) => address,
            Err(error) => {
                self.ghm.revoke_assignments(vm_id)?;
                return Err(error);
            }
        };

        if let Err(error) = vm.allocate_memory(hpa as usize) {
            self.ghm.revoke_assignments(vm_id)?;
            return Err(error);
        }

        let mut ept = match Ept::new(false) {
            Ok(ept) => ept,
            Err(error) => {
                self.ghm.revoke_assignments(vm_id)?;
                return Err(error);
            }
        };
        if let Err(error) = ept.map_range(
            0,
            hpa,
            memory_size as u64,
            EptPermissions::READ | EptPermissions::WRITE | EptPermissions::EXECUTE,
        ) {
            drop(ept);
            self.ghm.revoke_assignments(vm_id)?;
            return Err(error);
        }
        vm.ept = Some(ept);
        vm.mapper.add_memory_mapping(0, hpa, memory_size);

        if let Err(error) =
            self.partitioner
                .create_silicon_unit(vm_id, allocated_cores, hpa, memory_size)
        {
            self.ghm.revoke_assignments(vm_id)?;
            return Err(error);
        }

        self.vms.insert(vm_id, vm);
        self.vm_count += 1;

        Ok(vm_id)
    }

    /// Attach isolated emulated disk, UART, and framebuffer devices to a VM.
    pub fn configure_vm_devices(
        &mut self,
        vm_id: u32,
        disk_bytes: usize,
        width: usize,
        height: usize,
    ) -> Result<(), &'static str> {
        let vm = self.vms.get_mut(&vm_id).ok_or("VM not found")?;
        if matches!(vm.state, VmState::Running | VmState::Paused) {
            return Err("cannot change virtual devices while VM is active");
        }
        vm.devices = Some(VirtualDevices::new(disk_bytes, width, height)?);
        Ok(())
    }

    pub fn configure_vm_execution_state(
        &mut self,
        vm_id: u32,
        guest: GuestState,
        host: HostState,
    ) -> Result<(), &'static str> {
        guest.validate()?;
        host.validate()?;
        let vm = self.vms.get_mut(&vm_id).ok_or("VM not found")?;
        if matches!(vm.state, VmState::Running | VmState::Paused) {
            return Err("cannot change execution state while VM is active");
        }
        if vm.ept.is_none() {
            return Err("VM has no EPT address space");
        }
        vm.guest_state = Some(guest);
        vm.host_state = Some(host);
        Ok(())
    }

    /// Build VMCS controls and state for one vCPU on an active VMX session.
    ///
    /// Keep the returned VMCS alive in the per-CPU scheduling map and enter it
    /// with `schedule_vcpu_once`.
    pub fn prepare_vmcs_for_vm<'session>(
        &self,
        vm_id: u32,
        session: &'session VmxHostSession,
        unrestricted_guest: bool,
    ) -> Result<Vmcs<'session>, &'static str> {
        let vm = self.vms.get(&vm_id).ok_or("VM not found")?;
        let ept = vm.ept.as_ref().ok_or("VM has no EPT address space")?;
        let guest = vm
            .guest_state
            .as_ref()
            .ok_or("VM guest state is not configured")?;
        let host = vm
            .host_state
            .as_ref()
            .ok_or("VM host state is not configured")?;
        if vm.devices.is_none() {
            return Err("VM virtual devices are not configured");
        }
        vmcs::prepare_vmcs(session, ept.ept_pointer(), unrestricted_guest, guest, host)
    }

    /// Run one pinned, preemption-timer-bounded quantum for a configured vCPU.
    ///
    /// # Safety
    /// The supplied VMCS must have been prepared for this VM on the same VMX
    /// session/processor, and its guest state must map only memory/devices owned
    /// by this VM. The guest executes privileged instructions until a VM exit.
    pub unsafe fn run_vcpu_quantum(
        &mut self,
        vm_id: u32,
        vcpu_id: u32,
        vmcs: &mut Vmcs<'_>,
        exit_stack: &VmxExitStack,
    ) -> Result<VmxRunOutcome, &'static str> {
        let vm = self.vms.get_mut(&vm_id).ok_or("VM not found")?;
        if matches!(
            vm.state,
            VmState::Paused | VmState::Stopped | VmState::Failed | VmState::Decommissioned
        ) {
            return Err("VM cannot execute in its current state");
        }
        let (vcpus, vm_devices) = (&mut vm.vcpus, &mut vm.devices);
        let vcpu = vcpus.get_mut(vcpu_id as usize).ok_or("vCPU not found")?;
        let devices = vm_devices
            .as_mut()
            .ok_or("VM virtual devices are not configured")?;
        let (outcome, fatal) = unsafe {
            vmexit::run_guest(
                vmcs,
                exit_stack,
                &mut vcpu.registers,
                &mut vcpu.fpu_state,
                devices,
            )?
        };
        match outcome {
            VmxRunOutcome::Yielded => {
                vm.state = VmState::Running;
                vcpu.resume();
            }
            VmxRunOutcome::Halted => {
                vm.state = VmState::Running;
                vcpu.halt();
            }
            VmxRunOutcome::Fatal | VmxRunOutcome::EntryFailed => {
                vm.state = VmState::Failed;
                return Err(fatal.unwrap_or("guest VM entry or exit failed"));
            }
        }
        Ok(outcome)
    }

    pub fn register_scheduled_vcpu(
        &self,
        scheduler: &mut VcpuScheduler,
        vm_id: u32,
        vcpu_id: u32,
    ) -> Result<(), &'static str> {
        let vm = self.vms.get(&vm_id).ok_or("VM not found")?;
        if vm.get_vcpu(vcpu_id).is_none() {
            return Err("vCPU not found");
        }
        if vm.ept.is_none()
            || vm.guest_state.is_none()
            || vm.host_state.is_none()
            || vm.devices.is_none()
        {
            return Err("VM execution state, EPT, and devices must be configured");
        }
        scheduler.register(vm_id, vcpu_id)
    }

    pub fn wake_scheduled_vcpu(
        &mut self,
        scheduler: &mut VcpuScheduler,
        vm_id: u32,
        vcpu_id: u32,
    ) -> Result<(), &'static str> {
        let vm = self.vms.get_mut(&vm_id).ok_or("VM not found")?;
        let vcpu = vm.get_vcpu_mut(vcpu_id).ok_or("vCPU not found")?;
        scheduler.wake(vm_id, vcpu_id)?;
        vcpu.resume();
        Ok(())
    }

    /// Execute one ready vCPU quantum using its VMCS from the caller-owned
    /// pinned-session map. Timer exits are requeued round-robin; HLT exits park
    /// the vCPU until `wake_scheduled_vcpu` is called.
    pub fn schedule_vcpu_once<'session>(
        &mut self,
        scheduler: &mut VcpuScheduler,
        vmscs: &mut BTreeMap<(u32, u32), Vmcs<'session>>,
        exit_stack: &VmxExitStack,
    ) -> Result<Option<(u32, u32, VmxRunOutcome)>, &'static str> {
        scheduler.schedule_once(|vm_id, vcpu_id| {
            let vmcs = vmscs
                .get_mut(&(vm_id, vcpu_id))
                .ok_or("scheduled vCPU has no prepared VMCS")?;
            unsafe { self.run_vcpu_quantum(vm_id, vcpu_id, vmcs, exit_stack) }
        })
    }

    /// Validate an external-media launch request.
    ///
    /// The VM is not created until guest-state setup and VM entry are available.
    pub fn run_system_in_vm(
        &mut self,
        name: &str,
        media_path: &str,
        memory_mb: u32,
        vcpu_count: u32,
    ) -> Result<BootedSystem, &'static str> {
        crate::vdebug!(
            "Boot",
            "run-system request: name='{}' media='{}' mem={}MB vcpus={}",
            name,
            media_path,
            memory_mb,
            vcpu_count
        );
        validate_vm_profile(memory_mb, vcpu_count)?;
        validate_boot_media_path(media_path)?;
        Err("guest media boot requires VMCS guest-state setup, VM entry, and virtual devices")
    }

    /// Trigger the Autolytic Protocol for a VM
    pub fn trigger_autolytic_response(
        &mut self,
        vm_id: u32,
        error_code: u32,
    ) -> Result<(), &'static str> {
        AutolyticProtocol::detect_violation(vm_id, error_code);

        if let Some(vm) = self.vms.get_mut(&vm_id) {
            vm.state = VmState::Failed;

            // Phase 3: Forensic Deep Scan
            let vhd_path = "disk0.vhd"; // Example
            if AutolyticProtocol::forensic_scan(vhd_path, error_code) {
                // Phase 4: Decommissioning & Zeroing
                AutolyticProtocol::decommissioning_and_zeroing(vm_id);
                self.ghm.revoke_assignments(vm_id)?;
                self.partitioner.remove_silicon_unit(vm_id);
                vm.state = VmState::Decommissioned;
            }
        }
        Ok(())
    }

    /// Run Deep Level Security over the VM's pending VMBUS traffic.
    pub fn inspect_vm_security(
        &mut self,
        vm_id: u32,
    ) -> Result<SoftwareAnalysisSample, &'static str> {
        let bus = &self.vms.get(&vm_id).ok_or("VM not found")?.vmbus;
        match self.security.inspect_bus(bus) {
            Ok(sample) => {
                crate::vdebug!("DLS", "VM {} analysis verdict: {:?}", vm_id, sample.verdict);
                Ok(sample)
            }
            Err(e) => {
                hpvm_error!("DLS", "VM {} security violation: {}", vm_id, e);
                self.trigger_autolytic_response(vm_id, 0xD15)?;
                Err(e)
            }
        }
    }

    /// Run Deep Level Security over the VM hardware-manager bus.
    pub fn inspect_vm_hardware_security(
        &mut self,
        vm_id: u32,
    ) -> Result<SoftwareAnalysisSample, &'static str> {
        let bus = &self.vms.get(&vm_id).ok_or("VM not found")?.hwbus;
        match self.security.inspect_hwbus(bus) {
            Ok(sample) => {
                crate::vdebug!(
                    "DLS",
                    "VM {} HWBUS analysis verdict: {:?}",
                    vm_id,
                    sample.verdict
                );
                Ok(sample)
            }
            Err(e) => {
                hpvm_error!("DLS", "VM {} HWBUS security violation: {}", vm_id, e);
                self.trigger_autolytic_response(vm_id, 0xD16)?;
                Err(e)
            }
        }
    }

    /// Run Deep Level Security over both diagram buses for the VM unit.
    pub fn inspect_vm_unit_security(&mut self, vm_id: u32) -> Result<(), &'static str> {
        self.inspect_vm_security(vm_id)?;
        self.inspect_vm_hardware_security(vm_id)?;
        Ok(())
    }

    /// Latest serializable sample captured by the DLS engine.
    pub fn latest_security_training_sample(&self) -> Option<&SoftwareAnalysisSample> {
        self.security.latest_analysis_sample()
    }

    /// Serializable DLS memory for software analysis and model training.
    pub fn security_training_memory(&self) -> &SoftwareAnalysisMemory {
        self.security.analysis_memory()
    }

    /// Persist the learned software-analysis memory in a JSON-lines friendly report.
    pub fn save_security_training_report(&self, path: &str) -> Result<(), &'static str> {
        let memory = self.security_training_memory();
        let mut data = String::from("{\"schema_version\":");
        data.push_str(&format!("{}", memory.schema_version));
        data.push_str(",\"stats\":{\"samples_seen\":");
        data.push_str(&format!("{}", memory.stats.samples_seen));
        data.push_str(",\"benign_samples\":");
        data.push_str(&format!("{}", memory.stats.benign_samples));
        data.push_str(",\"suspicious_samples\":");
        data.push_str(&format!("{}", memory.stats.suspicious_samples));
        data.push_str(",\"malicious_samples\":");
        data.push_str(&format!("{}", memory.stats.malicious_samples));
        data.push_str(",\"cumulative_risk_score\":");
        data.push_str(&format!("{}", memory.stats.cumulative_risk_score));
        data.push_str(",\"last_risk_score\":");
        data.push_str(&format!("{}", memory.stats.last_risk_score));
        data.push_str("},\"samples\":[");

        for (idx, sample) in memory.samples.iter().enumerate() {
            if idx > 0 {
                data.push(',');
            }
            data.push_str("{\"vm_id\":");
            data.push_str(&format!("{}", sample.vm_id));
            data.push_str(",\"artifact_name\":\"");
            push_json_string(&mut data, &sample.artifact_name);
            data.push_str("\",\"surface\":\"");
            data.push_str(&format!("{:?}", sample.surface));
            data.push_str("\",\"security_level\":\"");
            data.push_str(&format!("{:?}", sample.security_level));
            data.push_str("\",\"verdict\":\"");
            data.push_str(&format!("{:?}", sample.verdict));
            data.push_str("\",\"label\":\"");
            data.push_str(&format!("{:?}", sample.label));
            data.push_str("\",\"risk_score\":");
            data.push_str(&format!("{}", sample.features.risk_score));
            data.push_str(",\"message_count\":");
            data.push_str(&format!("{}", sample.features.message_count));
            data.push_str(",\"instructions\":");
            data.push_str(&format!("{}", sample.features.instructions));
            data.push_str(",\"calls\":");
            data.push_str(&format!("{}", sample.features.calls));
            data.push_str(",\"signals\":");
            data.push_str(&format!("{}", sample.signals.len()));
            data.push('}');
        }

        data.push_str("]}");

        let _ = FileSystem::remove(path);
        FileSystem::touch(path)?;
        FileSystem::write_to_file(path, &data, 'w')?;
        crate::vdebug!(
            "DLS",
            "saved {} software-analysis samples to {}",
            memory.samples.len(),
            path
        );
        Ok(())
    }

    /// Start a virtual machine
    pub fn start_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        let vm = self.vms.get(&vm_id).ok_or("VM not found")?;
        match vm.state {
            VmState::Running => return Err("VM is already running"),
            VmState::Created | VmState::Stopped => {}
            VmState::Paused => return Err("VM resume is not implemented"),
            VmState::Failed | VmState::Decommissioned => {
                return Err("VM cannot be started in its current state");
            }
        }
        Err("VM start requires guest state, devices, an active VMX session, and a prepared VMCS")
    }

    /// Stop a virtual machine
    pub fn stop_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        let vm = self.vms.get(&vm_id).ok_or("VM not found")?;
        match vm.state {
            VmState::Running | VmState::Paused => {
                Err("stopping a guest requires an active VM-exit backend")
            }
            VmState::Stopped => Err("VM is already stopped"),
            VmState::Created => Err("VM is not running"),
            VmState::Failed | VmState::Decommissioned => {
                Err("VM cannot be stopped in its current state")
            }
        }
    }

    /// Reset a virtual machine
    pub fn reset_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        self.stop_vm(vm_id)?;
        self.start_vm(vm_id)?;
        Ok(())
    }

    /// Zero out a virtual machine's resources (Wipe memory and disk assignments)
    pub fn zero_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        let vm = self.vms.get(&vm_id).ok_or("VM not found")?;
        if matches!(vm.state, VmState::Running | VmState::Paused) {
            return Err("Cannot zero a running or paused VM");
        }

        self.ghm.revoke_assignments(vm_id)?;
        self.partitioner.remove_silicon_unit(vm_id);
        self.vms.get_mut(&vm_id).ok_or("VM not found")?.state = VmState::Decommissioned;
        Ok(())
    }

    /// Get VM by ID
    pub fn get_vm(&self, vm_id: u32) -> Option<&VirtualMachine> {
        self.vms.get(&vm_id)
    }

    /// Get mutable VM by ID
    pub fn get_vm_mut(&mut self, vm_id: u32) -> Option<&mut VirtualMachine> {
        self.vms.get_mut(&vm_id)
    }

    /// List all VMs
    pub fn list_vms(&self) -> Vec<(u32, String, VmState)> {
        self.vms
            .iter()
            .map(|(id, vm)| (*id, vm.name.clone(), vm.state))
            .collect()
    }

    /// Delete a VM
    pub fn delete_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        let vm = self.vms.get(&vm_id).ok_or("VM not found")?;
        if matches!(vm.state, VmState::Running | VmState::Paused) {
            return Err("cannot delete a running or paused VM");
        }
        self.ghm.revoke_assignments(vm_id)?;
        self.partitioner.remove_silicon_unit(vm_id);
        self.vms.remove(&vm_id);
        self.vm_count = self.vm_count.saturating_sub(1);
        Ok(())
    }

    /// Get hypervisor statistics
    pub fn get_stats(&self) -> HypervisorStats {
        let running_vms = self
            .vms
            .iter()
            .filter(|(_, vm)| matches!(vm.state, VmState::Running))
            .count() as u32;

        let total_vm_memory = self.vms.iter().map(|(_, vm)| vm.memory_mb).sum();

        let mut total_phys_memory_mb = crate::get_total_physical_memory_mb();
        let mut free_phys_memory_mb = 0;

        match uefi::boot::memory_map(uefi::boot::MemoryType::LOADER_DATA) {
            Ok(map) => {
                for entry in map.entries() {
                    let size_mb = (entry.page_count * 4096) / (1024 * 1024);
                    // If TOTAL_PHYSICAL_MEMORY_MB wasn't captured correctly at boot, accumulate it here as fallback
                    if total_phys_memory_mb == 0 {
                        total_phys_memory_mb += size_mb as u32;
                    }
                    if entry.ty == uefi::boot::MemoryType::CONVENTIONAL {
                        free_phys_memory_mb += size_mb as u32;
                    }
                }
            }
            Err(_) => {
                if total_phys_memory_mb == 0 {
                    total_phys_memory_mb = 1024; // Fallback
                }
                free_phys_memory_mb = 512;
            }
        }

        HypervisorStats {
            initialized: self.is_initialized,
            total_vms: self.vm_count,
            running_vms,
            total_memory_mb: total_vm_memory,
            total_physical_memory_mb: total_phys_memory_mb,
            used_physical_memory_mb: total_phys_memory_mb.saturating_sub(free_phys_memory_mb),
        }
    }

    pub fn get_stats_advanced(&self) -> (HypervisorStats, String) {
        let stats = self.get_stats();

        let vms = self.vms.iter();

        let mut list_str = String::new();

        for vm in vms {
            if !list_str.is_empty() {
                list_str.push('\n');
            }
            list_str.push_str(&format!(
                "  INT {}  ID {}  NAME {}  STATE {}  MEM {} #CPU {}",
                vm.0, vm.1.id, vm.1.name, vm.1.state, vm.1.memory_mb, vm.1.vcpu_count
            ));
        }

        (stats, list_str)
    }

    /// Validate boot media, but do not treat loading bytes as executing a guest.
    pub fn boot_vm_with_media(&mut self, vm_id: u32, media_path: &str) -> Result<(), &'static str> {
        self.vms.get(&vm_id).ok_or("VM not found")?;
        validate_boot_media_path(media_path)?;
        Err("guest media boot requires VMCS guest-state setup, VM entry, and virtual devices")
    }

    pub fn save_vm_metadata(&self, path: &str) -> Result<(), &'static str> {
        let mut data = String::from("HPVMX_VMSTATE_V1\n");
        for (id, vm) in &self.vms {
            data.push_str(&format!(
                "{},{},{},{},{}\n",
                id, vm.name, vm.memory_mb, vm.vcpu_count, vm.state
            ));
        }

        let _ = FileSystem::remove(path);
        FileSystem::touch(path)?;
        FileSystem::write_to_file(path, &data, 'w')?;
        crate::vdebug!("VMM", "saved {} VM definitions to {}", self.vms.len(), path);
        Ok(())
    }

    pub fn restore_vm_metadata(&mut self, path: &str) -> Result<u32, &'static str> {
        let data = FileSystem::read_file_to_string(path)?;
        let mut restored = 0;

        for line in data.lines().skip(1) {
            let cols: Vec<&str> = line.split(',').collect();
            if cols.len() < 5 {
                continue;
            }

            let name = cols[1];
            if self.vms.values().any(|vm| vm.name == name) {
                continue;
            }

            let memory_mb = cols[2].parse::<u32>().unwrap_or(256);
            let vcpu_count = cols[3].parse::<u32>().unwrap_or(1);
            if self.create_vm(name, memory_mb, vcpu_count).is_ok() {
                restored += 1;
            }
        }

        crate::vdebug!("VMM", "restored {} VM definitions from {}", restored, path);
        Ok(restored)
    }
}

/// Global statistics for the hypervisor.
#[derive(Debug, Clone)]
pub struct HypervisorStats {
    /// Whether the hypervisor is initialized.
    pub initialized: bool,
    /// Number of virtual machines currently defined.
    pub total_vms: u32,
    /// Number of virtual machines currently running.
    pub running_vms: u32,
    /// Total memory allocated to all VMs in MB.
    pub total_memory_mb: u32,
    /// Total system memory in MB.
    pub total_physical_memory_mb: u32,
    /// Memory used by system in MB.
    pub used_physical_memory_mb: u32,
}

fn push_json_string(data: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '"' => data.push_str("\\\""),
            '\\' => data.push_str("\\\\"),
            '\n' => data.push_str("\\n"),
            '\r' => data.push_str("\\r"),
            '\t' => data.push_str("\\t"),
            _ => data.push(ch),
        }
    }
}
