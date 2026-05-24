use criterion::{Criterion, black_box, criterion_group, criterion_main};
use int_interval::I8CO;
use rust_intervals::Interval;

const BASE: (i8, i8) = (-32, 96);

fn contiguous(c: &mut Criterion) {
    macro_rules! contiguous_case {
        ($name:literal, $other:expr) => {{
            let lhs = I8CO::try_new(BASE.0, BASE.1).unwrap();
            let rhs = I8CO::try_new($other.0, $other.1).unwrap();

            let rust_lhs = Interval::new_closed_open(BASE.0, BASE.1);
            let rust_rhs = Interval::new_closed_open($other.0, $other.1);

            let mut group = c.benchmark_group(concat!("contiguous/", $name));

            group.bench_function("int_interval", |b| {
                b.iter(|| black_box(lhs).is_contiguous_with(black_box(rhs)));
            });

            group.bench_function("rust_intervals", |b| {
                b.iter(|| black_box(&rust_lhs).contiguous(black_box(&rust_rhs)));
            });

            group.finish();
        }};
    }

    contiguous_case!("equal", (-32, 96));
    contiguous_case!("contained", (-16, 32));
    contiguous_case!("overlap_left", (-64, 0));
    contiguous_case!("overlap_right", (32, 112));
    contiguous_case!("adjacent_left", (-64, -32));
    contiguous_case!("adjacent_right", (96, 112));
    contiguous_case!("gap_left", (-64, -33));
    contiguous_case!("gap_right", (97, 112));
}

criterion_group!(benches, contiguous);
criterion_main!(benches);
