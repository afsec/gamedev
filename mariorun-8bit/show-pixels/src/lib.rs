struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[no_mangle]
pub fn color(width: u32, height: u32) {
    for i in 0..width * height {
        let ptr = (i * 4) as u64 as *mut Pixel;
        let mut pixel = unsafe { &mut *ptr };
        pixel.r = 255;
        pixel.g = 0;
        pixel.b = 0;
        pixel.a = 255;
    }
}