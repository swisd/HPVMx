//! CPU hardware abstraction and virtualization support

extern crate alloc;
use crate::Color;
use alloc::string::String;
use raw_cpuid::CpuId;

#[cfg(target_arch = "x86_64")]
pub mod vmx;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CpuInfo {
    pub brand: String,
    pub cores: u32,
    pub threads: u32,
    pub supports_64bit: bool,
    pub supports_vmx: bool,
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

        let brand = cpuid
            .get_extended_feature_info()
            .and_then(|_info| Some("Intel"))
            .map(|s: &str| {
                let mut result = String::new();
                for c in s.chars() {
                    result.push(c);
                }
                result
            })
            .unwrap_or_else(|| String::from("Unknown CPU"));

        hpvm_info!("cpu", "cpu brand: {}", brand);

        let cores = cpuid
            .get_feature_info()
            .map(|info| {
                let logical = info.max_logical_processor_ids();
                // Simple heuristic; actual core count needs MSR reading
                logical as u32
            })
            .unwrap_or(1);

        hpvm_info!("cpu", "cpu cores: {}", cores);

        let supports_64bit = cpuid
            .get_extended_feature_info()
            .map(|info| info.has_avx2())
            .unwrap_or(false);

        hpvm_info!("cpu", "supports 64-bit AVX2: {:?}", supports_64bit);

        let supports_vmx = cpuid
            .get_feature_info()
            .map(|info| info.has_vmx())
            .unwrap_or(false);

        hpvm_info!("cpu", "supports 64-bit VMX: {:?}", supports_64bit);

        Self {
            brand,
            cores,
            threads: cores,
            supports_64bit,
            supports_vmx,
        }
    }
}