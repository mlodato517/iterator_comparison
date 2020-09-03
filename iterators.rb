require 'benchmark'
require 'memory_profiler'

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

nums = Array.new(100_000) { |n| n }

def multiple_filters(nums)
  nums
    .filter { |n| divisible_by_3?(n) }
    .filter { |n| divisible_by_5?(n) }
    .filter { |n| divisible_by_7?(n) }
    .filter { |n| divisible_by_11?(n) }
end

def single_filter(nums)
  nums.filter { |n| divisible_by_1155?(n) }
end

def divisible_by_1155?(n)
  divisible_by_3?(n) &&
    divisible_by_5?(n) &&
    divisible_by_7?(n) &&
    divisible_by_11?(n)
end

def single_loop_filter(nums)
  return_value = []
  idx = 0
  while idx < nums.length do
    return_value << nums[idx] if divisible_by_1155?(nums[idx])
    idx += 1
  end

  return_value
end

puts multiple_filters([11 * 7 * 5 * 3, 1])
puts single_filter([11 * 7 * 5 * 3, 1])
puts single_loop_filter([11 * 7 * 5 * 3, 1])
puts multiple_filters(nums) == single_filter(nums) && multiple_filters(nums) == single_loop_filter(nums)

Benchmark.bmbm do |x|
  x.report("multiple filters") { multiple_filters(nums) }
  x.report("single filter") { single_filter(nums) }
  x.report("single loop filter") { single_loop_filter(nums) }
end

puts "\nMemory Usage:\n"

report = MemoryProfiler.report { multiple_filters(nums) }
report.pretty_print(detailed_report: false)

report = MemoryProfiler.report { single_filter(nums) }
report.pretty_print(detailed_report: false)

report = MemoryProfiler.report { single_loop_filter(nums) }
report.pretty_print(detailed_report: false)
