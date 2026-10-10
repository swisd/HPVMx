//! In-memory virtual serial, block, and framebuffer devices.

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;

const UART_DATA: u16 = 0x3f8;
const UART_INTERRUPT_ENABLE: u16 = 0x3f9;
const UART_LINE_CONTROL: u16 = 0x3fb;
const UART_LINE_STATUS: u16 = 0x3fd;
const UART_RX_READY: u8 = 1;
const UART_TX_EMPTY: u8 = 1 << 5;
const SECTOR_SIZE: usize = 512;

pub trait VirtioDevice {
    fn device_name(&self) -> &str;
    fn handle_io_read(&mut self, port: u16, size: u32) -> Result<u32, &'static str>;
    fn handle_io_write(&mut self, port: u16, size: u32, data: u32) -> Result<(), &'static str>;
}

/// UART-compatible serial endpoint. Guest output and host-injected input are
/// buffered independently so a terminal window can drain output at its pace.
pub struct VirtualConsole {
    pub name: String,
    input: VecDeque<u8>,
    output: VecDeque<u8>,
    output_limit: usize,
    interrupt_enable: u8,
    line_control: u8,
}

impl VirtualConsole {
    pub fn new() -> Self {
        Self::with_capacity(16 * 1024)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            name: String::from("Virtual Serial Console"),
            input: VecDeque::new(),
            output: VecDeque::new(),
            output_limit: capacity.max(1),
            interrupt_enable: 0,
            line_control: 0,
        }
    }

    pub fn push_input(&mut self, bytes: &[u8]) {
        self.input.extend(bytes.iter().copied());
    }

    pub fn drain_output(&mut self, max_len: usize) -> Vec<u8> {
        let count = min(max_len, self.output.len());
        self.output.drain(..count).collect()
    }

    pub fn pending_output(&self) -> usize {
        self.output.len()
    }

    fn read_port(&mut self, port: u16) -> u8 {
        match port {
            UART_DATA => self.input.pop_front().unwrap_or(0),
            UART_INTERRUPT_ENABLE => self.interrupt_enable,
            UART_LINE_CONTROL => self.line_control,
            UART_LINE_STATUS => {
                let receive_ready = if self.input.is_empty() {
                    0
                } else {
                    UART_RX_READY
                };
                receive_ready | UART_TX_EMPTY
            }
            _ => 0,
        }
    }

    fn write_port(&mut self, port: u16, value: u8) -> Result<(), &'static str> {
        match port {
            UART_DATA => {
                if self.output.len() == self.output_limit {
                    return Err("virtual serial output buffer is full");
                }
                self.output.push_back(value);
            }
            UART_INTERRUPT_ENABLE => self.interrupt_enable = value,
            UART_LINE_CONTROL => self.line_control = value,
            _ => {}
        }
        Ok(())
    }
}

impl Default for VirtualConsole {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtioDevice for VirtualConsole {
    fn device_name(&self) -> &str {
        &self.name
    }

    fn handle_io_read(&mut self, port: u16, size: u32) -> Result<u32, &'static str> {
        if size != 1 {
            return Err("UART supports byte-sized port reads only");
        }
        Ok(self.read_port(port) as u32)
    }

    fn handle_io_write(&mut self, port: u16, size: u32, data: u32) -> Result<(), &'static str> {
        if size != 1 {
            return Err("UART supports byte-sized port writes only");
        }
        self.write_port(port, data as u8)
    }
}

/// Mutable in-memory disk used by the VM storage backend.
pub struct VirtualBlockDevice {
    bytes: Vec<u8>,
}

impl VirtualBlockDevice {
    pub fn new(size_bytes: usize) -> Result<Self, &'static str> {
        if size_bytes == 0 || size_bytes % SECTOR_SIZE != 0 {
            return Err("virtual disk size must be a non-zero multiple of 512 bytes");
        }
        Ok(Self {
            bytes: vec![0; size_bytes],
        })
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, &'static str> {
        if bytes.is_empty() || bytes.len() % SECTOR_SIZE != 0 {
            return Err("virtual disk image must be a non-empty multiple of 512 bytes");
        }
        Ok(Self { bytes })
    }

    pub fn capacity_bytes(&self) -> usize {
        self.bytes.len()
    }

    pub fn read_sectors(
        &self,
        lba: u64,
        count: u32,
        output: &mut [u8],
    ) -> Result<(), &'static str> {
        let (start, end) = self.sector_range(lba, count)?;
        if output.len() != end - start {
            return Err("virtual disk read buffer length does not match sector count");
        }
        output.copy_from_slice(&self.bytes[start..end]);
        Ok(())
    }

    pub fn write_sectors(
        &mut self,
        lba: u64,
        count: u32,
        input: &[u8],
    ) -> Result<(), &'static str> {
        let (start, end) = self.sector_range(lba, count)?;
        if input.len() != end - start {
            return Err("virtual disk write buffer length does not match sector count");
        }
        self.bytes[start..end].copy_from_slice(input);
        Ok(())
    }

    fn sector_range(&self, lba: u64, count: u32) -> Result<(usize, usize), &'static str> {
        if count == 0 {
            return Err("virtual disk request must contain at least one sector");
        }
        let start = usize::try_from(lba)
            .ok()
            .and_then(|sector| sector.checked_mul(SECTOR_SIZE))
            .ok_or("virtual disk sector offset overflows")?;
        let length = (count as usize)
            .checked_mul(SECTOR_SIZE)
            .ok_or("virtual disk request length overflows")?;
        let end = start
            .checked_add(length)
            .ok_or("virtual disk range overflows")?;
        if end > self.bytes.len() {
            return Err("virtual disk request exceeds capacity");
        }
        Ok((start, end))
    }
}

