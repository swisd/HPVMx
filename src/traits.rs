use alloc::{string::String, vec::Vec};

// Platform services required by x4. Implement these at the OS or kernel boundary.
// This module uses only core/alloc-compatible types and remains usable in no_std builds.
pub trait X4Platform {
    type Heap: X4Heap;
    type Synchronization: X4Synchronization;
    type Clock: X4Clock;
    type Filesystem: Filesystem;
    type Registry: HPVMRegistry;
    type Hardware: HardwareAccess;
    type Devices: DeviceAccess;
    type Gui: GuiAccess;
}

pub trait X4Heap {
    /// Allocate bytes with alignment. The default is a deliberate placeholder
    /// that reports allocation failure until the host supplies a real allocator.
    unsafe fn allocate(&self, _size: usize, _alignment: usize) -> *mut u8 {
        core::ptr::null_mut()
    }
    /// Release a pointer returned by `allocate`.
    unsafe fn deallocate(&self, _ptr: *mut u8, _size: usize, _alignment: usize) {}
    /// Change memory permissions. The default reports unsupported.
    unsafe fn set_protection(&self, _ptr: *mut u8, _size: usize, _flags: u32) -> bool {
        false
    }
}

pub trait X4Synchronization {
    type Lock;
    fn lock_exclusive(&self, lock: &Self::Lock);
    fn unlock_exclusive(&self, lock: &Self::Lock);
    fn current_thread_id(&self) -> u32;
}

pub trait X4Clock {
    fn ticks_ms(&self) -> u64;
    fn sleep_ms(&self, milliseconds: u32);
}

// src/traits.rs

// --- File System Abstraction ---
// This trait defines safe operations over file and directory management,
// abstracting away the raw WinAPI calls found in externals.rs.
pub trait Filesystem {
    /// Reads content from a file path.
    fn read_file(&self, path: &str) -> Result<Vec<u8>, String>;

    /// Writes content to a file path.
    fn write_file(&self, path: &str, data: &[u8]) -> Result<(), String>;

    /// Creates a new directory structure or file.
    fn create_path(&self, path: &str) -> Result<(), String>;

    // Add more methods as needed (e.g., read_dir, create_file, etc.)
}

// --- Registry/Database Abstraction ---
// This trait abstracts interaction with the Windows Registry hives or object databases.
pub trait RegistryAccess {
    /// Reads a value from a specified registry key/database path.
    fn read_value(&self, key_path: &str, value_name: &str) -> Result<String, String>;

    /// Writes a value to a specified registry key/database path.
    fn write_value(&self, key_path: &str, value_name: &str, value: &str) -> Result<(), String>;

    /// Creates or initializes a necessary database/key structure.
    fn initialize_structure(&self, path: &str) -> Result<(), String>;
}

// --- Virtual Machine Manager Abstraction ---
// This trait encapsulates operations related to VM state and memory.
pub trait VMManager {
    /// Reads the current state of the virtual machine.
    fn get_state(&self) -> Result<String, String>;

    /// Saves the current VM state to persistent storage.
    fn save_state(&self, path: &str) -> Result<(), String>;

    /// Allocates or frees memory for the VM context.
    fn manage_memory(&self, size: usize, operation: MemoryOperation) -> Result<usize, String>;
}

// --- Supporting Enums (Example) ---
#[derive(Debug)]
pub enum MemoryOperation {
    Allocate,
    Free,
    SetProtection,
}

/// Adapter for HPVMx's UEFI-backed filesystem implementation.
#[derive(Clone, Copy, Default)]
pub struct KernelFilesystem;

impl Filesystem for KernelFilesystem {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, String> {
        crate::filesystem::FileSystem::read_file(path).map_err(String::from)
    }
    fn write_file(&self, path: &str, data: &[u8]) -> Result<(), String> {
        crate::filesystem::FileSystem::write_to_file_bytes(path, data, 'w').map_err(String::from)
    }
    fn create_path(&self, path: &str) -> Result<(), String> {
        crate::filesystem::FileSystem::mkdir(path).map_err(String::from)
    }
}

