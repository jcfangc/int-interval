use criterion::{Criterion, black_box, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const BASE: (i8, i8) = (-32, 96);

fn intersection(c: &mut Criterion) {
    macro_rules! intersection_case {
        ($name:literal, $other:expr) => {{
            let lhs = I8CO::try_new(BASE.0, BASE.1).unwrap();
            let rhs = I8CO::try_new($other.0, $other.1).unwrap();

            let rust_lhs = Interval::new_closed_open(BASE.0, BASE.1);
            let rust_rhs = Interval::new_closed_open($other.0, $other.1);

            let mut group = c.benchmark_group(concat!("intersection/", $name));

            group.bench_function("int_interval", |b| {
                b.iter(|| black_box(lhs).intersection(black_box(rhs)));
            });

            group.bench_function("rust_intervals", |b| {
                b.iter(|| black_box(&rust_lhs).intersection(black_box(&rust_rhs)));
            });

            group.finish();
        }};
    }

    intersection_case!("equal", (-32, 96));
    intersection_case!("contained", (-16, 32));
    intersection_case!("contains_base", (-64, 112));
    intersection_case!("overlap_left", (-64, 0));
    intersection_case!("overlap_right", (32, 112));
    intersection_case!("adjacent_left", (-64, -32));
    intersection_case!("adjacent_right", (96, 112));
    intersection_case!("disjoint_left", (-96, -64));
    intersection_case!("disjoint_right", (112, 127));
}

mod support;

criterion_group! {
    name = benches;
    config = support::config();
    targets = intersection
}

criterion_main!(benches);
