//! Minimal networking scaffolding for HPVMx
//! Provides placeholder implementations for ping, lanscan, and an HTTP management server
//! so shell commands are available without requiring a fully wired NIC.

#![allow(dead_code)]

use crate::{hpvm_info, Color};
use crate::hpvm_log;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use uefi::boot;
use crate::{hpvm_error, hpvm_warn};
use crate::devices::net_stack::{EthHeader, ETHERTYPE_IPV4};
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

#[repr(C, packed)]
struct DhcpReply {
    _unused: [u8; 16],   // op, htype, hlen, hops, xid, secs, flags, ciaddr
    yiaddr: [u8; 4],     // "Your" (client) IP address
    _unused2: [u8; 216], // siaddr, giaddr, chaddr, sname, file
    magic: u32,          // Magic Cookie (0x63538263)
    options: [u8; 312],
}

fn parse_dhcp_offer(data: &[u8]) -> Option<([u8; 4], [u8; 4], [u8; 4])> {
    if data.len() < size_of::<DhcpReply>() { return None; }

    let reply = unsafe { &*(data.as_ptr() as *const DhcpReply) };

    // Verify Magic Cookie (0x63538263 in big-endian)
    if u32::from_be(reply.magic) != 0x63538263 { return None; }

    let mut offered_ip = reply.yiaddr;
    let mut subnet_mask = [0u8; 4];
    let mut router = [0u8; 4];
    let mut is_offer_or_ack = false;

    let mut cursor = 0;
    while cursor < reply.options.len() {
        let opt_type = reply.options[cursor];
        if opt_type == 255 { break; } // End of options
        if opt_type == 0 { // Padding
            cursor += 1;
            continue;
        }

        let opt_len = reply.options[cursor + 1] as usize;
        let opt_value = &reply.options[cursor + 2..cursor + 2 + opt_len];

        match opt_type {
            53 => { // DHCP Message Type
                // Value 2 = Offer, Value 5 = Ack
                if opt_value[0] == 2 || opt_value[0] == 5 {
                    is_offer_or_ack = true;
                }
            }
            1 => { // Subnet Mask
                if opt_len == 4 { subnet_mask.copy_from_slice(opt_value); }
            }
            3 => { // Router (Gateway)
                if opt_len >= 4 { router.copy_from_slice(&opt_value[0..4]); }
            }
            _ => {}
        }
        cursor += 2 + opt_len;
    }

    if is_offer_or_ack {
        Some((offered_ip, router, subnet_mask))
    } else {
        None
    }
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

    let mut timeout = 32;
    let mut cfg: ([u8; 4], [u8; 4], [u8; 4]) = ([0; 4], [0; 4], [0; 4]);

    while timeout > 0 {
        // 2. Poll the hardware
        if let Some(config) = poll_for_dhcp_response() {
            cfg = config;
            hpvm_info!("dhcp", "cfg: {:?}", cfg);
            break;
        }

        timeout -= 1;
        boot::stall(core::time::Duration::from_micros(32_000));
        // Optional: arch::pause() or similar to be nice to the CPU
    }

    send_dhcp_request(cfg.0, cfg.1);


    let mut timeout = 32;
    while timeout > 0 {
        // 2. Poll the hardware
        if let Some(response) = poll_for_dhcp_response() {
            hpvm_info!("dhcp", "response: {:?}", response);
            return Some(response)
        }

        timeout -= 1;
        boot::stall(core::time::Duration::from_micros(32_000));
        // Optional: arch::pause() or similar to be nice to the CPU
    }

    hpvm_info!("NET", "no dhcp response, using fallback IP. configure static ip in network tab");
    Some(([192, 168, 1, 50], [192, 168, 1, 1], [255, 255, 255, 0]))
}

pub fn send_dhcp_request(offered_ip: [u8; 4], server_ip: [u8; 4]) {
    let mac = crate::devices::net_hw::get_mac();
    let mut packet = DhcpPacket {
        op: 1, htype: 1, hlen: 6, hops: 0,
        xid: 0x12345678, // Use the same XID as the Discover
        secs: 0, flags: 0x8000u16.to_be(),
        ciaddr: [0; 4], yiaddr: [0; 4], siaddr: [0; 4], giaddr: [0; 4],
        chaddr: [0; 16], sname: [0; 64], file: [0; 128],
        magic: 0x63538263u32.to_be(),
        options: [0; 312],
    };
    packet.chaddr[..6].copy_from_slice(&mac[..6]);

    let mut cursor = 0;

    // 1. DHCP Message Type: Request (Value 3)
    packet.options[cursor..cursor+3].copy_from_slice(&[53, 1, 3]);
    cursor += 3;

    // 2. Requested IP Address (Option 50)
    packet.options[cursor] = 50;
    packet.options[cursor + 1] = 4;
    packet.options[cursor + 2..cursor + 6].copy_from_slice(&offered_ip);
    cursor += 6;

    // 3. Server Identifier (Option 54) - The IP of the DHCP Server
    packet.options[cursor] = 54;
    packet.options[cursor + 1] = 4;
    packet.options[cursor + 2..cursor + 6].copy_from_slice(&server_ip);
    cursor += 6;

    // 4. End Option
    packet.options[cursor] = 255;

    // Broadcast the request
    net_stack::send_raw_udp(
        [0, 0, 0, 0],
        [255, 255, 255, 255],
        [0xFF; 6],
        68, 67,
        unsafe {
            core::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<DhcpPacket>())
        }
    );
}

fn poll_for_dhcp_response() -> Option<([u8; 4], [u8; 4], [u8; 4])> {
    let mut buf = [0u8; 1514];

    // Drain the RX buffer specifically looking for DHCP
    if let Ok(sz) = crate::devices::net_hw::rx(&mut buf) {
        let frame = &buf[..sz];
        if frame.len() < size_of::<EthHeader>() + 20 + 8 { return None; } // Eth + IP + UDP min

        let eth = unsafe { &*(frame.as_ptr() as *const EthHeader) };
        if u16::from_be(eth.ethertype) != ETHERTYPE_IPV4 { return None; }

        let ip_payload = &frame[size_of::<EthHeader>()..];
        // Basic check for UDP (Protocol 17) and Port 68
        if ip_payload[9] == 17 {
            let udp_payload = &ip_payload[20..]; // Assuming no IP options
            let dst_port = u16::from_be_bytes([udp_payload[2], udp_payload[3]]);

            if dst_port == 68 {
                return parse_dhcp_offer(&udp_payload[8..]);
            }
        }
    }
    None
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
