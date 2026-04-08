use raw_cpuid::CpuId;
use crate::message;

fn check() {
    let cpuid = CpuId::new();
    let has_sse = cpuid.get_feature_info()
        .map_or(false, |finfo| finfo.has_sse());
    if has_sse {
        message!("\n", "CPU supports SSE!");
    }
}