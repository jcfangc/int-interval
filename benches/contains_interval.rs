use divan::Bencher;
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const OUTER: (i8, i8) = (-32, 96);

macro_rules! contains_interval_case {
    ($group:ident, $inner:expr) => {
        #[divan::bench_group]
        mod $group {
            use super::*;

            #[divan::bench]
            fn int_interval(bencher: Bencher) {
                bencher
                    .with_inputs(|| {
                        (
                            I8CO::try_new(OUTER.0, OUTER.1).unwrap(),
                            I8CO::try_new($inner.0, $inner.1).unwrap(),
                        )
                    })
                    .bench_values(|(outer, inner)| outer.contains_interval(inner));
            }

            #[divan::bench]
            fn rust_intervals(bencher: Bencher) {
                bencher
                    .with_inputs(|| {
                        (
                            Interval::new_closed_open(OUTER.0, OUTER.1),
                            Interval::new_closed_open($inner.0, $inner.1),
                        )
                    })
                    .bench_values(|(outer, inner)| outer.contains_interval(inner));
            }
        }
    };
}

contains_interval_case!(equal, (-32, 96));
contains_interval_case!(contains_strict, (-16, 32));
contains_interval_case!(contains_left_edge, (-32, 32));
contains_interval_case!(contains_right_edge, (32, 96));
contains_interval_case!(miss_left, (-64, 32));
contains_interval_case!(miss_right, (32, 112));
