require 'benchmark'
require "./convert"

n = 1
Benchmark.bm do |x|
  x.report("Rust#string_to_libc_char_test:") do
   for i in 1..n
      Convert.string_to_libc_char_test
   end
  end
  puts ""

  x.report("Rust#i32_to_libc_char_test:") do
   for i in 1..n
      Convert.i32_to_libc_char_test
   end
  end
  puts ""

  x.report("Rust#libc_char_to_libc_char_test:") do
   for i in 1..n
      Convert.libc_char_to_libc_char_test
   end
  end
end
