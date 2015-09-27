#![feature(test)]
extern crate libc;

pub mod stringify;
#[macro_use]
pub mod convert_macro;

use stringify::Stringify;
use std::ffi::{CStr, CString};
use std::borrow::Cow;

mod tests;
mod benches;
pub mod external_function_tests;

#[no_mangle]
pub extern fn no_op() {}

#[no_mangle]
pub extern fn test() {
  let x = convert!(34 => String) as String;
  println!("{:?}", x);
  let y = convert!(34 => CString) as CString;
  println!("{:?}", y);
  // let q = convert!(34 => *const libc::c_char) as *const libc::c_char;
  // println!("{:?}", q);
}
