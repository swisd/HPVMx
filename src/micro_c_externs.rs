//! Stable C ABI service symbols declared by the standalone Micro-C `HPVMx` header.
//! Micro-C source is compiled separately; consumers link these symbols into a
//! kernel image or another host that provides the same ABI.

use alloc::{alloc::Layout, string::String};
use alloc::vec::Vec;
use core::ffi::c_void;

/// Resolve the stable HPVMx C ABI symbols used by relocatable HPX programs.
pub(crate) fn resolve_import(name: &str) -> Option<usize> {
    Some(match name {
        "hpx_ui_resolution_x" => hpx_ui_resolution_x as *const () as usize,
        "hpx_ui_resolution_y" => hpx_ui_resolution_y as *const () as usize,
        "hpx_ui_clear" => hpx_ui_clear as *const () as usize,
        "hpx_ui_fill_rect" => hpx_ui_fill_rect as *const () as usize,
        "hpx_ui_draw_pixel" => hpx_ui_draw_pixel as *const () as usize,
        "hpx_ui_draw_text" => hpx_ui_draw_text as *const () as usize,
        "hpx_fs_read_file" => hpx_fs_read_file as *const () as usize,
        "hpx_fs_write_file" => hpx_fs_write_file as *const () as usize,
        "hpx_fs_make_dir" => hpx_fs_make_dir as *const () as usize,
        "hpx_fs_remove" => hpx_fs_remove as *const () as usize,
        "hpx_fs_rename" => hpx_fs_rename as *const () as usize,
        "hpx_cpu_core_count" => hpx_cpu_core_count as *const () as usize,
        "hpx_cpu_thread_count" => hpx_cpu_thread_count as *const () as usize,
        "hpx_pci_device_count" => hpx_pci_device_count as *const () as usize,
        "hpx_pci_read_u32" => hpx_pci_read_u32 as *const () as usize,
        "hpx_pci_write_u32" => hpx_pci_write_u32 as *const () as usize,
        "hpx_network_initialize" => hpx_network_initialize as *const () as usize,
        "hpx_network_is_initialized" => hpx_network_is_initialized as *const () as usize,
        "hpx_network_link_up" => hpx_network_link_up as *const () as usize,
        "hpx_network_transmit" => hpx_network_transmit as *const () as usize,
        "hpx_network_receive" => hpx_network_receive as *const () as usize,
        "hpx_network_get_mac" => hpx_network_get_mac as *const () as usize,
        "hpx_network_get_ip" => hpx_network_get_ip as *const () as usize,
        "hpx_network_set_static_ip" => hpx_network_set_static_ip as *const () as usize,
        "hpx_network_ping" => hpx_network_ping as *const () as usize,
        "hpx_network_httpd_start" => hpx_network_httpd_start as *const () as usize,
        "hpx_network_httpd_stop" => hpx_network_httpd_stop as *const () as usize,
        "hpx_network_webdav_start" => hpx_network_webdav_start as *const () as usize,
        "hpx_network_webdav_stop" => hpx_network_webdav_stop as *const () as usize,
        "hpx_network_rpcd_start" => hpx_network_rpcd_start as *const () as usize,
        "hpx_network_rpcd_stop" => hpx_network_rpcd_stop as *const () as usize,
        "hpx_network_status" => hpx_network_status as *const () as usize,
        "hpx_network_lanscan" => hpx_network_lanscan as *const () as usize,
        "hpx_network_save_config" => hpx_network_save_config as *const () as usize,
        "hpx_network_load_config" => hpx_network_load_config as *const () as usize,
        "hpx_beep" => hpx_beep as *const () as usize,
        "hpx_mute" => hpx_mute as *const () as usize,
        "hpx_sleep_ms" => hpx_sleep_ms as *const () as usize,
        "hpx_alloc" => hpx_alloc as *const () as usize,
        "hpx_free" => hpx_free as *const () as usize,
        "hpx_set_global_variable" => hpx_set_global_variable as *const () as usize,
        "hpx_get_global_variable" => hpx_get_global_variable as *const () as usize,
        "hpx_pack_dimensions" => hpx_pack_dimensions as *const () as usize,
        _ => return None,
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn hpx_pack_dimensions(width: i64, height: i64) -> i64 {
    ((((height as u32) as u64) << 32) | ((width as u32) as u64)) as i64
}

use crate::{devices, filesystem::FileSystem, hardware, ui::pixel_graphics::PixelGraphics, HYPERVISOR};
use crate::keys::keyring::{KeyringError, GLOBAL_KEYRING};
use crate::keys::types::KUID;

unsafe fn c_string<'a>(ptr: *const u8, max_len: usize) -> Option<&'a str> {
    if ptr.is_null() { return None; }
    let mut len = 0usize;
    while len < max_len && unsafe { *ptr.add(len) } != 0 { len += 1; }
    if len == max_len { return None; }
    core::str::from_utf8(unsafe { core::slice::from_raw_parts(ptr, len) }).ok()
}

#[unsafe(no_mangle)]
pub extern "C" fn hpx_ui_resolution_x() -> i64 {
    PixelGraphics::new().map(|g| g.resolution().0 as i64).unwrap_or(0)
}
#[unsafe(no_mangle)]
pub extern "C" fn hpx_ui_resolution_y() -> i64 {
    PixelGraphics::new().map(|g| g.resolution().1 as i64).unwrap_or(0)
}
#[unsafe(no_mangle)]
pub extern "C" fn hpx_ui_clear(color: i64) {
    if let Some(mut g) = PixelGraphics::new() { g.clear(color as u32); g.flip(); }
}
#[unsafe(no_mangle)]
/// `dimensions` packs width in its low 32 bits and height in its high 32 bits.
pub extern "C" fn hpx_ui_fill_rect(x: i64, y: i64, dimensions: i64, color: i64) {
    if x < 0 || y < 0 { return; }
    let packed = dimensions as u64;
    let width = packed as u32 as usize;
    let height = (packed >> 32) as u32 as usize;
    if let Some(mut g) = PixelGraphics::new() {
        g.fill_rect(x as usize, y as usize, width, height, color as u32);
        g.flip();
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn hpx_ui_draw_pixel(x: i64, y: i64, color: i64) {
    if x < 0 || y < 0 { return; }
    if let Some(mut g) = PixelGraphics::new() { g.draw_pixel(x as usize, y as usize, color as u32); g.flip(); }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_ui_draw_text(x: i64, y: i64, text: *const u8, color: i64) {
    if x < 0 || y < 0 { return; }
    if let Some(text) = unsafe { c_string(text, 4096) } {
        if let Some(mut g) = PixelGraphics::new() { g.draw_text(x as usize, y as usize, text, color as u32); g.flip(); }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_fs_read_file(path: *const u8, out: *mut u8, capacity: i64) -> i64 {
    if capacity < 0 || capacity > 64 * 1024 * 1024 { return -1; }
    let Some(path) = (unsafe { c_string(path, 4096) }) else { return -1; };
    let Ok(data) = FileSystem::read_file(path) else { return -1; };
    if out.is_null() || capacity == 0 { return data.len().min(i64::MAX as usize) as i64; }
    if data.len() > capacity as usize { return -2; }
    unsafe { core::ptr::copy_nonoverlapping(data.as_ptr(), out, data.len()); }
    data.len() as i64
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_fs_write_file(path: *const u8, data: *const u8, len: i64) -> i64 {
    if len < 0 || len > 64 * 1024 * 1024 || (data.is_null() && len != 0) { return -1; }
    let Some(path) = (unsafe { c_string(path, 4096) }) else { return -1; };
    let bytes = if len == 0 { &[] } else { unsafe { core::slice::from_raw_parts(data, len as usize) } };
    FileSystem::write_to_file_bytes(path, bytes, 'w').map(|_| 0).unwrap_or(-1)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_fs_make_dir(path: *const u8) -> i64 {
    let Some(path) = (unsafe { c_string(path, 4096) }) else { return -1; };
    FileSystem::mkdir(path).map(|_| 0).unwrap_or(-1)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_fs_remove(path: *const u8) -> i64 {
    let Some(path) = (unsafe { c_string(path, 4096) }) else { return -1; };
    FileSystem::remove(path).map(|_| 0).unwrap_or(-1)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_fs_rename(from: *const u8, to: *const u8) -> i64 {
    let (Some(from), Some(to)) = (unsafe { c_string(from, 4096) }, unsafe { c_string(to, 4096) }) else { return -1; };
    FileSystem::rename(from, to).map(|_| 0).unwrap_or(-1)
}

#[unsafe(no_mangle)]
pub extern "C" fn hpx_cpu_core_count() -> i64 { hardware::cpu::core_count() as i64 }
#[unsafe(no_mangle)]
pub extern "C" fn hpx_cpu_thread_count() -> i64 { hardware::cpu::CpuInfo::get().threads as i64 }
#[unsafe(no_mangle)]
pub extern "C" fn hpx_pci_device_count() -> i64 { hardware::pci::scan_bus().len() as i64 }
#[unsafe(no_mangle)]
pub extern "C" fn hpx_pci_read_u32(bdf: i64, offset: i64) -> i64 {
    if !(0..=252).contains(&offset) || offset % 4 != 0 { return -1; }
    let packed = bdf as u32;
    let bus = (packed >> 16) as u8;
    let slot = ((packed >> 8) & 0xff) as u8;
    let function = (packed & 0xff) as u8;
    if slot > 31 || function > 7 { return -1; }
    hardware::pci::pci_config_read_u32(bus, slot, function, offset as u8) as i64
}
#[unsafe(no_mangle)]
pub extern "C" fn hpx_pci_write_u32(bdf: i64, offset: i64, value: i64) -> i64 {
    if !(0..=252).contains(&offset) || offset % 4 != 0 { return -1; }
    let packed = bdf as u32;
    let bus = (packed >> 16) as u8;
    let slot = ((packed >> 8) & 0xff) as u8;
    let function = (packed & 0xff) as u8;
    if slot > 31 || function > 7 { return -1; }
    hardware::pci::pci_config_write_u32(bus, slot, function, offset as u8, value as u32);
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn hpx_network_initialize() -> i64 { devices::net_hw::init().map(|_| 0).unwrap_or(-1) }
#[unsafe(no_mangle)]
pub extern "C" fn hpx_network_is_initialized() -> i64 { devices::net::is_initialized() as i64 }
#[unsafe(no_mangle)]
pub extern "C" fn hpx_network_link_up() -> i64 { devices::net_hw::link_up() as i64 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_network_transmit(frame: *const u8, len: i64) -> i64 {
    if frame.is_null() || !(14..=65535).contains(&len) { return -1; }
    devices::net_hw::tx(unsafe { core::slice::from_raw_parts(frame, len as usize) }).map(|_| 0).unwrap_or(-1)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_network_receive(out: *mut u8, capacity: i64) -> i64 {
    if out.is_null() || !(1..=65535).contains(&capacity) { return -1; }
    devices::net_hw::rx(unsafe { core::slice::from_raw_parts_mut(out, capacity as usize) })
        .map(|size| size as i64).unwrap_or(-1)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_network_get_mac(out: *mut u8, capacity: i64) -> i64 {
    if out.is_null() || capacity < 6 { return -1; }
    let mac = devices::net_hw::get_mac();
    let len = 6.min(capacity as usize);
    unsafe { core::ptr::copy_nonoverlapping(mac.as_ptr(), out, len); }
    0
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_network_get_ip(out: *mut u8, capacity: i64) -> i64 {
    if out.is_null() || capacity == 0 { return -1; }
    let ip_str = b"0.0.0.0";
    if ip_str.len() > capacity as usize { return -2; }
    unsafe { core::ptr::copy_nonoverlapping(ip_str.as_ptr(), out, ip_str.len()); }
    ip_str.len() as i64
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_network_set_static_ip(ip: *const u8, ip_len: i64, gw: *const u8, gw_len: i64, mask: *const u8, mask_len: i64) -> i64 {
    if ip_len <= 0 || gw_len <= 0 || mask_len <= 0 { return -1; }
    let (Ok(ip), Ok(gw), Ok(mask)) = (
        core::str::from_utf8(unsafe { core::slice::from_raw_parts(ip, ip_len as usize) }),
        core::str::from_utf8(unsafe { core::slice::from_raw_parts(gw, gw_len as usize) }),
        core::str::from_utf8(unsafe { core::slice::from_raw_parts(mask, mask_len as usize) }),
    ) else { return -1; };
    crate::devices::net::save_network_config(ip, gw, mask).map(|_| 0).unwrap_or(-1)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_network_ping(target: *const u8, target_len: i64) -> i64 {
    if target.is_null() || target_len <= 0 || target_len > 64 { return -1; }
    let Ok(target) = core::str::from_utf8(unsafe { core::slice::from_raw_parts(target, target_len as usize) }) else { return -1; };
    crate::devices::net::ping(target, 1, 1000).map(|_| 0).unwrap_or(-1)
}
#[unsafe(no_mangle)]
pub extern "C" fn hpx_network_httpd_start(port: i64) -> i64 {
    if !(1..=65535).contains(&port) { return -1; }
    crate::devices::net::httpd_start(port as u16);
    0
}
#[unsafe(no_mangle)]
pub extern "C" fn hpx_network_httpd_stop() -> i64 {
    crate::devices::net::httpd_stop();
    0
}
#[unsafe(no_mangle)]
pub extern "C" fn hpx_network_webdav_start(port: i64) -> i64 {
    if !(1..=65535).contains(&port) { return -1; }
    crate::devices::net::webdav_start(port as u16);
    0
}
#[unsafe(no_mangle)]
pub extern "C" fn hpx_network_webdav_stop() -> i64 {
    crate::devices::net::webdav_stop();
    0
}
#[unsafe(no_mangle)]
pub extern "C" fn hpx_network_rpcd_start(port: i64) -> i64 {
    if !(1..=65535).contains(&port) { return -1; }
    crate::devices::net::rpcd_start(port as u16);
    0
}
#[unsafe(no_mangle)]
pub extern "C" fn hpx_network_rpcd_stop() -> i64 {
    crate::devices::net::rpcd_stop();
    0
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_network_status(out: *mut u8, capacity: i64) -> i64 {
    if out.is_null() || capacity == 0 { return -1; }
    let mut status = alloc::string::String::new();
    if crate::devices::net::is_initialized() {
        if let Some(info) = crate::devices::net_hw::get_info() {
            status = alloc::format!("MAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x} MTU: {} Link: {}",
                info.mac[0], info.mac[1], info.mac[2], info.mac[3], info.mac[4], info.mac[5],
                info.mtu, if crate::devices::net_hw::link_up() { "UP" } else { "DOWN" });
        }
    } else {
        status = alloc::string::String::from("Not initialized");
    }
    if status.len() > capacity as usize { return -2; }
    unsafe { core::ptr::copy_nonoverlapping(status.as_ptr(), out, status.len()); }
    status.len() as i64
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_network_lanscan(prefix: *const u8, prefix_len: i64) -> i64 {
    if prefix.is_null() || prefix_len <= 0 || prefix_len > 64 { return -1; }
    let Ok(prefix) = core::str::from_utf8(unsafe { core::slice::from_raw_parts(prefix, prefix_len as usize) }) else { return -1; };
    crate::devices::net::lanscan(prefix);
    0
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_network_save_config(ip: *const u8, ip_len: i64, gw: *const u8, gw_len: i64, mask: *const u8, mask_len: i64) -> i64 {
    if ip_len <= 0 || gw_len <= 0 || mask_len <= 0 { return -1; }
    let (Ok(ip), Ok(gw), Ok(mask)) = (
        core::str::from_utf8(unsafe { core::slice::from_raw_parts(ip, ip_len as usize) }),
        core::str::from_utf8(unsafe { core::slice::from_raw_parts(gw, gw_len as usize) }),
        core::str::from_utf8(unsafe { core::slice::from_raw_parts(mask, mask_len as usize) }),
    ) else { return -1; };
    crate::devices::net::save_network_config(ip, gw, mask).map(|_| 0).unwrap_or(-1)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_network_load_config(out: *mut u8, capacity: i64) -> i64 {
    if out.is_null() || capacity == 0 { return -1; }
    let Some((ip, gw, mask)) = crate::devices::net::load_network_config() else { return -1; };
    let config = alloc::format!("{};{};{}", ip, gw, mask);
    if config.len() > capacity as usize { return -2; }
    unsafe { core::ptr::copy_nonoverlapping(config.as_ptr(), out, config.len()); }
    config.len() as i64
}
#[unsafe(no_mangle)]
pub extern "C" fn hpx_beep(frequency_hz: i64) { if (0..=u32::MAX as i64).contains(&frequency_hz) { devices::audio::beep(frequency_hz as u32); } }
#[unsafe(no_mangle)]
pub extern "C" fn hpx_mute() { devices::audio::mute(); }
#[unsafe(no_mangle)]
pub extern "C" fn hpx_sleep_ms(milliseconds: i64) { if (0..=60_000).contains(&milliseconds) { devices::timer::sleep_ms(milliseconds as u64); } }

#[unsafe(no_mangle)]
pub extern "C" fn hpx_alloc(size: i64) -> *mut c_void {
    if !(1..=64 * 1024 * 1024).contains(&size) { return core::ptr::null_mut(); }
    let Ok(layout) = Layout::from_size_align(size as usize, core::mem::align_of::<usize>()) else { return core::ptr::null_mut(); };
    unsafe { alloc::alloc::alloc(layout).cast() }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_free(ptr: *mut c_void, size: i64) {
    if ptr.is_null() || !(1..=64 * 1024 * 1024).contains(&size) { return; }
    if let Ok(layout) = Layout::from_size_align(size as usize, core::mem::align_of::<usize>()) {
        unsafe { alloc::alloc::dealloc(ptr.cast(), layout); }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_set_global_variable(key: *const u8, value: *const u8) -> i64 {
    let (Some(key), Some(value)) = (unsafe { c_string(key, 4096) }, unsafe { c_string(value, 65536) }) else { return -1; };
    crate::env::set_global_var(key, value);
    0
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_get_global_variable(key: *const u8, out: *mut u8, capacity: i64) -> i64 {
    if capacity < 0 || capacity > 65536 { return -1; }
    let Some(key) = (unsafe { c_string(key, 4096) }) else { return -1; };
    let Some(value) = crate::env::get_global_var(key) else { return -1; };
    if out.is_null() || capacity == 0 { return value.len() as i64; }
    if value.len() > capacity as usize { return -2; }
    unsafe { core::ptr::copy_nonoverlapping(value.as_ptr(), out, value.len()); }
    value.len() as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_keyring_set_secret(service: &[u8], name: &[u8], secret: &[u8], caller_uid: KUID) {
    GLOBAL_KEYRING.lock().set_secret(service, name, secret, caller_uid);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_keyring_get_secret(service: &[u8], name: &[u8], caller_uid: KUID) -> Result<Vec<u8>, KeyringError> {
    GLOBAL_KEYRING.lock().get_secret(service, name, caller_uid)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hpx_keyring_delete_secret(service: &[u8], name: &[u8], secret: KUID) {
    GLOBAL_KEYRING.lock().delete_secret(service, name, secret);
}


// functions for vm/cvm/container management

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hv_fn_1() {
    let _ = HYPERVISOR.as_ref();
}

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn