//! HPVMx Virtual Machine Monitor - Core hypervisor coordinator

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::vmm::vm::{VirtualMachine, VmState};
use crate::vmm::ghm::GlobalHardwareManager;
use crate::vmm::hwbus::HwBusMessage;
use crate::vmm::partitioner::HardwarePartitioner;
use crate::vmm::security::{DeepLevelSecurity, AutolyticProtocol};
use crate::vmm::vmbus::VmBusMessage;
use crate::dls::{SoftwareAnalysisMemory, SoftwareAnalysisSample};
use crate::{hpvm_info, hpvm_error, hpvm_log, vdebug};
use crate::filesystem::FileSystem;
use uefi::proto::console::text::Color;
use uefi::mem::memory_map::MemoryMap;

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
const MAX_BOOT_MEDIA_BYTES: usize = 1024 * 1024 * 1024;
const VM_DISPLAY_GPA: u64 = 0xE0000000;
const VM_MMIO_CONFIG_PORT: u16 = 0x3D0;
const VM_SERIAL_PORT: u16 = 0x3F8;

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

#[derive(Debug, Clone, Copy)]
struct DisplayDescriptor {
    width: u32,
    height: u32,
    stride: u32,
    bytes: usize,
    host_fb_hpa: u64,
}

fn detect_host_display_descriptor() -> Option<DisplayDescriptor> {
    let gop_handle = uefi::boot::get_handle_for_protocol::<uefi::proto::console::gop::GraphicsOutput>().ok()?;
    let mut gop = uefi::boot::open_protocol_exclusive::<uefi::proto::console::gop::GraphicsOutput>(gop_handle).ok()?;
    let mode = gop.current_mode_info();
    let (width, height) = mode.resolution();
    let stride = mode.stride() as u32;
    let host_fb_hpa = gop.frame_buffer().as_mut_ptr() as u64;
    let bytes = gop.frame_buffer().size();

    if width == 0 || height == 0 || bytes == 0 || host_fb_hpa == 0 {
        return None;
    }

    Some(DisplayDescriptor {
        width: width as u32,
        height: height as u32,
        stride,
        bytes,
        host_fb_hpa,
    })
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
    fn prime_vm_hardware_interfaces(vm: &mut VirtualMachine) {
        vm.hwbus.send_message(HwBusMessage::DevicePort {
            port: VM_SERIAL_PORT,
            size: 1,
            write: false,
            data: None,
        });
        vm.hwbus.send_message(HwBusMessage::Call {
            from: 0,
            to: 0x1000000,
            target_name: Some(String::from("hardware.interface.probe")),
        });
        vm.vmbus.send_message(VmBusMessage::IoRequest {
            address: VM_MMIO_CONFIG_PORT as u64,
            size: 4,
            write: false,
            data: None,
        });
    }

    fn map_vm_display(vm: &mut VirtualMachine) -> bool {
        let Some(display) = detect_host_display_descriptor() else {
            hpvm_error!("Display", "GOP framebuffer unavailable; VM display passthrough not ready");
            return false;
        };

        vm.mapper.add_memory_mapping(VM_DISPLAY_GPA, display.host_fb_hpa, display.bytes);
        vm.hwbus.send_message(HwBusMessage::MemoryAccess {
            gpa: VM_DISPLAY_GPA,
            hpa: display.host_fb_hpa,
            size: display.bytes,
            write: true,
        });

        let mut mode_blob = Vec::with_capacity(16);
        mode_blob.extend_from_slice(&display.width.to_le_bytes());
        mode_blob.extend_from_slice(&display.height.to_le_bytes());
        mode_blob.extend_from_slice(&display.stride.to_le_bytes());
        mode_blob.extend_from_slice(&(VM_DISPLAY_GPA as u32).to_le_bytes());

        vm.vmbus.send_message(VmBusMessage::IoRequest {
            address: VM_MMIO_CONFIG_PORT as u64,
            size: mode_blob.len(),
            write: true,
            data: Some(mode_blob),
        });
        vm.vmbus.send_message(VmBusMessage::Interrupt { vector: 0x20 });
        true
    }

    /// Create a new hypervisor manager instance
    pub fn new() -> Self {
        Self {
            vms: BTreeMap::new(),
            is_initialized: false,
            vm_count: 0,
            ghm: GlobalHardwareManager::new(8, 4096), // Example defaults
            partitioner: HardwarePartitioner::new(),
            security: DeepLevelSecurity::new(),
        }
    }

    /// Initialize the hypervisor
    pub fn initialize(&mut self) -> Result<(), &'static str> {
        if self.is_initialized {
            return Err("Hypervisor already initialized");
        }

        #[cfg(target_arch = "x86_64")]
        {
            use crate::hardware::vmx::VtxCapabilities;
            let vtx = VtxCapabilities::detect();
            if !vtx.available {
                return Err("VT-x not available on this CPU");
            }
        }

        self.is_initialized = true;
        Ok(())
    }

    /// Creates a new virtual machine.
    pub fn create_vm(&mut self, name: &str, memory_mb: u32, vcpu_count: u32) -> Result<u32, &'static str> {
        if !self.is_initialized {
            return Err("Hypervisor not initialized");
        }

        let vm_id = VM_ID_COUNTER.fetch_add(1, Ordering::SeqCst);

        let mut vm = VirtualMachine::new(vm_id, name, memory_mb, vcpu_count)?;

        // Use GHM to allocate physical resources (Push-Only)
        let mut allocated_cores = Vec::new();
        for _ in 0..vcpu_count {
            allocated_cores.push(self.ghm.allocate_core_to_vm(vm_id)?);
        }
        let hpa = self.ghm.allocate_memory_to_vm(vm_id, (memory_mb as usize) * 1024 * 1024)?;

        // Map the resources
        vm.mapper.add_memory_mapping(0, hpa, (memory_mb as usize) * 1024 * 1024);

        // Partition the hardware into a "Silicon"
        self.partitioner.create_silicon_unit(vm_id, allocated_cores, hpa, (memory_mb as usize) * 1024 * 1024)?;

        self.vms.insert(vm_id, vm);
        self.vm_count += 1;

        Ok(vm_id)
    }

    /// Create a VM, boot the supplied system media, and start DLS inspection.
    pub fn run_system_in_vm(
        &mut self,
        name: &str,
        media_path: &str,
        memory_mb: u32,
        vcpu_count: u32,
    ) -> Result<BootedSystem, &'static str> {
        crate::vdebug!("Boot", "run-system request: name='{}' media='{}' mem={}MB vcpus={}", name, media_path, memory_mb, vcpu_count);
        validate_vm_profile(memory_mb, vcpu_count)?;
        let media_kind = validate_boot_media_path(media_path)?;
        let vm_id = self.create_vm(name, memory_mb, vcpu_count)?;

        if let Some(vm) = self.vms.get_mut(&vm_id) {
            vm.vmbus.send_message(VmBusMessage::Call {
                from: 0,
                to: 0x1000000,
                target_name: Some(String::from("boot.media.load")),
            });
            vm.vmbus.send_message(VmBusMessage::StorageRequest {
                sector: 0,
                count: 1,
                write: false,
                data: None,
            });
            vm.hwbus.send_message(HwBusMessage::Call {
                from: 0,
                to: 0x1000000,
                target_name: Some(String::from("hardware_manager.map_boot_media")),
            });
        }

        self.boot_vm_with_media(vm_id, media_path)?;
        let _ = self.inspect_vm_unit_security(vm_id);

        Ok(BootedSystem {
            vm_id,
            media_path: String::from(media_path),
            media_kind,
        })
    }

    /// Trigger the Autolytic Protocol for a VM
    pub fn trigger_autolytic_response(&mut self, vm_id: u32, error_code: u32) -> Result<(), &'static str> {
        AutolyticProtocol::detect_violation(vm_id, error_code);

        if let Some(vm) = self.vms.get_mut(&vm_id) {
            vm.state = VmState::Failed;
            
            // Phase 3: Forensic Deep Scan
            let vhd_path = "disk0.vhd"; // Example
            if AutolyticProtocol::forensic_scan(vhd_path, error_code) {
                // Phase 4: Decommissioning & Zeroing
                AutolyticProtocol::decommissioning_and_zeroing(vm_id);
                self.ghm.revoke_assignments(vm_id);
                self.partitioner.remove_silicon_unit(vm_id);
                vm.state = VmState::Decommissioned;
            }
        }
        Ok(())
    }

    /// Run Deep Level Security over the VM's pending VMBUS traffic.
    pub fn inspect_vm_security(&mut self, vm_id: u32) -> Result<SoftwareAnalysisSample, &'static str> {
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
    pub fn inspect_vm_hardware_security(&mut self, vm_id: u32) -> Result<SoftwareAnalysisSample, &'static str> {
        let bus = &self.vms.get(&vm_id).ok_or("VM not found")?.hwbus;
        match self.security.inspect_hwbus(bus) {
            Ok(sample) => {
                crate::vdebug!("DLS", "VM {} HWBUS analysis verdict: {:?}", vm_id, sample.verdict);
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
        crate::vdebug!("DLS", "saved {} software-analysis samples to {}", memory.samples.len(), path);
        Ok(())
    }

    /// Start a virtual machine
    pub fn start_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        let vm = self.vms.get_mut(&vm_id).ok_or("VM not found")?;
        
        if vm.state == VmState::Running {
            return Err("VM is already running");
        }

        // In a real implementation, we would:
        // 1. Initialize VMX for this core if not already done
        // 2. Set up VMCS for this VM
        // 3. VMLAUNCH
        
        vm.state = VmState::Running;
        Ok(())
    }

    /// Stop a virtual machine
    pub fn stop_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        let vm = self.vms.get_mut(&vm_id).ok_or("VM not found")?;
        
        if vm.state == VmState::Stopped {
            return Err("VM is already stopped");
        }

        // In a real implementation, we would:
        // 1. Signal the vCPU to stop/exit
        // 2. Clear VMCS
        
        vm.state = VmState::Stopped;
        Ok(())
    }

    /// Reset a virtual machine
    pub fn reset_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        self.stop_vm(vm_id)?;
        self.start_vm(vm_id)?;
        Ok(())
    }

    /// Zero out a virtual machine's resources (Wipe memory and disk assignments)
    pub fn zero_vm(&mut self, vm_id: u32) -> Result<(), &'static str> {
        let vm = self.vms.get_mut(&vm_id).ok_or("VM not found")?;
        
        if vm.state == VmState::Running {
            return Err("Cannot zero a running VM");
        }

        // Phase 4 of Autolytic Protocol: Zeroing
        // 1. Zero the memory regions
        for mapping in vm.mapper.get_memory_mappings() {
            // In a real bare-metal environment, we would use a DMA-based zeroing 
            // or a fast loop to clear the physical memory at mapping.hpa
            // For now, we simulate this.
            crate::vdebug!("VMM", "Zeroing physical memory at 0x{:x} ({} bytes)", mapping.hpa, mapping.size);
        }

        // 2. Zero the disk regions if applicable
        // ...

        vm.state = VmState::Decommissioned;
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
        match self.vms.remove(&vm_id) {
            Some(_) => {
                self.vm_count = self.vm_count.saturating_sub(1);
                Ok(())
            }
            None => Err("VM not found"),
        }
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
            list_str.push_str(&format!("  INT {}  ID {}  NAME {}  STATE {}  MEM {} #CPU {}",
                                       vm.0, vm.1.id, vm.1.name, vm.1.state, vm.1.memory_mb, vm.1.vcpu_count));
        }

        (stats, list_str)
    }

    pub fn boot_vm_with_media(&mut self, vm_id: u32, media_path: &str) -> Result<(), &'static str> {
        let vm = self.vms.get_mut(&vm_id).ok_or("VM not found")?;
        let media_kind = validate_boot_media_path(media_path)?;

        crate::vdebug!("Boot", "Loading {} media: {}", media_kind, media_path);
        
        // 1. Load the media (kernel/ISO)
        let data = crate::kernel::KernelLoader::load_kernel(media_path)?;
        crate::vdebug!("Boot", "media loaded: vm={} kind={} bytes={}", vm_id, media_kind, data.len());
        if data.is_empty() {
            return Err("boot media is empty");
        }
        if data.len() > MAX_BOOT_MEDIA_BYTES {
            return Err("boot media size exceeds secure limit");
        }
        let sector_count = ((data.len() + 511) / 512).max(1).min(u32::MAX as usize) as u32;
        vm.vmbus.send_message(VmBusMessage::StorageRequest {
            sector: 0,
            count: sector_count,
            write: false,
            data: None,
        });
        
        // 2. Map the media data into guest memory
        // For simplicity, we map it at 0x1000000 (16MB)
        let guest_addr = 0x1000000;
        let hpa = self.ghm.allocate_memory_to_vm(vm_id, data.len())?;
        vm.hwbus.send_message(HwBusMessage::MemoryAccess {
            gpa: guest_addr,
            hpa,
            size: data.len(),
            write: true,
        });
        
        // In a real system, we'd use a physical copy.
        // For UEFI, we must ensure hpa is accessible or use boot services.
        // Here we use a safe copy since we are in the same address space (flat).
        unsafe {
            let dest = hpa as *mut u8;
            core::ptr::copy_nonoverlapping(data.as_ptr(), dest, data.len());
        }

        vm.mapper.add_memory_mapping(guest_addr, hpa, data.len());
        crate::vdebug!("Boot", "mapped boot media vm={} guest=0x{:x} host=0x{:x}", vm_id, guest_addr, hpa);
        vm.vmbus.send_message(VmBusMessage::Call {
            from: 0,
            to: guest_addr,
            target_name: Some(String::from("vm.entry.boot_media")),
        });
        vm.vmbus.send_message(VmBusMessage::InstructionTrace {
            rip: guest_addr,
            opcode: 0xEA,
            mnemonic: String::from("jmp_far"),
            length: 5,
        });

        // 3. Set up vCPU state
        let stack_size = 64 * 1024; // 64KB
        let stack_hpa = self.ghm.allocate_memory_to_vm(vm_id, stack_size)?;
        let stack_gpa = guest_addr + data.len() as u64 + 4096; // 4KB guard
        vm.mapper.add_memory_mapping(stack_gpa, stack_hpa, stack_size);
        vm.hwbus.send_message(HwBusMessage::MemoryAccess {
            gpa: stack_gpa,
            hpa: stack_hpa,
            size: stack_size,
            write: true,
        });

        if let Some(vcpu) = vm.get_vcpu_mut(0) {
            vcpu.set_instruction_pointer(guest_addr);
            vcpu.set_stack_pointer(stack_gpa + stack_size as u64);
        }

        Self::prime_vm_hardware_interfaces(vm);
        let display_ready = Self::map_vm_display(vm);
        if display_ready {
            crate::vdebug!("Display", "VM {} display configured (GPA 0x{:x})", vm_id, VM_DISPLAY_GPA);
        }

        vm.state = VmState::Running;
        crate::vdebug!("Boot", "VM {} is now running from {}", vm_id, media_path);

        Ok(())
    }

    pub fn save_vm_metadata(&self, path: &str) -> Result<(), &'static str> {
        let mut data = String::from("HPVMX_VMSTATE_V1\n");
        for (id, vm) in &self.vms {
            data.push_str(&format!(
                "{},{},{},{},{}\n",
                id,
                vm.name,
                vm.memory_mb,
                vm.vcpu_count,
                vm.state
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

