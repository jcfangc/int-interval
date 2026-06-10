use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

#[divan::bench(name = "iter/int_interval/len_1")]
fn iter_int_interval_len_1(bencher: Bencher) {
    bench_int_interval(bencher, 0, 1);
}

#[divan::bench(name = "iter/rust_intervals/len_1")]
fn iter_rust_intervals_len_1(bencher: Bencher) {
    bench_rust_intervals(bencher, 0, 1);
}

#[divan::bench(name = "iter/std_range/len_1")]
fn iter_std_range_len_1(bencher: Bencher) {
    bench_std_range(bencher, 0, 1);
}

#[divan::bench(name = "iter/int_interval/len_16")]
fn iter_int_interval_len_16(bencher: Bencher) {
    bench_int_interval(bencher, -8, 8);
}

#[divan::bench(name = "iter/rust_intervals/len_16")]
fn iter_rust_intervals_len_16(bencher: Bencher) {
    bench_rust_intervals(bencher, -8, 8);
}

#[divan::bench(name = "iter/std_range/len_16")]
fn iter_std_range_len_16(bencher: Bencher) {
    bench_std_range(bencher, -8, 8);
}

#[divan::bench(name = "iter/int_interval/max_span")]
fn iter_int_interval_max_span(bencher: Bencher) {
    bench_int_interval(bencher, i8::MIN, i8::MAX);
}

#[divan::bench(name = "iter/rust_intervals/max_span")]
fn iter_rust_intervals_max_span(bencher: Bencher) {
    bench_rust_intervals(bencher, i8::MIN, i8::MAX);
}

#[divan::bench(name = "iter/std_range/max_span")]
fn iter_std_range_max_span(bencher: Bencher) {
    bench_std_range(bencher, i8::MIN, i8::MAX);
}

fn bench_int_interval(bencher: Bencher, start: i8, end_excl: i8) {
    let interval = I8CO::try_new(start, end_excl).unwrap();

    bencher.bench(|| consume(black_box(interval).iter()));
}

fn bench_rust_intervals(bencher: Bencher, start: i8, end_excl: i8) {
    let interval = Interval::new_closed_open(start, end_excl);

    bencher.bench(|| consume(black_box(&interval).iter()));
}

fn bench_std_range(bencher: Bencher, start: i8, end_excl: i8) {
    let bounds = (start, end_excl);

    bencher.bench(|| {
        let (start, end_excl) = black_box(bounds);
        consume(start..end_excl)
    });
}

#[inline]
fn consume<I: Iterator<Item = i8>>(iter: I) -> i32 {
    iter.fold(0_i32, |acc, x| acc.wrapping_add(x as i32))
}
