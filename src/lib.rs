extern crate libc;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern fn run(s: *const libc::c_char) -> *const libc::c_char {
  let changed_string = s.convert_to_string() + "slkjfd";
  changed_string.convert_to_libc_char()
}

trait Stringify {
  fn convert_to_str(&self) -> &str;
  fn convert_to_string(&self) -> String;
  fn convert_to_libc_char(&self) -> *const libc::c_char;
}

impl Stringify for *const libc::c_char {
  fn convert_to_str(&self) -> &str {
    let cstr = unsafe { CStr::from_ptr(*self) };
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

impl Stringify for String {
  fn convert_to_str(&self) -> &str {
    std::str::from_utf8(self.as_bytes()).unwrap()
  }

  fn convert_to_string(&self) -> String {
    self.to_string()
  }

  fn convert_to_libc_char(&self) -> *const libc::c_char {
    CString::new(self.to_string()).unwrap().as_ptr()
  }
}

#[test]
fn libc_char_convert_to_str_test() {
  let libc_char = CString::new("something".to_string()).unwrap().as_ptr();
  assert_eq!(libc_char.convert_to_str(), "something");
}

#[test]
fn libc_char_convert_to_string_test() {
  let libc_char = CString::new("something".to_string()).unwrap().as_ptr();
  assert_eq!(libc_char.convert_to_string(), "something".to_string());
}

#[test]
fn libc_char_convert_to_libc_char_test() {
  let libc_char1 = CString::new("something".to_string()).unwrap().as_ptr();
  let libc_char2 = CString::new("something".to_string()).unwrap().as_ptr();
  assert_eq!(libc_char1.convert_to_libc_char(), libc_char2);
}

#[test]
fn string_convert_to_str_test() {
  let string = "something".to_string();
  assert_eq!(string.convert_to_str(), "something");
}

#[test]
fn string_convert_to_string_test() {
  let string = "something".to_string();
  assert_eq!(string.convert_to_string(), "something".to_string());
}

#[test]
fn string_convert_to_libc_char_test() {
  let libc_char1 = CString::new("something".to_string()).unwrap().as_ptr();
  let libc_char2 = CString::new("something".to_string()).unwrap().as_ptr();
  assert_eq!(libc_char1.convert_to_libc_char(), libc_char2);
}
