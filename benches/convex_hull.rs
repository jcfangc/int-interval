use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use int_interval::I8CO;
use rust_intervals::Interval;

const BASE: (i8, i8) = (-32, 96);

macro_rules! convex_hull_case {
    ($case:ident, $other:expr) => {
        fn $case(c: &mut Criterion) {
            let mut group = c.benchmark_group(concat!("convex_hull/", stringify!($case)));

            group.bench_function("int_interval", |b| {
                b.iter_batched(
                    || {
                        (
                            I8CO::try_new(BASE.0, BASE.1).unwrap(),
                            I8CO::try_new($other.0, $other.1).unwrap(),
                        )
                    },
                    |(lhs, rhs)| lhs.convex_hull(rhs),
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
                    |(lhs, rhs)| lhs.convex_hull(rhs),
                    BatchSize::SmallInput,
                );
            });

            group.finish();
        }
    };
}

convex_hull_case!(equal, (-32, 96));
convex_hull_case!(other_contained, (-16, 32));
convex_hull_case!(base_contained, (-64, 112));
convex_hull_case!(extends_left, (-64, 32));
convex_hull_case!(extends_right, (32, 112));
convex_hull_case!(disjoint_left, (-96, -64));
convex_hull_case!(disjoint_right, (112, 127));

criterion_group!(
    benches,
    equal,
    other_contained,
    base_contained,
    extends_left,
    extends_right,
    disjoint_left,
    disjoint_right,
);
criterion_main!(benches);
