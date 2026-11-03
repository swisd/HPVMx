//! PS/2 Keyboard and Mouse Driver
//!
//! Directly interacts with PS/2 hardware ports using raw I/O calls from `rawio.rs`.
//! Supports both keyboard (port 0x60/0x64) and mouse (auxiliary channel).

use crate::raw::rawio;

// PS/2 Controller Ports
pub const PS2_COMMAND_PORT: u16 = 0x64;
pub const PS2_DATA_PORT: u16 = 0x60;

// PS/2 Controller Commands
pub const PS2_CMD_WRITE_COMMAND: u8 = 0x60;  // Write to PS/2 command
pub const PS2_CMD_WRITE_CONFIG: u8 = 0x60;   // Write configuration byte
pub const PS2_CMD_READ_CONFIG: u8 = 0x20;    // Read configuration byte
pub const PS2_CMD_TEST_CONTROLLER: u8 = 0xAA; // Test controller
pub const PS2_CMD_TEST_PORT1: u8 = 0xAB;     // Test port 1 (keyboard)
pub const PS2_CMD_TEST_PORT2: u8 = 0xA9;     // Test port 2 (mouse)
pub const PS2_CMD_DISABLE_PORT2: u8 = 0xA7;  // Disable port 2 (mouse)
pub const PS2_CMD_ENABLE_PORT2: u8 = 0xA8;   // Enable port 2 (mouse)
pub const PS2_CMD_WRITE_PORT2: u8 = 0xD4;    // Write to port 2 device (mouse)
pub const PS2_CMD_READ_PORT1: u8 = 0x20;     // Read from port 1 (keyboard)
pub const PS2_CMD_READ_PORT2: u8 = 0x20;     // Read from port 2 (mouse)

// PS/2 Device Commands
pub const PS2_DEVICE_RESET: u8 = 0xFF;
pub const PS2_DEVICE_SET_DEFAULTS: u8 = 0xF6;
pub const PS2_DEVICE_ENABLE: u8 = 0xF4;
pub const PS2_DEVICE_DISABLE: u8 = 0xF5;
pub const PS2_DEVICE_GET_ID: u8 = 0xF2;

// Mouse-specific commands
pub const PS2_MOUSE_SET_SAMPLE_RATE: u8 = 0xF3;
pub const PS2_MOUSE_SET_SCALING_1TO1: u8 = 0xE7;
pub const PS2_MOUSE_SET_SCALING_2TO1: u8 = 0xE8;
pub const PS2_MOUSE_SET_RESOLUTION: u8 = 0xE1;
pub const PS2_MOUSE_GET_STATUS: u8 = 0xE9;

// PS/2 Status Register bits
pub const PS2_STATUS_OBF: u8 = 0x01;   // Output buffer full
pub const PS2_STATUS_IBF: u8 = 0x02;   // Input buffer full
pub const PS2_STATUS_CMD_DATA: u8 = 0x04; // Command/data select
pub const PS2_STATUS_SYS_FLAG: u8 = 0x08; // System flag
pub const PS2_STATUS_TIMEOUT1: u8 = 0x20; // Timeout
pub const PS2_STATUS_TIMEOUT2: u8 = 0x28; // Timeout
pub const PS2_STATUS_PORT1: u8 = 0x20; // Auxiliary input output
pub const PS2_STATUS_PORT2: u8 = 0x20; // Auxiliary input output

// PS/2 Mouse byte 0 bits
pub const PS2_MOUSE_LEFT: u8 = 0x01;
pub const PS2_MOUSE_RIGHT: u8 = 0x02;
pub const PS2_MOUSE_MIDDLE: u8 = 0x04;
pub const PS2_MOUSE_X_OVERFLOW: u8 = 0x10;
pub const PS2_MOUSE_Y_OVERFLOW: u8 = 0x20;
pub const PS2_MOUSE_X_SIGN: u8 = 0x40;
pub const PS2_MOUSE_Y_SIGN: u8 = 0x80;

