use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const BASE: (i8, i8) = (-32, 96);

#[divan::bench(name = "symmetric_difference/equal/int_interval")]
fn symmetric_difference_int_interval_equal(bencher: Bencher) {
    bench_int_interval(bencher, (-32, 96));
}

#[divan::bench(name = "symmetric_difference/equal/rust_intervals")]
fn symmetric_difference_rust_intervals_equal(bencher: Bencher) {
    bench_rust_intervals(bencher, (-32, 96));
}

#[divan::bench(name = "symmetric_difference/same_left/int_interval")]
fn symmetric_difference_int_interval_same_left(bencher: Bencher) {
    bench_int_interval(bencher, (-32, 32));
}

#[divan::bench(name = "symmetric_difference/same_left/rust_intervals")]
fn symmetric_difference_rust_intervals_same_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-32, 32));
}

#[divan::bench(name = "symmetric_difference/same_right/int_interval")]
fn symmetric_difference_int_interval_same_right(bencher: Bencher) {
    bench_int_interval(bencher, (32, 96));
}

#[divan::bench(name = "symmetric_difference/same_right/rust_intervals")]
fn symmetric_difference_rust_intervals_same_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (32, 96));
}

#[divan::bench(name = "symmetric_difference/contained_strict/int_interval")]
fn symmetric_difference_int_interval_contained_strict(bencher: Bencher) {
    bench_int_interval(bencher, (-16, 32));
}

#[divan::bench(name = "symmetric_difference/contained_strict/rust_intervals")]
fn symmetric_difference_rust_intervals_contained_strict(bencher: Bencher) {
    bench_rust_intervals(bencher, (-16, 32));
}

#[divan::bench(name = "symmetric_difference/contains_base/int_interval")]
fn symmetric_difference_int_interval_contains_base(bencher: Bencher) {
    bench_int_interval(bencher, (-64, 112));
}

#[divan::bench(name = "symmetric_difference/contains_base/rust_intervals")]
fn symmetric_difference_rust_intervals_contains_base(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, 112));
}

#[divan::bench(name = "symmetric_difference/overlap_left/int_interval")]
fn symmetric_difference_int_interval_overlap_left(bencher: Bencher) {
    bench_int_interval(bencher, (-64, 0));
}

#[divan::bench(name = "symmetric_difference/overlap_left/rust_intervals")]
fn symmetric_difference_rust_intervals_overlap_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, 0));
}

#[divan::bench(name = "symmetric_difference/overlap_right/int_interval")]
fn symmetric_difference_int_interval_overlap_right(bencher: Bencher) {
    bench_int_interval(bencher, (32, 112));
}

#[divan::bench(name = "symmetric_difference/overlap_right/rust_intervals")]
fn symmetric_difference_rust_intervals_overlap_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (32, 112));
}

#[divan::bench(name = "symmetric_difference/adjacent_left/int_interval")]
fn symmetric_difference_int_interval_adjacent_left(bencher: Bencher) {
    bench_int_interval(bencher, (-64, -32));
}

#[divan::bench(name = "symmetric_difference/adjacent_left/rust_intervals")]
fn symmetric_difference_rust_intervals_adjacent_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, -32));
}

#[divan::bench(name = "symmetric_difference/adjacent_right/int_interval")]
fn symmetric_difference_int_interval_adjacent_right(bencher: Bencher) {
    bench_int_interval(bencher, (96, 112));
}

#[divan::bench(name = "symmetric_difference/adjacent_right/rust_intervals")]
fn symmetric_difference_rust_intervals_adjacent_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (96, 112));
}

#[divan::bench(name = "symmetric_difference/disjoint_left/int_interval")]
fn symmetric_difference_int_interval_disjoint_left(bencher: Bencher) {
    bench_int_interval(bencher, (-96, -64));
}

#[divan::bench(name = "symmetric_difference/disjoint_left/rust_intervals")]
fn symmetric_difference_rust_intervals_disjoint_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-96, -64));
}

#[divan::bench(name = "symmetric_difference/disjoint_right/int_interval")]
fn symmetric_difference_int_interval_disjoint_right(bencher: Bencher) {
    bench_int_interval(bencher, (112, 127));
}

#[divan::bench(name = "symmetric_difference/disjoint_right/rust_intervals")]
fn symmetric_difference_rust_intervals_disjoint_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (112, 127));
}

fn bench_int_interval(bencher: Bencher, other: (i8, i8)) {
    bencher
        .with_inputs(|| {
            (
                I8CO::try_new(BASE.0, BASE.1).unwrap(),
                I8CO::try_new(other.0, other.1).unwrap(),
            )
        })
        .bench_values(|(lhs, rhs)| black_box(lhs.symmetric_difference(rhs)));
}

fn bench_rust_intervals(bencher: Bencher, other: (i8, i8)) {
    bencher
        .with_inputs(|| {
            (
                Interval::new_closed_open(BASE.0, BASE.1),
                Interval::new_closed_open(other.0, other.1),
            )
        })
        .bench_values(|(lhs, rhs)| black_box(lhs.symmetric_difference(rhs)));
}
