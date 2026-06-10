use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const BASE: (i8, i8) = (-32, 96);

#[divan::bench(name = "intersection/equal/int_interval")]
fn intersection_int_interval_equal(bencher: Bencher) {
    bench_int_interval(bencher, (-32, 96));
}

#[divan::bench(name = "intersection/equal/rust_intervals")]
fn intersection_rust_intervals_equal(bencher: Bencher) {
    bench_rust_intervals(bencher, (-32, 96));
}

#[divan::bench(name = "intersection/contained/int_interval")]
fn intersection_int_interval_contained(bencher: Bencher) {
    bench_int_interval(bencher, (-16, 32));
}

#[divan::bench(name = "intersection/contained/rust_intervals")]
fn intersection_rust_intervals_contained(bencher: Bencher) {
    bench_rust_intervals(bencher, (-16, 32));
}

#[divan::bench(name = "intersection/contains_base/int_interval")]
fn intersection_int_interval_contains_base(bencher: Bencher) {
    bench_int_interval(bencher, (-64, 112));
}

#[divan::bench(name = "intersection/contains_base/rust_intervals")]
fn intersection_rust_intervals_contains_base(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, 112));
}

#[divan::bench(name = "intersection/overlap_left/int_interval")]
fn intersection_int_interval_overlap_left(bencher: Bencher) {
    bench_int_interval(bencher, (-64, 0));
}

#[divan::bench(name = "intersection/overlap_left/rust_intervals")]
fn intersection_rust_intervals_overlap_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, 0));
}

#[divan::bench(name = "intersection/overlap_right/int_interval")]
fn intersection_int_interval_overlap_right(bencher: Bencher) {
    bench_int_interval(bencher, (32, 112));
}

#[divan::bench(name = "intersection/overlap_right/rust_intervals")]
fn intersection_rust_intervals_overlap_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (32, 112));
}

#[divan::bench(name = "intersection/adjacent_left/int_interval")]
fn intersection_int_interval_adjacent_left(bencher: Bencher) {
    bench_int_interval(bencher, (-64, -32));
}

#[divan::bench(name = "intersection/adjacent_left/rust_intervals")]
fn intersection_rust_intervals_adjacent_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, -32));
}

#[divan::bench(name = "intersection/adjacent_right/int_interval")]
fn intersection_int_interval_adjacent_right(bencher: Bencher) {
    bench_int_interval(bencher, (96, 112));
}

#[divan::bench(name = "intersection/adjacent_right/rust_intervals")]
fn intersection_rust_intervals_adjacent_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (96, 112));
}

#[divan::bench(name = "intersection/disjoint_left/int_interval")]
fn intersection_int_interval_disjoint_left(bencher: Bencher) {
    bench_int_interval(bencher, (-96, -64));
}

#[divan::bench(name = "intersection/disjoint_left/rust_intervals")]
fn intersection_rust_intervals_disjoint_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-96, -64));
}

#[divan::bench(name = "intersection/disjoint_right/int_interval")]
fn intersection_int_interval_disjoint_right(bencher: Bencher) {
    bench_int_interval(bencher, (112, 127));
}

#[divan::bench(name = "intersection/disjoint_right/rust_intervals")]
fn intersection_rust_intervals_disjoint_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (112, 127));
}

fn bench_int_interval(bencher: Bencher, other: (i8, i8)) {
    let lhs = I8CO::try_new(BASE.0, BASE.1).unwrap();
    let rhs = I8CO::try_new(other.0, other.1).unwrap();

    bencher.bench(|| black_box(lhs).intersection(black_box(rhs)));
}

fn bench_rust_intervals(bencher: Bencher, other: (i8, i8)) {
    let lhs = Interval::new_closed_open(BASE.0, BASE.1);
    let rhs = Interval::new_closed_open(other.0, other.1);

    bencher.bench(|| black_box(&lhs).intersection(black_box(&rhs)));
}
