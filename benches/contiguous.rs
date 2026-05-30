use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const BASE: (i8, i8) = (-32, 96);

const CASES: &[(&str, (i8, i8))] = &[
    ("equal", (-32, 96)),
    ("contained", (-16, 32)),
    ("overlap_left", (-64, 0)),
    ("overlap_right", (32, 112)),
    ("adjacent_left", (-64, -32)),
    ("adjacent_right", (96, 112)),
    ("gap_left", (-64, -33)),
    ("gap_right", (97, 112)),
];

fn bench_contiguous(c: &mut Criterion) {
    let mut group = c.benchmark_group("contiguous");

    for &(case, other) in CASES {
        let lhs = I8CO::try_new(BASE.0, BASE.1).unwrap();
        let rhs = I8CO::try_new(other.0, other.1).unwrap();

        let rust_lhs = Interval::new_closed_open(BASE.0, BASE.1);
        let rust_rhs = Interval::new_closed_open(other.0, other.1);

        group.bench_function(BenchmarkId::new("int_interval", case), |b| {
            b.iter(|| black_box(lhs).is_contiguous_with(black_box(rhs)));
        });

        group.bench_function(BenchmarkId::new("rust_intervals", case), |b| {
            b.iter(|| black_box(&rust_lhs).contiguous(black_box(&rust_rhs)));
        });
    }

    group.finish();
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_contiguous
}

criterion_main!(benches);