// PS/2 Keyboard scan code sets
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScanCodeSet {
    Set1,
    Set2,
    Set3,
}

/// PS/2 Keyboard state
pub struct Ps2Keyboard {
    pub initialized: bool,
    pub scan_code_set: ScanCodeSet,
    pub make_code: bool,  // true = key pressed, false = key released
    pub shift_pressed: bool,
    pub ctrl_pressed: bool,
    pub alt_pressed: bool,
}

/// PS/2 Mouse state
pub struct Ps2Mouse {
    pub initialized: bool,
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
    pub x_movement: i8,
    pub y_movement: i8,
    pub x_overflow: bool,
    pub y_overflow: bool,
}

/// Combined PS/2 driver state
pub struct Ps2Driver {
    pub keyboard: Ps2Keyboard,
    pub mouse: Ps2Mouse,
}

// Delay helper for PS/2 timing
unsafe fn ps2_delay() {
    for _ in 0..100 {
        core::hint::spin_loop();
    }
}

/// Wait until the input buffer is not full (safe to write)
unsafe fn wait_input_buffer_empty() -> bool {
    for _ in 0..1000 {
        let status = rawio::__inbyte(PS2_COMMAND_PORT);
        if status & PS2_STATUS_IBF == 0 {
            return true;
        }
        ps2_delay();
    }
    false
}

/// Wait until the output buffer is full (data ready to read)
unsafe fn wait_output_buffer_full() -> bool {
    for _ in 0..1000 {
        let status = rawio::__inbyte(PS2_COMMAND_PORT);
        if status & PS2_STATUS_OBF != 0 {
            return true;
        }
        ps2_delay();
    }
    false
}

/// Write a command to the PS/2 controller
unsafe fn ps2_write_command(cmd: u8) -> bool {
    if !wait_input_buffer_empty() {
        return false;
    }
    rawio::__outbyte(PS2_COMMAND_PORT, cmd);
    true
}

/// Write data to the PS/2 data port
unsafe fn ps2_write_data(data: u8) -> bool {
    if !wait_input_buffer_empty() {
        return false;
    }
    rawio::__outbyte(PS2_DATA_PORT, data);
    true
}

/// Read data from the PS/2 data port
unsafe fn ps2_read_data() -> Option<u8> {
    if !wait_output_buffer_full() {
        return None;
    }
    Some(rawio::__inbyte(PS2_DATA_PORT))
}

/// Send a command to a PS/2 device and wait for the response
unsafe fn ps2_device_command(cmd: u8) -> Option<u8> {
    if !ps2_write_command(cmd) {
        return None;
    }
    ps2_delay();
    ps2_read_data()
}

/// Check if there's data available to read from the keyboard
pub unsafe fn keyboard_data_available() -> bool {
    let status = rawio::__inbyte(PS2_COMMAND_PORT);
    status & PS2_STATUS_OBF != 0
}

/// Read a scan code from the keyboard (non-blocking, returns None if no data)
pub unsafe fn keyboard_read_scan_code() -> Option<u8> {
    if !keyboard_data_available() {
        return None;
    }
    Some(rawio::__inbyte(PS2_DATA_PORT))
}

/// Initialize the PS/2 keyboard
pub unsafe fn keyboard_init() -> bool {
    // Test the controller
    ps2_write_command(PS2_CMD_TEST_CONTROLLER);
    ps2_delay();
    
    if let Some(result) = ps2_read_data() {
        if result != 0x55 {
            return false;
        }
    } else {
        return false;
    }

    // Test port 1 (keyboard port)
    ps2_write_command(PS2_CMD_TEST_PORT1);
    ps2_delay();
    
    if let Some(result) = ps2_read_data() {
        if result != 0x55 {
            return false;
        }
    } else {
        return false;
    }

    // Enable port 1
    ps2_write_command(PS2_CMD_ENABLE_PORT2);
    ps2_delay();

    // Send reset command to keyboard
    ps2_write_data(PS2_DEVICE_RESET);
    ps2_delay();
    
    // Read the ACK (0xFA) or reset complete (0xAA)
    if let Some(resp) = ps2_read_data() {
        if resp == 0xAA {
            // Keyboard initialized successfully
            return true;
        }
    }

    false
}

