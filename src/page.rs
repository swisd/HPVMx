use alloc::vec;
use alloc::vec::Vec;
use crate::filesystem::FileSystem;
use crate::vdebug;

static mut GLOBALPAGE: [u8; 134217728] = [0; 134217728]; // should be with capacity 134217728

#[repr(C, packed)]
pub struct PagefileHeader {
    magic: u32,
    header_sz: u16,
    block_sz: u16,
    block_count: u16,
    table_pos: u32,
    padding: [u8; 2],  // Align to 16 bytes
    // 4080 bytes * 8 = 32,640 blocks supported.
    bitmap: [u8; 4080],

}

#[repr(C, packed)]
pub struct BlockMetadata {
    // 16 bytes total
    name: [u8; 12],     // Short ASCII name (e.g., "STACK", "KERNEL", "HEAP")
    flags: u16,         // Read/Write permissions, Dirty bit, etc.
    owner_id: u16,      // Process ID or Module ID that owns this page
}

impl PagefileHeader {
    fn to_hex_bytes(&self) -> [u8; 4096] {
        let mut buf = [0u8; 4096];
        buf[0..4].copy_from_slice(&self.magic.to_be_bytes());
        buf[4..6].copy_from_slice(&self.header_sz.to_le_bytes());
        buf[6..8].copy_from_slice(&self.block_sz.to_le_bytes());
        buf[8..10].copy_from_slice(&self.block_count.to_le_bytes());
        buf
    }
    pub const fn DefaultHeader() -> PagefileHeader {
        PagefileHeader {
            magic: 0x50414745,
            header_sz: 0x1000,
            block_sz: 0x1000,
            block_count: 0x7F80,
            table_pos: 0x7F81000,
            padding: [0; 2],
            bitmap: [0; 4080],

        }
    }
}

pub struct Pagefile {
    pub(crate) header: PagefileHeader
}

impl Default for Pagefile {
    fn default() -> Self {
        Self::new()
    }
}

impl Pagefile {
    pub const fn new() -> Self {
        Self {
            header: PagefileHeader::DefaultHeader(),
        }
    }

    pub fn max_blocks(&self) -> u32 {
        self.header.block_count as u32
    }

    pub fn create_pagefile(&mut self) {
        let mut page = vec![0u8; 134217728];
        let mut zvec = [0u8; 256];

        FileSystem::cd("/");
        match FileSystem::read_from_file_bytes_position("PAGEFILE", &mut zvec, 2048) {
            Ok(_r) => vdebug!("PAGE", "pagefile ok"),
            Err(e) => {
                vdebug!("PAGE", "cannot read pagefile: {:#?}", e);
                vdebug!("PAGE", "creating new pagefile");
                let _ = FileSystem::write_to_file_bytes("PAGEFILE", &*page, 'w');
            }
        }
        vdebug!("PAGE", "updating pagefile headers");
        // Header PAGE magic, header size, block size (00 10 is 4096 in hex LE), block count (32767) (80 7F)
        let _ = FileSystem::write_to_file_bytes_position("PAGEFILE", &self.header.to_hex_bytes(), 0x00);
    }

    pub fn write_block(&self, id: u32, data: &[u8; 4096]) -> Result<(), &'static str> {
        if id >= self.header.block_count as u32 {
            return Err("Block ID out of bounds");
        }
        FileSystem::cd("/");
        let position = 4096 + (4096 * id as u64);
        FileSystem::write_to_file_bytes_position("PAGEFILE", data, position)
    }

    pub fn read_block(&self, id: u32) -> Result<[u8; 4096], &'static str> {
        if id >= self.header.block_count as u32 {
            return Err("Block ID out of bounds");
        }
        let mut buf = [0u8; 4096];
        FileSystem::cd("/");
        let position = 4096 + (4096 * id as u64);
        FileSystem::read_from_file_bytes_position("PAGEFILE", &mut buf, position)?;
        Ok(buf)
    }
}

