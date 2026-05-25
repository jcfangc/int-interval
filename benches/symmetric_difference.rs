use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const BASE: (i8, i8) = (-32, 96);

macro_rules! symmetric_difference_case {
    ($case:ident, $other:expr) => {
        fn $case(c: &mut Criterion) {
            let mut group = c.benchmark_group(concat!("symmetric_difference/", stringify!($case)));

            group.bench_function("int_interval", |b| {
                b.iter_batched(
                    || {
                        (
                            I8CO::try_new(BASE.0, BASE.1).unwrap(),
                            I8CO::try_new($other.0, $other.1).unwrap(),
                        )
                    },
                    |(lhs, rhs)| lhs.symmetric_difference(rhs),
                    BatchSize::SmallInput,
                );
            });

            group.bench_function("rust_intervals", |b| {
                b.iter_batched(
                    || {
                        (
                            Interval::new_closed_open(BASE.0, BASE.1),
                            Interval::new_closed_open($other.0, $other.1),
                        )
                    },
                    |(lhs, rhs)| lhs.symmetric_difference(rhs),
                    BatchSize::SmallInput,
                );
            });

            group.finish();
        }
    };
}

symmetric_difference_case!(equal, (-32, 96));

symmetric_difference_case!(same_left, (-32, 32));
symmetric_difference_case!(same_right, (32, 96));

symmetric_difference_case!(contained_strict, (-16, 32));
symmetric_difference_case!(contains_base, (-64, 112));

symmetric_difference_case!(overlap_left, (-64, 0));
symmetric_difference_case!(overlap_right, (32, 112));

symmetric_difference_case!(adjacent_left, (-64, -32));
symmetric_difference_case!(adjacent_right, (96, 112));

symmetric_difference_case!(disjoint_left, (-96, -64));
symmetric_difference_case!(disjoint_right, (112, 127));

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets =
        equal,
        same_left,
        same_right,
        contained_strict,
        contains_base,
        overlap_left,
        overlap_right,
        adjacent_left,
        adjacent_right,
        disjoint_left,
        disjoint_right,
}

criterion_main!(benches);
