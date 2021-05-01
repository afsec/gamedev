#![warn(clippy::all)]
#![no_std]

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn cpuMainLoop(x: u32) -> u32 {
    if x < 30 {
        30 - x
    } else if x > 30 {
        x - 30
    } else {
        0
    }
}
