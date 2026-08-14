use alloc::{format, vec};
use alloc::fmt::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::Identify;
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::{pixel_graphics, DashboardTab, EditorMode, FileEntry, FilePendingAction, TextEditor, DiskTabInfo};
use crate::ui::pixel_graphics::PixelGraphics;
use crate::vdebug;

#[derive(Clone)]
pub struct X_Storage {
    pub current_path: String,
    pub files: Vec<FileEntry>,
    pub selected_file_idx: usize,
    pub storage_disks: Vec<DiskTabInfo>,
    pub selected_disk_idx: usize,
    pub filesys_action_idx: usize,
    pub filesys_pending_action: Option<FilePendingAction>,
    pub status_line: String,
    pub filesys_new_counter: usize,
}

impl X_Storage {
    pub fn new() -> Self {
        X_Storage {
            current_path: "/".to_string(),
            files: vec![],
            selected_file_idx: 0,
            storage_disks: vec![],
            selected_disk_idx: 0,
            filesys_action_idx: 0,
            filesys_pending_action: None,
            status_line: "".to_string(),
            filesys_new_counter: 0,
        }
    }

    pub fn select_disk_tab(&mut self, idx: usize) {
        if idx < self.storage_disks.len() {
            self.selected_disk_idx = idx;
            let alias = self.storage_disks[idx].alias.clone();
            self.current_path = format!("{}:\\", alias);
            self.selected_file_idx = 0;
            self.refresh_storage();
        }
    }

    pub fn refresh_storage(&mut self) {
        use uefi::proto::media::file::{File, FileMode, FileAttribute};
        use uefi::proto::media::fs::SimpleFileSystem;

        self.files.clear();

        // Refresh and query all detected disk filesystems
        let fs_handles = uefi::boot::locate_handle_buffer(uefi::boot::SearchType::ByProtocol(&SimpleFileSystem::GUID))
            .map(|hb| hb.to_vec())
            .unwrap_or_default();

        let fs_state = crate::filesystem::FileSystem::get_state();
        let mut detected_disks = Vec::new();
        let mut disk_handles: Vec<(String, uefi::Handle)> = Vec::new();

        for (alias, handle) in &fs_state.drive_handles {
            if alias.starts_with("dsk") && !disk_handles.iter().any(|(_, h)| *h == *handle) {
                disk_handles.push((alias.clone(), *handle));
            }
        }

        for handle in &fs_handles {
            if !disk_handles.iter().any(|(_, h)| *h == *handle) {
                disk_handles.push((format!("dsk{}", disk_handles.len()), *handle));
            }
        }

        if disk_handles.is_empty() {
            if let Ok(h) = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>() {
                disk_handles.push((String::from("dsk0"), h));
            }
        }

        for (alias, handle) in &disk_handles {
            let mut volume_label = String::new();
            let mut total_bytes = 0u64;
            let mut free_bytes = 0u64;
            let mut block_size = 512u32;
            let mut media_type = String::from("Storage Volume");

            let dp_res = unsafe {
                uefi::boot::open_protocol::<uefi::proto::device_path::DevicePath>(
                    uefi::boot::OpenProtocolParams {
                        handle: *handle,
                        agent: uefi::boot::image_handle(),
                        controller: None,
                    },
                    uefi::boot::OpenProtocolAttributes::GetProtocol,
                )
            };

            if let Ok(dp) = dp_res {
                for node in dp.node_iter() {
                    use uefi_raw::protocol::device_path::{DeviceType, DeviceSubType};
                    match (node.device_type(), node.sub_type()) {
                        (DeviceType::MESSAGING, DeviceSubType::MESSAGING_NVME_NAMESPACE) => {
                            media_type = String::from("NVMe SSD");
                        }
                        (DeviceType::MESSAGING, DeviceSubType::MESSAGING_SATA) => {
                            media_type = String::from("SATA AHCI");
                        }
                        (DeviceType::MESSAGING, DeviceSubType::MESSAGING_USB)
                        | (DeviceType::MESSAGING, DeviceSubType::MESSAGING_USB_CLASS) => {
                            media_type = String::from("USB Drive");
                        }
                        (DeviceType::MEDIA, DeviceSubType::MEDIA_CD_ROM) => {
                            media_type = String::from("CD/DVD-ROM");
                        }
                        (DeviceType::MEDIA, DeviceSubType::MEDIA_RAM_DISK) => {
                            media_type = String::from("RAM Disk");
                        }
                        (DeviceType::MEDIA, DeviceSubType::MEDIA_HARD_DRIVE) => {
                            if media_type == "Storage Volume" {
                                media_type = String::from("Hard Disk");
                            }
                        }
                        _ => {}
                    }
                }
            }

            if let Ok(mut sfs) = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(*handle) {
                if let Ok(mut root_dir) = sfs.open_volume() {
                    let mut info_buf = [0u8; 1024];
                    if let Ok(fs_info) = root_dir.get_info::<uefi::proto::media::file::FileSystemInfo>(&mut info_buf) {
                        volume_label = fs_info.volume_label().to_string();
                        total_bytes = fs_info.volume_size();
                        free_bytes = fs_info.free_space();
                        block_size = fs_info.block_size();
                    }
                }
            }

            detected_disks.push(DiskTabInfo {
                alias: alias.clone(),
                volume_label,
                total_bytes,
                free_bytes,
                block_size,
                media_type,
                handle: Some(*handle),
            });
        }

        if detected_disks.is_empty() {
            detected_disks.push(DiskTabInfo {
                alias: String::from("dsk0"),
                volume_label: String::from("System"),
                total_bytes: 0,
                free_bytes: 0,
                block_size: 512,
                media_type: String::from("Default Volume"),
                handle: None,
            });
        }

        self.storage_disks = detected_disks;

        if self.selected_disk_idx >= self.storage_disks.len() {
            self.selected_disk_idx = self.storage_disks.len().saturating_sub(1);
        }

        let active_disk = self.storage_disks.get(self.selected_disk_idx);
        let disk_handle = active_disk.and_then(|d| d.handle);
        let sfs_handle = disk_handle.or_else(|| uefi::boot::get_handle_for_protocol::<SimpleFileSystem>().ok());

        let handle = match sfs_handle {
            Some(h) => h,
            None => {
                ui_error(27);
                return;
            }
        };

        let mut sfs = match uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle) {
            Ok(s) => s,
            Err(_) => {
                ui_error(19);
                return;
            }
        };

