struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

struct Pixel {
    width: u8,
    height: u8,
    color: Color
}


#[no_mangle]
pub fn initial_frame(width: u32, height: u32) {
    let mut tela: Vec<u64> = Vec::new();

    // Percorre todo o a Array
    // Ou seja percorre toda a tela. Tipo uma refresh function
    // Meu controle para posição de memória
    let mut coordinate: u32 = 0;

    for i in 0..width * height {
            coordinate = i;
            let ptr = (coordinate * 4) as u64 as *mut Color;
            let mut pixel = unsafe { &mut *ptr };
            pixel.r = 0; // Preto
            pixel.g = 0;
            pixel.b = 0;
            pixel.a = 255;
    }
}


// #[no_mangle]
// pub fn one_pixel(width: u32, height: u32) {
//     // Percorre todo o a Array
//     // Ou seja percorre toda a tela. Tipo uma refresh function
//     // Meu controle para posição de memória
//     let mut coordinate: u32 = 0;

//     for i in 0..width * height {
//         if i % 10 == 0 {
//             coordinate = i;
//             let ptr = (coordinate * 4) as u64 as *mut Color;
//             let mut pixel = unsafe { &mut *ptr };
//             pixel.r = 255; // ! Vermelho
//             pixel.g = 0;
//             pixel.b = 0;
//             pixel.a = 255;
//         }
//     }
// }

// #[no_mangle]
// pub fn colorize(width: u32, height: u32) {
//     for i in 0..width * height {
//         let ptr = (i * 4) as u64 as *mut Color;
//         let mut pixel = unsafe { &mut *ptr };
//         pixel.r = 255;
//         pixel.g = 0;
//         pixel.b = 0;
//         pixel.a = 255;
//     }
// }
