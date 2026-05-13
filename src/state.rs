use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::boot::get_image_file_system;
use crate::consts::fs;
use crate::filesystem::{FileSystem, State};
use crate::hpvmlog::{LogEntry, get_log_buffer};

pub struct KernelState {
    state: u64,
    entropy: u64,
}


pub trait Persistable {
    fn magic() -> u32;
    fn version() -> u32 { 1 }
    fn serialize(&self) -> Vec<u8>;
}




impl KernelState {
    pub unsafe fn new(_state: u64, _entropy: u64) -> KernelState {
        unsafe {
            KernelState {
                state: _state, // ok state
                entropy: _entropy,
            }
        }
    }
    pub fn save(&self) {
        FileSystem::cd("/");

        let mut export_buffer = Vec::new();

        // 1. Capture the raw struct (The "Address" part)
        let size = size_of::<Self>();
        let ptr = (self as *const Self) as *const u8;
        let self_bytes = unsafe { core::slice::from_raw_parts(ptr, size) };
        export_buffer.extend_from_slice(self_bytes);

    }

    pub fn restore(&mut self) {
        // let bytes = FileSystem::read_file_bytes("/STATE");
        // if bytes.is_empty() { return; }
        //
        // let struct_size = core::mem::size_of::<Self>();
        //
        // // 1. Restore the basic fields (state, entropy, etc.)
        // unsafe {
        //     let self_ptr = (self as *mut Self) as *mut u8;
        //     core::ptr::copy_nonoverlapping(bytes.as_ptr(), self_ptr, struct_size);
        // }
        //
        // // 2. FIX THE POINTER: The old log_bfr pointer is now GARBAGE.
        // // We must manually reconstruct the Vec from the trailing data in the file.
        // let entry_size = core::mem::size_of::<LogEntry>();
        // let data_part = &bytes[struct_size..];
        // let num_entries = data_part.len() / entry_size;
        //
        // let mut restored_logs = Vec::with_capacity(num_entries);
        // for i in 0..num_entries {
        //     let start = i * entry_size;
        //     let end = start + entry_size;
        //     let entry_raw = &data_part[start..end];
        //
        //     unsafe {
        //         restored_logs.push(core::ptr::read(entry_raw.as_ptr() as *const LogEntry));
        //     }
        // }
        //
        // // Overwrite the broken pointer with the fresh Vec we just built
        // self.log_bfr = Some(restored_logs);
    }
}

impl Persistable for KernelState {
    fn magic() -> u32 { 0x4154534B } // "KSTA" in hex

    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.state.to_le_bytes());
        data.extend_from_slice(&self.entropy.to_le_bytes());
        data
    }
}

impl Persistable for crate::ui::UiSettings {
    fn magic() -> u32 { 0x53455454 } // "SETT" in hex

    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(self.extra_debug_info as u8);
        data.push(self.folder_absolute_sizes as u8);
        data.push(self.state_save_restore as u8);
        data.push(self.extended_symbol_library as u8);
        data.push(self.ring0_udmi_udxi as u8);
        data.push(self.controllang_support as u8);
        data.push(self.pg_vshaders as u8);
        data.push(self.experimental_mem_comp as u8);
        data.push(self.auto_refresh_storage as u8);
        data.push(self.show_hidden_files as u8);
        data.extend_from_slice(&(self.general_profile as u32).to_le_bytes());
        data.extend_from_slice(&(self.boot_target as u32).to_le_bytes());
        data.extend_from_slice(&(self.interface_density as u32).to_le_bytes());
        data.extend_from_slice(&(self.vm_safety_policy as u32).to_le_bytes());
        data.extend_from_slice(&(self.network_profile as u32).to_le_bytes());
        data.extend_from_slice(&(self.storage_policy as u32).to_le_bytes());
        data.extend_from_slice(&(self.package_policy as u32).to_le_bytes());
        data.extend_from_slice(&(self.developer_level as u32).to_le_bytes());
        data.extend_from_slice(&(self.security_policy as u32).to_le_bytes());
        data.extend_from_slice(&(self.ui_scaling as u32).to_le_bytes());
        data.extend_from_slice(&(self.terminal_font as u32).to_le_bytes());
        data
    }
}

impl Persistable for crate::ui::DashboardUI {
    fn magic() -> u32 { 0x44425549 } // "DBUI" in hex

    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        // Serialize current_path
        data.extend_from_slice(&(self.current_path.len() as u32).to_le_bytes());
        data.extend_from_slice(self.current_path.as_bytes());
        
        // Serialize selected indices
        data.extend_from_slice(&(self.selected_file_idx as u32).to_le_bytes());
        data.extend_from_slice(&(self.selected_device_idx as u32).to_le_bytes());
        data.extend_from_slice(&(self.selected_vm_idx as u32).to_le_bytes());
        data.extend_from_slice(&(self.selected_app_idx as u32).to_le_bytes());
        data.extend_from_slice(&(self.selected_package_idx as u32).to_le_bytes());
        
        // Serialize settings
        data.extend_from_slice(&self.settings.serialize());
        
        data
    }
}

