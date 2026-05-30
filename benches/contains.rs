use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const START: i8 = -32;
const END_EXCL: i8 = 96;

const CASES: &[(&str, i8)] = &[
    ("hit_start", START),
    ("hit_middle", 16),
    ("hit_end_incl", END_EXCL - 1),
    ("miss_before", START - 1),
    ("miss_end_excl", END_EXCL),
];

fn bench_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains");

    for &(case, value) in CASES {
        let interval = I8CO::try_new(START, END_EXCL).unwrap();
        group.bench_function(BenchmarkId::new("int_interval", case), |b| {
            b.iter(|| black_box(interval).contains(black_box(value)))
        });

        let interval = Interval::new_closed_open(START, END_EXCL);
        group.bench_function(BenchmarkId::new("rust_intervals", case), |b| {
            b.iter(|| black_box(&interval).contains(black_box(value)))
        });

        let interval = START..END_EXCL;
        group.bench_function(BenchmarkId::new("std_range", case), |b| {
            b.iter(|| black_box(&interval).contains(&black_box(value)))
        });
    }

    group.finish();
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_contains
}

criterion_main!(benches);
