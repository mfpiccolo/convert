extern crate libc;

use std;
use stringify::Stringify;
use std::ffi::{CStr, CString};
use std::borrow::Cow;

impl Stringify for String {
  fn convert_to_cstring(&self) -> CString {
    CString::new(self.convert_to_str()).unwrap()
  }

  fn convert_to_cstr(&self) -> &CStr {
    let str = self.convert_to_str();
    unsafe { CStr::from_ptr(CString::new(str).unwrap().as_ptr()) }
  }

  fn convert_to_cow_str(&self) -> Cow<str> {
    Cow::Borrowed(self.convert_to_str())
  }

  fn convert_to_str(&self) -> &str {
    self as &str
  }

  fn convert_to_string(&self) -> String {
    self.to_string()
  }

  fn convert_to_libc_char(&self) -> *const libc::c_char {
    let cstring = CString::new(self.to_string()).unwrap();
    let c_ptr: *const libc::c_char = cstring.as_ptr();
    c_ptr
  }
}
