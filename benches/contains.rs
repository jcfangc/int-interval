use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const START: i8 = -32;
const END_EXCL: i8 = 96;

macro_rules! contains_case {
    ($group:ident, $value:expr) => {
        #[divan::bench_group]
        mod $group {
            use super::*;

            #[divan::bench]
            fn int_interval(bencher: Bencher) {
                bencher
                    .with_inputs(|| I8CO::try_new(START, END_EXCL).unwrap())
                    .bench_values(|interval| interval.contains(black_box($value)));
            }

            #[divan::bench]
            fn rust_intervals(bencher: Bencher) {
                bencher
                    .with_inputs(|| Interval::new_closed_open(START, END_EXCL))
                    .bench_values(|interval| interval.contains(black_box($value)));
            }

            #[divan::bench]
            fn std_range(bencher: Bencher) {
                bencher
                    .with_inputs(|| START..END_EXCL)
                    .bench_values(|interval| interval.contains(&black_box($value)));
            }
        }
    };
}

contains_case!(hit_start, START);
contains_case!(hit_middle, 16);
contains_case!(hit_end_incl, END_EXCL - 1);
contains_case!(miss_before, START - 1);
contains_case!(miss_end_excl, END_EXCL);
