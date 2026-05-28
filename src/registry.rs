use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Write;

use crate::filesystem::FileSystem;
use crate::ui::UiSettings;

pub const DEFAULT_SYSTEM_REG_PATH: &str = "/SYSTEM.REG";
pub const DEFAULT_DEVICE_REG_PATH: &str = "/DEVICES.REG";

const HEADER: &str = "HPVMX Registry Version 1";

#[derive(Clone)]
pub struct SuperHive {
    pub name: String,
    pub hives: Vec<Hive>,
}

#[derive(Clone)]
pub struct Hive {
    pub name: String,
    pub data: Vec<Registry>,
}

#[derive(Clone)]
pub struct Registry {
    pub name: String,
    pub data_groups: Vec<DataGroup>,
}

#[derive(Clone)]
pub struct DataGroup {
    pub name: String,
    pub data: Vec<Pair>,
}

#[derive(Clone)]
pub struct Pair {
    pub name: String,
    pub value: DataType,
}

#[derive(Clone)]
pub enum DataType {
    Bool(bool),
    Int(i64),
    String(String),
    Hex(u64),
    Bin(Vec<u8>),
}

pub fn save_system_registry(path: &str, settings: &UiSettings) -> Result<(), &'static str> {
    let super_hive = build_system_hive(settings);
    write_text_file(path, &serialize_super_hive(&super_hive)?)
}

pub fn load_system_registry(path: &str, settings: &mut UiSettings) -> Result<(), &'static str> {
    let text = FileSystem::read_file_to_string(path)?;
    load_from_str(&text, Some(settings), true)
}

pub fn save_device_registry(path: &str) -> Result<(), &'static str> {
    let super_hive = SuperHive {
        name: String::from("HPVMx"),
        hives: vec![Hive {
            name: String::from("Hardware"),
            data: vec![Registry {
                name: String::from("Devices"),
                data_groups: vec![device_group()],
            }],
        }],
    };
    write_text_file(path, &serialize_super_hive(&super_hive)?)
}

pub fn load_device_registry(path: &str) -> Result<(), &'static str> {
    let text = FileSystem::read_file_to_string(path)?;
    load_from_str(&text, None, true)
}

fn build_system_hive(settings: &UiSettings) -> SuperHive {
    SuperHive {
        name: String::from("HPVMx"),
        hives: vec![
            Hive {
                name: String::from("System"),
                data: vec![Registry {
                    name: String::from("Settings"),
                    data_groups: vec![settings_group(settings)],
                }],
            },
            Hive {
                name: String::from("Hardware"),
                data: vec![Registry {
                    name: String::from("Devices"),
                    data_groups: vec![device_group()],
                }],
            },
        ],
    }
}

fn settings_group(settings: &UiSettings) -> DataGroup {
    let mut data = Vec::new();
    push_bool(&mut data, "extra_debug_info", settings.extra_debug_info);
    push_bool(&mut data, "folder_absolute_sizes", settings.folder_absolute_sizes);
    push_bool(&mut data, "state_save_restore", settings.state_save_restore);
    push_bool(&mut data, "extended_symbol_library", settings.extended_symbol_library);
    push_bool(&mut data, "ring0_udmi_udxi", settings.ring0_udmi_udxi);
    push_bool(&mut data, "controllang_support", settings.controllang_support);
    push_bool(&mut data, "pg_vshaders", settings.pg_vshaders);
    push_bool(&mut data, "experimental_mem_comp", settings.experimental_mem_comp);
    push_bool(&mut data, "auto_refresh_storage", settings.auto_refresh_storage);
    push_bool(&mut data, "show_hidden_files", settings.show_hidden_files);
    push_int(&mut data, "general_profile", settings.general_profile);
    push_int(&mut data, "boot_target", settings.boot_target);
    push_int(&mut data, "interface_density", settings.interface_density);
    push_int(&mut data, "vm_safety_policy", settings.vm_safety_policy);
    push_int(&mut data, "network_profile", settings.network_profile);
    push_int(&mut data, "storage_policy", settings.storage_policy);
    push_int(&mut data, "package_policy", settings.package_policy);
    push_int(&mut data, "developer_level", settings.developer_level);
    push_int(&mut data, "security_policy", settings.security_policy);
    push_int(&mut data, "ui_scaling", settings.ui_scaling);
    push_int(&mut data, "terminal_font", settings.terminal_font);
    push_bool(&mut data, "pg_scanlines", settings.pg_scanlines);
    push_bool(&mut data, "pg_dither", settings.pg_dither);
    push_bool(&mut data, "pg_glitch", settings.pg_glitch);
    push_int(&mut data, "pg_aberration", settings.pg_aberration);

    DataGroup {
        name: String::from("settings.ui"),
        data,
    }
}

fn device_group() -> DataGroup {
    let mut data = Vec::new();
    for (alias, path) in &FileSystem::get_state().device_map {
        data.push(Pair {
            name: alias.clone(),
            value: DataType::String(path.clone()),
        });
    }

    DataGroup {
        name: String::from("devices.map"),
        data,
    }
}

fn push_bool(data: &mut Vec<Pair>, name: &str, value: bool) {
    data.push(Pair {
        name: name.to_string(),
        value: DataType::Bool(value),
    });
}

fn push_int(data: &mut Vec<Pair>, name: &str, value: usize) {
    data.push(Pair {
        name: name.to_string(),
        value: DataType::Int(value as i64),
    });
}