/// Guest-visible pixel buffer. It never aliases the host GOP framebuffer.
pub struct VirtualDisplay {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
    selected_register: u8,
    registers: [u8; 8],
}

impl VirtualDisplay {
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        if width == 0 || height == 0 {
            return Err("virtual display dimensions must be non-zero");
        }
        let pixel_count = width
            .checked_mul(height)
            .ok_or("virtual display size overflows")?;
        Ok(Self {
            width,
            height,
            pixels: vec![0; pixel_count],
            selected_register: 0,
            registers: [0; 8],
        })
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, rgb: u32) -> Result<(), &'static str> {
        if x >= self.width || y >= self.height {
            return Err("virtual display pixel is outside the framebuffer");
        }
        self.pixels[y * self.width + x] = rgb & 0x00ff_ffff;
        Ok(())
    }

    pub fn write_pixels(&mut self, offset: usize, pixels: &[u32]) -> Result<(), &'static str> {
        let end = offset
            .checked_add(pixels.len())
            .ok_or("virtual display write overflows")?;
        if end > self.pixels.len() {
            return Err("virtual display write exceeds framebuffer");
        }
        for (dest, source) in self.pixels[offset..end].iter_mut().zip(pixels) {
            *dest = *source & 0x00ff_ffff;
        }
        Ok(())
    }

    fn read_port(&self, port: u16) -> Result<u8, &'static str> {
        match port {
            0x3d0 => Ok(self.selected_register),
            0x3d1 => Ok(self.registers[self.selected_register as usize]),
            _ => Err("unsupported virtual display register"),
        }
    }

    fn write_port(&mut self, port: u16, value: u8) -> Result<(), &'static str> {
        match port {
            0x3d0 => {
                if value as usize >= self.registers.len() {
                    return Err("virtual display register index is invalid");
                }
                self.selected_register = value;
            }
            0x3d1 => {
                self.registers[self.selected_register as usize] = value;
                if self.selected_register == 7 && value & 1 != 0 {
                    let x = u16::from_le_bytes([self.registers[0], self.registers[1]]) as usize;
                    let y = u16::from_le_bytes([self.registers[2], self.registers[3]]) as usize;
                    let rgb = self.registers[4] as u32
                        | (self.registers[5] as u32) << 8
                        | (self.registers[6] as u32) << 16;
                    self.write_pixel(x, y, rgb)?;
                    self.registers[7] = 0;
                }
            }
            _ => return Err("unsupported virtual display register"),
        }
        Ok(())
    }
}

/// Minimal ATA PIO task-file controller backed by an isolated virtual disk.
pub struct VirtualAtaPio {
    sector_count: u8,
    lba: u32,
    status: u8,
    transfer: Vec<u8>,
    transfer_offset: usize,
    writing: bool,
}

impl VirtualAtaPio {
    pub fn new() -> Self {
        Self {
            sector_count: 0,
            lba: 0,
            status: 0x40,
            transfer: Vec::new(),
            transfer_offset: 0,
            writing: false,
        }
    }

