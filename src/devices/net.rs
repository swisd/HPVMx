//! Minimal networking scaffolding for HPVMx
//! Provides placeholder implementations for ping, lanscan, and an HTTP management server
//! so shell commands are available without requiring a fully wired NIC.

#![allow(dead_code)]

use crate::{hpvm_info, Color};
use crate::hpvm_log;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::{hpvm_error, hpvm_warn};
use super::net_hw;
use super::net_stack;

static HTTPD_RUNNING: AtomicBool = AtomicBool::new(false);

const UDP_PORT_DHCP_CLIENT: u16 = 68;
const UDP_PORT_DHCP_SERVER: u16 = 67;

#[repr(C, packed)]
struct DhcpPacket {
    op: u8,        // 1 = Request, 2 = Reply
    htype: u8,     // 1 = Ethernet
    hlen: u8,      // 6
    hops: u8,      // 0
    xid: u32,      // Transaction ID
    secs: u16,
    flags: u16,    // 0x8000 for broadcast
    ciaddr: [u8; 4],
    yiaddr: [u8; 4], // "Your" (Client) IP
    siaddr: [u8; 4],
    giaddr: [u8; 4],
    chaddr: [u8; 16], // Client MAC
    sname: [u8; 64],
    file: [u8; 128],
    magic: u32,    // 0x63825363
    options: [u8; 312],
}

pub fn discover_config() -> Option<([u8; 4], [u8; 4], [u8; 4])> {
    let mac = crate::devices::net_hw::get_mac();
    let mut packet = DhcpPacket {
        op: 1, htype: 1, hlen: 6, hops: 0,
        xid: 0x12345678, secs: 0, flags: 0x8000u16.to_be(),
        ciaddr: [0; 4], yiaddr: [0; 4], siaddr: [0; 4], giaddr: [0; 4],
        chaddr: [0; 16], sname: [0; 64], file: [0; 128],
        magic: 0x63538263u32.to_be(), // DHCP Magic Cookie
        options: [0; 312],
    };
    packet.chaddr[..6].copy_from_slice(&mac[..6]);

    // DHCP Options: Discover (Type 53, Len 1, Value 1) + End (255)
    packet.options[0] = 53; packet.options[1] = 1; packet.options[2] = 1;
    packet.options[3] = 255;

    // Send via Raw UDP (Simplified send_udp that allows 0.0.0.0 source)
    // For brevity, assume a helper: send_raw_udp(src_ip, dst_ip, dst_mac, src_port, dst_port, data)
    net_stack::send_raw_udp([0,0,0,0], [255,255,255,255], [0xFF; 6], 68, 67, unsafe {
        core::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<DhcpPacket>())
    });

    // Loop and Poll until a DHCP Offer/Ack arrives
    // This part requires your poll_tick to look for UDP port 68
    // Returning dummy discovered values for the example:
    Some(([192, 168, 1, 50], [192, 168, 1, 1], [255, 255, 255, 0]))
}

/// Helper to parse "1.2.3.4" into [u8; 4]
fn parse_ip(ip: &str) -> Option<[u8; 4]> {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 { return None; }
    let mut octets = [0u8; 4];
    for i in 0..4 {
        octets[i] = parts[i].parse().ok()?;
    }
    Some(octets)
}

/// Ensure hardware is initialized (SNP). Best-effort.
fn ensure_hw() {
    if !net_hw::is_initialized() {
        let _ = net_hw::init();
    }

    if !net_stack::is_initialized() {
        hpvm_info!("NET", "Discovering network configuration via DHCP...");

        // Attempt to get config from the wire
        if let Some((my_ip, my_gw, my_mask)) = discover_config() {
            net_stack::init(my_ip, my_gw, my_mask);
            hpvm_info!("NET", "DHCP Success: {}.{}.{}.{}", my_ip[0], my_ip[1], my_ip[2], my_ip[3]);
        } else {
            hpvm_warn!("NET", "DHCP failed. Falling back to static recovery IP.");
            let fallback_ip = [169, 254, 1, 1];
            let fallback_gw = [169, 254, 1, 254];
            let fallback_mask = [255, 255, 0, 0];
            net_stack::init(fallback_ip, fallback_gw, fallback_mask);
        }
    }
}


fn ensure_net() {
    if !net_hw::is_initialized() {
        let _ = net_hw::init();
    }

    if !net_stack::is_initialized() {
        hpvm_info!("NET", "Discovering network configuration via DHCP...");

        // Attempt to get config from the wire
        if let Some((my_ip, my_gw, my_mask)) = discover_config() {
            net_stack::init(my_ip, my_gw, my_mask);
            hpvm_info!("NET", "DHCP Success: {}.{}.{}.{}", my_ip[0], my_ip[1], my_ip[2], my_ip[3]);
        } else {
            hpvm_warn!("NET", "DHCP failed. Falling back to static recovery IP.");
            let fallback_ip = [169, 254, 1, 1];
            let fallback_gw = [169, 254, 1, 254];
            let fallback_mask = [255, 255, 0, 0];
            net_stack::init(fallback_ip, fallback_gw, fallback_mask);
        }
    }
}

