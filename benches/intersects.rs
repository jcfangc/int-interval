use divan::Bencher;
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const BASE: (i8, i8) = (-32, 96);

macro_rules! intersects_case {
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
                    .bench_values(|(lhs, rhs)| lhs.intersects(rhs));
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
                    .bench_values(|(lhs, rhs)| lhs.intersects(rhs));
            }
        }
    };
}

intersects_case!(equal, (-32, 96));
intersects_case!(contained, (-16, 32));
intersects_case!(overlap_left, (-64, 0));
intersects_case!(overlap_right, (32, 112));
intersects_case!(adjacent_left, (-64, -32));
intersects_case!(adjacent_right, (96, 112));
intersects_case!(disjoint_left, (-96, -64));
intersects_case!(disjoint_right, (112, 127));
