use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

#[inline]
fn consume<I: Iterator<Item = i8>>(iter: I) -> i32 {
    iter.fold(0_i32, |acc, x| acc.wrapping_add(x as i32))
}

fn iter(c: &mut Criterion) {
    macro_rules! iter_case {
        ($name:literal, $start:expr, $end_excl:expr, $items:expr) => {{
            let interval = I8CO::try_new($start, $end_excl).unwrap();
            let rust_interval = Interval::new_closed_open($start, $end_excl);
            let range_bounds = ($start, $end_excl);

            let mut group = c.benchmark_group(concat!("iter/", $name));
            group.throughput(Throughput::Elements($items as u64));

            group.bench_function("int_interval", |b| {
                b.iter(|| consume(black_box(interval).iter()));
            });

            group.bench_function("rust_intervals", |b| {
                b.iter(|| consume(black_box(&rust_interval).iter()));
            });

            group.bench_function("std_range", |b| {
                b.iter(|| {
                    let (start, end_excl) = black_box(range_bounds);
                    consume(start..end_excl)
                });
            });

            group.finish();
        }};
    }

    iter_case!("len_1", 0_i8, 1_i8, 1_usize);
    iter_case!("len_16", -8_i8, 8_i8, 16_usize);
    iter_case!("max_span", i8::MIN, i8::MAX, 255_usize);
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = iter
}

criterion_main!(benches);
