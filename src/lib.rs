extern crate ferris_says;

use std::ffi::CStr;
use std::io::{stdout, BufWriter};
use std::os::raw::{c_char, c_float, c_int, c_longlong, c_ulong};

#[repr(C)]
pub struct Point {
    pub x: c_float,
    pub y: c_float,
}

/// Simple write in console
#[no_mangle]
pub extern "C" fn hello_world() {
    // println!("Hello world from Rust -> Swift -> Flutter");
    let phrase = b"Hello world from Rust -> Swift -> Flutter";
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    ferris_says::say(phrase, 41, &mut writer).unwrap();
}

/// Add two signed integer
///
/// On a 64-bit system, arguments are 32 bit and return is 64 bit.
#[no_mangle]
pub extern "C" fn add_numbers(x: c_int, y: c_int) -> c_longlong {
    x as c_longlong + y as c_longlong
}

/// Take a zero-terminated C String and return its length as a
/// machine-size integer.
#[no_mangle]
pub extern "C" fn string_length(sz_msg: *const c_char) -> c_ulong {
    let slice = unsafe { CStr::from_ptr(sz_msg) };
    slice.to_bytes().len() as c_ulong
}

/// Some exemple with struct
#[no_mangle]
pub extern "C" fn give_me_a_point() -> Point {
    Point { x: 3.14, y: 12.0 }
}

/// Some other exemple with struct
#[no_mangle]
pub extern "C" fn magnitude(p: Point) -> c_float {
    (p.x * p.x + p.y * p.y).sqrt()
}

/// Simple test
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_addition() {
        assert_eq!(add_numbers(2, 5), 7);
    }
}