impl crate::ui::DashboardUI {
    pub fn restore(&mut self, data: &[u8]) {
        let mut offset = 0;
        if offset + 4 <= data.len() {
            let path_len = u32::from_le_bytes(data[offset..offset+4].try_into().unwrap()) as usize;
            offset += 4;
            if offset + path_len <= data.len() {
                self.current_path = core::str::from_utf8(&data[offset..offset+path_len]).unwrap().to_string();
                offset += path_len;
            }
        }
        
        if offset + 20 <= data.len() {
            self.selected_file_idx = u32::from_le_bytes(data[offset..offset+4].try_into().unwrap()) as usize;
            offset += 4;
            self.selected_device_idx = u32::from_le_bytes(data[offset..offset+4].try_into().unwrap()) as usize;
            offset += 4;
            self.selected_vm_idx = u32::from_le_bytes(data[offset..offset+4].try_into().unwrap()) as usize;
            offset += 4;
            self.selected_app_idx = u32::from_le_bytes(data[offset..offset+4].try_into().unwrap()) as usize;
            offset += 4;
            self.selected_package_idx = u32::from_le_bytes(data[offset..offset+4].try_into().unwrap()) as usize;
            offset += 4;
        }

        if offset < data.len() {
            let s_data = &data[offset..];
            let mut s_offset = 0;
            if s_offset + 10 <= s_data.len() {
                self.settings.extra_debug_info = s_data[s_offset] != 0; s_offset += 1;
                self.settings.folder_absolute_sizes = s_data[s_offset] != 0; s_offset += 1;
                self.settings.state_save_restore = s_data[s_offset] != 0; s_offset += 1;
                self.settings.extended_symbol_library = s_data[s_offset] != 0; s_offset += 1;
                self.settings.ring0_udmi_udxi = s_data[s_offset] != 0; s_offset += 1;
                self.settings.controllang_support = s_data[s_offset] != 0; s_offset += 1;
                self.settings.pg_vshaders = s_data[s_offset] != 0; s_offset += 1;
                self.settings.experimental_mem_comp = s_data[s_offset] != 0; s_offset += 1;
                self.settings.auto_refresh_storage = s_data[s_offset] != 0; s_offset += 1;
                self.settings.show_hidden_files = s_data[s_offset] != 0; s_offset += 1;
            }
            if s_offset + 44 <= s_data.len() {
                self.settings.general_profile = u32::from_le_bytes(s_data[s_offset..s_offset+4].try_into().unwrap()) as usize; s_offset += 4;
                self.settings.boot_target = u32::from_le_bytes(s_data[s_offset..s_offset+4].try_into().unwrap()) as usize; s_offset += 4;
                self.settings.interface_density = u32::from_le_bytes(s_data[s_offset..s_offset+4].try_into().unwrap()) as usize; s_offset += 4;
                self.settings.vm_safety_policy = u32::from_le_bytes(s_data[s_offset..s_offset+4].try_into().unwrap()) as usize; s_offset += 4;
                self.settings.network_profile = u32::from_le_bytes(s_data[s_offset..s_offset+4].try_into().unwrap()) as usize; s_offset += 4;
                self.settings.storage_policy = u32::from_le_bytes(s_data[s_offset..s_offset+4].try_into().unwrap()) as usize; s_offset += 4;
                self.settings.package_policy = u32::from_le_bytes(s_data[s_offset..s_offset+4].try_into().unwrap()) as usize; s_offset += 4;
                self.settings.developer_level = u32::from_le_bytes(s_data[s_offset..s_offset+4].try_into().unwrap()) as usize; s_offset += 4;
                self.settings.security_policy = u32::from_le_bytes(s_data[s_offset..s_offset+4].try_into().unwrap()) as usize; s_offset += 4;
                self.settings.ui_scaling = u32::from_le_bytes(s_data[s_offset..s_offset+4].try_into().unwrap()) as usize; s_offset += 4;
                self.settings.terminal_font = u32::from_le_bytes(s_data[s_offset..s_offset+4].try_into().unwrap()) as usize; s_offset += 4;
            }
        }
    }
}

pub struct PersistenceManager;
impl PersistenceManager {
    pub fn save<T: Persistable>(path: &str, obj: &T, mode: char) {
        FileSystem::cd("/");
        let data = obj.serialize();

        let mut file_data = Vec::new();
        file_data.extend_from_slice(&T::magic().to_le_bytes());
        file_data.extend_from_slice(&T::version().to_le_bytes());
        file_data.extend_from_slice(&(data.len() as u32).to_le_bytes());
        file_data.extend_from_slice(&[0u8; 4]); // Padding for compatibility

        file_data.extend_from_slice(&data);

        FileSystem::write_to_file_bytes(path, &file_data, mode).unwrap();
    }

