extern crate libc;

use stringify::Stringify;

#[no_mangle]
pub extern fn string_to_libc_char_test() -> *const libc::c_char {
  String::from("string").convert_to_libc_char()
}
