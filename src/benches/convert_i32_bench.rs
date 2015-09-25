extern crate test;

use stringify::Stringify;
use std::ffi::{CStr, CString};
use std::borrow::Cow;
use self::test::Bencher;

#[bench]
fn convert_to_cow_str_bench(b: &mut Bencher) {
  let i32var: i32 = 1;
  b.iter(|| i32var.convert_to_cow_str());
}

#[bench]
fn convert_to_cstr_bench(b: &mut Bencher) {
  let i32var: i32 = 1;
  b.iter(|| i32var.convert_to_cstr());
}

#[bench]
fn convert_to_str_bench(b: &mut Bencher) {
  let i32var: i32 = 1;
  b.iter(|| i32var.convert_to_str());
}

#[bench]
fn convert_to_string_bench(b: &mut Bencher) {
  let i32var: i32 = 1;
  b.iter(|| i32var.convert_to_string());
}

#[bench]
fn convert_to_libc_char_bench(b: &mut Bencher) {
  let i32var: i32 = 1;
  b.iter(|| i32var.convert_to_libc_char());
}
