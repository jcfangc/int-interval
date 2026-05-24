use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

#[inline]
fn bounds(start: i8, end_excl: i8) -> (i8, i8) {
    black_box((start, end_excl))
}

#[inline]
fn consume<I: Iterator<Item = i8>>(iter: I) -> i32 {
    iter.fold(0_i32, |acc, x| acc.wrapping_add(x as i32))
}

macro_rules! iter_case {
    ($group:ident, $start:expr, $end_excl:expr, $items:expr) => {
        #[divan::bench_group(items_count = $items)]
        mod $group {
            use super::*;

            #[divan::bench]
            fn int_interval(bencher: Bencher) {
                bencher
                    .with_inputs(|| {
                        let (start, end_excl) = bounds($start, $end_excl);
                        I8CO::try_new(start, end_excl).unwrap()
                    })
                    .bench_values(|interval| consume(interval.iter()));
            }

            #[divan::bench]
            fn rust_intervals(bencher: Bencher) {
                bencher
                    .with_inputs(|| {
                        let (start, end_excl) = bounds($start, $end_excl);
                        Interval::new_closed_open(start, end_excl)
                    })
                    .bench_values(|interval| consume(interval.iter()));
            }

            #[divan::bench]
            fn std_range(bencher: Bencher) {
                bencher
                    .with_inputs(|| {
                        let (start, end_excl) = bounds($start, $end_excl);
                        start..end_excl
                    })
                    .bench_values(consume);
            }
        }
    };
}

iter_case!(len_1, 0_i8, 1_i8, 1_usize);
iter_case!(len_16, -8_i8, 8_i8, 16_usize);
iter_case!(max_span, i8::MIN, i8::MAX, 255_usize);
