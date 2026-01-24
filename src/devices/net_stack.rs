//! Minimal networking stubs for HPVMx (loopback-only behavior, no smoltcp runtime)
//! This module emulates a tiny subset of networking so shell commands work
//! without pulling in extra device backends or std on the UEFI target.

#![allow(dead_code)]

use crate::Color;
use crate::hpvm_log;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::{hpvm_info, hpvm_warn};

static STACK_INIT: AtomicBool = AtomicBool::new(false);
static HTTPD: AtomicBool = AtomicBool::new(false);

/// Report which networking backend is active.
/// - "SNP (raw)" when a NIC via UEFI SNP is initialized.
/// - "loopback-stub" otherwise.
pub fn backend_name() -> &'static str {
    if crate::devices::net_hw::is_initialized() {
        "SNP (raw)"
    } else {
        "loopback-stub"
    }
}

#[derive(Copy, Clone, Default)]
pub struct NetStats {
    pub rx_pkts: u64,
    pub rx_bytes: u64,
    pub tx_pkts: u64,
    pub tx_bytes: u64,
}

struct NetState {
    now_ms: u64,
    stats: NetStats,
}

static mut NET_STATE: MaybeUninit<Option<NetState>> = MaybeUninit::uninit();

#[allow(static_mut_refs)]
pub fn init() {
    if STACK_INIT.load(Ordering::SeqCst) { return; }

    let state = NetState { now_ms: 0, stats: NetStats::default() };

    unsafe { NET_STATE.write(Some(state)); }
    STACK_INIT.store(true, Ordering::SeqCst);
    hpvm_info!("NET", "loopback stub initialized (127.0.0.1/8)");
}

#[inline]
pub fn is_initialized() -> bool { STACK_INIT.load(Ordering::SeqCst) }

/// Advance timers once. Call this periodically from the main loop.
#[allow(static_mut_refs)]
pub fn poll_tick() {
    if !is_initialized() { return; }
    unsafe {
        if let Some(state) = NET_STATE.assume_init_mut().as_mut() {
            state.now_ms = state.now_ms.saturating_add(1);
        }
    }

    // If a NIC is present via SNP, try to drain a few RX frames to keep stats up-to-date.
    if crate::devices::net_hw::is_initialized() {
        let mut buf = [0u8; 2048];
        // Try a small bounded loop to avoid spending too long in one shell tick.
        for _ in 0..8 {
            match crate::devices::net_hw::rx(&mut buf) {
                Ok(sz) => {
                    unsafe {
                        if let Some(st) = NET_STATE.assume_init_mut().as_mut() {
                            st.stats.rx_pkts = st.stats.rx_pkts.saturating_add(1);
                            st.stats.rx_bytes = st.stats.rx_bytes.saturating_add(sz as u64);
                        }
                    }
                }
                Err(_) => break, // no more packets available
            }
        }
    }
}

/// Very small loopback ping: if dst is 127.0.0.1, report success with tiny RTT.
pub fn ping_loopback(dst: &str) -> Result<u32, &'static str> {
    init();
    if dst == "127.0.0.1" || dst.eq_ignore_ascii_case("localhost") {
        // Pretend we sent an ICMP echo and received it immediately.
        hpvm_info!("PING", "loopback echo reply from {}: bytes=56 time=1ms TTL=64", dst);
        Ok(1)
    } else {
        hpvm_warn!("PING", "only loopback is available currently; cannot reach {}", dst);
        Err("no route to host (non-loopback)")
    }
}

pub fn httpd_start(_port: u16) {
    init();
    if HTTPD.swap(true, Ordering::SeqCst) {
        hpvm_warn!("HTTPD", "already running");
        return;
    }
    hpvm_info!("HTTPD", "loopback HTTPD placeholder started (no external clients)");
}

pub fn httpd_stop() {
    if !HTTPD.swap(false, Ordering::SeqCst) {
        hpvm_warn!("HTTPD", "not running");
        return;
    }
    hpvm_info!("HTTPD", "stopped");
}

/// Return a snapshot of current network stats (RX/TX counters).
#[allow(static_mut_refs)]
pub fn stats() -> NetStats {
    unsafe {
        if let Some(state) = NET_STATE.assume_init_ref().as_ref() {
            state.stats
        } else {
            NetStats::default()
        }
    }
}

/// Transmit a raw frame via SNP if available. Increments TX counters on success.
#[allow(static_mut_refs)]
pub fn snp_tx(frame: &[u8]) -> Result<(), &'static str> {
    if !crate::devices::net_hw::is_initialized() {
        return Err("no nic");
    }
    match crate::devices::net_hw::tx(frame) {
        Ok(()) => {
            unsafe {
                if let Some(st) = NET_STATE.assume_init_mut().as_mut() {
                    st.stats.tx_pkts = st.stats.tx_pkts.saturating_add(1);
                    st.stats.tx_bytes = st.stats.tx_bytes.saturating_add(frame.len() as u64);
                }
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}
