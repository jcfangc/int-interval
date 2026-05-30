use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const BASE: (i8, i8) = (-32, 96);

const CASES: &[(&str, (i8, i8))] = &[
    ("equal", (-32, 96)),
    ("contained", (-16, 32)),
    ("contains_base", (-64, 112)),
    ("overlap_left", (-64, 0)),
    ("overlap_right", (32, 112)),
    ("adjacent_left", (-64, -32)),
    ("adjacent_right", (96, 112)),
    ("disjoint_left", (-96, -64)),
    ("disjoint_right", (112, 127)),
];

fn bench_intersection(c: &mut Criterion) {
    let mut group = c.benchmark_group("intersection");

    for &(case, other) in CASES {
        let lhs = I8CO::try_new(BASE.0, BASE.1).unwrap();
        let rhs = I8CO::try_new(other.0, other.1).unwrap();

        let rust_lhs = Interval::new_closed_open(BASE.0, BASE.1);
        let rust_rhs = Interval::new_closed_open(other.0, other.1);

        group.bench_function(BenchmarkId::new("int_interval", case), |b| {
            b.iter(|| black_box(lhs).intersection(black_box(rhs)));
        });

        group.bench_function(BenchmarkId::new("rust_intervals", case), |b| {
            b.iter(|| black_box(&rust_lhs).intersection(black_box(&rust_rhs)));
        });
    }

    group.finish();
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_intersection
}

criterion_main!(benches);
