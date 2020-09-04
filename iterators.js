const { performance } = require("perf_hooks");

function multipleFiltersInline(nums) {
  return nums
    .filter((n) => n % 3 === 0)
    .filter((n) => n % 5 === 0)
    .filter((n) => n % 7 === 0)
    .filter((n) => n % 11 === 0);
}

const singleFilterInline = (nums) =>
  nums.filter((n) => n % 3 === 0 && n % 5 === 0 && n % 7 === 0 && n % 11 === 0);

function singleLoopFilterInline(nums) {
  let returnValue = [];
  for (let i = 0; i < nums.length; i++) {
    const n = nums[i];
    if (n % 3 === 0 && n % 5 === 0 && n % 7 === 0 && n % 11 === 0) {
      returnValue.push(n);
    }
  }
  return returnValue;
}

const divisibleBy3 = (n) => n % 3 === 0;
const divisibleBy5 = (n) => n % 5 === 0;
const divisibleBy7 = (n) => n % 7 === 0;
const divisibleBy11 = (n) => n % 11 === 0;
const divisibleBy1155 = (n) =>
  divisibleBy3(n) && divisibleBy5(n) && divisibleBy7(n) && divisibleBy11(n);

function multipleFilters(nums) {
  return nums
    .filter(divisibleBy3)
    .filter(divisibleBy5)
    .filter(divisibleBy7)
    .filter(divisibleBy11);
}

const singleFilter = (nums) => nums.filter(divisibleBy1155);

function singleLoopFilter(nums) {
  let returnValue = [];
  for (let i = 0; i < nums.length; i++) {
    if (divisibleBy1155(nums[i])) {
      returnValue.push(nums[i]);
    }
  }
  return returnValue;
}

function testFunc(method) {
  const smallNums = Array.from(Array(3000), (_, i) => i + 1);
  const result = method(smallNums);
  return result.length === 2 && result[0] === 1155 && result[1] === 2310;
}

if (
  !testFunc(multipleFilters) ||
  !testFunc(singleFilter) ||
  !testFunc(singleLoopFilter) ||
  !testFunc(multipleFiltersInline) ||
  !testFunc(singleFilterInline) ||
  !testFunc(singleLoopFilterInline)
) {
  throw new Error("One of these is wrong!");
}

let nums = Array.from(Array(100000), (_, i) => i + 1);

function timeMethod(method) {
  for (let i = 0; i < 1000; i++) {
    method(nums);
  }

  let start = performance.now();
  const output = method(nums);
  let end = performance.now();
  console.log("Ignore this", output[0]);

  // Convert to sec
  return (end - start) / 1000;
}

function weighMethod(method) {
  const heapBase = process.memoryUsage().heapUsed;
  const output = method(nums);
  const newHeap = process.memoryUsage().heapUsed;
  console.log("Ignore this", output[0]);

  return newHeap - heapBase;
}

console.log("Times (sec):\n");
console.log({
  multipleTime: timeMethod(multipleFilters),
  singleTime: timeMethod(singleFilter),
  singleLoopTime: timeMethod(singleLoopFilter),
  multipleInlineTime: timeMethod(multipleFiltersInline),
  singleInlineTime: timeMethod(singleFilterInline),
  singleLoopInlineTime: timeMethod(singleLoopFilterInline),
});

console.log("\nWeights (bytes):\n");
console.log({
  multipleWeight: weighMethod(multipleFilters),
  singleWeight: weighMethod(singleFilter),
  singleLoopWeight: weighMethod(singleLoopFilter),
  multipleInlineWeight: weighMethod(multipleFiltersInline),
  singleInlineWeight: weighMethod(singleFilterInline),
  singleLoopInlineWeight: weighMethod(singleLoopFilterInline),
});
