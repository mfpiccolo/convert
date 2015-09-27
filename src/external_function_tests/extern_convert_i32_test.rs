extern crate libc;

use stringify::Stringify;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern fn i32_convert_to_string_test() {
  let y = convert!(100 => String) as String;
  println!("{:?}", y);
}

#[no_mangle]
pub extern fn i32_to_libc_char_test() -> *const libc::c_char {
  convert!(1 => *const libc::c_char) as *const libc::c_char
}