    fn read_port(
        &mut self,
        disk: &VirtualBlockDevice,
        port: u16,
        width: usize,
    ) -> Result<u32, &'static str> {
        if port == 0x1f0 {
            if width != 2 && width != 4 {
                return Err("ATA data port requires 16-bit or 32-bit access");
            }
            let mut value = 0u32;
            for byte_offset in 0..width {
                if self.transfer_offset < self.transfer.len() {
                    value |= (self.transfer[self.transfer_offset] as u32) << (byte_offset * 8);
                    self.transfer_offset += 1;
                }
            }
            if self.transfer_offset >= self.transfer.len() {
                self.status = 0x40;
            }
            return Ok(value);
        }
        let value = match port {
            0x1f2 => self.sector_count,
            0x1f3 => self.lba as u8,
            0x1f4 => (self.lba >> 8) as u8,
            0x1f5 => (self.lba >> 16) as u8,
            0x1f6 => 0xe0 | ((self.lba >> 24) as u8 & 0x0f),
            0x1f7 => self.status,
            _ => return Err("unsupported ATA task-file register"),
        };
        Ok(value as u32)
    }

    fn write_port(
        &mut self,
        disk: &mut VirtualBlockDevice,
        port: u16,
        width: usize,
        value: u32,
    ) -> Result<(), &'static str> {
        if port == 0x1f0 {
            if width != 2 && width != 4 {
                return Err("ATA data port requires 16-bit or 32-bit access");
            }
            if !self.writing || self.transfer.is_empty() {
                return Err("ATA data port has no active write transfer");
            }
            for byte_offset in 0..width {
                if self.transfer_offset < self.transfer.len() {
                    self.transfer[self.transfer_offset] = (value >> (byte_offset * 8)) as u8;
                    self.transfer_offset += 1;
                }
            }
            if self.transfer_offset == self.transfer.len() {
                disk.write_sectors(self.lba as u64, 1, &self.transfer)?;
                self.lba = self.lba.wrapping_add(1);
                self.sector_count = self.sector_count.saturating_sub(1);
                if self.sector_count == 0 {
                    self.status = 0x40;
                    self.writing = false;
                    self.transfer.clear();
                } else {
                    self.transfer.fill(0);
                    self.transfer_offset = 0;
                }
            }
            return Ok(());
        }

        match port {
            0x1f2 => self.sector_count = value as u8,
            0x1f3 => self.lba = (self.lba & 0xffff_ff00) | (value as u8 as u32),
            0x1f4 => self.lba = (self.lba & 0xffff_00ff) | ((value as u8 as u32) << 8),
            0x1f5 => self.lba = (self.lba & 0xff00_ffff) | ((value as u8 as u32) << 16),
            0x1f6 => self.lba = (self.lba & 0x00ff_ffff) | (((value as u8 & 0x0f) as u32) << 24),
            0x1f7 => self.execute_command(disk, value as u8)?,
            _ => return Err("unsupported ATA task-file register"),
        }
        Ok(())
    }

    fn execute_command(
        &mut self,
        disk: &VirtualBlockDevice,
        command: u8,
    ) -> Result<(), &'static str> {
        self.transfer_offset = 0;
        self.writing = false;
        match command {
            0xec => {
                self.transfer = alloc::vec![0; SECTOR_SIZE];
                self.transfer[0..2].copy_from_slice(&0x0040u16.to_le_bytes());
                self.transfer[120..124]
                    .copy_from_slice(&((disk.capacity_bytes() / SECTOR_SIZE) as u32).to_le_bytes());
                self.status = 0x58;
            }
            0x20 | 0x24 => {
                let count = if self.sector_count == 0 {
                    256
                } else {
                    self.sector_count as u32
                };
                let byte_count = (count as usize)
                    .checked_mul(SECTOR_SIZE)
                    .ok_or("ATA read size overflows")?;
                self.transfer = alloc::vec![0; byte_count];
                disk.read_sectors(self.lba as u64, count, &mut self.transfer)?;
                self.status = 0x58;
            }
            0x30 | 0x34 => {
                if self.sector_count == 0 {
                    return Err("ATA writes larger than 255 sectors per command are not supported");
                }
                self.transfer = alloc::vec![0; SECTOR_SIZE];
                self.writing = true;
                self.status = 0x58;
            }
            _ => {
                self.status = 0x41;
                return Err("unsupported ATA command");
            }
        }
        Ok(())
    }
}

impl Default for VirtualAtaPio {
    fn default() -> Self {
        Self::new()
    }
}

/// Device bundle for one guest. Storage and display remain isolated from host
/// devices; a VM-exit dispatcher can route decoded accesses through this API.
pub struct VirtualDevices {
    pub console: VirtualConsole,
    pub disk: VirtualBlockDevice,
    pub display: VirtualDisplay,
    pub ata: VirtualAtaPio,
}

