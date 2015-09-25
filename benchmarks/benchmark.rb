require 'benchmark'
require "./convert"

n = 1
Benchmark.bm do |x|
  x.report("Rust#concat:") do
   for i in 1..n
      Convert.concat('toge', 'ther')
   end
  end
  x.report("Ruby#concat:") do
    for i in 1..n
      'toge'.concat('ther')
    end
  end
  puts ""

  x.report("Rust#sum_to_s:") do
   for i in 1..n
      Convert.sum_to_s(55, 45)
   end
  end
  x.report("Ruby#sum_to_s:") do
    for i in 1..n
      (55 + 45).to_s
    end
  end
  puts ""

  x.report("Rust#concat_nums:") do
   for i in 1..n
      Convert.concat_nums(55, 45)
   end
  end
  x.report("Ruby#concat_nums:") do
    for i in 1..n
      55.to_s + 45.to_s
    end
  end
end
