use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const VALID: (i8, i8) = (-32, 96);

#[divan::bench(name = "construct/valid_closed_open/int_interval")]
fn construct_valid_closed_open_int_interval(bencher: Bencher) {
    bencher.bench(|| {
        let (start, end_excl) = black_box(VALID);
        I8CO::try_new(start, end_excl)
    });
}

#[divan::bench(name = "construct/valid_closed_open/rust_intervals")]
fn construct_valid_closed_open_rust_intervals(bencher: Bencher) {
    bencher.bench(|| {
        let (start, end_excl) = black_box(VALID);
        Interval::new_closed_open(start, end_excl)
    });
}

#[divan::bench(name = "construct/valid_closed_open/std_range")]
fn construct_valid_closed_open_std_range(bencher: Bencher) {
    bencher.bench(|| {
        let (start, end_excl) = black_box(VALID);
        start..end_excl
    });
}