fn serialize_super_hive(super_hive: &SuperHive) -> Result<String, &'static str> {
    let mut out = String::new();
    writeln!(&mut out, "{}", HEADER).map_err(|_| "format error")?;
    writeln!(&mut out, "# super_hive={}", super_hive.name).map_err(|_| "format error")?;
    writeln!(&mut out).map_err(|_| "format error")?;

    for hive in &super_hive.hives {
        writeln!(&mut out, "# hive={}", hive.name).map_err(|_| "format error")?;
        for registry in &hive.data {
            writeln!(&mut out, "# registry={}", registry.name).map_err(|_| "format error")?;
            for group in &registry.data_groups {
                writeln!(&mut out, "[{}]", group.name).map_err(|_| "format error")?;
                for pair in &group.data {
                    write_pair(&mut out, pair)?;
                }
                writeln!(&mut out).map_err(|_| "format error")?;
            }
        }
    }

    Ok(out)
}

fn write_pair(out: &mut String, pair: &Pair) -> Result<(), &'static str> {
    match &pair.value {
        DataType::Bool(value) => writeln!(out, "{}={}", pair.name, if *value { "true" } else { "false" }),
        DataType::Int(value) => writeln!(out, "{}={}", pair.name, value),
        DataType::String(value) => writeln!(out, "{}=\"{}\"", pair.name, escape_value(value)),
        DataType::Hex(value) => writeln!(out, "{}=hex:{:X}", pair.name, value),
        DataType::Bin(value) => {
            write!(out, "{}=bin:", pair.name).map_err(|_| "format error")?;
            for byte in value {
                write!(out, "{:02X}", byte).map_err(|_| "format error")?;
            }
            writeln!(out)
        }
    }
    .map_err(|_| "format error")
}

fn write_text_file(path: &str, contents: &str) -> Result<(), &'static str> {
    let _ = FileSystem::remove(path);
    let _ = FileSystem::touch(path);
    FileSystem::write_to_file(path, contents, 'w')
}

fn load_from_str(text: &str, mut settings: Option<&mut UiSettings>, load_devices: bool) -> Result<(), &'static str> {
    let mut section = "";
    let mut imported_devices = false;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') || line == HEADER {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            section = &line[1..line.len().saturating_sub(1)];
            if section == "devices.map" && load_devices && !imported_devices {
                FileSystem::get_state().device_map.clear();
                imported_devices = true;
            }
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        match section {
            "settings.ui" => {
                if let Some(settings) = settings.as_deref_mut() {
                    apply_setting(settings, key.trim(), value.trim())?;
                }
            }
            "devices.map" if load_devices => {
                let alias = key.trim();
                let path = unquote_value(value.trim());
                if !alias.is_empty() && !path.is_empty() {
                    FileSystem::get_state().device_map.push((alias.to_string(), path));
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn apply_setting(settings: &mut UiSettings, key: &str, value: &str) -> Result<(), &'static str> {
    match key {
        "extra_debug_info" => settings.extra_debug_info = parse_bool(value)?,
        "folder_absolute_sizes" => settings.folder_absolute_sizes = parse_bool(value)?,
        "state_save_restore" => settings.state_save_restore = parse_bool(value)?,
        "extended_symbol_library" => settings.extended_symbol_library = parse_bool(value)?,
        "ring0_udmi_udxi" => settings.ring0_udmi_udxi = parse_bool(value)?,
        "controllang_support" => settings.controllang_support = parse_bool(value)?,
        "pg_vshaders" => settings.pg_vshaders = parse_bool(value)?,
        "experimental_mem_comp" => settings.experimental_mem_comp = parse_bool(value)?,
        "auto_refresh_storage" => settings.auto_refresh_storage = parse_bool(value)?,
        "show_hidden_files" => settings.show_hidden_files = parse_bool(value)?,
        "general_profile" => settings.general_profile = parse_usize(value)?,
        "boot_target" => settings.boot_target = parse_usize(value)?,
        "interface_density" => settings.interface_density = parse_usize(value)?,
        "vm_safety_policy" => settings.vm_safety_policy = parse_usize(value)?,
        "network_profile" => settings.network_profile = parse_usize(value)?,
        "storage_policy" => settings.storage_policy = parse_usize(value)?,
        "package_policy" => settings.package_policy = parse_usize(value)?,
        "developer_level" => settings.developer_level = parse_usize(value)?,
        "security_policy" => settings.security_policy = parse_usize(value)?,
        "ui_scaling" => settings.ui_scaling = parse_usize(value)?,
        "terminal_font" => settings.terminal_font = parse_usize(value)?,
        "pg_scanlines" => settings.pg_scanlines = parse_bool(value)?,
        "pg_dither" => settings.pg_dither = parse_bool(value)?,
        "pg_glitch" => settings.pg_glitch = parse_bool(value)?,
        "pg_aberration" => settings.pg_aberration = parse_usize(value)?,
        _ => {}
    }
    Ok(())
}

fn parse_bool(value: &str) -> Result<bool, &'static str> {
    match value.trim().trim_matches('"') {
        "true" | "1" | "on" | "yes" => Ok(true),
        "false" | "0" | "off" | "no" => Ok(false),
        _ => Err("invalid registry bool"),
    }
}

fn parse_usize(value: &str) -> Result<usize, &'static str> {
    value.trim().trim_matches('"').parse::<usize>().map_err(|_| "invalid registry number")
}

fn escape_value(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            _ => out.push(ch),
        }
    }
    out
}

fn unquote_value(value: &str) -> String {
    let trimmed = value.trim();
    let body = if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    };

    let mut out = String::new();
    let mut escaping = false;
    for ch in body.chars() {
        if escaping {
            out.push(ch);
            escaping = false;
        } else if ch == '\\' {
            escaping = true;
        } else {
            out.push(ch);
        }
    }
    out
}
