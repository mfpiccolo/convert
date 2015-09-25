#[allow(unused_imports)]
use stringify::Stringify;
#[allow(unused_imports)]
use std::ffi::{CStr, CString};
#[allow(unused_imports)]
use std::borrow::Cow;

#[test]
fn convert_to_cow_str_test() {
  let libc_char = CString::new("something".to_string()).unwrap().as_ptr();
  let cow_str = Cow::Borrowed(libc_char.convert_to_str());
  assert_eq!(libc_char.convert_to_cow_str(), cow_str);
}

#[test]
fn convert_to_cstr_test() {
  let libc_char = CString::new("something".to_string()).unwrap().as_ptr();
  let cstr = unsafe { CStr::from_ptr(CString::new(libc_char.convert_to_str()).unwrap().as_ptr()) };
  assert_eq!(libc_char.convert_to_cstr(), cstr);
}

#[test]
fn convert_to_str_test() {
  let libc_char = CString::new("something".to_string()).unwrap().as_ptr();
  assert_eq!(libc_char.convert_to_str(), "something");
}

#[test]
fn convert_to_string_test() {
  let libc_char = CString::new("something".to_string()).unwrap().as_ptr();
  assert_eq!(libc_char.convert_to_string(), "something".to_string());
}

#[test]
fn convert_to_libc_char_test() {
  let libc_char1 = CString::new("something".to_string()).unwrap().as_ptr();
  let libc_char2 = CString::new("something".to_string()).unwrap().as_ptr();
  assert_eq!(libc_char1.convert_to_libc_char(), libc_char2);
}
