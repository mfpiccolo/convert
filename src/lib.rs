extern crate libc;

pub mod stringify;

use stringify::Stringify;
mod tests;

#[no_mangle]
pub extern fn run(s: *const libc::c_char) -> *const libc::c_char {
  let changed_string = s.convert_to_string() + "slkjfd";
  changed_string.convert_to_libc_char()
}