        let mut root_dir = match sfs.open_volume() {
            Ok(d) => d,
            Err(_) => {
                ui_error(28);
                return;
            }
        };

        let sub_path = if let Some(colon_idx) = self.current_path.find(':') {
            &self.current_path[colon_idx + 1..]
        } else {
            &self.current_path
        };

        let mut target_dir = if sub_path == "\\" || sub_path == "/" || sub_path.is_empty() {
            root_dir
        } else {
            let clean_path = sub_path.trim_start_matches('\\').trim_start_matches('/');
            if clean_path.is_empty() {
                root_dir
            } else {
                let mut u16_path: Vec<u16> = clean_path.encode_utf16().collect();
                u16_path.push(0);
                let path_cstr = match uefi::data_types::CStr16::from_u16_with_nul(&u16_path) {
                    Ok(c) => c,
                    Err(_) => {
                        ui_error(24);
                        return;
                    }
                };

                let handle = match root_dir.open(path_cstr, FileMode::Read, FileAttribute::DIRECTORY) {
                    Ok(h) => h,
                    Err(_) => {
                        ui_error(10);
                        return;
                    }
                };

                match handle.into_directory() {
                    Some(d) => d,
                    None => {
                        ui_error(28);
                        return;
                    }
                }
            }
        };

        let mut buffer = [0u8; 4096];
        loop {
            match target_dir.read_entry(&mut buffer) {
                Ok(Some(entry)) => {
                    let name = entry.file_name().to_string();
                    let size = entry.file_size();
                    let is_dir = entry.attribute().contains(FileAttribute::DIRECTORY);

                    self.files.push(FileEntry {
                        name,
                        size,
                        is_dir,
                    });
                }
                _ => break,
            }
        }

