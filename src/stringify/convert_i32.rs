extern crate libc;

use std;
use stringify::Stringify;
use std::ffi::{CStr, CString};
use std::borrow::Cow;
use std::mem;

impl Stringify for i32 {
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
    // TODO: figure out why this won't work without unsafe
    unsafe {
      let ret = mem::transmute(&self.to_string() as &str);
      mem::forget(self);
      ret
    }
  }

  fn convert_to_string(self) -> String {
    self.to_string()
  }

  fn convert_to_libc_char(self) -> *const libc::c_char {
    // TODO why does the convert_to_libc_char() not work here?
    // self.to_string().convert_to_libc_char()
    let cstring = CString::new(self.to_string()).unwrap();
    let c_ptr: *const libc::c_char = cstring.as_ptr();
    c_ptr
  }
}
