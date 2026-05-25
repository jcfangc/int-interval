use criterion::{Criterion, black_box, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const VALID: (i8, i8) = (-32, 96);

fn valid_closed_open(c: &mut Criterion) {
    let mut group = c.benchmark_group("construct/valid_closed_open");

    group.bench_function("int_interval", |b| {
        b.iter(|| {
            let (start, end_excl) = black_box(VALID);
            I8CO::try_new(start, end_excl)
        });
    });

    group.bench_function("rust_intervals", |b| {
        b.iter(|| {
            let (start, end_excl) = black_box(VALID);
            Interval::new_closed_open(start, end_excl)
        });
    });

    group.bench_function("std_range", |b| {
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
    targets = valid_closed_open
}

criterion_main!(benches);
