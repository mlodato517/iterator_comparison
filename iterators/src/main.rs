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

    let multiple_time = time_function(&nums, filter_map_filter_callback);
    let single_time = time_function(&nums, fold_callback);
    let single_loop_time = time_function(&nums, for_loop_callback);
    let multiple_inline_time = time_function(&nums, filter_map_filter_inline);
    let single_inline_time = time_function(&nums, fold_inline);
    let single_loop_inline_time = time_function(&nums, for_loop_inline);
    let fold_custom_time = time_function(&nums, fold_custom);

    println!("Times (sec):");
    println!("{0: <20}{1:}", "Multiple:", multiple_time);
    println!("{0: <20}{1:}", "Single:", single_time);
    println!("{0: <20}{1:}", "Loop:", single_loop_time);
    println!("{0: <20}{1:}", "Multiple Inline:", multiple_inline_time);
    println!("{0: <20}{1:}", "Single Inline:", single_inline_time);
    println!("{0: <20}{1:}", "Loop Inline:", single_loop_inline_time);
    println!("{0: <20}{1:}", "Fold Custom:", fold_custom_time);

    let multiple_weight = weigh_function(&nums, filter_map_filter_callback);
    let single_weight = weigh_function(&nums, fold_callback);
    let single_loop_weight = weigh_function(&nums, for_loop_callback);
    let multiple_inline_weight = weigh_function(&nums, filter_map_filter_inline);
    let single_inline_weight = weigh_function(&nums, fold_inline);
    let single_loop_inline_weight = weigh_function(&nums, for_loop_inline);
    let fold_custom_weight = weigh_function(&nums, fold_custom);

    println!("\nWeights (bytes):");
    println!("{0: <20}{1:}", "Multiple:", multiple_weight);
    println!("{0: <20}{1:}", "Single:", single_weight);
    println!("{0: <20}{1:}", "Loop:", single_loop_weight);
    println!("{0: <20}{1:}", "Multiple Inline:", multiple_inline_weight);
    println!("{0: <20}{1:}", "Single Inline:", single_inline_weight);
    println!("{0: <20}{1:}", "Loop Inline:", single_loop_inline_weight);
    println!("{0: <20}{1:}", "Fold Custom:", fold_custom_weight);
}

type Func = fn(args: &[u64]) -> Vec<u64>;
fn time_function(nums: &[u64], f: Func) -> f64 {
    for _ in 0..1_000 {
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
