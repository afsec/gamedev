// use core::str;

const DEBUG: bool = true;

pub fn convert_u32_to_str(mut number: u32) -> [u8; 10] {
    let mut result: [u8; 10] = [0; 10];

    let mut result_str: [u8; 10] = [0; 10];

    let mut pos: usize = 0;

    if DEBUG {
        //dbg!(number);
    }

    let mut number_detected = false;

    // * billions
    if number > 999_999_999 {
        result[pos] = (number / 1_000_000_000) as u8;
        number = number - (1_000_000_000 * result[pos] as u32);

        result_str[pos] = result[pos] + 48;
        number_detected = true;

        if DEBUG {
            //dbg!(number);
        }
    } else {
        result[pos] = 0 as u8;
        if number_detected {
            result_str[pos] = 48;
        } else {
            result_str[pos] = 32;
        }

        if DEBUG {
            //dbg!(number);
        }
    }
    pos += 1;

    // * hundreds_of_millions
    if number > 99_999_999 {
        result[pos] = (number / 100_000_000) as u8;
        number = number - (100_000_000 * result[pos] as u32);

        result_str[pos] = result[pos] + 48;
        number_detected = true;

        if DEBUG {
            //dbg!(number);
        }
    } else {
        result[pos] = 0 as u8;
        if number_detected {
            result_str[pos] = 48;
        } else {
            result_str[pos] = 32;
        }

        if DEBUG {
            //dbg!(number);
        }
    }
    pos += 1;

    // * tens_of_millions
    if number > 9_999_999 {
        result[pos] = (number / 10_000_000) as u8;
        number = number - (10_000_000 * result[pos] as u32);

        result_str[pos] = result[pos] + 48;
        number_detected = true;

        if DEBUG {
            //dbg!(number);
        }
    } else {
        result[pos] = 0 as u8;
        if number_detected {
            result_str[pos] = 48;
        } else {
            result_str[pos] = 32;
        }

        if DEBUG {
            //dbg!(number);
        }
    }
    pos += 1;

    // * millions
    if number > 999_999 {
        result[pos] = (number / 1_000_000) as u8;
        number = number - (1_000_000 * result[pos] as u32);

        result_str[pos] = result[pos] + 48;
        number_detected = true;

        if DEBUG {
            //dbg!(number);
        }
    } else {
        result[pos] = 0 as u8;
        if number_detected {
            result_str[pos] = 48;
        } else {
            result_str[pos] = 32;
        }

        if DEBUG {
            //dbg!(number);
        }
    }
    pos += 1;

    // * hundreds_of_thousands
    if number > 99_999 {
        result[pos] = (number / 100_000) as u8;
        number = number - (100_000 * result[pos] as u32);

        result_str[pos] = result[pos] + 48;
        number_detected = true;

        if DEBUG {
            //dbg!(number);
        }
    } else {
        result[pos] = 0 as u8;
        if number_detected {
            result_str[pos] = 48;
        } else {
            result_str[pos] = 32;
        }

        if DEBUG {
            //dbg!(number);
        }
    }
    pos += 1;

    // * tens_of_thousands
    if number > 9_999 {
        result[pos] = (number / 10_000) as u8;
        number = number - (10_000 * result[pos] as u32);

        result_str[pos] = result[pos] + 48;
        number_detected = true;

        if DEBUG {
            //dbg!(number);
        }
    } else {
        result[pos] = 0 as u8;
        if number_detected {
            result_str[pos] = 48;
        } else {
            result_str[pos] = 32;
        }

        if DEBUG {
            //dbg!(number);
        }
    }
    pos += 1;

    //* thousands
    if number > 999 {
        result[pos] = (number / 1_000) as u8;
        number = number - (1_000 * result[pos] as u32);

        result_str[pos] = result[pos] + 48;
        number_detected = true;

        if DEBUG {
            //dbg!(number);
        }
    } else {
        result[pos] = 0 as u8;
        if number_detected {
            result_str[pos] = 48;
        } else {
            result_str[pos] = 32;
        }

        if DEBUG {
            //dbg!(number);
        }
    }
    pos += 1;

    //* hundreds
    if number > 99 {
        result[pos] = (number / 100) as u8;
        number = number - (100 * result[pos] as u32);

        result_str[pos] = result[pos] + 48;
        number_detected = true;

        if DEBUG {
            //dbg!(number);
        }
    } else {
        result[pos] = 0 as u8;
        if number_detected {
            result_str[pos] = 48;
        } else {
            result_str[pos] = 32;
        }

        if DEBUG {
            //dbg!(number);
        }
    }
    pos += 1;

    //* tens
    if number > 9 {
        result[pos] = (number / 10) as u8;
        number = number - (10 * result[pos] as u32);

        result_str[pos] = result[pos] + 48;
        number_detected = true;

        if DEBUG {
            //dbg!(number);
        }
    } else {
        result[pos] = 0 as u8;
        if number_detected {
            result_str[pos] = 48;
        } else {
            result_str[pos] = 32;
        }

        if DEBUG {
            //dbg!(number);
        }
    }
    pos += 1;

    //* units
    if number > 0 {
        result[pos] = (number / 1) as u8;
        number = number - (1 * result[pos] as u32);

        result_str[pos] = result[pos] + 48;

        if DEBUG {
            //dbg!(number);
        }
    } else {
        result[pos] = 0 as u8;
        result_str[pos] = 48;

        if DEBUG {
            //dbg!(number);
        }
    }

    if number == 0 {
        if DEBUG {
            //dbg!(&result);
            //dbg!(&result_str);
        }
    } else {
        panic!("Error: Check algorithm")
    }
    for ascii_code in result_str.iter_mut() {
        if *ascii_code == 0 {
            // 32 is ascii code to ' ' whitespace
            *ascii_code = 32;
        }
    }
    if DEBUG {
        // //dbg!(&result_str);
    }

    // let converted_number = str::from_utf8(&result_str).unwrap();
    // //dbg!(converted_number);
    // //dbg!(converted_number.trim_start());
    // println!();

    result_str
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn valid() {
//         assert!(*b"         1" == convert_u32_to_str(1));
//         assert!(*b"        42" == convert_u32_to_str(42));
//         assert!(*b"       537" == convert_u32_to_str(537));
//         assert!(*b"       999" == convert_u32_to_str(999));
//         assert!(*b"    123121" == convert_u32_to_str(123121)); // 0x00_01_E0_F1
//         assert!(*b"  10000000" == convert_u32_to_str(10_000_000));
//         assert!(*b"  10020030" == convert_u32_to_str(10_020_030));
//         assert!(*b"4294967295" == convert_u32_to_str(4_294_967_295));
//     }

//     #[test]
//     fn zero() {
//         assert!(*b"         0" == convert_u32_to_str(0));
//     }
// }
