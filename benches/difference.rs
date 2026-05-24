use divan::Bencher;
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const BASE: (i8, i8) = (-32, 96);

macro_rules! difference_case {
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
                    .bench_values(|(lhs, rhs)| lhs.difference(rhs));
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
                    .bench_values(|(lhs, rhs)| lhs.difference(rhs));
            }
        }
    };
}

// No removal: result remains BASE.
difference_case!(disjoint_left, (-96, -64));
difference_case!(disjoint_right, (112, 127));
difference_case!(adjacent_left, (-64, -32));
difference_case!(adjacent_right, (96, 112));

// Full removal: result is empty.
difference_case!(equal, (-32, 96));
difference_case!(covered_by_other, (-64, 112));

// One remaining segment.
difference_case!(remove_left_edge, (-32, 32));
difference_case!(remove_right_edge, (32, 96));
difference_case!(overlap_left, (-64, 0));
difference_case!(overlap_right, (32, 112));

// Two remaining segments.
difference_case!(remove_middle, (-16, 32));
