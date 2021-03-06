use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};

use sha1::Sha1;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn digest(data: *mut c_char) -> *mut c_char {
    unsafe {
        let data = CStr::from_ptr(data);

        let mut m = Sha1::new();
        m.update(data.to_bytes());
        let dgst = m.digest().to_string();
        let s = CString::new(dgst).unwrap();
        s.into_raw()
    }
}

// References:
// http://cliffle.com/blog/bare-metal-wasm/
// https://www.hellorust.com/demos/canvas/index.html
// https://blog.scottlogic.com/2017/12/13/chip8-emulator-webassembly-rust.html
// https://mir3z.github.io/chip8-emu/
// http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/Chip8.pdf
// http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/reports/Chip8.pdf
// https://johnearnest.github.io/Octo/

// https://www.mschweighauser.com/make-your-own-assembler-simulator-in-javascript-part1/
