use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iterators::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let nums = black_box((0..100_000).collect::<Vec<u64>>());

    let mut group = c.benchmark_group("Iterators");
    group.bench_function("filter-map-filter with inline logic", |b| {
        b.iter(|| filter_map_filter_inline(&nums))
    });
    group.bench_function("filter-map-filter with callbacks", |b| {
        b.iter(|| filter_map_filter_callback(&nums))
    });
    group.bench_function("fold with inline logic", |b| b.iter(|| fold_inline(&nums)));
    group.bench_function("fold with callbacks", |b| b.iter(|| fold_callback(&nums)));
    group.bench_function("for loop with inline logic", |b| {
        b.iter(|| for_loop_inline(&nums))
    });
    group.bench_function("for loop with callbacks", |b| {
        b.iter(|| for_loop_callback(&nums))
    });
    group.bench_function("fold custom", |b| b.iter(|| fold_custom(&nums)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