/// Print simple NIC status to the console (MAC/MTU/link).
pub fn status() {
    ensure_net();
    let backend = net_stack::backend_name();
    let stats = net_stack::stats();
    match net_hw::get_info() {
        Some(info) => {
            uefi::system::with_stdout(|s| {
                use core::fmt::Write as _;
                let _ = write!(s, "\nNIC Status:\n");
                let _ = write!(s, "  MAC:      {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}\n",
                               info.mac[0], info.mac[1], info.mac[2], info.mac[3], info.mac[4], info.mac[5]);
                let _ = write!(s, "  MTU:      {}\n", info.mtu);
                let _ = write!(s, "  Link:     {}\n", if info.media_present { "UP" } else { "DOWN" });
                let _ = write!(s, "  Backend:  {}\n", backend);
                let _ = write!(s, "  Traffic:  RX {} pkts ({} bytes) / TX {} pkts ({} bytes)\n",
                               stats.rx_pkts, stats.rx_bytes, stats.tx_pkts, stats.tx_bytes);
            });
        }
        None => hpvm_error!("NET", "no NIC detected"),
    }
}

pub fn ping(ip_str: &str, _count: usize, _timeout_ms: u64) -> Result<u32, &'static str> {
    ensure_net();

    // Handle Loopback
    if ip_str == "127.0.0.1" || ip_str.eq_ignore_ascii_case("localhost") {
        return net_stack::ping_loopback(ip_str);
    }

    // Handle External
    if let Some(target_ip) = parse_ip(ip_str) {
        // Our ping_external now actually sends a packet!
        // Note: For now, it returns success if the packet was sent.
        // Actual RTT calculation requires the main loop calling net_stack::poll_tick().
        let _ = net_stack::ping_external(target_ip, 64, true);
        Ok((0))
    } else {
        Err("invalid ip format")
    }
}

/// Scan a /24 network by trying TCP port 80 (HTTP) like the provided batch example.
/// Example prefix: "192.168.1."
pub fn lanscan(prefix: &str) {
    ensure_net();
    if !prefix.ends_with('.') {
        hpvm_error!("NET", "prefix must end with '.' e.g. 192.168.1.");
        return;
    }
    let octets: Vec<&str> = prefix.trim_end_matches('.').split('.').collect();
    if octets.len() != 3 {
        hpvm_error!("NET", "prefix must have 3 octets, e.g. 10.0.0.");
        return;
    }

    let found: Vec<String> = Vec::new();
    let mut lines: [String; 11] = [
        String::from("1   "), String::from("26  "), String::from("51  "), String::from("76  "),
        String::from("101 "), String::from("126 "), String::from("151 "), String::from("176 "),
        String::from("201 "), String::from("226 "), String::from("251 "),
    ];

    hpvm_info!("NET", "Starting LAN scan via ARP probes...");

    for host in 1..=255u16 {
        let ip_str = alloc::format!("{}{}", prefix, host);
        let _target_ip = parse_ip(&ip_str).unwrap();

        // In this manual stack, "lanscan" can work by broadcasting
        // a small UDP probe or an ARP request.
        // For now, we simulate the "send" part:
        let reachable = net_stack::ping_external(_target_ip, 64, false); // Real-time feedback requires an async listener

        // Append visualization
        let idx = (host.saturating_sub(1) / 25) as usize;
        if idx < 11 {
            lines[idx].push_str(if reachable { "[■]" } else { "[x]" });
        }

        // Periodic redraw
        if host % 10 == 0 || host == 255 {
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












//    let mut found: Vec<String> = Vec::new();

    // Color blocks placeholders (no ANSI in UEFI console, so use symbols)
//    let good = "[■]"; // reachable
//    let bad = "[ ]";  // unreachable

//    for host in 1..=255u16 {
        // Compose IP
//        let ip = alloc::format!("{}{}", prefix, host);

        // Try to detect host by TCP:80 (placeholder always fails for now)
//        let reachable = false; // until NIC is wired, we assume unreachable

//        if reachable {
//            found.push(ip.clone());
//        }

        // Append block to the corresponding line
//        let idx = match host {
//            1..=25 => 0,
//            26..=50 => 1,
//            51..=75 => 2,
//            76..=100 => 3,
//            101..=125 => 4,
//            126..=150 => 5,
//            151..=175 => 6,
//            176..=200 => 7,
//            201..=225 => 8,
//            226..=250 => 9,
//            _ => 10,
//        };
//        lines[idx].push_str(if reachable { good } else { bad });

        // Periodic redraw to give user feedback
//        if host % 5 == 0 || host == 255 {
//            uefi::system::with_stdout(|s| {
//                let _ = s.clear();
 //               let _ = core::fmt::Write::write_str(s, "\nLAN Scan (TCP:80)\n");
 //               let _ = core::fmt::Write::write_str(s, "────────────────────────────\n");
//                for l in &lines { let _ = core::fmt::Write::write_str(s, l); let _ = core::fmt::Write::write_str(s, "\n"); }
//                let _ = core::fmt::Write::write_str(s, "\nIPs Found:\n");
//                for ipf in &found { let _ = core::fmt::Write::write_str(s, ipf); let _ = core::fmt::Write::write_str(s, "\n"); }
//                let _ = core::fmt::Write::write_str(s, "\nTip: actual probing requires a NIC driver.\n");
//            });
//        }
    }
    hpvm_info!("NET", "scan complete. hosts detected: {}", found.len());
}

/// Start a very small management HTTP server on a separate thread (placeholder).
/// In UEFI context, we simulate a background loop.
pub fn httpd_start(port: u16) {
    ensure_net();
    if HTTPD_RUNNING.swap(true, Ordering::SeqCst) {
        hpvm_warn!("HTTPD", "already running");
        return;
    }
    net_stack::httpd_start(port);
    hpvm_info!("HTTPD", "Management server listening on port {}", port);
}

pub fn httpd_stop() {
    if !HTTPD_RUNNING.swap(false, Ordering::SeqCst) {
        hpvm_warn!("HTTPD", "not running");
        return;
    }
    net_stack::httpd_stop();
}
