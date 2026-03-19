use alloc::string::String;
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

    /// Returns the raw bytes of the struct itself (the stack/address part)
    fn get_stack_bytes(&self) -> &[u8] {
        let size = core::mem::size_of_val(self);
        unsafe { core::slice::from_raw_parts(self as *const _ as *const u8, size) }
    }

    /// Every struct must implement how to save its specific heap data
    fn get_heap_bytes(&self) -> Vec<u8>;
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
        let size = core::mem::size_of::<Self>();
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

    fn get_heap_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let size = core::mem::size_of::<LogEntry>();
        let ptr = &self as *const _ as *const u8;
        unsafe {
            data.extend_from_slice(core::slice::from_raw_parts(ptr, size));
        }
        data
    }
}

pub struct PersistenceManager;
impl PersistenceManager {
    pub fn save<T: Persistable>(path: &str, obj: &T, mode: char) {
        FileSystem::cd("/");
        let stack_bytes = obj.get_stack_bytes();
        let heap_bytes = obj.get_heap_bytes();

        let mut file_data = Vec::new();
        file_data.extend_from_slice(&T::magic().to_le_bytes());
        file_data.extend_from_slice(&T::version().to_le_bytes());
        file_data.extend_from_slice(&(stack_bytes.len() as u32).to_le_bytes());
        file_data.extend_from_slice(&(heap_bytes.len() as u32).to_le_bytes());

        file_data.extend_from_slice(stack_bytes);
        file_data.extend_from_slice(&heap_bytes);

        FileSystem::write_to_file_bytes(path, &file_data, mode).unwrap();
    }

    // pub unsafe fn restore_stack<T: Persistable>(path: &str, obj: &mut T) -> Vec<u8> {
    //     let file_bytes = FileSystem::read_file_bytes(path);
    //     if file_bytes.len() < 16 { return Vec::new(); }
    //
    //     // 1. Validate Magic
    //     let magic = u32::from_le_bytes(file_bytes[0..4].try_into().unwrap());
    //     if magic != T::magic() { panic!("Invalid Magic Number!"); }
    //
    //     // 2. Extract sizes
    //     let stack_size = u32::from_le_bytes(file_bytes[8..12].try_into().unwrap()) as usize;
    //     let heap_size = u32::from_le_bytes(file_bytes[12..16].try_into().unwrap()) as usize;
    //
    //     // 3. Restore the "Address/Stack" part
    //     let stack_start = 16;
    //     let stack_end = stack_start + stack_size;
    //     let obj_ptr = obj as *mut T as *mut u8;
    //     core::ptr::copy_nonoverlapping(file_bytes[stack_start..stack_end].as_ptr(), obj_ptr, stack_size);
    //
    //     // 4. Return the heap bytes so the struct can "re-hook" them
    //     file_bytes[stack_end..stack_end + heap_size].to_vec()
    // }
}




pub unsafe fn SAVE() {
    unsafe {
        FileSystem::remove("/STATE");
        FileSystem::touch("/STATE");
        PersistenceManager::save("/STATE", &KernelState { state: 0, entropy: 0x0 }, 'w');
        PersistenceManager::save("/STATE", &FileSystem::get_state(), 'a');
        PersistenceManager::save("/STATE", &get_log_buffer(), 'a');
    }
}


