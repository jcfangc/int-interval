use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const OUTER: (i8, i8) = (-32, 96);

#[divan::bench(name = "contains_interval/int_interval/equal")]
fn contains_interval_int_interval_equal(bencher: Bencher) {
    bench_int_interval(bencher, (-32, 96));
}

#[divan::bench(name = "contains_interval/rust_intervals/equal")]
fn contains_interval_rust_intervals_equal(bencher: Bencher) {
    bench_rust_intervals(bencher, (-32, 96));
}

#[divan::bench(name = "contains_interval/int_interval/contains_strict")]
fn contains_interval_int_interval_contains_strict(bencher: Bencher) {
    bench_int_interval(bencher, (-16, 32));
}

#[divan::bench(name = "contains_interval/rust_intervals/contains_strict")]
fn contains_interval_rust_intervals_contains_strict(bencher: Bencher) {
    bench_rust_intervals(bencher, (-16, 32));
}

#[divan::bench(name = "contains_interval/int_interval/contains_left_edge")]
fn contains_interval_int_interval_contains_left_edge(bencher: Bencher) {
    bench_int_interval(bencher, (-32, 32));
}

#[divan::bench(name = "contains_interval/rust_intervals/contains_left_edge")]
fn contains_interval_rust_intervals_contains_left_edge(bencher: Bencher) {
    bench_rust_intervals(bencher, (-32, 32));
}

#[divan::bench(name = "contains_interval/int_interval/contains_right_edge")]
fn contains_interval_int_interval_contains_right_edge(bencher: Bencher) {
    bench_int_interval(bencher, (32, 96));
}

#[divan::bench(name = "contains_interval/rust_intervals/contains_right_edge")]
fn contains_interval_rust_intervals_contains_right_edge(bencher: Bencher) {
    bench_rust_intervals(bencher, (32, 96));
}

#[divan::bench(name = "contains_interval/int_interval/miss_left")]
fn contains_interval_int_interval_miss_left(bencher: Bencher) {
    bench_int_interval(bencher, (-64, 32));
}

#[divan::bench(name = "contains_interval/rust_intervals/miss_left")]
fn contains_interval_rust_intervals_miss_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, 32));
}

#[divan::bench(name = "contains_interval/int_interval/miss_right")]
fn contains_interval_int_interval_miss_right(bencher: Bencher) {
    bench_int_interval(bencher, (32, 112));
}

#[divan::bench(name = "contains_interval/rust_intervals/miss_right")]
fn contains_interval_rust_intervals_miss_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (32, 112));
}

fn bench_int_interval(bencher: Bencher, inner: (i8, i8)) {
    let outer = I8CO::try_new(OUTER.0, OUTER.1).unwrap();
    let inner = I8CO::try_new(inner.0, inner.1).unwrap();

    bencher.bench(|| black_box(outer).contains_interval(black_box(inner)));
}

fn bench_rust_intervals(bencher: Bencher, inner: (i8, i8)) {
    let outer = Interval::new_closed_open(OUTER.0, OUTER.1);
    let inner = Interval::new_closed_open(inner.0, inner.1);

    bencher.bench(|| black_box(outer).contains_interval(black_box(inner)));
}
