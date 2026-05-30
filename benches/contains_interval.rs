use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const OUTER: (i8, i8) = (-32, 96);

const CASES: &[(&str, (i8, i8))] = &[
    ("equal", (-32, 96)),
    ("contains_strict", (-16, 32)),
    ("contains_left_edge", (-32, 32)),
    ("contains_right_edge", (32, 96)),
    ("miss_left", (-64, 32)),
    ("miss_right", (32, 112)),
];

fn bench_contains_interval(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains_interval");

    for &(case, inner) in CASES {
        group.bench_function(BenchmarkId::new("int_interval", case), |b| {
            b.iter_batched(
                || {
                    (
                        I8CO::try_new(OUTER.0, OUTER.1).unwrap(),
                        I8CO::try_new(inner.0, inner.1).unwrap(),
                    )
                },
                |(outer, inner)| outer.contains_interval(inner),
                BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::new("rust_intervals", case), |b| {
            b.iter_batched(
                || {
                    (
                        Interval::new_closed_open(OUTER.0, OUTER.1),
                        Interval::new_closed_open(inner.0, inner.1),
                    )
                },
                |(outer, inner)| outer.contains_interval(inner),
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
    targets = bench_contains_interval
}

criterion_main!(benches);
