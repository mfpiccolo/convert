extern crate libc;

use std;
use stringify::Stringify;
use std::ffi::{CStr, CString};
use std::borrow::Cow;
use std::mem;

impl Stringify for CString {
  fn convert_to_cstring(self) -> CString {
    CString::new(self.convert_to_str()).unwrap()
  }

  fn convert_to_cstr(&self) -> &CStr {
    let cstring = self.convert_to_cstring();
    unsafe { CStr::from_ptr(cstring.as_ptr()) }
  }

  fn convert_to_cow_str(&self) -> Cow<str> {
    Cow::Borrowed(self.convert_to_str())
  }

  fn convert_to_str(&self) -> &str {
    let buf: &[u8] = self.to_owned().as_bytes();
    std::str::from_utf8(buf).unwrap()
  }

  fn convert_to_string(self) -> String {
    self.convert_to_str().to_string()
  }

  fn convert_to_libc_char(self) -> *const libc::c_char {
    self.as_ptr()
  }
}
