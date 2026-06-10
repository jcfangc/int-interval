use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const BASE: (i8, i8) = (-32, 96);

#[divan::bench(name = "between/gap_left/int_interval")]
fn between_gap_left_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-96, -64));
}

#[divan::bench(name = "between/gap_left/rust_intervals")]
fn between_gap_left_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-96, -64));
}

#[divan::bench(name = "between/gap_right/int_interval")]
fn between_gap_right_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (112, 127));
}

#[divan::bench(name = "between/gap_right/rust_intervals")]
fn between_gap_right_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (112, 127));
}

#[divan::bench(name = "between/adjacent_left/int_interval")]
fn between_adjacent_left_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-64, -32));
}

#[divan::bench(name = "between/adjacent_left/rust_intervals")]
fn between_adjacent_left_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, -32));
}

#[divan::bench(name = "between/adjacent_right/int_interval")]
fn between_adjacent_right_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (96, 112));
}

#[divan::bench(name = "between/adjacent_right/rust_intervals")]
fn between_adjacent_right_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (96, 112));
}

#[divan::bench(name = "between/overlap_left/int_interval")]
fn between_overlap_left_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-64, 0));
}

#[divan::bench(name = "between/overlap_left/rust_intervals")]
fn between_overlap_left_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, 0));
}

#[divan::bench(name = "between/overlap_right/int_interval")]
fn between_overlap_right_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (32, 112));
}

#[divan::bench(name = "between/overlap_right/rust_intervals")]
fn between_overlap_right_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (32, 112));
}

#[divan::bench(name = "between/contained/int_interval")]
fn between_contained_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-16, 32));
}

#[divan::bench(name = "between/contained/rust_intervals")]
fn between_contained_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-16, 32));
}

#[divan::bench(name = "between/equal/int_interval")]
fn between_equal_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, (-32, 96));
}

#[divan::bench(name = "between/equal/rust_intervals")]
fn between_equal_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, (-32, 96));
}

fn bench_int_interval(bencher: Bencher, other: (i8, i8)) {
    let lhs = I8CO::try_new(BASE.0, BASE.1).unwrap();
    let rhs = I8CO::try_new(other.0, other.1).unwrap();

    bencher.bench(|| black_box(lhs).between(black_box(rhs)));
}

fn bench_rust_intervals(bencher: Bencher, other: (i8, i8)) {
    let lhs = Interval::new_closed_open(BASE.0, BASE.1);
    let rhs = Interval::new_closed_open(other.0, other.1);

    bencher.bench(|| black_box(lhs).between(black_box(rhs)));
}
