use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const VALID: (i8, i8) = (-32, 96);

#[divan::bench(name = "construct/int_interval/valid_closed_open")]
fn construct_int_interval_valid_closed_open(bencher: Bencher) {
    bencher.bench(|| {
        let (start, end_excl) = black_box(VALID);
        I8CO::try_new(start, end_excl)
    });
}

#[divan::bench(name = "construct/rust_intervals/valid_closed_open")]
fn construct_rust_intervals_valid_closed_open(bencher: Bencher) {
    bencher.bench(|| {
        let (start, end_excl) = black_box(VALID);
        Interval::new_closed_open(start, end_excl)
    });
}

#[divan::bench(name = "construct/std_range/valid_closed_open")]
fn construct_std_range_valid_closed_open(bencher: Bencher) {
    bencher.bench(|| {
        let (start, end_excl) = black_box(VALID);
        start..end_excl
    });
}
