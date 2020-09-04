require 'benchmark'
require 'memory_profiler'

def multiple_filters_inline(nums)
  nums
    .filter { |n| n % 3 == 0 }
    .filter { |n| n % 5 == 0 }
    .filter { |n| n % 7 == 0 }
    .filter { |n| n % 11 == 0 }
end

def single_filter_inline(nums)
  nums.filter do |n|
    n % 3 == 0 && n % 5 == 0 && n % 7 == 0 && n % 11 == 0
  end
end

def single_loop_filter_inline(nums)
  return_value = []
  idx = 0
  while idx < nums.length do
    n = nums[idx]
    if n % 3 == 0 && n % 5 == 0 && n % 7 == 0 && n % 11 == 0
      return_value << n
    end
    idx += 1
  end

  return_value
end

def divisible_by_3?(n)
  n % 3 == 0
end

def divisible_by_5?(n)
  n % 5 == 0
end

def divisible_by_7?(n)
  n % 7 == 0
end

def divisible_by_11?(n)
  n % 11 == 0
end

def multiple_filters(nums)
  nums
    .filter { |n| divisible_by_3?(n) }
    .filter { |n| divisible_by_5?(n) }
    .filter { |n| divisible_by_7?(n) }
    .filter { |n| divisible_by_11?(n) }
end

def single_filter(nums)
  nums.filter do |n|
    divisible_by_3?(n) &&
      divisible_by_5?(n) &&
      divisible_by_7?(n) &&
      divisible_by_11?(n)
  end
end

def single_loop_filter(nums)
  return_value = []
  idx = 0
  while idx < nums.length do
    n = nums[idx]
    if divisible_by_3?(n) &&
      divisible_by_5?(n) &&
      divisible_by_7?(n) &&
      divisible_by_11?(n)
      return_value << n
    end
    idx += 1
  end

  return_value
end

small_nums = Array.new(3_000) { |n| n + 1 }
expected = [1155, 2310]
unless  multiple_filters(small_nums) == expected  &&
     single_filter(small_nums) == expected  &&
     single_loop_filter(small_nums) == expected  &&
     multiple_filters_inline(small_nums) == expected  &&
     single_filter_inline(small_nums) == expected  &&
     single_loop_filter_inline(small_nums) == expected
  puts "ONE OF THESE IS BROKEN"
  return
end

nums = Array.new(100_000) { |n| n + 1 }

Benchmark.bmbm do |x|
  x.report("multiple filters") { multiple_filters(nums) }
  x.report("single filter") { single_filter(nums) }
  x.report("single loop filter") { single_loop_filter(nums) }
  x.report("multiple filters inline") { multiple_filters_inline(nums) }
  x.report("single filter inline") { single_filter_inline(nums) }
  x.report("single loop filter inline") { single_loop_filter_inline(nums) }
end

puts "\nMemory Usage (bytes):\n"

puts "Multiple filter allocation:".ljust(40) +
  "#{MemoryProfiler.report { multiple_filters(nums) }.total_allocated_memsize}"
puts "Single filter allocation:".ljust(40) +
  "#{MemoryProfiler.report { single_filter(nums) }.total_allocated_memsize}"
puts "Single Loop filter allocation:".ljust(40) +
  "#{MemoryProfiler.report { single_loop_filter(nums) }.total_allocated_memsize}"
puts "Multiple filter inline allocation:".ljust(40) +
  "#{MemoryProfiler.report { multiple_filters_inline(nums) }.total_allocated_memsize}"
puts "Single filter inline allocation:".ljust(40) +
  "#{MemoryProfiler.report { single_filter_inline(nums) }.total_allocated_memsize}"
puts "Single Loop filter inline allocation:".ljust(40) +
  "#{MemoryProfiler.report { single_loop_filter_inline(nums) }.total_allocated_memsize}"
