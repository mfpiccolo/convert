#[macro_use]
mod ConvertMacro {
  use stringify::Stringify;
  macro_rules! convert {
    ($from:expr => String) => ($from.convert_to_string() as String);
    ($from:expr => &'static str) => ($from.convert_to_str() as &'static str);
    ($from:expr => CString) => ($from.convert_to_cstring() as CString);
    ($from:expr => *const libc::c_char) => ($from.convert_to_libc_char() as *const libc::c_char);
    ($from:expr => Cow<str>) => ($from.convert_to_cow_str() as Cow<str>);
  }
}
