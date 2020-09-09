const { performance } = require("perf_hooks");

if (!global.gc) {
  throw new Error(
    "Need to execute Node with --expose-gc. Ex: `node --expose-gc iterators.js`"
  );
}

const filterMapFilterInline = (nums) =>
  nums
    .filter((n) => n % 3 == 0)
    .map((n) => n & (255 << 8))
    .filter((n) => n % 3 == 0);

const reduceInline = (nums) =>
  nums.reduce((result, n) => {
    if (n % 3 == 0) {
      const highBits = n & (255 << 8);
      if (highBits % 3 == 0) {
        result.push(highBits);
      }
    }
    return result;
  }, []);

const forLoopInline = (nums) => {
  const result = [];
  for (let i = 0; i < nums.length; i++) {
    n = nums[i];
    if (n % 3 == 0) {
      const highBits = n & (255 << 8);
      if (highBits % 3 == 0) {
        result.push(highBits);
      }
    }
  }
  return result;
};

const divisibleBy3 = (n) => n % 3 === 0;
const secondByte = (n) => n & (255 << 8);

const filterMapFilterCallback = (nums) =>
  nums.filter(divisibleBy3).map(secondByte).filter(divisibleBy3);

const reduceCallback = (nums) =>
  nums.reduce((result, n) => {
    if (divisibleBy3(n)) {
      const highBits = secondByte(n);
      if (divisibleBy3(highBits)) {
        result.push(highBits);
      }
    }
    return result;
  }, []);

const forLoopCallback = (nums) => {
  const result = [];
  for (let i = 0; i < nums.length; i++) {
    n = nums[i];
    if (divisibleBy3(n)) {
      const highBits = secondByte(n);
      if (divisibleBy3(highBits)) {
        result.push(highBits);
      }
    }
  }
  return result;
};

function testFunc(method) {
  const smallNums = [0, (3 << 8) | 3, (4 << 8) + 2, (3 << 8) + 1, (6 << 8) | 3];
  const result = method(smallNums);

  return (
    result.length === 3 &&
    result[0] === 0 &&
    result[1] === 3 << 8 &&
    result[2] === 6 << 8
  );
}

if (
  !testFunc(filterMapFilterInline) ||
  !testFunc(reduceInline) ||
  !testFunc(forLoopInline) ||
  !testFunc(filterMapFilterCallback) ||
  !testFunc(reduceCallback) ||
  !testFunc(forLoopCallback)
) {
  throw new Error("One of these is wrong!");
}

let nums = Array.from(Array(100000), (_, i) => i);

function timeMethod(method) {
  for (let i = 0; i < 1000; i++) {
    method(nums);
  }

  global.gc();
  let start = performance.now();
  const output = method(nums);
  let end = performance.now();
  console.log("Ignore this", output[0]);

  // Convert to sec
  return (end - start) / 1000;
}

function weighMethod(method) {
  global.gc();
  const heapBase = process.memoryUsage().heapUsed;
  const output = method(nums);
  const newHeap = process.memoryUsage().heapUsed;
  console.log("Ignore this", output[0]);

  return newHeap - heapBase;
}

const filterMapFilterInlineTime = timeMethod(filterMapFilterInline);
const reduceInlineTime = timeMethod(reduceInline);
const forLoopInlineTime = timeMethod(forLoopInline);

const filterMapFilterCallbackTime = timeMethod(filterMapFilterCallback);
const reduceCallbackTime = timeMethod(reduceCallback);
const forLoopCallbackTime = timeMethod(forLoopCallback);

const filterMapFilterInlineWeight = weighMethod(filterMapFilterInline);
const reduceInlineWeight = weighMethod(reduceInline);
const forLoopInlineWeight = weighMethod(forLoopInline);

const filterMapFilterCallbackWeight = weighMethod(filterMapFilterCallback);
const reduceCallbackWeight = weighMethod(reduceCallback);
const forLoopCallbackWeight = weighMethod(forLoopCallback);

console.table([
  {
    title: "runtime of filter-map-filter with inline functions",
    "time in sec": filterMapFilterInlineTime,
  },
  {
    title: "runtime of reduce with inline functions",
    "time in sec": reduceInlineTime,
  },
  {
    title: "runtime of for-loop with inline functions",
    "time in sec": forLoopInlineTime,
  },
  {
    title: "runtime of filter-map-filter with callbacks",
    "time in sec": filterMapFilterCallbackTime,
  },
  {
    title: "runtime of reduce with callbacks",
    "time in sec": reduceCallbackTime,
  },
  {
    title: "runtime of for-loop with callbacks",
    "time in sec": forLoopCallbackTime,
  },
]);

console.table([
  {
    title: "weight of filter-map-filter with inline functions",
    "bytes allocated": filterMapFilterInlineWeight,
  },
  {
    title: "weight of reduce with inline functions",
    "bytes allocated": reduceInlineWeight,
  },
  {
    title: "weight of for-loop with inline functions",
    "bytes allocated": forLoopInlineWeight,
  },
  {
    title: "weight of filter-map-filter with callbacks",
    "bytes allocated": filterMapFilterCallbackWeight,
  },
  {
    title: "weight of reduce with callbacks",
    "bytes allocated": reduceCallbackWeight,
  },
  {
    title: "weight of for-loop with callbacks",
    "bytes allocated": forLoopCallbackWeight,
  },
]);
