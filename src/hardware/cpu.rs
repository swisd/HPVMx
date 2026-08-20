//! CPU hardware abstraction and virtualization support

extern crate alloc;
use crate::Color;
use crate::env::SpinLock;
use alloc::string::String;
use raw_cpuid::CpuId;

#[cfg(target_arch = "x86_64")]
pub mod vmx;

pub mod mp;
pub use mp::{MpManager, MpTopology, ProcessorCoreInfo};

/// Cached CPU information detected during startup.
pub static CACHED_CPU_INFO: SpinLock<Option<CpuInfo>> = SpinLock::new(None);

/// Information about the host CPU's capabilities.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CpuInfo {
    /// The CPU vendor string.
    pub vendor: String,
    /// The CPU brand string.
    pub brand: String,
    /// The number of physical cores.
    pub cores: u32,
    /// The number of logical threads.
    pub threads: u32,
    /// Whether the CPU supports 64-bit operations.
    pub supports_64bit: bool,
    /// Whether the CPU supports Intel VMX (Virtual Machine Extensions).
    pub supports_vmx: bool,
    /// Whether the CPU supports AMD SVM (Secure Virtual Machine).
    pub supports_svm: bool,
    /// Whether the CPU supports AVX2 extensions.
    pub supports_avx2: bool,
    /// Whether the CPU supports SSE4.2 extensions.
    pub supports_sse42: bool,
    /// Whether UEFI MP Services / multi-core APs are supported.
    pub supports_mp: bool,
    /// The number of enabled Application Processors (APs).
    pub ap_count: u32,
    /// Discovered MP processor topology (if available).
    pub topology: Option<MpTopology>,
}

#[allow(dead_code)]
macro_rules! hpvm_log {
    ($color:expr, $prefix:expr, $($arg:tt)*) => {
        uefi::system::with_stdout(|stdout| {
            // Bring the trait into scope INSIDE the closure
            //use uefi::proto::console::text::Output;
            use core::fmt::Write;

            // let old_attribute = stdout.get_attribute().ok();

            // Set prefix color
            let _ = stdout.set_color($color, uefi::proto::console::text::Color::Black);
            let _ = write!(stdout, "[{}] ", $prefix);

            // Reset to white for message
            match $color {
                Color::Yellow => {}
                Color::Red => {}
                _ => {let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);}
            }
            let _ = write!(stdout, $($arg)*);
            let _ = write!(stdout, "\n");
            let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);

            // Restore original attributes if they existed
            // if let Some(attr) = old_attribute {
            //     let _ = stdout.set_attribute(attr);
            // }
        })
    };
}

macro_rules! hpvm_info {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::LightCyan, $tag, $($arg)*) };
}

#[allow(dead_code)]
impl CpuInfo {
    /// Detect CPU capabilities
    pub fn detect() -> Self {
        let cpuid = CpuId::new();

        let vendor = cpuid
            .get_vendor_info()
            .map(|v| {
                let mut result = String::new();
                for c in v.as_str().chars() {
                    result.push(c);
                }
                result
            })
            .unwrap_or_else(|| String::from("Unknown"));

        let brand = cpuid
            .get_processor_brand_string()
            .map(|b| {
                let mut result = String::new();
                for c in b.as_str().trim().chars() {
                    result.push(c);
                }
                result
            })
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| {
                if vendor == "GenuineIntel" {
                    String::from("Intel(R) Core / Xeon Processor")
                } else if vendor == "AuthenticAMD" {
                    String::from("AMD Ryzen / EPYC Processor")
                } else {
                    String::from("x86_64 Compatible Processor")
                }
            });

        crate::vdebug!("cpu", "cpu vendor: {}, brand: {}", vendor, brand);

        let feature_info = cpuid.get_feature_info();
        let ext_feature_info = cpuid.get_extended_feature_info();
        let ext_proc_info = cpuid.get_extended_processor_and_feature_identifiers();

        let cores = feature_info
            .as_ref()
            .map(|info| {
                let logical = info.max_logical_processor_ids();
                logical as u32
            })
            .unwrap_or(1);

        crate::vdebug!("cpu", "cpu cores: {}", cores);

        let supports_avx2 = ext_feature_info
            .as_ref()
            .map(|info| info.has_avx2())
            .unwrap_or(false);

        let supports_sse42 = feature_info
            .as_ref()
            .map(|info| info.has_sse42())
            .unwrap_or(false);

        let supports_vmx = feature_info
            .as_ref()
            .map(|info| info.has_vmx())
            .unwrap_or(false);

        let supports_svm = ext_proc_info
            .as_ref()
            .map(|info| info.has_svm())
            .unwrap_or(false);

        let supports_64bit = ext_proc_info
            .as_ref()
            .map(|info| info.has_64bit_mode())
            .unwrap_or(true);

        crate::vdebug!("cpu", "supports 64-bit: {:?}, VMX: {:?}, SVM: {:?}, AVX2: {:?}", supports_64bit, supports_vmx, supports_svm, supports_avx2);

        let topology = MpManager::detect_topology();
        let (cores, threads, supports_mp, ap_count) = if let Some(ref topo) = topology {
            MpManager::log_topology(topo);
            let total = topo.total_processors as u32;
            let enabled_aps = topo.enabled_ap_count() as u32;
            (total.max(cores), total.max(cores), topo.has_usable_aps(), enabled_aps)
        } else {
            crate::vdebug!("cpu:mp", "UEFI MP Services protocol not available; running single-core BSP");
            (cores, cores, false, 0)
        };

        let info = Self {
            vendor,
            brand,
            cores,
            threads,
            supports_64bit,
            supports_vmx,
            supports_svm,
            supports_avx2,
            supports_sse42,
            supports_mp,
            ap_count,
            topology,
        };

        *CACHED_CPU_INFO.lock() = Some(info.clone());
        info
    }

    /// Returns the cached `CpuInfo`, or detects CPU capabilities if not yet cached.
    pub fn get() -> Self {
        let guard = CACHED_CPU_INFO.lock();
        if let Some(ref info) = *guard {
            info.clone()
        } else {
            drop(guard);
            Self::detect()
        }
    }
}

/// Returns the detected CPU core count.
pub fn core_count() -> u32 {
    let guard = CACHED_CPU_INFO.lock();
    if let Some(ref info) = *guard {
        info.cores
    } else {
        drop(guard);
        CpuInfo::detect().cores
    }
}

/// Returns the detected enabled AP core count.
pub fn ap_count() -> u32 {
    let guard = CACHED_CPU_INFO.lock();
    if let Some(ref info) = *guard {
        info.ap_count
    } else {
        drop(guard);
        CpuInfo::detect().ap_count
    }
}