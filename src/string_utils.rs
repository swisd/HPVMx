use alloc::{string::String, vec::Vec};

/// Convert a Rust string to a NUL-terminated UTF-16 buffer, rejecting embedded NULs.
pub fn to_u16s(value: String) -> Result<Vec<u16>, String> {
    if value.contains('\0') {
        return Err(String::from("strings passed cannot contain NULs"));
    }
    let mut encoded: Vec<u16> = value.encode_utf16().collect();
    encoded.push(0);
    Ok(encoded)
}
