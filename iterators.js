const { performance } = require("perf_hooks");

let nums = Array.from(Array(100000), (_, i) => i);

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

console.log(multipleFilters([11 * 7 * 5 * 3, 1]));
console.log(singleFilter([11 * 7 * 5 * 3, 1]));
console.log(singleLoopFilter([11 * 7 * 5 * 3, 1]));

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

console.log({
  multipleTime: timeMethod(multipleFilters),
  singleTime: timeMethod(singleFilter),
  singleLoopTime: timeMethod(singleLoopFilter),
  multipleWeight: weighMethod(multipleFilters),
  singleWeight: weighMethod(singleFilter),
  singleLoopWeight: weighMethod(singleLoopFilter),
});
