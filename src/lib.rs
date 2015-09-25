#![feature(test)]
extern crate libc;

pub mod stringify;

use stringify::Stringify;
use std::ffi::{CStr, CString};

mod tests;
mod benches;

#[no_mangle]
pub extern fn no_op() {}

#[no_mangle]
pub extern fn string_to_libc_char_test() -> *const libc::c_char {
  let cstring = CString::new("a string".to_string()).unwrap();
  let libc_char: *const libc::c_char = cstring.as_ptr();
  libc_char.convert_to_libc_char()
}

#[no_mangle]
pub extern fn i32_to_libc_char_test() -> *const libc::c_char {
  let x: i32 = 1;
  x.convert_to_libc_char()
}

#[no_mangle]
pub extern fn libc_char_to_libc_char_test() -> *const libc::c_char {
  let cstring = CString::new("a string".to_string()).unwrap();
  let libc_char: *const libc::c_char = cstring.as_ptr();
  libc_char.convert_to_libc_char()
}

