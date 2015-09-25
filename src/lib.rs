extern crate libc;

pub mod stringify;

use stringify::Stringify;
mod tests;

#[no_mangle]
pub extern fn concat(s1: *const libc::c_char, s2: *const libc::c_char) -> *const libc::c_char {
  (s1.convert_to_string() + s2.convert_to_str()).convert_to_libc_char()
}

#[no_mangle]
pub extern fn sum_to_s(x: i32, y: i32) -> *const libc::c_char {
  (x + y).convert_to_libc_char()
}

#[no_mangle]
pub extern fn concat_nums(x: i32, y: i32) -> *const libc::c_char {
  (x.convert_to_string() + y.convert_to_str()).convert_to_libc_char()
}
