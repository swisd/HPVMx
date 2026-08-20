//! System Information aggregation module
//!
//! Gathers detailed hardware, CPU microarchitecture, UEFI firmware, memory layout,
//! GOP display subsystem, PCI devices, storage volumes, network stack, and hypervisor metrics.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::hardware::cpu::CpuInfo;
use crate::hardware::pci::{PciDeviceInfo, scan_bus};

#[derive(Debug, Clone)]
pub struct SystemInformation {
    // CPU
    pub cpu_brand: String,
    pub cpu_vendor: String,
    pub cpu_cores: u32,
    pub cpu_threads: u32,
    pub cpu_ap_count: u32,
    pub cpu_clock_mhz: u64,
    pub cpu_vmx: bool,
    pub cpu_svm: bool,
    pub cpu_avx2: bool,
    pub cpu_sse42: bool,
    pub cpu_64bit: bool,
    pub cpu_mp: bool,

    // Firmware & Platform
    pub fw_vendor: String,
    pub fw_revision: String,
    pub uefi_version: String,
    pub boot_mode: String,
    pub paging_mode: String,

    // Memory
    pub total_memory_mb: u64,
    pub used_memory_mb: u64,
    pub free_memory_mb: u64,

    // Display & GPU
    pub display_res: (usize, usize),
    pub display_stride: usize,
    pub display_format: String,
    pub gpu_device_name: String,
    pub gpu_usage: u32,
    pub fps: u32,
    pub frame_ms: u32,

    // Storage
    pub volume_count: usize,
    pub disk_read_total_bytes: u64,
    pub disk_write_total_bytes: u64,
    pub disk_read_ops: u64,
    pub disk_write_ops: u64,
    pub disk_read_kbps: u64,
    pub disk_write_kbps: u64,

    // Network
    pub net_backend: String,
    pub net_mac: String,
    pub net_ip: String,
    pub net_mask: String,
    pub net_gw: String,
    pub net_rx_total_bytes: u64,
    pub net_tx_total_bytes: u64,
    pub net_rx_pkts: u64,
    pub net_tx_pkts: u64,
    pub net_rx_kbps: u64,
    pub net_tx_kbps: u64,

    // Virtualization / Hypervisor
    pub hypervisor_engine: String,
    pub virt_hardware_assist: String,
    pub vm_count: usize,
    pub vm_running: usize,
}

impl SystemInformation {
    /// Collect current system information snapshot
    pub fn collect(resources: &crate::ui::SystemResources) -> Self {
        let cpu_info = CpuInfo::get();
        let tsc_mhz = unsafe { crate::TSC_PER_US as u64 };

        // Firmware information from UEFI System Table
        let (fw_vendor, fw_revision, uefi_version) = Self::query_uefi_firmware();

        // Memory
        let total_mem = resources.total_memory_mb as u64;
        let used_mem = resources.used_memory_mb as u64;
        let free_mem = total_mem.saturating_sub(used_mem);

        // Graphics / GOP info
        let (res_w, res_h) = crate::ui::pixel_graphics::PixelGraphics::new()
            .map(|pg| pg.resolution())
            .unwrap_or((1440, 900));
        let display_format = String::from("32-bit BGR / RGB (Direct Framebuffer)");

        // Find Display controller in PCI
        let pci_devices = scan_bus();
        let gpu_device_name = pci_devices.iter()
            .find(|dev| dev.class_id == 0x03)
            .map(|dev| {
                format!("{} {}", dev.vendor_name(), dev.subclass_name())
            })
            .unwrap_or_else(|| String::from("Standard UEFI GOP Display Adapter"));

        // Storage stats
        let (disk_r_bytes, disk_w_bytes, disk_r_ops, disk_w_ops) = crate::filesystem::disk_stats();
        let fs_state = crate::filesystem::FileSystem::get_state();
        let volume_count = fs_state.drive_handles.len().max(1);

        // Network stats
        let net_state = crate::devices::net_stack::get_state();
        let net_stats = crate::devices::net_stack::stats();
        let net_backend = String::from(crate::devices::net_stack::backend_name());
        let net_mac = format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            net_state.mac_addr[0], net_state.mac_addr[1], net_state.mac_addr[2],
            net_state.mac_addr[3], net_state.mac_addr[4], net_state.mac_addr[5]);
        let net_ip = format!("{}.{}.{}.{}",
            net_state.ip_addr[0], net_state.ip_addr[1], net_state.ip_addr[2], net_state.ip_addr[3]);
        let net_mask = format!("{}.{}.{}.{}",
            net_state.subnet_mask[0], net_state.subnet_mask[1], net_state.subnet_mask[2], net_state.subnet_mask[3]);
        let net_gw = format!("{}.{}.{}.{}",
            net_state.gateway[0], net_state.gateway[1], net_state.gateway[2], net_state.gateway[3]);

