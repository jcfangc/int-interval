use std::{ops::Range, sync::OnceLock};

use divan::{Bencher, black_box};
use int_interval::U64CO;
use rust_intervals::Interval as Ri;

const CASES: usize = 16_384;
const DOMAIN: u64 = 1 << 20;
const MAX_LEN: u64 = 1 << 10;

static SPANS: OnceLock<Box<[(u64, u64)]>> = OnceLock::new();

fn main() {
    divan::main();
}

fn spans() -> &'static [(u64, u64)] {
    SPANS.get_or_init(|| {
        let mut rng = XorShift64::new(0xD1CE_BA5E_F00D_1234);

        (0..CASES)
            .map(|_| gen_non_empty_span(&mut rng))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    })
}

#[derive(Clone, Copy)]
struct XorShift64(u64);

impl XorShift64 {
    #[inline]
    const fn new(seed: u64) -> Self {
        Self(seed)
    }

    #[inline]
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

#[inline]
fn gen_non_empty_span(rng: &mut XorShift64) -> (u64, u64) {
    let start = rng.next() % (DOMAIN - MAX_LEN - 1);
    let len = (rng.next() % MAX_LEN) + 1;
    (start, start + len)
}

#[inline]
fn mix(x: u64) -> u64 {
    x.wrapping_mul(0x9E37_79B9_7F4A_7C15).rotate_left(17)
}

#[inline]
fn digest_range(r: &Range<u64>) -> u64 {
    mix(r.start) ^ mix(r.end)
}

#[inline]
fn digest_co(x: U64CO) -> u64 {
    mix(x.start()) ^ mix(x.end_excl())
}

#[inline]
fn digest_ri(x: &Ri<u64>) -> u64 {
    let lo = x.lower().copied().unwrap_or(0);
    let hi = x.upper().copied().unwrap_or(0);
    let li = x.lower_inclusive() as u64;
    let ui = x.upper_inclusive() as u64;

    mix(lo) ^ mix(hi).rotate_left(5) ^ mix((li << 1) | ui)
}

#[divan::bench]
fn co(bencher: Bencher) {
    let spans = spans();

    bencher.bench_local(|| {
        spans.iter().fold(0_u64, |acc, &(start, end_excl)| {
            let x = U64CO::try_new(black_box(start), black_box(end_excl)).unwrap();
            acc ^ digest_co(black_box(x))
        })
    });
}

#[divan::bench]
fn std_range(bencher: Bencher) {
    let spans = spans();

    bencher.bench_local(|| {
        spans.iter().fold(0_u64, |acc, &(start, end_excl)| {
            let x = black_box(start)..black_box(end_excl);
            acc ^ digest_range(black_box(&x))
        })
    });
}

#[divan::bench]
fn rust_intervals(bencher: Bencher) {
    let spans = spans();

    bencher.bench_local(|| {
        spans.iter().fold(0_u64, |acc, &(start, end_excl)| {
            let x = Ri::new_closed_open(black_box(start), black_box(end_excl));
            acc ^ digest_ri(black_box(&x))
        })
    });
}
