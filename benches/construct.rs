use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const VALID: (i8, i8) = (-32, 96);

#[divan::bench_group]
mod valid_closed_open {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn int_interval() -> Option<I8CO> {
        let (start, end_excl) = black_box(VALID);
        I8CO::try_new(start, end_excl)
    }

    #[divan::bench]
    fn rust_intervals() -> Interval<i8> {
        let (start, end_excl) = black_box(VALID);
        Interval::new_closed_open(start, end_excl)
    }

    #[divan::bench]
    fn std_range() -> core::ops::Range<i8> {
        let (start, end_excl) = black_box(VALID);
        start..end_excl
    }
}