/// Initialize the PS/2 mouse
pub unsafe fn mouse_init() -> bool {
    // Enable port 2 (mouse port)
    ps2_write_command(PS2_CMD_ENABLE_PORT2);
    ps2_delay();

    // Send reset command to mouse
    ps2_write_command(PS2_CMD_WRITE_PORT2);
    ps2_delay();
    ps2_write_data(PS2_DEVICE_RESET);
    ps2_delay();

    // Read response bytes
    let mut responses = [0u8; 3];
    for i in 0..3 {
        if let Some(resp) = ps2_read_data() {
            responses[i] = resp;
        } else {
            return false;
        }
        ps2_delay();
    }

    // Check for ACK (0xFA) or reset complete (0xAA)
    if responses[0] == 0xFA || responses[0] == 0xAA {
        // Configure mouse
        // Enable data reporting
        ps2_write_command(PS2_CMD_WRITE_PORT2);
        ps2_delay();
        ps2_write_data(PS2_DEVICE_ENABLE);
        ps2_delay();

        // Set scaling to 1:1
        ps2_write_command(PS2_CMD_WRITE_PORT2);
        ps2_delay();
        ps2_write_data(PS2_MOUSE_SET_SCALING_1TO1);
        ps2_delay();
        ps2_read_data(); // ACK

        // Set sample rate to 100 Hz
        ps2_write_command(PS2_CMD_WRITE_PORT2);
        ps2_delay();
        ps2_write_data(PS2_MOUSE_SET_SAMPLE_RATE);
        ps2_delay();
        ps2_read_data(); // ACK
        ps2_write_data(100);
        ps2_delay();
        ps2_read_data(); // ACK

        // Set resolution (0 = 1 count/mm)
        ps2_write_command(PS2_CMD_WRITE_PORT2);
        ps2_delay();
        ps2_write_data(PS2_MOUSE_SET_RESOLUTION);
        ps2_delay();
        ps2_read_data(); // ACK
        ps2_write_data(0);
        ps2_delay();
        ps2_read_data(); // ACK

        return true;
    }

    false
}

/// Check if there's mouse data available
pub unsafe fn mouse_data_available() -> bool {
    let status = rawio::__inbyte(PS2_COMMAND_PORT);
    status & PS2_STATUS_OBF != 0
}

/// Read a 3-byte mouse packet (non-blocking)
/// Returns (button_state, x_movement, y_movement)
pub unsafe fn mouse_read_packet() -> Option<(u8, i8, i8)> {
    let mut bytes = [0u8; 3];
    
    for i in 0..3 {
        if let Some(b) = ps2_read_data() {
            bytes[i] = b;
        } else {
            return None;
        }
    }

    let button_state = bytes[0];
    let x = bytes[1] as i8;
    let y = bytes[2] as i8;

    Some((button_state, x, y))
}

/// Full PS/2 driver initialization
pub unsafe fn ps2_init() -> Ps2Driver {
    let keyboard_ok = keyboard_init();
    let mouse_ok = mouse_init();

    Ps2Driver {
        keyboard: Ps2Keyboard {
            initialized: keyboard_ok,
            scan_code_set: ScanCodeSet::Set1,
            make_code: true,
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
        },
        mouse: Ps2Mouse {
            initialized: mouse_ok,
            left_button: false,
            right_button: false,
            middle_button: false,
            x_movement: 0,
            y_movement: 0,
            x_overflow: false,
            y_overflow: false,
        },
    }
}

