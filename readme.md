Current Benchmarks

### Rust Benchmarks
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

### Ruby Benchmarks
Rust#string_to_libc_char_test:     0.000000   0.000000   0.000000 (  0.000045)

Rust#i32_to_libc_char_test:        0.000000   0.000000   0.000000 (  0.000017)

Rust#libc_char_to_libc_char_test:  0.000000   0.000000   0.000000 (  0.000008)
