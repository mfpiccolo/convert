#![feature(test)]
extern crate libc;

pub mod stringify;

use stringify::Stringify;
use std::ffi::{CStr, CString};

mod tests;
mod benches;
pub mod external_function_tests;

#[no_mangle]
pub extern fn no_op() {}