/// Process a keyboard scan code and return the ASCII character (if applicable)
/// Uses PS/2 Set 1 scan codes. Returns None for special keys (F1-F12, arrows, etc.).
pub fn process_keyboard_scan_code(scan_code: u8, shift: bool) -> Option<char> {
    // PS/2 Set 1 scan code to ASCII mapping
    // Each scan code appears exactly once — no duplicates.
    match scan_code {
        // Letters
        0x1C => Some(if shift { 'A' } else { 'a' }),
        0x32 => Some(if shift { 'B' } else { 'b' }),
        0x21 => Some(if shift { 'C' } else { 'c' }),
        0x23 => Some(if shift { 'D' } else { 'd' }),
        0x24 => Some(if shift { 'E' } else { 'e' }),
        0x2B => Some(if shift { 'F' } else { 'f' }),
        0x34 => Some(if shift { 'G' } else { 'g' }),
        0x33 => Some(if shift { 'H' } else { 'h' }),
        0x43 => Some(if shift { 'I' } else { 'i' }),
        0x3B => Some(if shift { 'J' } else { 'j' }),
        0x42 => Some(if shift { 'K' } else { 'k' }),
        0x4B => Some(if shift { 'L' } else { 'l' }),
        0x3A => Some(if shift { 'M' } else { 'm' }),
        0x31 => Some(if shift { 'N' } else { 'n' }),
        0x44 => Some(if shift { 'O' } else { 'o' }),
        0x4D => Some(if shift { 'P' } else { 'p' }),
        0x15 => Some(if shift { 'Q' } else { 'q' }),
        0x2D => Some(if shift { 'R' } else { 'r' }),
        0x1B => Some(if shift { 'S' } else { 's' }),
        0x2C => Some(if shift { 'T' } else { 't' }),
        0x3C => Some(if shift { 'U' } else { 'u' }),
        0x2A => Some(if shift { 'V' } else { 'v' }),
        0x1D => Some(if shift { 'W' } else { 'w' }),
        0x22 => Some(if shift { 'X' } else { 'x' }),
        0x35 => Some(if shift { 'Y' } else { 'y' }),
        0x1A => Some(if shift { 'Z' } else { 'z' }),
        
        // Numbers row
        0x16 => Some(if shift { '!' } else { '1' }),
        0x1E => Some(if shift { '@' } else { '2' }),
        0x26 => Some(if shift { '#' } else { '3' }),
        0x25 => Some(if shift { '$' } else { '4' }),
        0x2E => Some(if shift { '%' } else { '5' }),
        0x36 => Some(if shift { '^' } else { '6' }),
        0x3E => Some(if shift { '&' } else { '7' }),
        0x46 => Some(if shift { '*' } else { '8' }),
        0x45 => Some(if shift { '(' } else { '9' }),
        0x4E => Some(if shift { ')' } else { '0' }),
        
        // Special keys — unique scan codes
        0x5A => Some('\n'),       // Enter
        0x0E => Some('\x08'),     // Backspace
        0x39 => Some(' '),        // Space
        0x0F => Some('\t'),       // Tab
        0x29 => Some(if shift { '|' } else { '\\' }),
        0x55 => Some(if shift { '+' } else { '=' }),
        0x41 => Some(if shift { '[' } else { ';' }),
        0x49 => Some(if shift { '"' } else { '\'' }),
        0x53 => Some(if shift { '>' } else { ',' }),
        0x4C => Some(if shift { '_' } else { '-' }),
        0x47 => Some(if shift { '?' } else { '/' }),
        0x5D => Some(if shift { '~' } else { '`' }),
        0x54 => Some(if shift { '}' } else { ']' }),
        
        _ => None,
    }
}

/// Update mouse state from a 3-byte packet
pub fn update_mouse_state(mouse: &mut Ps2Mouse, button_state: u8, x: i8, y: i8) {
    mouse.left_button = button_state & PS2_MOUSE_LEFT != 0;
    mouse.right_button = button_state & PS2_MOUSE_RIGHT != 0;
    mouse.middle_button = button_state & PS2_MOUSE_MIDDLE != 0;
    mouse.x_movement = x;
    mouse.y_movement = y;
    mouse.x_overflow = button_state & PS2_MOUSE_X_OVERFLOW != 0;
    mouse.y_overflow = button_state & PS2_MOUSE_Y_OVERFLOW != 0;
}
