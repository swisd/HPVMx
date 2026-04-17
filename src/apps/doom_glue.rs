use crate::filesystem::FileSystem;
use alloc::alloc::{alloc, dealloc, Layout, realloc as rust_realloc};
use core::ffi::{c_void, c_char, CStr};
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::fmt::Write;
use crate::message;

struct DebugWriter;
impl core::fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // We could log to a buffer or serial port here
        Ok(())
    }
}

// Minimal malloc/free implementation using Rust allocator
// We store the size before the pointer to handle free() which doesn't provide size in C
#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
    if size == 0 { return core::ptr::null_mut(); }
    let layout = Layout::from_size_align(size + 16, 16).unwrap();
    let ptr = alloc(layout);
    if ptr.is_null() { return core::ptr::null_mut(); }
    *(ptr as *mut usize) = size;
    let result = ptr.add(16) as *mut c_void;
    // message!("\n", "malloc({}) -> {:?}", size, result);
    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
    if ptr.is_null() { return; }
    let actual_ptr = (ptr as *mut u8).sub(16);
    let size = *(actual_ptr as *mut usize);
    let layout = Layout::from_size_align(size + 16, 16).unwrap();
    dealloc(actual_ptr, layout);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void {
    if ptr.is_null() { return malloc(new_size); }
    if new_size == 0 { free(ptr); return core::ptr::null_mut(); }
    
    let actual_ptr = (ptr as *mut u8).sub(16);
    let old_size = *(actual_ptr as *mut usize);
    let old_layout = Layout::from_size_align(old_size + 16, 16).unwrap();
    
    let new_ptr = rust_realloc(actual_ptr, old_layout, new_size + 16);
    if new_ptr.is_null() { return core::ptr::null_mut(); }
    
    *(new_ptr as *mut usize) = new_size;
    new_ptr.add(16) as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn calloc(nmemb: usize, size: usize) -> *mut c_void {
    let total = nmemb * size;
    let ptr = malloc(total);
    if !ptr.is_null() {
        core::ptr::write_bytes(ptr, 0, total);
    }
    ptr
}

// Libc String and Utility Functions
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strcmp(s1: *const c_char, s2: *const c_char) -> i32 {
    let mut i = 0;
    while *s1.add(i) != 0 && *s1.add(i) == *s2.add(i) {
        i += 1;
    }
    (*s1.add(i) as i32) - (*s2.add(i) as i32)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _stricmp(s1: *const c_char, s2: *const c_char) -> i32 {
    let mut i = 0;
    loop {
        let c1 = (*s1.add(i) as u8).to_ascii_lowercase();
        let c2 = (*s2.add(i) as u8).to_ascii_lowercase();
        if c1 != c2 || c1 == 0 {
            return (c1 as i32) - (c2 as i32);
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub static mut __imp__stricmp: unsafe extern "C" fn(*const c_char, *const c_char) -> i32 = _stricmp;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _strnicmp(s1: *const c_char, s2: *const c_char, mut n: usize) -> i32 {
    if n == 0 { return 0; }
    let mut i = 0;
    loop {
        let c1 = (*s1.add(i) as u8).to_ascii_lowercase();
        let c2 = (*s2.add(i) as u8).to_ascii_lowercase();
        n -= 1;
        if n == 0 || c1 != c2 || c1 == 0 {
            return (c1 as i32) - (c2 as i32);
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub static mut __imp__strnicmp: unsafe extern "C" fn(*const c_char, *const c_char, usize) -> i32 = _strnicmp;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> i32 {
    if n == 0 { return 0; }
    for i in 0..n {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);
        if c1 != c2 || c1 == 0 {
            return (c1 as i32) - (c2 as i32);
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char {
    let mut i = 0;
    while i < n && *src.add(i) != 0 {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    while i < n {
        *dest.add(i) = 0;
        i += 1;
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char {
    if *needle == 0 { return haystack as *mut c_char; }
    let mut i = 0;
    while *haystack.add(i) != 0 {
        let mut j = 0;
        while *needle.add(j) != 0 && *haystack.add(i + j) == *needle.add(j) {
            j += 1;
        }
        if *needle.add(j) == 0 {
            return haystack.add(i) as *mut c_char;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strdup(s: *const c_char) -> *mut c_char {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    let ptr = malloc(len + 1) as *mut c_char;
    if !ptr.is_null() {
        core::ptr::copy_nonoverlapping(s, ptr, len + 1);
    }
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn toupper(c: i32) -> i32 {
    if c >= b'a' as i32 && c <= b'z' as i32 {
        c - (b'a' as i32 - b'A' as i32)
    } else {
        c
    }
}

#[unsafe(no_mangle)]
pub static mut __imp_toupper: unsafe extern "C" fn(i32) -> i32 = toupper;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strrchr(s: *const c_char, c: i32) -> *mut c_char {
    let mut last = core::ptr::null_mut();
    let mut i = 0;
    loop {
        let ch = *s.add(i);
        if ch as i32 == c {
            last = s.add(i) as *mut c_char;
        }
        if ch == 0 { break; }
        i += 1;
    }
    last
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn atoi(s: *const c_char) -> i32 {
    let mut res = 0;
    let mut i = 0;
    let mut sign = 1;
    if *s == b'-' as i8 {
        sign = -1;
        i += 1;
    }
    while *s.add(i) >= b'0' as i8 && *s.add(i) <= b'9' as i8 {
        res = res * 10 + (*s.add(i) - b'0' as i8) as i32;
        i += 1;
    }
    res * sign
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn atof(_s: *const c_char) -> f64 {
    0.0
}

// Minimal stdio stubs
#[unsafe(no_mangle)]
pub unsafe extern "C" fn puts(s: *const c_char) -> i32 {
    if !s.is_null() {
        if let Ok(st) = CStr::from_ptr(s).to_str() {
            message!("\n", "DOOM: {}", st);
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn putchar(c: i32) -> i32 {
    c
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn printf(format: *const c_char, ...) -> i32 {
    if !format.is_null() {
        if let Ok(st) = CStr::from_ptr(format).to_str() {
            message!("\n", "DOOM PRINTF: {}", st);
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fprintf(_stream: *mut c_void, format: *const c_char, ...) -> i32 {
    if !format.is_null() {
        if let Ok(st) = CStr::from_ptr(format).to_str() {
            message!("\n", "DOOM FPRINTF: {}", st);
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snprintf(s: *mut c_char, n: usize, _format: *const c_char, ...) -> i32 {
    if n > 0 {
        *s = 0;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _vsnprintf(s: *mut c_char, n: usize, _format: *const c_char, _argptr: *mut c_void) -> i32 {
    if n > 0 {
        *s = 0;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sscanf(_s: *const c_char, _format: *const c_char, ...) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __acrt_iob_func(_index: u32) -> *mut c_void {
    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub static mut __imp___acrt_iob_func: unsafe extern "C" fn(u32) -> *mut c_void = __acrt_iob_func;

// Filesystem Glue
#[repr(C)]
pub struct FILE {
    data: Vec<u8>,
    pos: usize,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fopen(filename: *const c_char, _mode: *const c_char) -> *mut FILE {
    let filename_str = CStr::from_ptr(filename).to_str().unwrap_or("");
    message!("\n", "DOOM: fopen({})", filename_str);
    
    // Normalize path for DOOM WAD
    let path = if filename_str.to_lowercase().contains("doom1.wad") {
        "/appfiles/doom/doom1.wad"
    } else {
        filename_str
    };

    if let Ok(data) = FileSystem::read_file(path) {
        message!("\n", "DOOM: Loaded {} ({} bytes)", path, data.len());
        let file = Box::new(FILE { data, pos: 0 });
        Box::into_raw(file)
    } else {
        message!("\n", "DOOM: Failed to load {}", path);
        core::ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fclose(stream: *mut FILE) -> i32 {
    if !stream.is_null() {
        let _ = Box::from_raw(stream);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize {
    if stream.is_null() || ptr.is_null() || size == 0 || nmemb == 0 { return 0; }
    let file = &mut *stream;
    let total_to_read = size * nmemb;
    let available = file.data.len().saturating_sub(file.pos);
    let to_read = if total_to_read < available { total_to_read } else { available };
    
    core::ptr::copy_nonoverlapping(file.data.as_ptr().add(file.pos), ptr as *mut u8, to_read);
    file.pos += to_read;
    to_read / size
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fseek(stream: *mut FILE, offset: i32, whence: i32) -> i32 {
    if stream.is_null() { return -1; }
    let file = &mut *stream;
    let new_pos = match whence {
        0 => offset as isize, // SEEK_SET
        1 => file.pos as isize + offset as isize, // SEEK_CUR
        2 => file.data.len() as isize + offset as isize, // SEEK_END
        _ => return -1,
    };
    
    if new_pos < 0 || new_pos > file.data.len() as isize {
        return -1;
    }
    file.pos = new_pos as usize;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ftell(stream: *mut FILE) -> i32 {
    if stream.is_null() { return -1; }
    (*stream).pos as i32
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn feof(stream: *mut FILE) -> i32 {
    if stream.is_null() { return 1; }
    if (*stream).pos >= (*stream).data.len() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn getenv(_name: *const c_char) -> *const c_char {
    core::ptr::null()
}

// Doom Engine Exports
// We don't define DG_ScreenBuffer here because it's defined in doomgeneric.c
// Instead, we link to it.
unsafe extern "C" {
    pub static mut DG_ScreenBuffer: *mut u32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fwrite(ptr: *const c_void, size: usize, nmemb: usize, _stream: *mut FILE) -> usize {
    size * nmemb
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn remove(_filename: *const c_char) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rename(_old: *const c_char, _new: *const c_char) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mkdir(_path: *const c_char, _mode: i32) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _errno() -> *mut i32 {
    static mut ERRNO: i32 = 0;
    &raw mut ERRNO
}

#[unsafe(no_mangle)]
pub static mut __imp__errno: unsafe extern "C" fn() -> *mut i32 = _errno;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MultiByteToWideChar(_CodePage: u32, _dwFlags: u32, _lpMultiByteStr: *const c_char, _cbMultiByte: i32, _lpWideCharStr: *mut u16, _cchWideChar: i32) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub static mut __imp_MultiByteToWideChar: unsafe extern "C" fn(u32, u32, *const c_char, i32, *mut u16, i32) -> i32 = MultiByteToWideChar;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WideCharToMultiByte(_CodePage: u32, _dwFlags: u32, _lpWideCharStr: *const u16, _cchWideChar: i32, _lpMultiByteStr: *mut c_char, _cbMultiByte: i32, _lpDefaultChar: *const c_char, _lpUsedDefaultChar: *mut i32) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub static mut __imp_WideCharToMultiByte: unsafe extern "C" fn(u32, u32, *const u16, i32, *mut c_char, i32, *const c_char, *mut i32) -> i32 = WideCharToMultiByte;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DG_SetWindowTitle(_title: *const c_char) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DG_Init() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DG_DrawFrame() {
    // This is called by doomgeneric when a frame is ready.
    // Our DoomApp::draw will read DG_ScreenBuffer directly.
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DG_SleepMs(_ms: u32) {
    // Simple busy wait or timer sleep if available
}

// I_Error is already defined in i_system.c
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn I_Error(format: *const c_char, ...) -> ! {
//     if !format.is_null() {
//         if let Ok(st) = CStr::from_ptr(format).to_str() {
//             message!("\n", "DOOM ERROR: {}", st);
//         }
//     }
//     loop {}
// }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DG_GetTicksMs() -> u32 {
    // Return system ticks if available
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn doomgeneric_key_pressed(key: u8) {
    unsafe {
        unsafe extern "C" {
            fn D_PostEvent(ev: *const c_void);
        }
        #[repr(C)]
        struct Event {
            ev_type: i32,
            data1: i32,
            data2: i32,
            data3: i32,
            data4: i32,
        }
        let ev = Event {
            ev_type: 0, // ev_keydown
            data1: key as i32,
            data2: 0,
            data3: 0,
            data4: 0,
        };
        D_PostEvent(&ev as *const _ as *const c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn doomgeneric_key_released(key: u8) {
    unsafe {
        unsafe extern "C" {
            fn D_PostEvent(ev: *const c_void);
        }
        #[repr(C)]
        struct Event {
            ev_type: i32,
            data1: i32,
            data2: i32,
            data3: i32,
            data4: i32,
        }
        let ev = Event {
            ev_type: 1, // ev_keyup
            data1: key as i32,
            data2: 0,
            data3: 0,
            data4: 0,
        };
        D_PostEvent(&ev as *const _ as *const c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn doomgeneric_tick() {
    unsafe {
        unsafe extern "C" {
             fn doomgeneric_Tick();
        }
        doomgeneric_Tick();
    }
}

// Doom Platform Stubs
#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_InitJoystick() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_InitSound(_use_sfx_prefix: i32) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_InitMusic() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_BindJoystickVariables() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_BindSoundVariables() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_Endoom(_data: *const c_void) {}

// Sound Stubs
#[unsafe(no_mangle)]
pub static mut snd_musicdevice: i32 = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_PrecacheSounds() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_SetMusicVolume(_volume: i32) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_ShutdownSound() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_ShutdownMusic() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_SoundIsPlaying(_handle: i32) -> i32 { 0 }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_StopSound(_handle: i32) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_ResumeSong(_handle: i32) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_StopSong(_handle: i32) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_UnRegisterSong(_handle: i32) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_RegisterSong(_data: *const c_void, _len: i32) -> i32 { 0 }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_PlaySong(_handle: i32, _looping: i32) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_GetSfxLumpNum(_sfx: *const c_void) -> i32 { 0 }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_StartSound(_id: i32, _vol: i32, _sep: i32, _pitch: i32, _priority: i32) -> i32 { 0 }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_PauseSong(_handle: i32) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_UpdateSound() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_UpdateSoundParams(_handle: i32, _vol: i32, _sep: i32, _pitch: i32) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn I_MusicIsPlaying() -> i32 { 0 }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vfprintf(_stream: *mut c_void, format: *const c_char, _argptr: *mut c_void) -> i32 {
    if !format.is_null() {
        if let Ok(st) = CStr::from_ptr(format).to_str() {
            message!("\n", "DOOM VFPRINTF: {}", st);
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fflush(_stream: *mut c_void) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MessageBoxW(_hwnd: *mut c_void, _text: *const u16, _caption: *const u16, _type: u32) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub static mut __imp_MessageBoxW: unsafe extern "C" fn(*mut c_void, *const u16, *const u16, u32) -> i32 = MessageBoxW;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DG_GetKey(_pressed: *mut i32, _key: *mut u8) -> i32 {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn doom_main() {
    unsafe extern "C" {
        fn doomgeneric_Create(argc: i32, argv: *mut *mut c_char);
    }
    let mut args = [core::ptr::null_mut()];
    doomgeneric_Create(0, args.as_mut_ptr());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exit(_status: i32) -> ! {
    loop {}
}