use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const BASE: (i8, i8) = (-32, 96);

const CASES: &[(&str, (i8, i8))] = &[
    ("equal", (-32, 96)),
    ("other_contained", (-16, 32)),
    ("base_contained", (-64, 112)),
    ("extends_left", (-64, 32)),
    ("extends_right", (32, 112)),
    ("disjoint_left", (-96, -64)),
    ("disjoint_right", (112, 127)),
];

fn bench_convex_hull(c: &mut Criterion) {
    let mut group = c.benchmark_group("convex_hull");

    for &(case, other) in CASES {
        group.bench_function(BenchmarkId::new("int_interval", case), |b| {
            b.iter_batched(
                || {
                    (
                        I8CO::try_new(BASE.0, BASE.1).unwrap(),
                        I8CO::try_new(other.0, other.1).unwrap(),
                    )
                },
                |(lhs, rhs)| lhs.convex_hull(rhs),
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
                |(lhs, rhs)| lhs.convex_hull(rhs),
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
    targets = bench_convex_hull
}

criterion_main!(benches);
