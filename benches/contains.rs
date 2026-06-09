use divan::{Bencher, black_box};
use int_interval::I8CO;
use rust_intervals::Interval;

fn main() {
    divan::main();
}

const START: i8 = -32;
const END_EXCL: i8 = 96;

#[divan::bench(name = "contains/int_interval/hit_start")]
fn contains_int_interval_hit_start(bencher: Bencher) {
    bench_int_interval(bencher, START);
}

#[divan::bench(name = "contains/rust_intervals/hit_start")]
fn contains_rust_intervals_hit_start(bencher: Bencher) {
    bench_rust_intervals(bencher, START);
}

#[divan::bench(name = "contains/std_range/hit_start")]
fn contains_std_range_hit_start(bencher: Bencher) {
    bench_std_range(bencher, START);
}

#[divan::bench(name = "contains/int_interval/hit_middle")]
fn contains_int_interval_hit_middle(bencher: Bencher) {
    bench_int_interval(bencher, 16);
}

#[divan::bench(name = "contains/rust_intervals/hit_middle")]
fn contains_rust_intervals_hit_middle(bencher: Bencher) {
    bench_rust_intervals(bencher, 16);
}

#[divan::bench(name = "contains/std_range/hit_middle")]
fn contains_std_range_hit_middle(bencher: Bencher) {
    bench_std_range(bencher, 16);
}

#[divan::bench(name = "contains/int_interval/hit_end_incl")]
fn contains_int_interval_hit_end_incl(bencher: Bencher) {
    bench_int_interval(bencher, END_EXCL - 1);
}

#[divan::bench(name = "contains/rust_intervals/hit_end_incl")]
fn contains_rust_intervals_hit_end_incl(bencher: Bencher) {
    bench_rust_intervals(bencher, END_EXCL - 1);
}

#[divan::bench(name = "contains/std_range/hit_end_incl")]
fn contains_std_range_hit_end_incl(bencher: Bencher) {
    bench_std_range(bencher, END_EXCL - 1);
}

#[divan::bench(name = "contains/int_interval/miss_before")]
fn contains_int_interval_miss_before(bencher: Bencher) {
    bench_int_interval(bencher, START - 1);
}

#[divan::bench(name = "contains/rust_intervals/miss_before")]
fn contains_rust_intervals_miss_before(bencher: Bencher) {
    bench_rust_intervals(bencher, START - 1);
}

#[divan::bench(name = "contains/std_range/miss_before")]
fn contains_std_range_miss_before(bencher: Bencher) {
    bench_std_range(bencher, START - 1);
}

#[divan::bench(name = "contains/int_interval/miss_end_excl")]
fn contains_int_interval_miss_end_excl(bencher: Bencher) {
    bench_int_interval(bencher, END_EXCL);
}

#[divan::bench(name = "contains/rust_intervals/miss_end_excl")]
fn contains_rust_intervals_miss_end_excl(bencher: Bencher) {
    bench_rust_intervals(bencher, END_EXCL);
}

#[divan::bench(name = "contains/std_range/miss_end_excl")]
fn contains_std_range_miss_end_excl(bencher: Bencher) {
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
