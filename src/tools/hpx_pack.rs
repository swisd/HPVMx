//! On-device HPX v1 packer. Kept separate from the compiler so compiled images
//! can be packaged independently and the file format can be reused by apps.

use alloc::{string::{String, ToString}, vec::Vec};
use crate::vdebug_autoprefix;

fn err_str(msg: &str) -> String {
    vdebug_autoprefix!(12, "HPX pack error [Advanced Debug]: {}", msg);
    msg.to_string()
}

pub const HEADER_SIZE: usize = 108;
pub const KIND_APP: u16 = 1;
pub const KIND_BACKGROUND: u16 = 2;
pub const MAX_IMAGE_SIZE: usize = 64 * 1024 * 1024;
pub const MAX_STATE_SIZE: usize = 16 * 1024 * 1024;
pub const RELOCATION_MAGIC: [u8; 4] = *b"HPR1";

#[derive(Clone, Debug)]
pub struct ImportRelocation { pub patch_offset: u32, pub symbol: alloc::string::String }

#[derive(Clone, Copy, Debug)]
pub struct PackOptions {
    pub kind: u16,
    pub state_size: u32,
    pub step_offset: u32,
    pub draw_offset: u32,
    pub input_offset: u32,
    pub width: u32,
    pub height: u32,
    pub file_version: u32,
    pub description: [u8; 64],
}

/// Create an HPX v1 file from a position-independent x86-64 image.
pub fn pack(image: &[u8], options: PackOptions) -> Result<Vec<u8>, String> {
    pack_with_imports(image, options, &[])
}

pub fn pack_with_imports(image: &[u8], options: PackOptions, imports: &[ImportRelocation]) -> Result<Vec<u8>, String> {
    if image.is_empty() || image.len() > MAX_IMAGE_SIZE { return Err(err_str("image size must be between 1 byte and 64 MiB")); }
    if options.state_size as usize > MAX_STATE_SIZE { return Err(err_str("state size exceeds 16 MiB")); }
    if !matches!(options.kind, KIND_APP | KIND_BACKGROUND) { return Err(err_str("unknown HPX executable kind")); }
    if options.step_offset as usize >= image.len() { return Err(err_str("step callback offset is outside the image")); }
    if options.draw_offset != 0 && options.draw_offset as usize >= image.len() { return Err(err_str("draw callback offset is outside the image")); }
    if options.input_offset != 0 && options.input_offset as usize >= image.len() { return Err(err_str("input callback offset is outside the image")); }
    if options.kind == KIND_APP && (options.width == 0 || options.height == 0) { return Err(err_str("app dimensions must be nonzero")); }

    let image_size = u32::try_from(image.len()).map_err(|_| err_str("image size does not fit HPX header"))?;
    let mut file = Vec::with_capacity(HEADER_SIZE + image.len());
    file.extend_from_slice(b"HPX1");
    file.extend_from_slice(&1u16.to_le_bytes());
    file.extend_from_slice(&options.kind.to_le_bytes());
    file.extend_from_slice(&(HEADER_SIZE as u32).to_le_bytes());
    file.extend_from_slice(&image_size.to_le_bytes());
    file.extend_from_slice(&options.state_size.to_le_bytes());
    file.extend_from_slice(&options.step_offset.to_le_bytes());
    file.extend_from_slice(&options.draw_offset.to_le_bytes());
    file.extend_from_slice(&options.input_offset.to_le_bytes());
    file.extend_from_slice(&options.width.to_le_bytes());
    file.extend_from_slice(&options.height.to_le_bytes());
    file.extend_from_slice(&options.file_version.to_le_bytes());
    file.extend_from_slice(&options.description);
    debug_assert_eq!(file.len(), HEADER_SIZE);
    file.extend_from_slice(image);
    if !imports.is_empty() {
        file.extend_from_slice(&encode_imports(imports)?);
    }
    Ok(file)
}

/// Serialize import fixups for the HPX trailer and the separate `.hrel` sidecar.
pub fn encode_imports(imports: &[ImportRelocation]) -> Result<Vec<u8>, String> {
    if imports.len() > 4096 { return Err(err_str("too many imported functions")); }
    let mut out = Vec::new();
    out.extend_from_slice(&RELOCATION_MAGIC);
    out.extend_from_slice(&(imports.len() as u32).to_le_bytes());
    for reloc in imports {
        if reloc.symbol.is_empty() || reloc.symbol.len() > 128 || !reloc.symbol.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
            return Err(err_str("invalid import symbol in relocation table"));
        }
        out.extend_from_slice(&reloc.patch_offset.to_le_bytes());
        out.extend_from_slice(&(reloc.symbol.len() as u16).to_le_bytes());
        out.extend_from_slice(reloc.symbol.as_bytes());
    }
    Ok(out)
}

