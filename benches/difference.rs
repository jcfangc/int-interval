use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const BASE: (i8, i8) = (-32, 96);

const CASES: &[(&str, (i8, i8))] = &[
    // No removal: result remains BASE.
    ("disjoint_left", (-96, -64)),
    ("disjoint_right", (112, 127)),
    ("adjacent_left", (-64, -32)),
    ("adjacent_right", (96, 112)),
    // Full removal: result is empty.
    ("equal", (-32, 96)),
    ("covered_by_other", (-64, 112)),
    // One remaining segment.
    ("remove_left_edge", (-32, 32)),
    ("remove_right_edge", (32, 96)),
    ("overlap_left", (-64, 0)),
    ("overlap_right", (32, 112)),
    // Two remaining segments.
    ("remove_middle", (-16, 32)),
];

fn bench_difference(c: &mut Criterion) {
    for &(case, other) in CASES {
        let mut group = c.benchmark_group(format!("difference/{case}"));

        let lhs = I8CO::try_new(BASE.0, BASE.1).unwrap();
        let rhs = I8CO::try_new(other.0, other.1).unwrap();

        group.bench_function("int_interval", |b| {
            b.iter(|| black_box(lhs).difference(black_box(rhs)))
        });

        let lhs = Interval::new_closed_open(BASE.0, BASE.1);
        let rhs = Interval::new_closed_open(other.0, other.1);

        group.bench_function("rust_intervals", |b| {
            b.iter(|| black_box(lhs).difference(black_box(rhs)))
        });

        group.finish();
    }
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_difference
}

criterion_main!(benches);