/// Project registry operations backed by HPVMx's serialized registry files.
pub trait HPVMRegistry {
    fn load_system(&self, settings: &mut crate::ui::UiSettings) -> Result<(), &'static str>;
    fn save_system(&self, settings: &crate::ui::UiSettings) -> Result<(), &'static str>;
    fn load_devices(&self) -> Result<(), &'static str>;
    fn save_devices(&self) -> Result<(), &'static str>;
}

#[derive(Clone, Copy, Default)]
pub struct KernelRegistry;
impl HPVMRegistry for KernelRegistry {
    fn load_system(&self, settings: &mut crate::ui::UiSettings) -> Result<(), &'static str> {
        crate::registry::load_system_registry(crate::registry::DEFAULT_SYSTEM_REG_PATH, settings)
    }
    fn save_system(&self, settings: &crate::ui::UiSettings) -> Result<(), &'static str> {
        crate::registry::save_system_registry(crate::registry::DEFAULT_SYSTEM_REG_PATH, settings)
    }
    fn load_devices(&self) -> Result<(), &'static str> {
        crate::registry::load_device_registry(crate::registry::DEFAULT_DEVICE_REG_PATH)
    }
    fn save_devices(&self) -> Result<(), &'static str> {
        crate::registry::save_device_registry(crate::registry::DEFAULT_DEVICE_REG_PATH)
    }
}

/// CPU and PCI services exposed by the hardware layer.
pub trait HardwareAccess {
    fn cpu_info(&self) -> crate::hardware::cpu::CpuInfo;
    fn pci_devices(&self) -> Vec<crate::hardware::pci::PciDeviceInfo>;
    fn read_pci_u32(&self, bus: u8, slot: u8, function: u8, offset: u8) -> u32;
    fn write_pci_u32(&self, bus: u8, slot: u8, function: u8, offset: u8, value: u32);
}

#[derive(Clone, Copy, Default)]
pub struct KernelHardware;
impl HardwareAccess for KernelHardware {
    fn cpu_info(&self) -> crate::hardware::cpu::CpuInfo { crate::hardware::cpu::CpuInfo::get() }
    fn pci_devices(&self) -> Vec<crate::hardware::pci::PciDeviceInfo> { crate::hardware::pci::scan_bus() }
    fn read_pci_u32(&self, bus: u8, slot: u8, function: u8, offset: u8) -> u32 {
        crate::hardware::pci::pci_config_read_u32(bus, slot, function, offset)
    }
    fn write_pci_u32(&self, bus: u8, slot: u8, function: u8, offset: u8, value: u32) {
        crate::hardware::pci::pci_config_write_u32(bus, slot, function, offset, value)
    }
}

/// Device operations backed by the current network, audio, and timer drivers.
pub trait DeviceAccess {
    fn initialize_network(&self) -> Result<(), &'static str>;
    fn transmit_frame(&self, frame: &[u8]) -> Result<(), &'static str>;
    fn receive_frame(&self, buffer: &mut [u8]) -> Result<usize, &'static str>;
    fn network_link_up(&self) -> bool;
    fn beep(&self, frequency_hz: u32);
    fn sleep_ms(&self, milliseconds: u64);
}

#[derive(Clone, Copy, Default)]
pub struct KernelDevices;
impl DeviceAccess for KernelDevices {
    fn initialize_network(&self) -> Result<(), &'static str> { crate::devices::net_hw::init() }
    fn transmit_frame(&self, frame: &[u8]) -> Result<(), &'static str> { crate::devices::net_hw::tx(frame) }
    fn receive_frame(&self, buffer: &mut [u8]) -> Result<usize, &'static str> { crate::devices::net_hw::rx(buffer) }
    fn network_link_up(&self) -> bool { crate::devices::net_hw::link_up() }
    fn beep(&self, frequency_hz: u32) { crate::devices::audio::beep(frequency_hz) }
    fn sleep_ms(&self, milliseconds: u64) { crate::devices::timer::sleep_ms(milliseconds) }
}

/// Framebuffer GUI operations backed by the current PixelGraphics implementation.
pub trait GuiAccess {
    fn resolution(&self) -> Option<(usize, usize)>;
    fn clear(&self, color: u32);
    fn draw_text(&self, x: usize, y: usize, text: &str, foreground: u32, background: u32);
    fn draw_button(&self, x: usize, y: usize, width: usize, height: usize, label: &str, focused: bool);
    fn fill_rect(&self, x: usize, y: usize, width: usize, height: usize, color: u32);
}

