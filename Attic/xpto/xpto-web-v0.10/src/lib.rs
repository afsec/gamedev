#![warn(clippy::all)]
#![no_std]

use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};

// Monotonic Clock
static MONOTONIC_CLOCK: AtomicU32 = AtomicU32::new(1);
// static REALTIME_CLOCK: AtomicU32 = AtomicU32::new(0);

// Keyboard Buffer
static KEYBOARD_BUFFER: AtomicU8 = AtomicU8::new(0);

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn cpuMainLoop() -> u8 {
    let current_cycle = MONOTONIC_CLOCK.fetch_add(1, Ordering::Relaxed);
    //current_cycle // Return current cycle_id
    KEYBOARD_BUFFER.load(Ordering::Relaxed)
}

#[no_mangle]
pub extern "C" fn keyboard_send_key(keyCode: u8) {
    match keyCode {
        48 => KEYBOARD_BUFFER.store(0, Ordering::Relaxed),
        49 => KEYBOARD_BUFFER.store(1, Ordering::Relaxed),
        50 => KEYBOARD_BUFFER.store(2, Ordering::Relaxed),
        51 => KEYBOARD_BUFFER.store(3, Ordering::Relaxed),
        52 => KEYBOARD_BUFFER.store(4, Ordering::Relaxed),
        53 => KEYBOARD_BUFFER.store(5, Ordering::Relaxed),
        54 => KEYBOARD_BUFFER.store(6, Ordering::Relaxed),
        55 => KEYBOARD_BUFFER.store(7, Ordering::Relaxed),
        56 => KEYBOARD_BUFFER.store(8, Ordering::Relaxed),
        57 => KEYBOARD_BUFFER.store(9, Ordering::Relaxed),
        _ => KEYBOARD_BUFFER.store(0, Ordering::Relaxed),
    }
}
