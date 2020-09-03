use iterators::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::time::Instant;

struct Counter;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), SeqCst);
        }
        return ret;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

#[global_allocator]
static A: Counter = Counter;

fn main() {
    let nums: Vec<u64> = (0..100_000).collect();

    let multiple_time = time_function(&nums, multiple_filters);
    let single_time = time_function(&nums, single_filter);
    let single_loop_time = time_function(&nums, single_loop_filter);

    println!(
        "Times:\nMultiple filters: {}s\nSingle filter: {}s\nSingle loop: {}s",
        multiple_time, single_time, single_loop_time
    );

    let multiple_weight = weigh_function(&nums, multiple_filters);
    let single_weight = weigh_function(&nums, single_filter);
    let single_loop_weight = weigh_function(&nums, single_loop_filter);

    println!(
        "\nWeights:\nMultiple filters: {}\nSingle filter: {}\nSingle loop: {}",
        multiple_weight, single_weight, single_loop_weight
    );
}

type Func = fn(args: &[u64]) -> Vec<u64>;
fn time_function(nums: &[u64], f: Func) -> f64 {
    for _ in 0..10 {
        let _ = f(nums);
    }
    let start = Instant::now();
    let output = f(nums);
    let end = start.elapsed().as_micros() as f64;

    println!("Ignore this {}", output[0]);

    // Convert to seconds
    end / 1_000_000.0
}

fn weigh_function(nums: &[u64], f: Func) -> usize {
    let start = ALLOCATED.load(SeqCst);
    let output = f(nums);
    let end = ALLOCATED.load(SeqCst);

    println!("Ignore this {}", output[0]);

    end - start
}