        // Clamp selected index to new list size
        if !self.files.is_empty() {
            if self.selected_file_idx >= self.files.len() {
                self.selected_file_idx = self.files.len() - 1;
            }
        } else {
            self.selected_file_idx = 0;
        }
    }
}

fn ui_error(err: usize) {
    // vdebug!("X_Storage", "error: {:?}", err);
}

impl Runnable for X_Storage {
    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {
        if let Some(data) = env.global_data.as_ref() {
            if self.current_path.is_empty() {
                self.current_path = data.current_path.clone();
            }
            self.files = data.files.clone();
            self.selected_file_idx = self.selected_file_idx.min(self.files.len().saturating_sub(1));
        }

        if self.current_path.is_empty() {
            self.current_path = String::from("/");
        }
        self.filesys_new_counter += 1;
    }
    fn input(&mut self, key: Key) {
        match key {
            Key::Printable(c) if u16::from(c) == 0x0D || u16::from(c) == 0x0A => {
                // Enter navigates directories without changing the selected
                // file action.  End remains the explicit action key.
                if self.files.get(self.selected_file_idx).map(|entry| entry.is_dir).unwrap_or(false) {
                    let selected_action = self.filesys_action_idx;
                    self.filesys_action_idx = 0; // "Open" enters a directory.
                    self.input(Key::Special(ScanCode::END));
                    self.filesys_action_idx = selected_action;
                }
            }
            Key::Printable(c) => {
                let ch = char::from(c);
                if ch == '[' || ch == '{' {
                    if self.selected_disk_idx > 0 {
                        self.select_disk_tab(self.selected_disk_idx - 1);
                    }
                } else if ch == ']' || ch == '}' {
                    if self.selected_disk_idx + 1 < self.storage_disks.len() {
                        self.select_disk_tab(self.selected_disk_idx + 1);
                    }
                } else if ch >= '1' && ch <= '9' {
                    let idx = (ch as u8 - b'1') as usize;
                    if idx < self.storage_disks.len() {
                        self.select_disk_tab(idx);
                    }
                } else if ch == '\t' {
                    if !self.storage_disks.is_empty() {
                        let next = (self.selected_disk_idx + 1) % self.storage_disks.len();
                        self.select_disk_tab(next);
                    }
                }
            }
            Key::Special(ScanCode::PAGE_UP) => {
                if self.selected_disk_idx > 0 {
                    self.select_disk_tab(self.selected_disk_idx - 1);
                }
            }
            Key::Special(ScanCode::PAGE_DOWN) => {
                if self.selected_disk_idx + 1 < self.storage_disks.len() {
                    self.select_disk_tab(self.selected_disk_idx + 1);
                }
            }
            Key::Special(ScanCode::UP) => {
                self.selected_file_idx = self.selected_file_idx.saturating_sub(1);
            }
            Key::Special(ScanCode::DOWN) => {
                if !self.files.is_empty() {
                    self.selected_file_idx = (self.selected_file_idx + 1).min(self.files.len() - 1);
                }
            }
            Key::Special(ScanCode::LEFT) => {
                if self.filesys_action_idx >= 1 {self.filesys_action_idx -= 1 } else { self.filesys_action_idx = 0 }
            }
            Key::Special(ScanCode::RIGHT) => {
                if self.filesys_action_idx < 7 {self.filesys_action_idx += 1 } else { self.filesys_action_idx = 7 }
            }
            Key::Special(ScanCode::END) => {
                if self.files.is_empty() {
                    if self.filesys_action_idx == 2 {
                        let new_file = format!("{}{}new_file_{}.txt", self.current_path, if self.current_path.ends_with('\\') || self.current_path.ends_with('/') { "" } else { "\\" }, self.filesys_new_counter);
                        match crate::FileSystem::touch(&new_file) {
                            Ok(_) => {
                                self.filesys_new_counter += 1;
                                self.status_line = format!("Created {}", new_file);
                                self.refresh_storage();
                            }
                            Err(e) => self.status_line = format!("Create failed: {}", e),
                        }
                    } else if self.filesys_action_idx == 3 {
                        let new_dir = format!("{}{}new_folder_{}", self.current_path, if self.current_path.ends_with('\\') || self.current_path.ends_with('/') { "" } else { "\\" }, self.filesys_new_counter);
                        match crate::FileSystem::mkdir(&new_dir) {
                            Ok(_) => {
                                self.filesys_new_counter += 1;
                                self.status_line = format!("Created {}", new_dir);
                                self.refresh_storage();
                            }
                            Err(e) => self.status_line = format!("Create folder failed: {}", e),
                        }
                    }
                    return;
                }
                let entry = self.files[self.selected_file_idx].clone();
                let sep = if self.current_path.ends_with('\\') || self.current_path.ends_with('/') { "" } else { "\\" };
                let full_path = format!("{}{}{}", self.current_path, sep, entry.name);

                if let Some(action) = self.filesys_pending_action {
                    let result = match action {
                        FilePendingAction::Rename => {
                            let dst = format!("{}{}renamed_{}", self.current_path, sep, entry.name);
                            crate::FileSystem::move_file(&full_path, &dst)
                        }
                        FilePendingAction::Copy => {
                            let dst = format!("{}{}{}_copy", self.current_path, sep, entry.name);
                            if entry.is_dir {
                                crate::FileSystem::clone_dir(&full_path, &dst)
                            } else {
                                crate::FileSystem::copy(&full_path, &dst)
                            }
                        }
                        FilePendingAction::Move => {
                            let dst = format!("{}{}{}_moved", self.current_path, sep, entry.name);
                            crate::FileSystem::move_file(&full_path, &dst)
                        }
                        FilePendingAction::Delete => crate::FileSystem::remove(&full_path),
                    };

                    match result {
                        Ok(_) => {
                            self.status_line = format!("{:?} complete for {}", action, entry.name);
                            self.filesys_pending_action = None;
                            self.refresh_storage();
                        }
                        Err(e) => {
                            self.status_line = format!("{:?} failed: {}", action, e);
                            self.filesys_pending_action = None;
                        }
                    }
                    return;
                }

                match self.filesys_action_idx {
                    0 => {
                        if entry.is_dir {
                            if entry.name == "." {
                                return;
                            } else if entry.name == ".." {
                                if let Some(pos) = self.current_path.rfind('\\') {
                                    if pos == 0 {
                                        self.current_path = String::from("\\");
                                    } else {
                                        self.current_path.truncate(pos);
                                    }
                                }
                                self.refresh_storage();
                                return;
                            } else {
                                if !self.current_path.ends_with('\\') {
                                    self.current_path.push('\\');
                                }
                                self.current_path.push_str(&entry.name);
                                self.selected_file_idx = 0;
                                self.refresh_storage();
                                return;
                            }
                        }
                        if (entry.name == "PAGEFILE") || (entry.name == "BOOTX64.EFI") {
                            ui_error(25);
                        } else {
                            // Dashboard will handle editor opening when it sees we're on the Storage tab
                            // and selected_file_idx is what it is, and we've "requested" an action.
                            // For now, we'll just set the status line and rely on sync.
                            self.status_line = format!("Opening {}...", entry.name);
                        }
                    }
                    1 => {
                        self.status_line = format!("{}: {} bytes, {}", entry.name, entry.size, if entry.is_dir { "directory" } else { "file" });
                    }
                    2 => {
                        let new_file = format!("{}{}new_file_{}.txt", self.current_path, sep, self.filesys_new_counter);
                        match crate::FileSystem::touch(&new_file) {
                            Ok(_) => {
                                self.filesys_new_counter += 1;
                                self.status_line = format!("Created {}", new_file);
                                self.refresh_storage();
                            }
                            Err(e) => self.status_line = format!("Create failed: {}", e),
                        }
                    }
                    3 => {
                        let new_dir = format!("{}{}new_folder_{}", self.current_path, sep, self.filesys_new_counter);
                        match crate::FileSystem::mkdir(&new_dir) {
                            Ok(_) => {
                                self.filesys_new_counter += 1;
                                self.status_line = format!("Created {}", new_dir);
                                self.refresh_storage();
                            }
                            Err(e) => self.status_line = format!("Create folder failed: {}", e),
                        }
                    }
                    4 => {
                        self.filesys_pending_action = Some(FilePendingAction::Rename);
                        self.status_line = format!("Confirm rename of {}", entry.name);
                    }
                    5 => {
                        self.filesys_pending_action = Some(FilePendingAction::Copy);
                        self.status_line = format!("Confirm copy of {}", entry.name);
                    }
                    6 => {
                        self.filesys_pending_action = Some(FilePendingAction::Move);
                        self.status_line = format!("Confirm move of {}", entry.name);
                    }
                    7 => {
                        self.filesys_pending_action = Some(FilePendingAction::Delete);
                        self.status_line = format!("Confirm delete of {}", entry.name);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }

    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {


        let content_top = y;
        let margin = 16usize;
        let gutter = 12usize; // space between widgets/rows
        let line_h = 15usize; // standard text line height
        let width = 600usize;
        let height = 500usize;
        // Title and path
        let base_y = content_top + margin;
        let strx = format!("File Explorer ({:#?})", self.filesys_new_counter);
        pg.draw_text(margin + x, base_y - 4, strx.as_str(), 0x00FF00);
        pg.draw_text(margin + x, base_y + 8, &alloc::format!("Path: {}", self.current_path), 0xAAAAAA);

        // Table area
        let list_x = margin + x;
        let list_y = base_y + 28;
        let list_w = core::cmp::min(width - margin * 2, 720);
        // `list_y` is screen-space; keep the layout calculation local to the
        // 500px content area so window placement cannot underflow it.
        let list_h = core::cmp::min(height.saturating_sub(margin + 28 + 90), 460);
        pg.draw_rect_outline(list_x, list_y, list_w, list_h, 0x888888);

        // Header row with better spacing and column guides
        pg.fill_rect(list_x + 1, list_y + 1, list_w - 2, line_h, 0x333333);
        pg.draw_text(list_x + 8, list_y + 4, "TYPE  NAME                                 SIZE (BYTES)  ATTR", 0xCCCCCC);
        // Optional column separators
        pg.draw_line(list_x + 48, list_y + 1, list_x + 48, list_y + list_h - 1, 0x444444);
        pg.draw_line(list_x + 340, list_y + 1, list_x + 340, list_y + list_h - 1, 0x444444);

        // Rows
        let mut y = list_y + line_h + gutter;
        for (i, entry) in self.files.iter().enumerate() {
            if y + line_h > list_y + list_h - 2 { break; }
            let color = if i == self.selected_file_idx { 0xFFFF00 } else { 0xFFFFFF };
            let icon = if entry.is_dir { pixel_graphics::icons::FOLDER_ICON_DATA } else {
                let dec_syn = ["json", "xml", "toml", "yaml", "yml"];
                let sys_syn = ["sys", "efi", "asm"];
                let prog_syn = ["micro", "module", "dmx", "bin", "rs"];
                let exec_syn = ["cxf", "cxp"];


                let ext = entry.name.split(".").last().unwrap();
                if dec_syn.contains(&ext) {
                    pixel_graphics::icons::JSON_ICON_DATA
                } else if sys_syn.contains(&ext) {
                    pixel_graphics::icons::EXECUTABLE_ICON_DATA
                } else if prog_syn.contains(&ext) {
                    pixel_graphics::icons::CODE_ICON_DATA
                } else if exec_syn.contains(&ext) {
                    pixel_graphics::icons::EXECUTABLE_ICON_DATA
                } else {
                    pixel_graphics::icons::FILE_ICON_DATA
                }
            };

            let size: String = if entry.size < 10000 {
                format!("{}", entry.size)
            } else if entry.size/1024 < 10000 {
                format!("{}K", (entry.size/1024))
            } else {
                format!("{}M", (entry.size/1024)/1024)
            };


            let background = if i == self.selected_file_idx { 0x333333 } else { 0x222222 };
            pg.draw_icon(list_x + 16, y, 16, 16, &icon);
            pg.draw_text_bg(list_x + 56, y, &alloc::format!("{:<32}", entry.name), color, background);
            pg.draw_text_bg(list_x + 348, y, &alloc::format!("{:>12}", size), 0xCCCCCC, background);
            pg.draw_text_bg(list_x + 470, y, if entry.is_dir { "DIR" } else { "FILE" }, 0x6666FF, background);
            y += line_h;
        }

        let props_x = list_x + list_w + gutter;
        let props_w = core::cmp::min(width.saturating_sub(props_x + margin), 360);
        if props_w > 120 {
            pg.draw_rect_outline(props_x, list_y, props_w, list_h, 0x777777);
            pg.draw_text(props_x + 10, list_y + 10, "Properties", 0x00FF00);
            if let Some(entry) = self.files.get(self.selected_file_idx) {
                let sep = if self.current_path.ends_with('\\') || self.current_path.ends_with('/') { "" } else { "\\" };
                let full_path = format!("{}{}{}", self.current_path, sep, entry.name);
                pg.draw_text(props_x + 10, list_y + 40, &format!("Name: {}", entry.name), 0xFFFFFF);
                pg.draw_text(props_x + 10, list_y + 60, &format!("Type: {}", if entry.is_dir { "Directory" } else { "File" }), 0xCCCCCC);
                pg.draw_text(props_x + 10, list_y + 80, &format!("Size: {} bytes", entry.size), 0xCCCCCC);
                pg.draw_text(props_x + 10, list_y + 100, &format!("Path: {}", full_path), 0x888888);
                pg.draw_text(props_x + 10, list_y + 130, &format!("Index: {} / {}", self.selected_file_idx + 1, self.files.len()), 0x888888);
            } else {
                pg.draw_text(props_x + 10, list_y + 40, "No item selected", 0x888888);
            }

            if let Some(action) = self.filesys_pending_action {
                let confirm_y = list_y + list_h - 90;
                pg.fill_rect(props_x + 8, confirm_y, props_w - 16, 72, 0x332222);
                pg.draw_rect_outline(props_x + 8, confirm_y, props_w - 16, 72, 0xFFAA00);
                pg.draw_text(props_x + 16, confirm_y + 10, "Confirm Operation", 0xFFAA00);
                pg.draw_text(props_x + 16, confirm_y + 30, &format!("{:?}", action), 0xFFFFFF);
                pg.draw_text(props_x + 16, confirm_y + 50, "END confirms, ESC cancels", 0xCCCCCC);
            }
        }

        let actions_y = list_h + margin*8;
        pg.draw_text(margin, actions_y, "Actions for Selected Item", 0xCCCCCC);
        let actions = ["Open", "Props", "New File", "New Dir", "Rename", "Copy", "Move", "Delete"];
        let mut action_x = margin + x;
        let action_y = actions_y + 20;
        for (idx, action) in actions.iter().enumerate() {
            let is_focused = idx == self.filesys_action_idx;
            let color = if is_focused { 0x00AA00 } else { 0x444444 };
            pg.fill_rect(action_x, action_y, 92, 24, color);
            pg.draw_text(action_x + 6, action_y + 4, action, 0xFFFFFF);
            action_x += 100;
        }
        pg.draw_text(margin + x, action_y + 34, "UP/DOWN selects, ENTER opens folders, END runs the selected action", 0x888888);
        pg.draw_text(margin + x, action_y + 52, &self.status_line, 0xFFFF00);
    }
}

impl AppInfo for X_Storage {
    fn name(&self) -> &str {
        "File Manager"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn icon(&self) -> [u32; 1024] {
        crate::ui::pixel_graphics::icons::FLOPPY_SAVE_32_ICON_DATA
    }

    fn dimensions(&self) -> (usize, usize) {
        (600, 500)
    }
}