    pub fn restore_data(path: &str, magic: u32) -> Option<Vec<u8>> {
        let file_bytes = FileSystem::read_file(path).ok()?;
        let mut offset = 0;
        
        while offset + 16 <= file_bytes.len() {
            let chunk_magic = u32::from_le_bytes(file_bytes[offset..offset+4].try_into().unwrap());
            let _version = u32::from_le_bytes(file_bytes[offset+4..offset+8].try_into().unwrap());
            let size = u32::from_le_bytes(file_bytes[offset+8..offset+12].try_into().unwrap()) as usize;
            
            offset += 16;
            if chunk_magic == magic {
                if offset + size <= file_bytes.len() {
                    return Some(file_bytes[offset..offset+size].to_vec());
                }
                return None;
            }
            offset += size;
        }
        None
    }
}




pub unsafe fn SAVE(dashboard: Option<&crate::ui::DashboardUI>) {
    unsafe {
        FileSystem::remove("/STATE");
        FileSystem::touch("/STATE");
        PersistenceManager::save("/STATE", &KernelState { state: 0, entropy: 0x0 }, 'w');
        PersistenceManager::save("/STATE", FileSystem::get_state(), 'a');
        if let Some(ref buffer) = crate::hpvmlog::LOG_BUFFER {
            PersistenceManager::save("/STATE", buffer, 'a');
        }
        if let Some(db) = dashboard {
            PersistenceManager::save("/STATE", db, 'a');
        }
    }
}

pub unsafe fn RESTORE(dashboard: Option<&mut crate::ui::DashboardUI>) {
    unsafe {
        // 1. Restore FS state
        if let Some(fs_data) = PersistenceManager::restore_data("/STATE", 0x54535346) {
            let state = FileSystem::get_state();
            let mut offset = 0;
            
            // Restore cwd
            if offset + 4 <= fs_data.len() {
                let len = u32::from_le_bytes(fs_data[offset..offset+4].try_into().unwrap()) as usize;
                offset += 4;
                if offset + len <= fs_data.len() {
                    state.set_cwd(core::str::from_utf8(&fs_data[offset..offset+len]).unwrap_or("\\"));
                    offset += len;
                }
            }
            
            // Restore device_map
            if offset + 4 <= fs_data.len() {
                let count = u32::from_le_bytes(fs_data[offset..offset+4].try_into().unwrap()) as usize;
                offset += 4;
                state.device_map.clear();
                for _ in 0..count {
                    if offset + 4 <= fs_data.len() {
                        let k_len = u32::from_le_bytes(fs_data[offset..offset+4].try_into().unwrap()) as usize;
                        offset += 4;
                        if offset + k_len <= fs_data.len() {
                            let k = core::str::from_utf8(&fs_data[offset..offset+k_len]).unwrap().to_string();
                            offset += k_len;
                            if offset + 4 <= fs_data.len() {
                                let v_len = u32::from_le_bytes(fs_data[offset..offset+4].try_into().unwrap()) as usize;
                                offset += 4;
                                if offset + v_len <= fs_data.len() {
                                    let v = core::str::from_utf8(&fs_data[offset..offset+v_len]).unwrap().to_string();
                                    offset += v_len;
                                    state.device_map.push((k, v));
                                }
                            }
                        }
                    }
                }
            }
        }

        // 2. Restore Logs
        if let Some(log_data) = PersistenceManager::restore_data("/STATE", 0x474F4C48) {
            if let Some(ref mut buffer) = crate::hpvmlog::LOG_BUFFER {
                let mut offset = 0;
                let count = u32::from_le_bytes(log_data[offset..offset+4].try_into().unwrap()) as usize;
                offset += 4;
                buffer.clear();
                for _ in 0..count {
                    if offset + 1 <= log_data.len() {
                        let level_u8 = log_data[offset];
                        offset += 1;
                        let tag_len = u32::from_le_bytes(log_data[offset..offset+4].try_into().unwrap()) as usize;
                        offset += 4;
                        if offset + tag_len <= log_data.len() {
                            let tag = core::str::from_utf8(&log_data[offset..offset+tag_len]).unwrap().to_string();
                            offset += tag_len;
                            if offset + 4 <= log_data.len() {
                                let msg_len = u32::from_le_bytes(log_data[offset..offset+4].try_into().unwrap()) as usize;
                                offset += 4;
                                if offset + msg_len <= log_data.len() {
                                    let msg = core::str::from_utf8(&log_data[offset..offset+msg_len]).unwrap().to_string();
                                    offset += msg_len;
                                    
                                    use uefi::proto::console::text::Color;
                                    let level = match level_u8 {
                                        0 => Color::Black, 1 => Color::Blue, 2 => Color::Green, 3 => Color::Cyan,
                                        4 => Color::Red, 5 => Color::Magenta, 6 => Color::Brown, 7 => Color::LightGray,
                                        8 => Color::DarkGray, 9 => Color::LightBlue, 10 => Color::LightGreen, 11 => Color::LightCyan,
                                        12 => Color::LightRed, 13 => Color::LightMagenta, 14 => Color::Yellow, 15 => Color::White,
                                        _ => Color::White,
                                    };
                                    buffer.push(LogEntry { level, tag, message: msg });
                                }
                            }
                        }
                    }
                }
            }
        }

        // 3. Restore Dashboard UI
        if let Some(db) = dashboard {
            if let Some(db_data) = PersistenceManager::restore_data("/STATE", 0x44425549) {
                db.restore(&db_data);
            }
        }
    }
}


