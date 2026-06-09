use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const BASE: (i8, i8) = (-32, 96);

#[divan::bench(name = "convex_hull/int_interval/equal")]
fn convex_hull_int_interval_equal(bencher: Bencher) {
    bench_int_interval(bencher, (-32, 96));
}

#[divan::bench(name = "convex_hull/rust_intervals/equal")]
fn convex_hull_rust_intervals_equal(bencher: Bencher) {
    bench_rust_intervals(bencher, (-32, 96));
}

#[divan::bench(name = "convex_hull/int_interval/other_contained")]
fn convex_hull_int_interval_other_contained(bencher: Bencher) {
    bench_int_interval(bencher, (-16, 32));
}

#[divan::bench(name = "convex_hull/rust_intervals/other_contained")]
fn convex_hull_rust_intervals_other_contained(bencher: Bencher) {
    bench_rust_intervals(bencher, (-16, 32));
}

#[divan::bench(name = "convex_hull/int_interval/base_contained")]
fn convex_hull_int_interval_base_contained(bencher: Bencher) {
    bench_int_interval(bencher, (-64, 112));
}

#[divan::bench(name = "convex_hull/rust_intervals/base_contained")]
fn convex_hull_rust_intervals_base_contained(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, 112));
}

#[divan::bench(name = "convex_hull/int_interval/extends_left")]
fn convex_hull_int_interval_extends_left(bencher: Bencher) {
    bench_int_interval(bencher, (-64, 32));
}

#[divan::bench(name = "convex_hull/rust_intervals/extends_left")]
fn convex_hull_rust_intervals_extends_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-64, 32));
}

#[divan::bench(name = "convex_hull/int_interval/extends_right")]
fn convex_hull_int_interval_extends_right(bencher: Bencher) {
    bench_int_interval(bencher, (32, 112));
}

#[divan::bench(name = "convex_hull/rust_intervals/extends_right")]
fn convex_hull_rust_intervals_extends_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (32, 112));
}

#[divan::bench(name = "convex_hull/int_interval/disjoint_left")]
fn convex_hull_int_interval_disjoint_left(bencher: Bencher) {
    bench_int_interval(bencher, (-96, -64));
}

#[divan::bench(name = "convex_hull/rust_intervals/disjoint_left")]
fn convex_hull_rust_intervals_disjoint_left(bencher: Bencher) {
    bench_rust_intervals(bencher, (-96, -64));
}

#[divan::bench(name = "convex_hull/int_interval/disjoint_right")]
fn convex_hull_int_interval_disjoint_right(bencher: Bencher) {
    bench_int_interval(bencher, (112, 127));
}

#[divan::bench(name = "convex_hull/rust_intervals/disjoint_right")]
fn convex_hull_rust_intervals_disjoint_right(bencher: Bencher) {
    bench_rust_intervals(bencher, (112, 127));
}

fn bench_int_interval(bencher: Bencher, other: (i8, i8)) {
    let lhs = I8CO::try_new(BASE.0, BASE.1).unwrap();
    let rhs = I8CO::try_new(other.0, other.1).unwrap();

    bencher.bench(|| black_box(lhs).convex_hull(black_box(rhs)));
}

fn bench_rust_intervals(bencher: Bencher, other: (i8, i8)) {
    let lhs = Interval::new_closed_open(BASE.0, BASE.1);
    let rhs = Interval::new_closed_open(other.0, other.1);

    bencher.bench(|| black_box(lhs).convex_hull(black_box(rhs)));
}
