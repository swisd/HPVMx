//! HPVMx-native relocatable object container used by the on-device C toolchain.
//!
//! The object currently stores the C backend's relocatable assembly section;
//! symbols and imports are resolved by the x64 linker so several objects can
//! be linked together with Micro-C assembly modules.

use alloc::{string::String, vec::Vec};
use alloc::string::ToString;
use crate::vdebug_autoprefix;

fn err_str(msg: &str) -> String {
    vdebug_autoprefix!(12, "C object error [Advanced Debug]: {}", msg);
    msg.to_string()
}

const MAGIC: [u8; 4] = *b"HXO1";
const VERSION: u16 = 1;
const MAX_ASSEMBLY_SIZE: usize = 16 * 1024 * 1024;

#[derive(Debug)]
pub struct CObject {
    pub assembly: String,
}

pub fn encode(assembly: &str) -> Result<Vec<u8>, String> {
    if assembly.is_empty() || assembly.len() > MAX_ASSEMBLY_SIZE {
        return Err(err_str("C object assembly section is empty or too large"));
    }
    let size = u32::try_from(assembly.len()).map_err(|_| err_str("C object section exceeds format limit"))?;
    let mut bytes = Vec::with_capacity(10 + assembly.len());
    bytes.extend_from_slice(&MAGIC);
    bytes.extend_from_slice(&VERSION.to_le_bytes());
    bytes.extend_from_slice(&size.to_le_bytes());
    bytes.extend_from_slice(assembly.as_bytes());
    Ok(bytes)
}

pub fn decode(bytes: &[u8]) -> Result<CObject, String> {
    if bytes.len() < 10 || bytes[..4] != MAGIC { return Err(err_str("not an HPVMx HXO object")); }
    let version = u16::from_le_bytes(bytes[4..6].try_into().unwrap());
    if version != VERSION { return Err(err_str("unsupported HPVMx object version")); }
    let size = u32::from_le_bytes(bytes[6..10].try_into().unwrap()) as usize;
    if size == 0 || size > MAX_ASSEMBLY_SIZE || bytes.len() != 10 + size {
        return Err(err_str("invalid HPVMx object section size"));
    }
    let assembly = core::str::from_utf8(&bytes[10..]).map_err(|_| err_str("object assembly section is not UTF-8"))?;
    Ok(CObject { assembly: String::from(assembly) })
}
