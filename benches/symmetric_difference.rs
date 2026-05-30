use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const BASE: (i8, i8) = (-32, 96);

const CASES: &[(&str, (i8, i8))] = &[
    ("equal", (-32, 96)),
    ("same_left", (-32, 32)),
    ("same_right", (32, 96)),
    ("contained_strict", (-16, 32)),
    ("contains_base", (-64, 112)),
    ("overlap_left", (-64, 0)),
    ("overlap_right", (32, 112)),
    ("adjacent_left", (-64, -32)),
    ("adjacent_right", (96, 112)),
    ("disjoint_left", (-96, -64)),
    ("disjoint_right", (112, 127)),
];

fn bench_symmetric_difference(c: &mut Criterion) {
    let mut group = c.benchmark_group("symmetric_difference");

    for &(case, other) in CASES {
        group.bench_function(BenchmarkId::new("int_interval", case), |b| {
            b.iter_batched(
                || {
                    (
                        I8CO::try_new(BASE.0, BASE.1).unwrap(),
                        I8CO::try_new(other.0, other.1).unwrap(),
                    )
                },
                |(lhs, rhs)| lhs.symmetric_difference(rhs),
                BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::new("rust_intervals", case), |b| {
            b.iter_batched(
                || {
                    (
                        Interval::new_closed_open(BASE.0, BASE.1),
                        Interval::new_closed_open(other.0, other.1),
                    )
                },
                |(lhs, rhs)| lhs.symmetric_difference(rhs),
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_symmetric_difference
}

criterion_main!(benches);
