use alloc::vec;
use alloc::vec::Vec;
use crate::filesystem::FileSystem;

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

impl Pagefile {
    pub fn create_pagefile(&mut self) {
        let mut page = vec![0u8; 134217728];
        FileSystem::cd("/");
        if let Ok(pagefile) = FileSystem::read_file("PAGEFILE") {
            if pagefile.len() > 2048 {} else {
                FileSystem::write_to_file_bytes("PAGEFILE", &*page, 'w');
            }
        } else {
            FileSystem::write_to_file_bytes("PAGEFILE", &*page, 'w');
        }
        // Header PAGE magic, header size, block size (00 10 is 4096 in hex LE), block count (32767) (80 7F)
        FileSystem::write_to_file_bytes_position("PAGEFILE", &self.header.to_hex_bytes(), 0x00);
    }

    pub fn write_block(&self, id: u32, data: &[u8; 4096]) {
        if id >= self.header.block_count as u32 {  }
        FileSystem::cd("/");
        let position = 4096 + (4096*id);
        FileSystem::write_to_file_bytes_position("PAGEFILE", data, position as u64);
    }
    pub fn read_block(&self, id: u32) -> [u8; 4096] {
        let mut buf = [0u8; 4096];
        if id >= self.header.block_count as u32 {
            [0u8; 4096];
        }

        FileSystem::cd("/");
        let position = 4096 + (4096 * id);
        FileSystem::read_from_file_bytes_position("PAGEFILE", &mut buf, position as u64);
        buf
    }
}

