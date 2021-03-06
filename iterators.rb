require 'benchmark'
require 'memory_profiler'

def filter_map_filter_inline(nums)
  nums
    .filter { |n| n % 3 == 0 }
    .map { |n| n & (255 << 8) }
    .filter { |n| n % 3 == 0 }
end

def reduce_inline(nums)
  nums.reduce([]) do |result, n|
    if n % 3 == 0
      high_bits = n & (255 << 8)
      if high_bits % 3 == 0
        result << high_bits
      end
    end

    result
  end
end

def while_loop_inline(nums)
  result = []
  idx = 0
  while idx < nums.length do
    n = nums[idx]
    if n % 3 == 0
      high_bits = n & (255 << 8)
      if high_bits % 3 == 0
        result << high_bits
      end
    end

    idx += 1
  end

  result
end

def divisible_by_3?(n)
  n % 3 == 0
end

def second_byte(n)
  n & (255 << 8)
end

def filter_map_filter_callback(nums)
  nums
    .filter { |n| divisible_by_3?(n) }
    .map { |n| second_byte(n) }
    .filter { |n| divisible_by_3?(n) }
end

def reduce_callback(nums)
  nums.reduce([]) do |result, n|
    if divisible_by_3?(n)
      high_bits = second_byte(n)
      if divisible_by_3?(high_bits)
        result << high_bits
      end
    end

    result
  end
end

def while_loop_callback(nums)
  result = []
  idx = 0
  while idx < nums.length do
    n = nums[idx]
    if divisible_by_3?(n)
      high_bits = second_byte(n)
      if divisible_by_3?(high_bits)
        result << high_bits
      end
    end

    idx += 1
  end

  result
end

small_nums = [0, (3 << 8) | 3, (4 << 8) + 2, (3 << 8) + 1, (6 << 8) | 3];
expected = [0, 3 << 8, 6 << 8]
unless  filter_map_filter_callback(small_nums) == expected  &&
     reduce_callback(small_nums) == expected  &&
     while_loop_callback(small_nums) == expected  &&
     filter_map_filter_inline(small_nums) == expected  &&
     reduce_inline(small_nums) == expected  &&
     while_loop_inline(small_nums) == expected
  puts "ONE OF THESE IS BROKEN"
  return
end

nums = Array.new(100_000) { |n| n }

Benchmark.bmbm do |x|
  x.report("filter-map-filter with inline functions") { filter_map_filter_inline(nums) }
  x.report("reduce with inline functions") { reduce_inline(nums) }
  x.report("while loop with inline functions") { while_loop_inline(nums) }
  x.report("filter-map-filter with callbacks") { filter_map_filter_callback(nums) }
  x.report("reduce with callbacks") { reduce_callback(nums) }
  x.report("while loop with callbacks") { while_loop_callback(nums) }
end

puts "\nMemory Usage (bytes):\n"

puts "filter-map-filter with inline functions".ljust(40) +
  "#{MemoryProfiler.report { filter_map_filter_inline(nums) }.total_allocated_memsize}"
puts "reduce with inline functions".ljust(40) +
  "#{MemoryProfiler.report { reduce_inline(nums) }.total_allocated_memsize}"
puts "while loop with inline functions".ljust(40) +
  "#{MemoryProfiler.report { while_loop_inline(nums) }.total_allocated_memsize}"
puts "filter-map-filter with callbacks".ljust(40) +
  "#{MemoryProfiler.report { filter_map_filter_callback(nums) }.total_allocated_memsize}"
puts "reduce with callbacks".ljust(40) +
  "#{MemoryProfiler.report { reduce_callback(nums) }.total_allocated_memsize}"
puts "while loop with callbacks".ljust(40) +
  "#{MemoryProfiler.report { while_loop_callback(nums) }.total_allocated_memsize}"
