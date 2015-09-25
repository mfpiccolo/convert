use stringify::Stringify;
use std::ffi::{CStr, CString};
use std::borrow::Cow;

#[test]
fn i32_convert_to_cow_str_test() {
  let integer = 1;
  assert_eq!(integer.convert_to_cow_str(), Cow::Borrowed("1"));
}

#[test]
fn i32_convert_to_cstr_test() {
  let integer = 1;
  let cstr = unsafe { CStr::from_ptr(CString::new("1").unwrap().as_ptr()) };
  assert!(integer.convert_to_cstr() == cstr);
}

#[test]
fn i32_convert_to_str_test() {
  let integer = 1;
  assert_eq!(integer.convert_to_str(), "1");
}

#[test]
fn i32_convert_to_string_test() {
  let integer = 1;
  assert_eq!(integer.convert_to_string(), "1".to_string());
}

#[test]
fn i32_convert_to_libc_char_test() {
  let integer = 1;
  let libc_char1 = integer.convert_to_libc_char();
  let libc_char2 = CString::new("1".to_string()).unwrap().as_ptr();
  unsafe {
    assert_eq!(*libc_char1, *libc_char2);
  }
}
