use divan::Bencher;
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const BASE: (i8, i8) = (-32, 96);

macro_rules! contiguous_case {
    ($group:ident, $other:expr) => {
        #[divan::bench_group]
        mod $group {
            use super::*;

            #[divan::bench]
            fn int_interval(bencher: Bencher) {
                bencher
                    .with_inputs(|| {
                        (
                            I8CO::try_new(BASE.0, BASE.1).unwrap(),
                            I8CO::try_new($other.0, $other.1).unwrap(),
                        )
                    })
                    .bench_values(|(lhs, rhs)| lhs.is_contiguous_with(rhs));
            }

            #[divan::bench]
            fn rust_intervals(bencher: Bencher) {
                bencher
                    .with_inputs(|| {
                        (
                            Interval::new_closed_open(BASE.0, BASE.1),
                            Interval::new_closed_open($other.0, $other.1),
                        )
                    })
                    .bench_values(|(lhs, rhs)| lhs.contiguous(rhs));
            }
        }
    };
}

contiguous_case!(equal, (-32, 96));
contiguous_case!(contained, (-16, 32));
contiguous_case!(overlap_left, (-64, 0));
contiguous_case!(overlap_right, (32, 112));
contiguous_case!(adjacent_left, (-64, -32));
contiguous_case!(adjacent_right, (96, 112));
contiguous_case!(gap_left, (-64, -33));
contiguous_case!(gap_right, (97, 112));
