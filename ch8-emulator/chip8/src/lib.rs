#![no_std]
#![warn(clippy::all)]

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, AtomicUsize, Ordering};
// use core::sync::atomic::{AtomicU32, AtomicUsize, AtomicU8, Ordering};

use wasm_stdlib::{collections::vectors::Vector, converters::convert_u32_to_str, mem::PAGE_SIZE};
// use wasm_stdlib::{converters::convert_u32_to_str, mem::PAGE_SIZE};

// Monotonic Clock
static MONOTONIC_CLOCK: AtomicU32 = AtomicU32::new(1);

#[no_mangle]
static SLEEP_DONE: AtomicBool = AtomicBool::new(false);

// Realtime Clock
static REALTIME_CLOCK: AtomicU32 = AtomicU32::new(0);

// Keyboard Buffer
static KEYBOARD_BUFFER: AtomicU8 = AtomicU8::new(0);

// Console Buffer
#[no_mangle]
static mut CONSOLE_BUFFER: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

#[no_mangle]
static CONSOLE_BUFFER_POINTER: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
static CONSOLE_BUFFER_BUSY: AtomicBool = AtomicBool::new(false);

/// # Safety
/// Do not use in production yet
/// For Debbuging purposes only
#[no_mangle]
pub unsafe extern "C" fn console_log(msg_input: &Vector) {
    // let mut message: [u8; 255] = [32; 255];

    let mut message = Vector::new();

    // ? Input
    let cycles = MONOTONIC_CLOCK.load(Ordering::Relaxed);

    let cycles_str = convert_u32_to_str(cycles);

    for i in 0..cycles_str.len() {
        let byte = cycles_str[i];
        if byte != 32 {
            let _ = message.push(byte);
        }
    }

    let separator = b": ";

    for i in 0..separator.len() {
        let byte = separator[i];
        let _ = message.push(byte);
    }

    for i in 0..msg_input.len() {
        if let Ok(byte) = msg_input.get(i) {
            let _ = message.push(byte);
        }
    }

    let line_feed = b"\n";

    for i in 0..line_feed.len() {
        let byte = line_feed[i];
        let _ = message.push(byte);
    }

    // * Output
    // TODO: sleep

    while CONSOLE_BUFFER_BUSY.load(Ordering::Relaxed) {}

    CONSOLE_BUFFER_BUSY.store(true, Ordering::Relaxed);

    let mut buffer_idx = CONSOLE_BUFFER_POINTER.load(Ordering::Relaxed);

    for i in 0..message.len() {
        if let Ok(byte) = message.get(i) {
            CONSOLE_BUFFER[buffer_idx] = byte;
        }
        buffer_idx += 1;
    }
    CONSOLE_BUFFER_POINTER.store(buffer_idx, Ordering::Relaxed);
    CONSOLE_BUFFER_BUSY.store(false, Ordering::Relaxed);
}

// #[no_mangle]
// pub unsafe extern "C" fn get_log() -> u8 {
//     42
// }

#[no_mangle]
pub extern "C" fn cpuMainLoop(realtime_clock: u32) -> u32 {
    SLEEP_DONE.store(true, Ordering::Relaxed);
    REALTIME_CLOCK.store(realtime_clock, Ordering::Relaxed);
    let current_clocktime = REALTIME_CLOCK.load(Ordering::Relaxed);
    let delay_converted = convert_u32_to_str(current_clocktime);
    let mut message = Vector::new();
    for i in 0..delay_converted.len() {
        let byte = delay_converted[i];
        if byte != 32 {
            let _ = message.push(byte);
        }
    }
    unsafe {
        console_log(&message);
    }
    let current_cycle = MONOTONIC_CLOCK.fetch_add(1, Ordering::Relaxed);
    // KEYBOARD_BUFFER.load(Ordering::Relaxed)
    current_clocktime
}

#[no_mangle]
pub extern "C" fn keyboard_send_key(key_code: u8) {
    match key_code {
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
        _ => {
            let message = Vector::from("Invalid Key!");
            unsafe {
                console_log(&message);
            }

            KEYBOARD_BUFFER.store(0, Ordering::Relaxed)
        }
    }
}