impl VirtualDevices {
    pub fn new(disk_bytes: usize, width: usize, height: usize) -> Result<Self, &'static str> {
        Ok(Self {
            console: VirtualConsole::new(),
            disk: VirtualBlockDevice::new(disk_bytes)?,
            display: VirtualDisplay::new(width, height)?,
            ata: VirtualAtaPio::new(),
        })
    }

    pub fn read_io(&mut self, port: u16, width: u8) -> Result<u32, &'static str> {
        if (0x3f8..=0x3ff).contains(&port) {
            self.console.handle_io_read(port, width as u32)
        } else if (0x1f0..=0x1f7).contains(&port) {
            self.ata.read_port(&self.disk, port, width as usize)
        } else if (0x3d0..=0x3d1).contains(&port) {
            if width != 1 {
                return Err("virtual display supports byte-sized port reads only");
            }
            self.display.read_port(port).map(u32::from)
        } else {
            Err("guest accessed an unassigned I/O port")
        }
    }

    pub fn write_io(&mut self, port: u16, width: u8, value: u32) -> Result<(), &'static str> {
        if (0x3f8..=0x3ff).contains(&port) {
            self.console.handle_io_write(port, width as u32, value)
        } else if (0x1f0..=0x1f7).contains(&port) {
            self.ata
                .write_port(&mut self.disk, port, width as usize, value)
        } else if (0x3d0..=0x3d1).contains(&port) {
            if width != 1 {
                return Err("virtual display supports byte-sized port writes only");
            }
            self.display.write_port(port, value as u8)
        } else {
            Err("guest accessed an unassigned I/O port")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uart_buffers_input_and_output() {
        let mut uart = VirtualConsole::with_capacity(2);
        uart.push_input(b"x");
        assert_eq!(
            uart.handle_io_read(UART_LINE_STATUS, 1),
            Ok(UART_RX_READY as u32 | UART_TX_EMPTY as u32)
        );
        assert_eq!(uart.handle_io_read(UART_DATA, 1), Ok(b'x' as u32));
        assert_eq!(uart.handle_io_write(UART_DATA, 1, b'o' as u32), Ok(()));
        assert_eq!(uart.handle_io_write(UART_DATA, 1, b'k' as u32), Ok(()));
        assert_eq!(
            uart.handle_io_write(UART_DATA, 1, b'!' as u32),
            Err("virtual serial output buffer is full")
        );
        assert_eq!(uart.drain_output(1), b"o");
        assert_eq!(uart.drain_output(8), b"k");
    }

    #[test]
    fn block_device_checks_sector_ranges_and_round_trips_data() {
        let mut disk = VirtualBlockDevice::new(SECTOR_SIZE * 2).unwrap();
        let input = [0x5a; SECTOR_SIZE];
        assert_eq!(disk.write_sectors(1, 1, &input), Ok(()));
        let mut output = [0; SECTOR_SIZE];
        assert_eq!(disk.read_sectors(1, 1, &mut output), Ok(()));
        assert_eq!(output, input);
        assert_eq!(
            disk.read_sectors(2, 1, &mut output),
            Err("virtual disk request exceeds capacity")
        );
    }

    #[test]
    fn virtual_display_bounds_writes() {
        let mut display = VirtualDisplay::new(2, 2).unwrap();
        assert_eq!(display.write_pixel(1, 1, 0xffff_ffff), Ok(()));
        assert_eq!(display.pixels()[3], 0x00ff_ffff);
        assert_eq!(
            display.write_pixel(2, 0, 0),
            Err("virtual display pixel is outside the framebuffer")
        );
    }

    #[test]
    fn guest_display_ports_update_the_framebuffer() {
        let mut devices = VirtualDevices::new(SECTOR_SIZE, 2, 2).unwrap();
        for (register, value) in [(0, 1), (2, 1), (4, 0x56), (5, 0x34), (6, 0x12), (7, 1)] {
            devices.write_io(0x3d0, 1, register).unwrap();
            devices.write_io(0x3d1, 1, value).unwrap();
        }
        assert_eq!(devices.display.pixels()[3], 0x0012_3456);
    }

    #[test]
    fn ata_pio_commands_read_and_write_guest_disk_sectors() {
        let mut devices = VirtualDevices::new(SECTOR_SIZE, 2, 2).unwrap();
        devices.write_io(0x1f2, 1, 1).unwrap();
        devices.write_io(0x1f3, 1, 0).unwrap();
        devices.write_io(0x1f4, 1, 0).unwrap();
        devices.write_io(0x1f5, 1, 0).unwrap();
        devices.write_io(0x1f7, 1, 0x30).unwrap();
        devices.write_io(0x1f0, 2, 0x5aa5).unwrap();
        for _ in 1..(SECTOR_SIZE / 2) {
            devices.write_io(0x1f0, 2, 0).unwrap();
        }

        devices.write_io(0x1f2, 1, 1).unwrap();
        devices.write_io(0x1f3, 1, 0).unwrap();
        devices.write_io(0x1f4, 1, 0).unwrap();
        devices.write_io(0x1f5, 1, 0).unwrap();
        devices.write_io(0x1f7, 1, 0x20).unwrap();
        assert_eq!(devices.read_io(0x1f0, 2), Ok(0x5aa5));
    }
}
