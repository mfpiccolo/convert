#[macro_use]
mod ConvertMacro {
  use stringify::Stringify;
  macro_rules! convert {
    ($from:expr => String) => ($from.convert_to_string());
    ($from:expr => str) => ($from.convert_to_str());
    ($from:expr => CString) => ($from.convert_to_cstring());
    ($from:expr => *const libc::c_char) => ($from.convert_to_libc_char());
    ($from:expr => Cow<str>) => ($from.convert_to_cow_str());
  }
}
