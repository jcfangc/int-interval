use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const OUTER: (i8, i8) = (-32, 96);

macro_rules! contains_interval_case {
    ($bench_fn:ident, $inner:expr) => {
        fn $bench_fn(c: &mut Criterion) {
            let mut group = c.benchmark_group(concat!("contains_interval/", stringify!($bench_fn)));

            group.bench_function("int_interval", |b| {
                b.iter_batched(
                    || {
                        let (inner_start, inner_end_excl) = $inner;
                        (
                            I8CO::try_new(OUTER.0, OUTER.1).unwrap(),
                            I8CO::try_new(inner_start, inner_end_excl).unwrap(),
                        )
                    },
                    |(outer, inner)| outer.contains_interval(inner),
                    BatchSize::SmallInput,
                );
            });

            group.bench_function("rust_intervals", |b| {
                b.iter_batched(
                    || {
                        let (inner_start, inner_end_excl) = $inner;
                        (
                            Interval::new_closed_open(OUTER.0, OUTER.1),
                            Interval::new_closed_open(inner_start, inner_end_excl),
                        )
                    },
                    |(outer, inner)| outer.contains_interval(inner),
                    BatchSize::SmallInput,
                );
            });

            group.finish();
        }
    };
}

contains_interval_case!(equal, (-32, 96));
contains_interval_case!(contains_strict, (-16, 32));
contains_interval_case!(contains_left_edge, (-32, 32));
contains_interval_case!(contains_right_edge, (32, 96));
contains_interval_case!(miss_left, (-64, 32));
contains_interval_case!(miss_right, (32, 112));

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets =
        equal,
        contains_strict,
        contains_left_edge,
        contains_right_edge,
        miss_left,
        miss_right,
}

criterion_main!(benches);
