//! PC Speaker audio driver for HPVMx
//!
//! This module provides a basic driver for the PC Speaker using the PIT (Programmable Interval Timer).

use core::time::Duration;
use crate::TSC_PER_US;

/// PC Speaker control IO ports
const PIT_CHANNEL_2: u16 = 0x42;
const PIT_COMMAND: u16 = 0x43;
const SPEAKER_CONTROL: u16 = 0x61;

/// State for non-blocking audio
struct AudioState {
    end_tsc: u64,
    active: bool,
}

static mut AUDIO_STATE: AudioState = AudioState {
    end_tsc: 0,
    active: false,
};

/// Play a sound at the given frequency
pub fn beep(frequency_hz: u32) {
    if frequency_hz == 0 {
        mute();
        return;
    }

    let divisor = 1193180 / frequency_hz;

    unsafe {
        // Set PIT to square wave mode on channel 2
        out_b(PIT_COMMAND, 0xB6);
        // Send divisor (low byte then high byte)
        out_b(PIT_CHANNEL_2, (divisor & 0xFF) as u8);
        out_b(PIT_CHANNEL_2, ((divisor >> 8) & 0xFF) as u8);

        // Enable speaker (bits 0 and 1 of port 0x61)
        let state = in_b(SPEAKER_CONTROL);
        if (state & 0x03) != 0x03 {
            out_b(SPEAKER_CONTROL, state | 0x03);
        }
    }
}

/// Stop all sound from the PC speaker
pub fn mute() {
    unsafe {
        let state = in_b(SPEAKER_CONTROL);
        out_b(SPEAKER_CONTROL, state & 0xFC);
        
        // Also clear non-blocking state
        AUDIO_STATE.active = false;
    }
}

/// Update non-blocking audio state. Should be called frequently in the main loop.
pub fn update() {
    unsafe {
        if AUDIO_STATE.active {
            let current_tsc = core::arch::x86_64::_rdtsc();
            if current_tsc >= AUDIO_STATE.end_tsc {
                mute();
            }
        }
    }
}

/// Play a tone non-blockingly
pub fn play_tone_nb(frequency_hz: u32, duration_ms: u64) {
    if frequency_hz == 0 {
        mute();
        return;
    }

    beep(frequency_hz);
    
    unsafe {
        let current_tsc = core::arch::x86_64::_rdtsc();
        let duration_tsc = duration_ms * 1000 * TSC_PER_US;
        AUDIO_STATE.end_tsc = current_tsc + duration_tsc;
        AUDIO_STATE.active = true;
    }
}

/// Play a tone for a specific duration (blocking)
pub fn play_tone(frequency_hz: u32, duration_ms: u64) {
    beep(frequency_hz);
    let _ = uefi::boot::stall(Duration::from_millis(duration_ms));
    mute();
}

/// Play a simple startup melody
pub fn play_startup_sound() {
    play_tone(440, 100); // A4
    play_tone(554, 100); // C#5
    play_tone(659, 150); // E5
}

// IO port helpers
unsafe fn out_b(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val);
}

unsafe fn in_b(port: u16) -> u8 {
    let val: u8;
    core::arch::asm!("in al, dx", out("al") val, in("dx") port);
    val
}
