use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
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
    for &(case, value) in CASES {
        let mut group = c.benchmark_group(format!("contains/{case}"));

        let interval = I8CO::try_new(START, END_EXCL).unwrap();
        group.bench_function("int_interval", |b| {
            b.iter(|| black_box(interval).contains(black_box(value)))
        });

        let interval = Interval::new_closed_open(START, END_EXCL);
        group.bench_function("rust_intervals", |b| {
            b.iter(|| black_box(&interval).contains(black_box(value)))
        });

        let interval = START..END_EXCL;
        group.bench_function("std_range", |b| {
            b.iter(|| black_box(&interval).contains(&black_box(value)))
        });

        group.finish();
    }
}

criterion_group!(benches, bench_contains);
criterion_main!(benches);
