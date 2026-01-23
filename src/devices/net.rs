//! Minimal networking scaffolding for HPVMx
//! Provides placeholder implementations for ping, lanscan, and an HTTP management server
//! so shell commands are available without requiring a fully wired NIC.

#![allow(dead_code)]

use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use log::{info, warn, error};

use super::net_hw;

static HTTPD_RUNNING: AtomicBool = AtomicBool::new(false);

/// Ensure hardware is initialized (SNP). Best-effort.
fn ensure_hw() {
    if !net_hw::is_initialized() {
        let _ = net_hw::init();
    }
}

/// Print simple NIC status to the console (MAC/MTU/link).
pub fn status() {
    ensure_hw();
    match net_hw::get_info() {
        Some(info) => {
            uefi::system::with_stdout(|s| {
                use core::fmt::Write as _;
                let _ = write!(s, "\nNIC: MAC=");
                for i in 0..info.mac_len.min(6) {
                    let _ = write!(s, "{:02x}{}", info.mac[i], if i < 5 { ":" } else { "" });
                }
                let _ = write!(s, " MTU={} Link={}\n", info.mtu, if info.media_present { "up" } else { "down" });
            });
        }
        None => {
            warn!("net: no NIC detected (SNP not found)");
        }
    }
}

/// Attempt to ping an IP address. Placeholder implementation.
/// Returns Ok(rtt_ms) on success, Err(message) on failure.
pub fn ping(ip: &str, _count: usize, _timeout_ms: u64) -> Result<u32, &'static str> {
    ensure_hw();
    if net_hw::get_info().is_some() {
        warn!("net: NIC present but network stack not yet implemented; cannot ping '{}" , ip);
        Err("network stack not implemented")
    } else {
        warn!("net: ping to '{}' not available: no NIC detected", ip);
        Err("no nic")
    }
}

/// Scan a /24 network by trying TCP port 80 (HTTP) like the provided batch example.
/// Example prefix: "192.168.1."
pub fn lanscan(prefix: &str) {
    // Validate prefix ends with a dot and has 3 octets.
    if !prefix.ends_with('.') {
        error!("net: prefix must end with '.' e.g. 192.168.1.");
        return;
    }
    let octets: Vec<&str> = prefix.trim_end_matches('.').split('.').collect();
    if octets.len() != 3 {
        error!("net: prefix must have 3 octets, e.g. 10.0.0.");
        return;
    }

    // Build 11 lines like the batch script to display a colored map.
    let mut lines: [String; 11] = [
        String::from("1   "), String::from("26  "), String::from("51  "), String::from("76  "),
        String::from("101 "), String::from("126 "), String::from("151 "), String::from("176 "),
        String::from("201 "), String::from("226 "), String::from("251 "),
    ];

    let mut found: Vec<String> = Vec::new();

    // Color blocks placeholders (no ANSI in UEFI console, so use symbols)
    let good = "[■]"; // reachable
    let bad = "[ ]";  // unreachable

    for host in 1..=255u16 {
        // Compose IP
        let ip = alloc::format!("{}{}", prefix, host);

        // Try to detect host by TCP:80 (placeholder always fails for now)
        let reachable = false; // until NIC is wired, we assume unreachable

        if reachable {
            found.push(ip.clone());
        }

        // Append block to the corresponding line
        let idx = match host {
            1..=25 => 0,
            26..=50 => 1,
            51..=75 => 2,
            76..=100 => 3,
            101..=125 => 4,
            126..=150 => 5,
            151..=175 => 6,
            176..=200 => 7,
            201..=225 => 8,
            226..=250 => 9,
            _ => 10,
        };
        lines[idx].push_str(if reachable { good } else { bad });

        // Periodic redraw to give user feedback
        if host % 5 == 0 || host == 255 {
            uefi::system::with_stdout(|s| {
                let _ = s.clear();
                let _ = core::fmt::Write::write_str(s, "\nLAN Scan (TCP:80)\n");
                let _ = core::fmt::Write::write_str(s, "────────────────────────────\n");
                for l in &lines { let _ = core::fmt::Write::write_str(s, l); let _ = core::fmt::Write::write_str(s, "\n"); }
                let _ = core::fmt::Write::write_str(s, "\nIPs Found:\n");
                for ipf in &found { let _ = core::fmt::Write::write_str(s, ipf); let _ = core::fmt::Write::write_str(s, "\n"); }
                let _ = core::fmt::Write::write_str(s, "\nTip: actual probing requires a NIC driver.\n");
            });
        }
    }
    info!("net: scan complete. hosts detected: {}", found.len());
}

/// Start a very small management HTTP server on a separate thread (placeholder).
/// In UEFI context, we simulate a background loop.
pub fn httpd_start(port: u16) {
    if HTTPD_RUNNING.swap(true, Ordering::SeqCst) {
        warn!("httpd: already running");
        return;
    }
    info!("httpd: starting on port {} (placeholder)", port);

    // Spawn a background timer using UEFI's timer events to simulate a running server.
    // We cannot handle real sockets yet.
    // No background thread support here; simply set the flag and log.
    // A future implementation can integrate with an async executor to accept HTTP connections.
    let _ = port;
}

pub fn httpd_stop() {
    if !HTTPD_RUNNING.swap(false, Ordering::SeqCst) {
        warn!("httpd: not running");
    }
}
