# Iterator Comparison

A small repo to test the runtime and memory footprint of various iteration
techniques in various languages.

## Beware

This are microbenchmarks and so should not be trusted. I'm not
confident in my ability to outsmart the compiler because
[it's hard](https://youtu.be/g0ek4vV7nEA).

## Setup

### Javascript

Install [node](https://nodejs.org/en/download/).

### Ruby

Install [ruby](https://www.ruby-lang.org/en/documentation/installation/).

### Rust

Install [rust](https://doc.rust-lang.org/book/ch01-01-installation.html).

## Running the benchmarks

### Javascript

The javascript benchmark can be run with

```
node --expose-gc iterators.js | grep -v Ignore
```

### Ruby

The ruby benchmark can be run with

```
ruby iterators.rb
```

### Rust

The rust benhcmark can be run with

```
cd iterators
cargo run --release | grep -v Ignore
```

## Current Results

### Javascript

```
┌─────────┬──────────────────────────────────────────────────────┬────────────────────────┐
│ (index) │                        title                         │      time in sec       │
├─────────┼──────────────────────────────────────────────────────┼────────────────────────┤
│    0    │ 'runtime of filter-map-filter with inline functions' │  0.003949884980916977  │
│    1    │      'runtime of reduce with inline functions'       │ 0.0022951160073280334  │
│    2    │     'runtime of for-loop with inline functions'      │ 0.00022856801748275758 │
│    3    │    'runtime of filter-map-filter with callbacks'     │ 0.0027249509990215303  │
│    4    │          'runtime of reduce with callbacks'          │ 0.0025348440110683442  │
│    5    │         'runtime of for-loop with callbacks'         │ 0.0002347480058670044  │
└─────────┴──────────────────────────────────────────────────────┴────────────────────────┘
┌─────────┬─────────────────────────────────────────────────────┬─────────────────┐
│ (index) │                        title                        │ bytes allocated │
├─────────┼─────────────────────────────────────────────────────┼─────────────────┤
│    0    │ 'weight of filter-map-filter with inline functions' │     1499304     │
│    1    │      'weight of reduce with inline functions'       │     384352      │
│    2    │     'weight of for-loop with inline functions'      │     383264      │
│    3    │    'weight of filter-map-filter with callbacks'     │     1497000     │
│    4    │          'weight of reduce with callbacks'          │     385232      │
│    5    │         'weight of for-loop with callbacks'         │     383344      │
└─────────┴─────────────────────────────────────────────────────┴─────────────────┘
```

### Ruby

```
Rehearsal ---------------------------------------------------------------------------
filter-map-filter with inline functions   0.010995   0.000203   0.011198 (  0.011236)
reduce with inline functions              0.011240   0.000078   0.011318 (  0.011359)
while loop with inline functions          0.006949   0.000064   0.007013 (  0.007024)
filter-map-filter with callbacks          0.014382   0.000354   0.014736 (  0.014853)
reduce with callbacks                     0.013826   0.000170   0.013996 (  0.014133)
while loop with callbacks                 0.010493   0.000129   0.010622 (  0.010687)
------------------------------------------------------------------ total: 0.068883sec

                                              user     system      total        real
filter-map-filter with inline functions   0.011477   0.000045   0.011522 (  0.011574)
reduce with inline functions              0.011226   0.000097   0.011323 (  0.011468)
while loop with inline functions          0.007436   0.000057   0.007493 (  0.007527)
filter-map-filter with callbacks          0.015420   0.000114   0.015534 (  0.015618)
reduce with callbacks                     0.014270   0.000056   0.014326 (  0.014399)
while loop with callbacks                 0.010829   0.000064   0.010893 (  0.010941)

Memory Usage (bytes):
filter-map-filter with inline functions 1333448
reduce with inline functions            134552
while loop with inline functions        134552
filter-map-filter with callbacks        1333448
reduce with callbacks                   134552
while loop with callbacks               134552
```

### Rust

```
Times (sec):
Multiple:           0.000124
Single:             0.000265
Loop:               0.000101
Multiple Inline:    0.000124
Single Inline:      0.000261
Loop Inline:        0.000101

Weights (bytes):
Multiple:           262120
Single:             262112
Loop:               262112
Multiple Inline:    262120
Single Inline:      262112
Loop Inline:        262112
```

## Notes

### Runtime

In ruby, most methods have similar runtime performance. The one exception is
the `while` loop with inlined logic. I don't totally understand this especially
because I would expect the `.filter.map.filter` methods to be slower than the
`.reduce` methods since they allocate 10x as much.

In javascript, we see a similar surprise as the `for` loop is 10x faster than
`.filter.map.filter` or `.reduce` which I also find surprising. I'd like to know
more about this!

In rust, we see some variance if we look at the higher precision
[`criterion`](https://docs.rs/criterion/0.3.3/criterion/) benchmarks that indicate
that `.fold` is slower than `.filter.map.filter` or the `for` loop. I would also
like to learn more about this.

### Memory

In javascript and ruby, far more memory is allocated when running `.filter.map.filter`
compared to `.reduce` or a manual loop. This is because each `.filter` and `.map`
invocation allocates a new array.

In ruby, `.select`
[allocates a new array](https://github.com/ruby/ruby/blob/v2_7_1/array.c#L3219) the
same size as the passed array (note that I used `.filter` which is an
[alias](https://github.com/ruby/ruby/blob/v2_7_1/array.c#L6984) of `.select`).

I imagine the same thing happens in NodeJS but I don't know where to look to prove that.

As for rust, iterators are lazy. `.filter` doesn't return a new vector, it returns a
[`Filter` struct](https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#725).
Nothing is allocated until the iterator is `collect`ed elements are returned one-by-one
and they aren't returned until they've been processed by every relevant iterator struct.
Therefore, we don't allocate space for elements that won't make it through all the
predicates.

### Summary

I'm a huge fan of hour rust works "best" when written "idiomatically". The `.filter.map.filter`
construct has sufficiently "optimal" runtime and memory costs so writing "readable" code
comes with (at least almost) no sacrifices. In ruby and javascript, however, one may need
to be careful of unnecessary allocations from intermediate results in iterators.

But again, in case you just skipped to this section, these microbenchmarks are:

1. microbenchmarks. Therefore they shouldn't be trusted. Very few microbenchmarks
   should be trusted.
2. probably written wrong. Outsmarting the compiler can be hard and these results
   may be completely biased based on how inputs were defined and what the
   runtime/interpreter/compiler optimized.

## Contributing

Feel free to contribute to any of the existing benchmarks and/or
add new languages/edge cases! Just fork the repo, add your work,
and make a PR into the master branch of this repo.
