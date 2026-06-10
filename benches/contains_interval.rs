use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const OUTER: (i8, i8) = (-32, 96);

#[divan::bench(name = "contains_interval/equal/int_interval")]
fn contains_interval_equal_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-32, 96));
}

#[divan::bench(name = "contains_interval/equal/rust_intervals")]
fn contains_interval_equal_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-32, 96));
}

#[divan::bench(name = "contains_interval/contains_strict/int_interval")]
fn contains_interval_contains_strict_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-16, 32));
}

#[divan::bench(name = "contains_interval/contains_strict/rust_intervals")]
fn contains_interval_contains_strict_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-16, 32));
}

#[divan::bench(name = "contains_interval/contains_left_edge/int_interval")]
fn contains_interval_contains_left_edge_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-32, 32));
}

#[divan::bench(name = "contains_interval/contains_left_edge/rust_intervals")]
fn contains_interval_contains_left_edge_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-32, 32));
}

#[divan::bench(name = "contains_interval/contains_right_edge/int_interval")]
fn contains_interval_contains_right_edge_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (32, 96));
}

#[divan::bench(name = "contains_interval/contains_right_edge/rust_intervals")]
fn contains_interval_contains_right_edge_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (32, 96));
}

#[divan::bench(name = "contains_interval/miss_left/int_interval")]
fn contains_interval_miss_left_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-64, 32));
}

#[divan::bench(name = "contains_interval/miss_left/rust_intervals")]
fn contains_interval_miss_left_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, 32));
}

#[divan::bench(name = "contains_interval/miss_right/int_interval")]
fn contains_interval_miss_right_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (32, 112));
}

#[divan::bench(name = "contains_interval/miss_right/rust_intervals")]
fn contains_interval_miss_right_rust_intervals(bencher: Bencher) {
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
