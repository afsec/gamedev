#![no_std]

#![warn(clippy::all)]

// mod converters;

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};
use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};

// use crate::converters::convert_u32_to_str;

// Monotonic Clock
static MONOTONIC_CLOCK: AtomicU32 = AtomicU32::new(1);

// Keyboard Buffer
static KEYBOARD_BUFFER: AtomicU8 = AtomicU8::new(0);

// Console Buffer
#[no_mangle]
static mut CONSOLE_BUFFER: [u8; 255] = [0; 255];

// TODO
// static CONSOLE_BUFFER_POINTER: AtomicU8 = AtomicU8::new(0);

// TODO
// static CONSOLE_BUFFER_BUSY: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub unsafe extern "C" fn console_log(code: u8) {
    let mut message: [u8; 255] = [32; 255];

    // ? Input
    let cycles = MONOTONIC_CLOCK.load(Ordering::Relaxed);
    let msg1 = b"Primeira Mensagem\n";
    // let msg1 = convert_u32_to_str(cycles);
    let msg2 = b": Invalid Key!";

    // ! Processing: Concat 2 array characters into a 3rd array
    let mut msg1_pos = 0;
    let mut msg2_pos = 0;

    for ptr in message.iter_mut() {
        if msg1_pos < msg1.len() {
            *ptr = msg1[msg1_pos];
            msg1_pos += 1;
        } else if msg2_pos < msg2.len() {
            *ptr = msg2[msg2_pos];
            msg2_pos += 1;
        } else {
            *ptr = 32;
        }
    }

    // * Output
    let mut x = 0;
    for ptr in CONSOLE_BUFFER.iter_mut() {
        match message.get(x) {
            Some(c) => *ptr = *c,
            None => *ptr = 0,
        }
        x += 1;
    }
}

// #[no_mangle]
// pub unsafe extern "C" fn get_log() -> u8 {
//     42
// }


#[no_mangle]
pub extern "C" fn cpuMainLoop() -> u8 {
    let current_cycle = MONOTONIC_CLOCK.fetch_add(1, Ordering::Relaxed);
    KEYBOARD_BUFFER.load(Ordering::Relaxed)
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
            unsafe {
                console_log(0);
            }
            KEYBOARD_BUFFER.store(0, Ordering::Relaxed)
        }
    }
}
