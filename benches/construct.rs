use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const VALID: (i8, i8) = (-32, 96);

fn bench_construct(c: &mut Criterion) {
    let mut group = c.benchmark_group("construct");

    group.bench_function(BenchmarkId::new("int_interval", "valid_closed_open"), |b| {
        b.iter(|| {
            let (start, end_excl) = black_box(VALID);
            I8CO::try_new(start, end_excl)
        });
    });

    group.bench_function(BenchmarkId::new("rust_intervals", "valid_closed_open"), |b| {
        b.iter(|| {
            let (start, end_excl) = black_box(VALID);
            Interval::new_closed_open(start, end_excl)
        });
    });

    group.bench_function(BenchmarkId::new("std_range", "valid_closed_open"), |b| {
        b.iter(|| {
            let (start, end_excl) = black_box(VALID);
            start..end_excl
        });
    });

    group.finish();
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_construct
}

criterion_main!(benches);
