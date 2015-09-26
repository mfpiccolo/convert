extern crate libc;

use stringify::Stringify;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern fn i32_convert_to_string_test() -> *const libc::c_char {
  let x: i32 = 1;
  let valid = x.convert_to_string() == "1".to_string();
  if valid {
    x.convert_to_libc_char()
  } else {
    "Fail".to_string().convert_to_libc_char()
  }
}

#[no_mangle]
pub extern fn i32_to_libc_char_test() -> *const libc::c_char {
  let x: i32 = 1;
  x.convert_to_libc_char()
}

