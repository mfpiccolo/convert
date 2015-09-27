extern crate libc;

use stringify::Stringify;

#[no_mangle]
pub extern fn str_to_libc_char_test() -> *const libc::c_char {
  "a string".convert_to_libc_char()
}
