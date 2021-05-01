// use std::mem;
// use std::ffi::CString;
// use std::os::raw::{c_char, c_void};

#![no_std]

// extern {
//     fn console_log(str_ptr: *const u8, address: u32);
// }

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// const WIDTH: usize = 600;
// const HEIGHT: usize = 600;

const WIDTH: usize = 160;
const HEIGHT: usize = 50;

const HM_MON_COLOR_BG: u32 = 0xFF_00_00_00; // Black
const HM_MON_COLOR_FG: u32 = 0xFF_FF_FF_FF; // White

// #[no_mangle]
type Screen = [u32; WIDTH * HEIGHT];

#[no_mangle]
static mut FRAME_BUFFER: Screen = [HM_MON_COLOR_BG; WIDTH * HEIGHT];


#[no_mangle]
static mut KEYBOARD_BUFFER: u8 = 42;


#[no_mangle]
pub unsafe extern fn go() {
    // This is called from JavaScript, and should *only* be called from
    // JavaScript. If you maintain that condition, then we know that the &mut
    // we're about to produce is unique, and therefore safe.
    render_frame(&mut FRAME_BUFFER)
}

// We split this out so that we can escape 'unsafe' as quickly as possible.
fn render_frame(frame_buffer: &mut Screen) {
    // let mut drawing: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    // drawing[0] = 0xFF_FF_00_FF;
    // drawing[1] = 0xFF_00_FF_FF;
    // drawing[2] = 0xFF_00_00_FF;
    // let mut idx: usize = 0;
    // for pixel in frame_buffer.iter_mut() {
    //     *pixel = drawing[idx];
    //     idx = idx + 1;
    // }
    // unsafe {
    //     let msg = b"RUST(memory address):\0";
    //     console_log(msg as *const u8, frame_buffer[0]);
    // }

    frame_buffer[1] = HM_MON_COLOR_FG;
    frame_buffer[2] = HM_MON_COLOR_FG;
    frame_buffer[160] = HM_MON_COLOR_FG;
    frame_buffer[163] = HM_MON_COLOR_FG;
    frame_buffer[320] = HM_MON_COLOR_FG;
    frame_buffer[321] = HM_MON_COLOR_FG;
    frame_buffer[322] = HM_MON_COLOR_FG;
    frame_buffer[323] = HM_MON_COLOR_FG;
    frame_buffer[480] = HM_MON_COLOR_FG;
    frame_buffer[483] = HM_MON_COLOR_FG;
    frame_buffer[640] = HM_MON_COLOR_FG;
    frame_buffer[643] = HM_MON_COLOR_FG;
}
