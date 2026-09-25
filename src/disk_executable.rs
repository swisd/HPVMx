//! Loader and ABI for position-independent `.hpx` executables stored on disk.
//!
//! The file is a 40-byte little-endian header followed by an x86-64 PIC image.
//! The image has no imports or relocations: it calls the kernel only through
//! `PluginHostApi`. Function offsets in the header select stepped app/task callbacks.
use alloc::{boxed::Box, string::String, sync::Arc, vec, vec::Vec};
use core::{alloc::Layout, ffi::c_void, mem::transmute, ptr::NonNull};
use crate::{env::{Application, Background, BackgroundSteppedApplicationContext, BackgroundTask, GlobalEnvironmentData, Runnable, SpinLock, SteppedApplicationContext}, filesystem::FileSystem};
use crate::ui::pixel_graphics::PixelGraphics;

pub const HPX_MAGIC: [u8; 4] = *b"HPX1";
pub const HPX_ABI_VERSION: u16 = 1;
pub const HPX_HEADER_SIZE: usize = 40;
pub const KIND_STEPPED_APP: u16 = 1;
pub const KIND_STEPPED_BACKGROUND: u16 = 2;

/// Stable C ABI passed to disk executables. All host calls are optional pointers
/// only in future ABI versions; v1 executables may rely on every entry here.
#[repr(C)]
pub struct PluginHostApi {
    pub abi_version: u32,
    pub draw_text: unsafe extern "C" fn(usize, usize, *const u8, usize, u32),
    pub fill_rect: unsafe extern "C" fn(usize, usize, usize, usize, u32),
    pub read_file: unsafe extern "C" fn(*const u8, usize, *mut u8, usize) -> i64,
    pub write_file: unsafe extern "C" fn(*const u8, usize, *const u8, usize) -> i32,
    pub allocate: unsafe extern "C" fn(usize, usize) -> *mut c_void,
    pub deallocate: unsafe extern "C" fn(*mut c_void, usize, usize),
}

static HOST_API: PluginHostApi = PluginHostApi {
    abi_version: HPX_ABI_VERSION as u32,
    draw_text: host_draw_text,
    fill_rect: host_fill_rect,
    read_file: host_read_file,
    write_file: host_write_file,
    allocate: host_allocate,
    deallocate: host_deallocate,
};

