use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const START: i8 = -32;
const END_EXCL: i8 = 96;

#[divan::bench(name = "contains/hit_start/int_interval")]
fn contains_hit_start_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, START);
}

#[divan::bench(name = "contains/hit_start/rust_intervals")]
fn contains_hit_start_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, START);
}

#[divan::bench(name = "contains/hit_start/std_range")]
fn contains_hit_start_std_range(bencher: Bencher) {
    bench_std_range(bencher, START);
}

#[divan::bench(name = "contains/hit_middle/int_interval")]
fn contains_hit_middle_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, 16);
}

#[divan::bench(name = "contains/hit_middle/rust_intervals")]
fn contains_hit_middle_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, 16);
}

#[divan::bench(name = "contains/hit_middle/std_range")]
fn contains_hit_middle_std_range(bencher: Bencher) {
    bench_std_range(bencher, 16);
}

#[divan::bench(name = "contains/hit_end_incl/int_interval")]
fn contains_hit_end_incl_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, END_EXCL - 1);
}

#[divan::bench(name = "contains/hit_end_incl/rust_intervals")]
fn contains_hit_end_incl_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, END_EXCL - 1);
}

#[divan::bench(name = "contains/hit_end_incl/std_range")]
fn contains_hit_end_incl_std_range(bencher: Bencher) {
    bench_std_range(bencher, END_EXCL - 1);
}

#[divan::bench(name = "contains/miss_before/int_interval")]
fn contains_miss_before_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, START - 1);
}

#[divan::bench(name = "contains/miss_before/rust_intervals")]
fn contains_miss_before_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, START - 1);
}

#[divan::bench(name = "contains/miss_before/std_range")]
fn contains_miss_before_std_range(bencher: Bencher) {
    bench_std_range(bencher, START - 1);
}

#[divan::bench(name = "contains/miss_end_excl/int_interval")]
fn contains_miss_end_excl_int_interval(bencher: Bencher) {
    bench_int_interval(bencher, END_EXCL);
}

#[divan::bench(name = "contains/miss_end_excl/rust_intervals")]
fn contains_miss_end_excl_rust_intervals(bencher: Bencher) {
    bench_rust_intervals(bencher, END_EXCL);
}

#[divan::bench(name = "contains/miss_end_excl/std_range")]
fn contains_miss_end_excl_std_range(bencher: Bencher) {
    bench_std_range(bencher, END_EXCL);
}

fn bench_int_interval(bencher: Bencher, value: i8) {
    let interval = I8CO::try_new(START, END_EXCL).unwrap();
    bencher.bench(|| black_box(interval).contains(black_box(value)));
}

fn bench_rust_intervals(bencher: Bencher, value: i8) {
    let interval = Interval::new_closed_open(START, END_EXCL);
    bencher.bench(|| black_box(&interval).contains(black_box(value)));
}

fn bench_std_range(bencher: Bencher, value: i8) {
    let interval = START..END_EXCL;
    bencher.bench(|| black_box(&interval).contains(&black_box(value)));
}