/// Read an HPR1 relocation block and validate each 64-bit patch site.
pub fn decode_imports(data: &[u8], image_size: usize) -> Result<Vec<ImportRelocation>, String> {
    if data.is_empty() { return Ok(Vec::new()); }
    if data.len() < 8 || data[..4] != RELOCATION_MAGIC { return Err(err_str("invalid HPX relocation trailer")); }
    let count = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
    if count > 4096 { return Err(err_str("too many HPX import relocations")); }
    let mut cursor = 8usize;
    let mut imports = Vec::with_capacity(count);
    for _ in 0..count {
        let offset_end = cursor.checked_add(6).ok_or_else(|| err_str("relocation table overflow"))?;
        let header = data.get(cursor..offset_end).ok_or_else(|| err_str("truncated relocation table"))?;
        let offset = u32::from_le_bytes(header[..4].try_into().unwrap());
        let name_len = u16::from_le_bytes(header[4..6].try_into().unwrap()) as usize;
        cursor = offset_end;
        let end = cursor.checked_add(name_len).ok_or_else(|| err_str("relocation name overflow"))?;
        let bytes = data.get(cursor..end).ok_or_else(|| err_str("truncated relocation symbol"))?;
        let symbol = core::str::from_utf8(bytes).map_err(|_| err_str("relocation symbol is not UTF-8"))?;
        if image_size < 8 || offset as usize > image_size - 8 { return Err(err_str("relocation patch lies outside image")); }
        if symbol.is_empty() || symbol.len() > 128 || !symbol.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') { return Err(err_str("invalid relocation symbol")); }
        imports.push(ImportRelocation { patch_offset: offset, symbol: alloc::string::String::from(symbol) });
        cursor = end;
    }
    if cursor != data.len() { return Err(err_str("extra bytes after relocation table")); }
    Ok(imports)
}

/// Parse decimal or `0x`-prefixed integers used by the shell pack command.
pub fn parse_u32(value: &str) -> Result<u32, String> {
    let parsed = if let Some(hex) = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).map_err(|_| err_str("invalid hexadecimal number"))?
    } else {
        value.parse::<u32>().map_err(|_| err_str("invalid decimal number"))?
    };
    Ok(parsed)
}

/// Metadata read from a `project.toml` file.
#[derive(Clone, Debug)]
pub struct ProjectMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
}

/// Parse a version string like "1.2.3" into a packed u32 (0x010203).
pub fn parse_version_to_u32(version: &str) -> u32 {
    let mut parts = version.split('.');
    let major = parts.next().and_then(|s| s.trim().parse::<u32>().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.trim().parse::<u32>().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.trim().parse::<u32>().ok()).unwrap_or(0);
    (major << 16) | (minor << 8) | patch
}

/// Simple TOML parser that extracts `name`, `version`, and `description` from a `[package]` section.
/// Supports only the subset needed for HPX metadata.
pub fn parse_project_toml(content: &str) -> Result<ProjectMetadata, String> {
    let mut in_package = false;
    let mut name = String::new();
    let mut version = String::new();
    let mut description = String::new();

    for line in content.lines() {
        let trimmed = line.trim();
        
        // Detect section headers
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let section = &trimmed[1..trimmed.len()-1].trim();
            in_package = *section == "package";
            continue;
        }
        
        if !in_package || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        
        // Parse key = value
        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim();
            let value = trimmed[eq_pos + 1..].trim();
            // Strip quotes from value
            let value = if (value.starts_with('"') && value.ends_with('"')) || 
                          (value.starts_with('\'') && value.ends_with('\'')) {
                &value[1..value.len()-1]
            } else {
                value
            };
            
            match key {
                "name" => name = value.to_string(),
                "version" => version = value.to_string(),
                "description" => description = value.to_string(),
                _ => {}
            }
        }
    }
    
    if name.is_empty() {
        Err(err_str("project.toml missing [package] name"))
    } else {
        Ok(ProjectMetadata { name, version, description })
    }
}

/// Fill PackOptions description field from a string (null-terminated, max 64 bytes).
pub fn fill_description(dest: &mut [u8; 64], text: &str) {
    let bytes = text.as_bytes();
    let len = bytes.len().min(63); // leave room for null terminator
    dest[..len].copy_from_slice(&bytes[..len]);
    dest[len] = 0;
}
