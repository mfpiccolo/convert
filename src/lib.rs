#![feature(test)]
extern crate libc;

pub mod stringify;

use stringify::Stringify;
use std::ffi::{CStr, CString};
use std::borrow::Cow;

mod tests;
mod benches;
pub mod external_function_tests;

#[no_mangle]
pub extern fn no_op() {}

macro_rules! convert {
  ($from:expr => String) => ($from.convert_to_string());
  ($from:expr => str) => ($from.convert_to_str());
  ($from:expr => CString) => ($from.convert_to_cstring());
  ($from:expr => *const libc::c_char) => ($from.convert_to_libc_char());
  ($from:expr => Cow<str>) => ($from.convert_to_cow_str());
}

#[no_mangle]
pub extern fn test() {
  let x = convert!(34 => String) as String;
  println!("{:?}", x);
  let y = convert!(34 => CString) as CString;
  println!("{:?}", y);
  let q = convert!(34 => *const libc::c_char) as *const libc::c_char;
  println!("{:?}", q);
}
