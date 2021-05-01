#![warn(clippy::all)]

#![no_std]

#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn the_answer() -> u32 {
    42
}