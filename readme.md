Welcome to convert for Rust
=======================


There are a lot of types in Rust!  This crate will help you perform common type conversions ease.  It will also be a place to encapsulate best practices for conversion.  That means that these conversions will always be fast and safe.

----------

Goals
-------------
> - Provide a concise API for all useful, difficult or tricky Rust type conversions.
> - Keep get conversions as fast as possible (i.e. zero cost abstraction).
> - Ensure memory safety.
> - Document and maintain best practices for type conversion.

Use
-------------

The `convert!()` macro is the only external api for this library.  It is that simple.

```rust
extern crate libc;
extern crate convert;

main {
  convert!(1 => String); // returns a String
  convert!(2 => CString); // returns a CString
  convert!(3 => *const libc::c_char); // returns a *const libc::c_char
  
  convert!("String1".to_string() => *const libc::c_char); // returns a *const libc::c_char
  convert!("String2".to_string() => CString); // returns a CString

  let libc_char1 = CString::new("c_char1").unwrap().as_ptr();
  convert!(libc_char1 => String); // returns a String

  let libc_char2 = CString::new("c_char2").unwrap().as_ptr();
  convert!(libc_char2 => CString); // returns a CString
}
```
Implemented Conversions
-------------
| From Type   | To Type       | Supported |Status       |
| :-------:   | :----:        | :---:     | :---:       |
| `String`    | `String`      |  √        | unstable    |
| `String`    | `CString`     |  √        | unstable    |
| `String`    | `libc::char`  |  √        | unstable    |
| `String`    | `&'static str`|  X        | not working |
| `String`    | `Cow<str`>    |  √        | unstable    |
| `String`    | `CStr`        |  X        | not working |
| `i32`       | `String`      |  √        | unstable    |
| `i32`       | `CString`     |  √        | unstable    |
| `i32`       | `libc::char`  |  √        | unstable    |
| `i32`       | `&'static str`|  X        | not working |
| `i32`       | `Cow<str`>    |  √        | unstable    |
| `i32`       | `CStr`        |  X        | not working |
| `&str`      | `String`      |  √        | unstable    |
| `&str`      | `CString`     |  √        | unstable    |
| `&str`      | `libc::char`  |  √        | unstable    |
| `&str`      | `&'static str`|  √        | stable      |
| `&str`      | `Cow<str`>    |  √        | unstable    |
| `&str`      | `CStr`        |  X        | not working |
| `libc::char`| `String`      |  √        | unstable    |
| `libc::char`| `CString`     |  √        | unstable    |
| `libc::char`| `libc::char`  |  √        | unstable    |
| `libc::char`| `&'static str`|  X        | not working |
| `libc::char`| `Cow<str>`    |  √        | unstable    |
| `libc::char`| `CStr`        |  X        | not working |

Future Conversions
-------------
| From Type   | To Type     | Supported |Status           |
| :-------:   | :----:      | :---:     | :---:           |     
| `String`    | `i32 (ect.)`|  X        | not implemented |
| `vec<_>`    | `[T; _]`    |  X        | not implemented |
| ?           | ?           |  X        | not implemented |

Contributing
-------------
#### Feature Requests
If you have a conversion that you would like to add to the repo open up an issue first to make it fits into the scope of this library.  If myself and/or other Rustaceans agree then you should implement it and open up a pull request.
#### Bugs/Memeory Safety
If you find a memeory safety problem or any other bug please open up an issue.  If you are willing and able then you can open up a pull request with a fix.
#### Performance
If you can implement a conversion faster then the current benchmarks and all the tests are still passing then open up a pull request.  Faster is better!

#### Tests
All tests must pass for any pull request to be merged in.  This mean all the following commands must pass.
```
cargo build
cargo test
ruby tests/convert_test.rb
```

> Note:  Working on setting up CI

#### Benchmarks
For a pull request, not concerning safety, to be merged the benchmarks must be at least as fast as the existing benchmark.
 
Current Benchmarks
-------------
#### Rust Benchmarks
```
test benches::convert_i32_bench::convert_to_cow_str_bench         ... bench:          76 ns/iter (+/- 23)
test benches::convert_i32_bench::convert_to_cstr_bench            ... bench:         147 ns/iter (+/- 65)
test benches::convert_i32_bench::convert_to_libc_char_bench       ... bench:         190 ns/iter (+/- 43)
test benches::convert_i32_bench::convert_to_str_bench             ... bench:          80 ns/iter (+/- 32)
test benches::convert_i32_bench::convert_to_string_bench          ... bench:          83 ns/iter (+/- 16)
test benches::convert_libc_char_bench::convert_to_cow_str_bench   ... bench:          14 ns/iter (+/- 4)
test benches::convert_libc_char_bench::convert_to_cstr_bench      ... bench:           3 ns/iter (+/- 1)
test benches::convert_libc_char_bench::convert_to_libc_char_bench ... bench:           0 ns/iter (+/- 0)
test benches::convert_libc_char_bench::convert_to_str_bench       ... bench:          13 ns/iter (+/- 2)
test benches::convert_libc_char_bench::convert_to_string_bench    ... bench:          95 ns/iter (+/- 22)
test benches::convert_string_bench::convert_to_cow_str_bench      ... bench:           0 ns/iter (+/- 1)
test benches::convert_string_bench::convert_to_cstr_bench         ... bench:         112 ns/iter (+/- 30)
test benches::convert_string_bench::convert_to_libc_char_bench    ... bench:         174 ns/iter (+/- 50)
test benches::convert_string_bench::convert_to_str_bench          ... bench:           0 ns/iter (+/- 0)
test benches::convert_string_bench::convert_to_string_bench       ... bench:          95 ns/iter (+/- 17)
```

#### Ruby Benchmarks
```
Rust#string_to_libc_char_test:     0.000000   0.000000   0.000000 (  0.000045)

Rust#i32_to_libc_char_test:        0.000000   0.000000   0.000000 (  0.000017)
```
Rust#libc_char_to_libc_char_test:  0.000000   0.000000   0.000000 (  0.000008)
```
