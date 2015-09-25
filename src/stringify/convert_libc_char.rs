extern crate libc;

use std;
use stringify::Stringify;
use std::ffi::{CStr, CString};
use std::borrow::Cow;

impl Stringify for *const libc::c_char {
  fn convert_to_cstring(&self) -> CString {
    CString::new(self.convert_to_str()).unwrap()
  }

  fn convert_to_cstr(&self) -> &CStr {
    unsafe { CStr::from_ptr(*self) }
  }

  fn convert_to_cow_str(&self) -> Cow<str> {
    Cow::Borrowed(self.convert_to_str())
  }

  fn convert_to_str(&self) -> &str {
    let cstr = self.convert_to_cstr();
    std::str::from_utf8(cstr.to_bytes()).unwrap()
  }

  fn convert_to_string(&self) -> String {
    let cstr = unsafe { CStr::from_ptr(*self) };
    std::str::from_utf8(cstr.to_bytes()).unwrap().to_string()
  }

  fn convert_to_libc_char(&self) -> *const libc::c_char {
    *self
  }
}