unsafe extern "C" fn host_draw_text(x: usize, y: usize, bytes: *const u8, len: usize, color: u32) {
    if bytes.is_null() || len > 1_048_576 { return; }
    let Ok(text) = core::str::from_utf8(unsafe { core::slice::from_raw_parts(bytes, len) }) else { return; };
    if let Some(mut pg) = PixelGraphics::new() { pg.draw_text(x, y, text, color); pg.flip(); }
}
unsafe extern "C" fn host_fill_rect(x: usize, y: usize, w: usize, h: usize, color: u32) {
    if let Some(mut pg) = PixelGraphics::new() { pg.fill_rect(x, y, w, h, color); pg.flip(); }
}
unsafe extern "C" fn host_read_file(path: *const u8, path_len: usize, out: *mut u8, capacity: usize) -> i64 {
    if path.is_null() || path_len == 0 || path_len > 4096 { return -1; }
    let Ok(path) = core::str::from_utf8(unsafe { core::slice::from_raw_parts(path, path_len) }) else { return -1; };
    let Ok(data) = FileSystem::read_file(path) else { return -1; };
    if out.is_null() || capacity == 0 { return data.len().min(i64::MAX as usize) as i64; }
    if data.len() > capacity { return -2; }
    unsafe { core::ptr::copy_nonoverlapping(data.as_ptr(), out, data.len()); }
    data.len() as i64
}
unsafe extern "C" fn host_write_file(path: *const u8, path_len: usize, data: *const u8, len: usize) -> i32 {
    if path.is_null() || path_len == 0 || path_len > 4096 || (data.is_null() && len != 0) { return -1; }
    let Ok(path) = core::str::from_utf8(unsafe { core::slice::from_raw_parts(path, path_len) }) else { return -1; };
    let bytes = if len == 0 { &[] } else { unsafe { core::slice::from_raw_parts(data, len) } };
    FileSystem::write_to_file_bytes(path, bytes, 'w').map(|_| 0).unwrap_or(-1)
}
unsafe extern "C" fn host_allocate(size: usize, alignment: usize) -> *mut c_void {
    let Some(alignment) = alignment.max(core::mem::align_of::<usize>()).checked_next_power_of_two() else { return core::ptr::null_mut(); };
    let Ok(layout) = Layout::from_size_align(size.max(1), alignment) else { return core::ptr::null_mut(); };
    unsafe { alloc::alloc::alloc(layout).cast() }
}
unsafe extern "C" fn host_deallocate(ptr: *mut c_void, size: usize, alignment: usize) {
    if ptr.is_null() { return; }
    if let Some(alignment) = alignment.max(core::mem::align_of::<usize>()).checked_next_power_of_two() {
      if let Ok(layout) = Layout::from_size_align(size.max(1), alignment) {
        unsafe { alloc::alloc::dealloc(ptr.cast(), layout); }
      }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HpxHeader {
    pub magic: [u8; 4],
    pub abi_version: u16,
    pub kind: u16,
    pub header_size: u32,
    pub image_size: u32,
    pub state_size: u32,
    pub step_offset: u32,
    pub draw_offset: u32,
    pub input_offset: u32,
    pub width: u32,
    pub height: u32,
}

impl HpxHeader {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < HPX_HEADER_SIZE { return Err("HPX file is shorter than its header"); }
        if bytes[..4] != HPX_MAGIC { return Err("invalid HPX magic"); }
        if read_u16(bytes, 4)? != HPX_ABI_VERSION { return Err("unsupported HPX ABI version"); }
        let header = Self {
            magic: HPX_MAGIC,
            abi_version: HPX_ABI_VERSION,
            kind: read_u16(bytes, 6)?,
            header_size: read_u32(bytes, 8)?,
            image_size: read_u32(bytes, 12)?,
            state_size: read_u32(bytes, 16)?,
            step_offset: read_u32(bytes, 20)?,
            draw_offset: read_u32(bytes, 24)?,
            input_offset: read_u32(bytes, 28)?,
            width: read_u32(bytes, 32)?,
            height: read_u32(bytes, 36)?,
        };
        if !matches!(header.kind, KIND_STEPPED_APP | KIND_STEPPED_BACKGROUND) { return Err("unknown HPX executable kind"); }
        if header.header_size as usize != HPX_HEADER_SIZE { return Err("unsupported HPX header size"); }
        if header.image_size == 0 || header.image_size as usize > bytes.len() - HPX_HEADER_SIZE { return Err("invalid HPX image size"); }
        if header.state_size as usize > 16 * 1024 * 1024 { return Err("HPX state size exceeds limit"); }
        if !valid_offset(header.step_offset, header.image_size) { return Err("invalid HPX step offset"); }
        if header.draw_offset != 0 && !valid_offset(header.draw_offset, header.image_size) { return Err("invalid HPX draw offset"); }
        if header.input_offset != 0 && !valid_offset(header.input_offset, header.image_size) { return Err("invalid HPX input offset"); }
        if header.kind == KIND_STEPPED_APP && (header.width == 0 || header.height == 0) { return Err("HPX app dimensions must be nonzero"); }
        if header.image_size as usize > 64 * 1024 * 1024 { return Err("HPX image exceeds 64 MiB limit"); }
        Ok(header)
    }
}

fn valid_offset(offset: u32, image_size: u32) -> bool { offset < image_size }
fn read_u16(bytes: &[u8], at: usize) -> Result<u16, &'static str> { Ok(u16::from_le_bytes(bytes.get(at..at+2).ok_or("truncated HPX header")?.try_into().unwrap())) }
fn read_u32(bytes: &[u8], at: usize) -> Result<u32, &'static str> { Ok(u32::from_le_bytes(bytes.get(at..at+4).ok_or("truncated HPX header")?.try_into().unwrap())) }

struct PluginState {
    image: NonNull<u8>,
    header: HpxHeader,
    state: Box<[u64]>,
    running: bool,
}

impl PluginState {
    fn call_step(&mut self) -> bool {
        let function: unsafe extern "C" fn(*const PluginHostApi, *mut u8) -> u32 = unsafe { transmute(self.image.as_ptr().add(self.header.step_offset as usize)) };
        self.running = unsafe { function(&HOST_API, self.state.as_mut_ptr().cast::<u8>()) != 0 };
        self.running
    }
    fn call_draw(&mut self, x: usize, y: usize) {
        if self.header.draw_offset == 0 { return; }
        let function: unsafe extern "C" fn(*const PluginHostApi, *mut u8, usize, usize) = unsafe { transmute(self.image.as_ptr().add(self.header.draw_offset as usize)) };
        unsafe { function(&HOST_API, self.state.as_mut_ptr().cast::<u8>(), x, y); }
    }
    fn call_input(&mut self, key: u32) {
        if self.header.input_offset == 0 { return; }
        let function: unsafe extern "C" fn(*const PluginHostApi, *mut u8, u32) = unsafe { transmute(self.image.as_ptr().add(self.header.input_offset as usize)) };
        unsafe { function(&HOST_API, self.state.as_mut_ptr().cast::<u8>(), key); }
    }
}

#[derive(Clone)]
struct PluginApplication(Arc<SpinLock<PluginState>>);
impl Runnable for PluginApplication {
    fn draw(&self, _graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) { self.0.lock().call_draw(x, y); }
    fn logic(&mut self, _vars: &mut Vec<String>, _env: &mut crate::env::Environment) { self.0.lock().call_step(); }
    fn input(&mut self, key: crate::env::Key) {
        let packed = match key {
            crate::env::Key::Printable(ch) => u16::from(ch) as u32,
            crate::env::Key::Special(scan) => 0x1_0000 | scan.0 as u32,
        };
        self.0.lock().call_input(packed);
    }
    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    fn is_done(&self) -> bool { !self.0.lock().running }
}

#[derive(Clone)]
struct PluginBackground(Arc<SpinLock<PluginState>>);
impl BackgroundTask for PluginBackground {
    fn tick(&mut self, _vars: &mut Vec<String>, _env: &mut crate::env::Environment) -> bool { !self.0.lock().call_step() }
}

/// Loads `.hpx` images from the UEFI filesystem and retains executable pages for
/// the lifetime of their stepped contexts. There is deliberately no unload yet.
pub struct DiskExecutableManager {
    image_pages: Vec<(NonNull<u8>, usize)>,
    background_tasks: Vec<BackgroundSteppedApplicationContext>,
}

impl Default for DiskExecutableManager { fn default() -> Self { Self::new() } }
impl DiskExecutableManager {
    pub const fn new() -> Self { Self { image_pages: Vec::new(), background_tasks: Vec::new() } }

    /// Load a `.hpx` file, inspect its header kind, and start it as an app or task.
    pub fn run_file(&mut self, path: &str, name: &str, version: &str, runtime: &mut GlobalEnvironmentData) -> Result<usize, &'static str> {
        let bytes = FileSystem::read_file(path)?;
        let header = HpxHeader::parse(&bytes)?;
        let image_start = header.header_size as usize;
        let image_end = image_start.checked_add(header.image_size as usize).ok_or("HPX image size overflow")?;
        let image_bytes = bytes.get(image_start..image_end).ok_or("truncated HPX image")?;
        let pages = (image_bytes.len() + 4095) / 4096;
        let allocation = uefi::boot::allocate_pages(uefi::boot::AllocateType::AnyPages, uefi::boot::MemoryType::LOADER_CODE, pages).map_err(|_| "could not allocate executable pages")?;
        let image_ptr = NonNull::new(allocation.as_ptr().cast::<u8>()).ok_or("UEFI returned a null executable image")?;
        unsafe { core::ptr::copy_nonoverlapping(image_bytes.as_ptr(), image_ptr.as_ptr(), image_bytes.len()); }
        let state_words = (header.state_size as usize).saturating_add(7) / 8;
        let state = vec![0u64; state_words].into_boxed_slice();
        let plugin = Arc::new(SpinLock::new(PluginState { image: image_ptr, header, state, running: true }));
        self.image_pages.push((image_ptr, pages));

        match header.kind {
            KIND_STEPPED_APP => {
                let mut app = Application::new(Box::new(PluginApplication(plugin)));
                app.name = String::from(name);
                app.version = String::from(version);
                app.dimensions = (header.width as usize, header.height as usize);
                let context = SteppedApplicationContext::new(app, None);
                let pid = context.pid;
                runtime.active_apps.push(context);
                Ok(pid)
            }
            KIND_STEPPED_BACKGROUND => {
                let mut background = Background::new(Box::new(PluginBackground(plugin)));
                background.name = String::from(name);
                background.version = String::from(version);
                let context = BackgroundSteppedApplicationContext::new(background);
                let pid = context.pid;
                self.background_tasks.push(context);
                Ok(pid)
            }
            _ => Err("unknown HPX kind"),
        }
    }

    /// Advance every loaded background executable once; finished tasks are reaped.
    pub fn step_background_tasks(&mut self) -> usize {
        let before = self.background_tasks.len();
        self.background_tasks.retain_mut(|task| task.step());
        before - self.background_tasks.len()
    }

    pub fn background_task_count(&self) -> usize { self.background_tasks.len() }
    pub fn loaded_image_count(&self) -> usize { self.image_pages.len() }
}
