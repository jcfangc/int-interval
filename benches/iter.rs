use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const CASES: &[(&str, i8, i8, usize)] = &[
    ("len_1", 0_i8, 1_i8, 1_usize),
    ("len_16", -8_i8, 8_i8, 16_usize),
    ("max_span", i8::MIN, i8::MAX, 255_usize),
];

#[inline]
fn consume<I: Iterator<Item = i8>>(iter: I) -> i32 {
    iter.fold(0_i32, |acc, x| acc.wrapping_add(x as i32))
}

fn bench_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter");

    for &(case, start, end_excl, items) in CASES {
        let interval = I8CO::try_new(start, end_excl).unwrap();
        let rust_interval = Interval::new_closed_open(start, end_excl);
        let range_bounds = (start, end_excl);

        group.throughput(Throughput::Elements(items as u64));

        group.bench_function(BenchmarkId::new("int_interval", case), |b| {
            b.iter(|| consume(black_box(interval).iter()));
        });

        group.bench_function(BenchmarkId::new("rust_intervals", case), |b| {
            b.iter(|| consume(black_box(&rust_interval).iter()));
        });

        group.bench_function(BenchmarkId::new("std_range", case), |b| {
            b.iter(|| {
                let (start, end_excl) = black_box(range_bounds);
                consume(start..end_excl)
            });
        });
    }

    group.finish();
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_iter
}

criterion_main!(benches);
