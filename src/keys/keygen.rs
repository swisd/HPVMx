use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use crate::vdebug_autoprefix;

struct ProductionKey {
    pub product_id: u32,       // e.g., internal product / channel grouping
    pub serial_number: u64,    //  unique sequence/serial block
    pub hardware_flags: u16,   // extra metadata bits (including 'N' flag configuration)
}

static ALPHABET: &[u8; 24] = b"BCDFGHJKMPQRTVWXY2346789";


impl ProductionKey {
    pub fn to_bytes(&self) -> [u8; 15] {
        let mut bytes = [0u8; 15];

        // Pack data into the byte array using bitwise shifts.
        // This is a typical layout where fields map out sequentially across the 114 bits.
        bytes[0] = (self.hardware_flags & 0xFF) as u8;
        bytes[1] = ((self.hardware_flags >> 8) & 0xFF) as u8;

        // Map the serial number across the next chunk of bytes
        let s_bytes = self.serial_number.to_le_bytes();
        bytes[2..10].copy_from_slice(&s_bytes);

        // Map the product ID to the remaining space
        let p_bytes = self.product_id.to_le_bytes();
        bytes[10..14].copy_from_slice(&p_bytes);

        // Leaving the 15th byte as zero padding for upper bit safety during big-int math
        bytes[14] = 0;

        bytes
    }

    /// Unpacks a raw 15-byte array back into the logical named struct fields.
    pub fn from_bytes(bytes: [u8; 15]) -> Self {
        let hardware_flags = u16::from_le_bytes([bytes[0], bytes[1]]);

        let mut s_bytes = [0u8; 8];
        s_bytes.copy_from_slice(&bytes[2..10]);
        let serial_number = u64::from_le_bytes(s_bytes);

        let mut p_bytes = [0u8; 4];
        p_bytes.copy_from_slice(&bytes[10..14]);
        let product_id = u32::from_le_bytes(p_bytes);

        ProductionKey {
            product_id,
            serial_number,
            hardware_flags,
        }
    }
}

pub fn decode_to_struct(product_key: &str) -> Result<ProductionKey, String> {
    let clean_key: String = product_key.replace("-", "").to_uppercase();
    if clean_key.len() != 25 {
        vdebug_autoprefix!("Product key must be exactly 25 characters long.");
        return Err("Product key must be exactly 25 characters long.".to_string());
    }

    let mut byte_structure = [0u8; 15];
    let mut n_bit_flag = 0;

    for i in (0..25).rev() {
        let current_char = clean_key.chars().nth(i).unwrap();

        let current_value = if current_char == 'N' {
            n_bit_flag = i;
            0
        } else if let Some(idx) = ALPHABET.iter().position(|&c| c == current_char as u8) {
            idx
        } else {
            vdebug_autoprefix!("Invalid character: {}", current_char);
            return Err(format!("Invalid character: {}", current_char));
        };

        let mut carry = current_value as u32;
        for j in (0..15).rev() {
            let value = (byte_structure[j] as u32 * 24) + carry;
            byte_structure[j] = (value & 0xFF) as u8;
            carry = value >> 8;
        }
    }

    if n_bit_flag != 0 {
        byte_structure[0] = (n_bit_flag & 0xFF) as u8;
    }

    // Convert our raw mathematical byte slice safely into our neat Struct layout
    Ok(ProductionKey::from_bytes(byte_structure))
}

pub fn encode_from_struct(key_struct: &ProductionKey) -> String {
    let mut byte_structure = key_struct.to_bytes();

    let n_bit_flag = byte_structure[0] as usize;
    let is_modern_n_style = (n_bit_flag & 1) == 0;

    byte_structure[0] = 0; // Strip flag bits out before starting polynomial math

    let mut key_chars = vec![' '; 25];

    for i in (0..25).rev() {
        let mut remainder = 0u32;
        for j in 0..15 {
            let value = (remainder << 8) + byte_structure[j] as u32;
            byte_structure[j] = (value / 24) as u8;
            remainder = value % 24;
        }
        key_chars[i] = ALPHABET[remainder as usize] as char;
    }

    if is_modern_n_style && n_bit_flag < 25 {
        for i in 0..n_bit_flag {
            key_chars[i] = key_chars[i + 1];
        }
        key_chars[n_bit_flag] = 'N';
    }

    let raw_string: String = key_chars.into_iter().collect();
    let chunks: Vec<&str> = (0..5).map(|i| &raw_string[i*5..(i+1)*5]).collect();
    chunks.join("-")
}