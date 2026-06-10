use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const BASE: (i8, i8) = (-32, 96);

#[divan::bench(name = "contiguous/equal/int_interval")]
fn contiguous_equal_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-32, 96));
}

#[divan::bench(name = "contiguous/equal/rust_intervals")]
fn contiguous_equal_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-32, 96));
}

#[divan::bench(name = "contiguous/contained/int_interval")]
fn contiguous_contained_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-16, 32));
}

#[divan::bench(name = "contiguous/contained/rust_intervals")]
fn contiguous_contained_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-16, 32));
}

#[divan::bench(name = "contiguous/overlap_left/int_interval")]
fn contiguous_overlap_left_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-64, 0));
}

#[divan::bench(name = "contiguous/overlap_left/rust_intervals")]
fn contiguous_overlap_left_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, 0));
}

#[divan::bench(name = "contiguous/overlap_right/int_interval")]
fn contiguous_overlap_right_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (32, 112));
}

#[divan::bench(name = "contiguous/overlap_right/rust_intervals")]
fn contiguous_overlap_right_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (32, 112));
}

#[divan::bench(name = "contiguous/adjacent_left/int_interval")]
fn contiguous_adjacent_left_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-64, -32));
}

#[divan::bench(name = "contiguous/adjacent_left/rust_intervals")]
fn contiguous_adjacent_left_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, -32));
}

#[divan::bench(name = "contiguous/adjacent_right/int_interval")]
fn contiguous_adjacent_right_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (96, 112));
}

#[divan::bench(name = "contiguous/adjacent_right/rust_intervals")]
fn contiguous_adjacent_right_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (96, 112));
}

#[divan::bench(name = "contiguous/gap_left/int_interval")]
fn contiguous_gap_left_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-64, -33));
}

#[divan::bench(name = "contiguous/gap_left/rust_intervals")]
fn contiguous_gap_left_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, -33));
}

#[divan::bench(name = "contiguous/gap_right/int_interval")]
fn contiguous_gap_right_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (97, 112));
}

#[divan::bench(name = "contiguous/gap_right/rust_intervals")]
fn contiguous_gap_right_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (97, 112));
}

fn bench_int_interval(bencher: Bencher, other: (i8, i8)) {
    let lhs = I8CO::try_new(BASE.0, BASE.1).unwrap();
    let rhs = I8CO::try_new(other.0, other.1).unwrap();

    bencher.bench(|| black_box(lhs).is_contiguous_with(black_box(rhs)));
}

fn bench_rust_intervals(bencher: Bencher, other: (i8, i8)) {
    let lhs = Interval::new_closed_open(BASE.0, BASE.1);
    let rhs = Interval::new_closed_open(other.0, other.1);

    bencher.bench(|| black_box(&lhs).contiguous(black_box(&rhs)));
}