        // Virtualization info
        let virt_assist = if cpu_info.supports_vmx {
            String::from("Intel VMX (Hardware Assist Active)")
        } else if cpu_info.supports_svm {
            String::from("AMD SVM (Hardware Assist Active)")
        } else {
            String::from("Software Virtualization / Passthrough")
        };

        // VM count
        let vms = unsafe {
            crate::HYPERVISOR.as_ref().map(|hv| hv.list_vms()).unwrap_or_default()
        };
        let vm_count = vms.len();
        let vm_running = vms.iter().filter(|(_, _, state)| state.to_string().contains("running")).count();

        Self {
            cpu_brand: cpu_info.brand,
            cpu_vendor: cpu_info.vendor,
            cpu_cores: cpu_info.cores,
            cpu_threads: cpu_info.threads,
            cpu_ap_count: cpu_info.ap_count,
            cpu_clock_mhz: tsc_mhz,
            cpu_vmx: cpu_info.supports_vmx,
            cpu_svm: cpu_info.supports_svm,
            cpu_avx2: cpu_info.supports_avx2,
            cpu_sse42: cpu_info.supports_sse42,
            cpu_64bit: cpu_info.supports_64bit,
            cpu_mp: cpu_info.supports_mp,

            fw_vendor,
            fw_revision,
            uefi_version,
            boot_mode: String::from("UEFI 64-bit Long Mode (x86_64)"),
            paging_mode: String::from("4-Level Paging (PML4 Active)"),

            total_memory_mb: total_mem,
            used_memory_mb: used_mem,
            free_memory_mb: free_mem,

            display_res: (res_w, res_h),
            display_stride: res_w,
            display_format,
            gpu_device_name,
            gpu_usage: resources.gpu_usage,
            fps: resources.fps as u32,
            frame_ms: resources.frame_ms as u32,

            volume_count,
            disk_read_total_bytes: disk_r_bytes,
            disk_write_total_bytes: disk_w_bytes,
            disk_read_ops: disk_r_ops,
            disk_write_ops: disk_w_ops,
            disk_read_kbps: resources.disk_read_kbps,
            disk_write_kbps: resources.disk_write_kbps,

            net_backend,
            net_mac,
            net_ip,
            net_mask,
            net_gw,
            net_rx_total_bytes: net_stats.rx_bytes,
            net_tx_total_bytes: net_stats.tx_bytes,
            net_rx_pkts: net_stats.rx_pkts,
            net_tx_pkts: net_stats.tx_pkts,
            net_rx_kbps: resources.net_rx_kbps,
            net_tx_kbps: resources.net_tx_kbps,

            hypervisor_engine: String::from("HPVMx Type-1 Hypervisor Core v1.15.1"),
            virt_hardware_assist: virt_assist,
            vm_count,
            vm_running,
        }
    }

    fn query_uefi_firmware() -> (String, String, String) {
        if let Some(st_raw_ptr) = uefi::table::system_table_raw() {
            let st = unsafe { &*st_raw_ptr.as_ptr() };
            
            // Firmware vendor string from UTF-16 pointer
            let mut vendor_str = String::new();
            let mut ptr = st.firmware_vendor;
            if !ptr.is_null() {
                unsafe {
                    while *ptr != 0 && vendor_str.len() < 128 {
                        let c = *ptr;
                        if let Some(ch) = core::char::from_u32(c as u32) {
                            vendor_str.push(ch);
                        }
                        ptr = ptr.add(1);
                    }
                }
            }
            if vendor_str.is_empty() {
                vendor_str = String::from("EDK II / UEFI Firmware");
            }

            let fw_rev = st.firmware_revision;
            let fw_rev_str = format!("{:X}.{:02X}", fw_rev >> 16, fw_rev & 0xFFFF);

            let uefi_rev = st.header.revision.0;
            let uefi_rev_str = format!("{}.{:02}", uefi_rev >> 16, uefi_rev & 0xFFFF);

            (vendor_str, fw_rev_str, uefi_rev_str)
        } else {
            (
                String::from("UEFI 64-bit Platform"),
                String::from("1.00"),
                String::from("2.80"),
            )
        }
    }
}
