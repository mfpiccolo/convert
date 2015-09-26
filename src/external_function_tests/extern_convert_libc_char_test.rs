extern crate libc;

use stringify::Stringify;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern fn libc_char_to_libc_char_test() -> *const libc::c_char {
  let cstring = CString::new("a string".to_string()).unwrap();
  let libc_char: *const libc::c_char = cstring.as_ptr();
  libc_char.convert_to_libc_char()
}
