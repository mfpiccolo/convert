extern crate libc;

pub mod convert_libc_char;
pub mod convert_i32;
pub mod convert_string;
pub mod convert_str;

use std::ffi::{CStr, CString};
use std::borrow::Cow;
use std::sync::Mutex;

pub trait Stringify {
  fn convert_to_cow_str(&self) -> Cow<str>;
  fn convert_to_cstring(self) -> CString;
  fn convert_to_cstr(&self) -> &CStr;
  fn convert_to_str(&self) -> &str;
  fn convert_to_string(self) -> String;
  fn convert_to_libc_char(self) -> *const libc::c_char;
}
