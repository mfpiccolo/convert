extern crate test;

use stringify::Stringify;
use std::ffi::{CStr, CString};
use std::borrow::Cow;
use self::test::Bencher;

#[bench]
fn convert_to_cow_str_bench(b: &mut Bencher) {
  let libc_char = CString::new("something".to_string()).unwrap().as_ptr();
  b.iter(|| libc_char.convert_to_cow_str());
}

#[bench]
fn convert_to_cstr_bench(b: &mut Bencher) {
  let libc_char = CString::new("something".to_string()).unwrap().as_ptr();
  b.iter(|| libc_char.convert_to_cstr());
}

#[bench]
fn convert_to_str_bench(b: &mut Bencher) {
  let libc_char = CString::new("something".to_string()).unwrap().as_ptr();
  b.iter(|| libc_char.convert_to_str());
}

#[bench]
fn convert_to_string_bench(b: &mut Bencher) {
  let libc_char = CString::new("something".to_string()).unwrap().as_ptr();
  b.iter(|| libc_char.convert_to_string());
}

#[bench]
fn convert_to_libc_char_bench(b: &mut Bencher) {
  let libc_char = CString::new("something".to_string()).unwrap().as_ptr();
  b.iter(|| libc_char.convert_to_libc_char());
}
