#[allow(unused_imports)]
use stringify::Stringify;
#[allow(unused_imports)]
use std::ffi::{CStr, CString};

#[test]
fn convert_to_cow_str_test() {
  let test_str = "something";
  assert_eq!(test_str.convert_to_str(), "something");
}

#[test]
fn convert_to_cstr_test() {
  let test_str = "something";
  let cstr = unsafe { CStr::from_ptr(CString::new("something").unwrap().as_ptr()) };
  assert_eq!(test_str.convert_to_cstr(), cstr);
}

#[test]
fn convert_to_str_test() {
  let test_str = "something";
  assert_eq!(test_str.convert_to_str(), "something");
}

#[test]
fn convert_to_string_test() {
  let test_str = "something";
  assert_eq!(test_str.convert_to_string(), "something".to_string());
}

#[test]
fn convert_to_libc_char_test() {
  let test_str = "something";
  let libc_char1 = test_str.convert_to_libc_char();
  let libc_char2 = CString::new("something".to_string()).unwrap().as_ptr();
  assert_eq!(libc_char1, libc_char2);
}
