use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const BASE: (i8, i8) = (-32, 96);

#[divan::bench(name = "between/int_interval/gap_left")]
fn between_int_interval_gap_left(bencher: Bencher) {
    bench_int_interval(bencher, (-96, -64));
}

#[divan::bench(name = "between/rust_intervals/gap_left")]
fn between_rust_intervals_gap_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-96, -64));
}

#[divan::bench(name = "between/int_interval/gap_right")]
fn between_int_interval_gap_right(bencher: Bencher) {
    bench_int_interval(bencher, (112, 127));
}

#[divan::bench(name = "between/rust_intervals/gap_right")]
fn between_rust_intervals_gap_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (112, 127));
}

#[divan::bench(name = "between/int_interval/adjacent_left")]
fn between_int_interval_adjacent_left(bencher: Bencher) {
    bench_int_interval(bencher, (-64, -32));
}

#[divan::bench(name = "between/rust_intervals/adjacent_left")]
fn between_rust_intervals_adjacent_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, -32));
}

#[divan::bench(name = "between/int_interval/adjacent_right")]
fn between_int_interval_adjacent_right(bencher: Bencher) {
    bench_int_interval(bencher, (96, 112));
}

#[divan::bench(name = "between/rust_intervals/adjacent_right")]
fn between_rust_intervals_adjacent_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (96, 112));
}

#[divan::bench(name = "between/int_interval/overlap_left")]
fn between_int_interval_overlap_left(bencher: Bencher) {
    bench_int_interval(bencher, (-64, 0));
}

#[divan::bench(name = "between/rust_intervals/overlap_left")]
fn between_rust_intervals_overlap_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, 0));
}

#[divan::bench(name = "between/int_interval/overlap_right")]
fn between_int_interval_overlap_right(bencher: Bencher) {
    bench_int_interval(bencher, (32, 112));
}

#[divan::bench(name = "between/rust_intervals/overlap_right")]
fn between_rust_intervals_overlap_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (32, 112));
}

#[divan::bench(name = "between/int_interval/contained")]
fn between_int_interval_contained(bencher: Bencher) {
    bench_int_interval(bencher, (-16, 32));
}

#[divan::bench(name = "between/rust_intervals/contained")]
fn between_rust_intervals_contained(bencher: Bencher) {
    bench_rust_intervals(bencher, (-16, 32));
}

#[divan::bench(name = "between/int_interval/equal")]
fn between_int_interval_equal(bencher: Bencher) {
    bench_int_interval(bencher, (-32, 96));
}

#[divan::bench(name = "between/rust_intervals/equal")]
fn between_rust_intervals_equal(bencher: Bencher) {
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