#[derive(Clone, Copy, Default)]
pub struct KernelGui;
impl GuiAccess for KernelGui {
    fn resolution(&self) -> Option<(usize, usize)> {
        crate::ui::pixel_graphics::PixelGraphics::new().map(|graphics| graphics.resolution())
    }
    fn clear(&self, color: u32) {
        if let Some(mut graphics) = crate::ui::pixel_graphics::PixelGraphics::new() {
            graphics.clear(color);
            graphics.flip();
        }
    }
    fn draw_text(&self, x: usize, y: usize, text: &str, foreground: u32, background: u32) {
        if let Some(mut graphics) = crate::ui::pixel_graphics::PixelGraphics::new() {
            graphics.draw_text_bg(x, y, text, foreground, background);
            graphics.flip();
        }
    }
    fn draw_button(&self, x: usize, y: usize, width: usize, height: usize, label: &str, focused: bool) {
        if let Some(mut graphics) = crate::ui::pixel_graphics::PixelGraphics::new() {
            graphics.draw_button(x, y, width, height, label, focused);
            graphics.flip();
        }
    }
    fn fill_rect(&self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        if let Some(mut graphics) = crate::ui::pixel_graphics::PixelGraphics::new() {
            graphics.fill_rect(x, y, width, height, color);
            graphics.flip();
        }
    }
}

/// The concrete set of services provided by this UEFI kernel.
#[derive(Clone, Copy, Default)]
pub struct KernelPlatform;

impl KernelPlatform {
    pub const fn filesystem(&self) -> KernelFilesystem { KernelFilesystem }
    pub const fn registry(&self) -> KernelRegistry { KernelRegistry }
    pub const fn hardware(&self) -> KernelHardware { KernelHardware }
    pub const fn devices(&self) -> KernelDevices { KernelDevices }
    pub const fn gui(&self) -> KernelGui { KernelGui }
    pub const fn clock(&self) -> KernelClock { KernelClock }
    pub const fn heap(&self) -> KernelHeapPlaceholder { KernelHeapPlaceholder }
    pub const fn synchronization(&self) -> KernelSynchronizationPlaceholder { KernelSynchronizationPlaceholder }
}

impl X4Platform for KernelPlatform {
    type Heap = KernelHeapPlaceholder;
    type Synchronization = KernelSynchronizationPlaceholder;
    type Clock = KernelClock;
    type Filesystem = KernelFilesystem;
    type Registry = KernelRegistry;
    type Hardware = KernelHardware;
    type Devices = KernelDevices;
    type Gui = KernelGui;
}

/// Placeholder until the kernel allocator is exposed through a stable interface.
#[derive(Clone, Copy, Default)]
pub struct KernelHeapPlaceholder;
impl X4Heap for KernelHeapPlaceholder {}

/// Placeholder for kernel lock primitives that are not yet exposed as a shared API.
#[derive(Clone, Copy, Default)]
pub struct KernelSynchronizationPlaceholder;
impl X4Synchronization for KernelSynchronizationPlaceholder {
    type Lock = ();
    fn lock_exclusive(&self, _lock: &Self::Lock) {}
    fn unlock_exclusive(&self, _lock: &Self::Lock) {}
    fn current_thread_id(&self) -> u32 { 0 }
}

/// Clock adapter for UEFI's boot-service delay. Tick precision is milliseconds.
#[derive(Clone, Copy, Default)]
pub struct KernelClock;
impl X4Clock for KernelClock {
    fn ticks_ms(&self) -> u64 {
        #[cfg(target_arch = "x86_64")]
        {
            let cycles_per_us = unsafe { crate::TSC_PER_US }.max(1);
            let cycles = unsafe { core::arch::x86_64::_rdtsc() };
            cycles / cycles_per_us.saturating_mul(1000)
        }
        #[cfg(not(target_arch = "x86_64"))]
        { 0 }
    }
    fn sleep_ms(&self, milliseconds: u32) { crate::devices::timer::sleep_ms(milliseconds as u64) }
}
