use std::{ops::Range, sync::OnceLock};

use divan::{Bencher, black_box};
use int_interval::{OneTwo, U64CO, ZeroOneTwo};
use rust_intervals::{Interval as Ri, Pair as RiPair};

const CASES: usize = 16_384;
const DOMAIN: u64 = 1 << 20;
const MAX_LEN: u64 = 1 << 10;

#[derive(Clone)]
struct UnaryCase {
    co: U64CO,
    rg: Range<u64>,
    ri: Ri<u64>,
    x: u64,
}

#[derive(Clone)]
struct BinaryCase {
    co_l: U64CO,
    co_r: U64CO,
    rg_l: Range<u64>,
    rg_r: Range<u64>,
    ri_l: Ri<u64>,
    ri_r: Ri<u64>,
}

#[derive(Clone, Debug)]
enum RangeOneTwo<T> {
    One(T),
    Two(T, T),
}

#[derive(Clone, Debug)]
enum RangeZeroOneTwo<T> {
    Zero,
    One(T),
    Two(T, T),
}

static UNARY_CASES: OnceLock<Vec<UnaryCase>> = OnceLock::new();
static BINARY_CASES: OnceLock<Vec<BinaryCase>> = OnceLock::new();

fn main() {
    divan::main();
}

fn unary_cases() -> &'static [UnaryCase] {
    UNARY_CASES.get_or_init(|| {
        let mut rng = XorShift64::new(0x5EED_CAFE_F00D_BAAD);
        (0..CASES)
            .map(|_| {
                let (lo, hi) = gen_non_empty_span(&mut rng);
                let x = rng.next() % DOMAIN;

                UnaryCase {
                    co: U64CO::try_new(lo, hi).unwrap(),
                    rg: lo..hi,
                    ri: Ri::new_closed_open(lo, hi),
                    x,
                }
            })
            .collect()
    })
}

fn binary_cases() -> &'static [BinaryCase] {
    BINARY_CASES.get_or_init(|| {
        let mut rng = XorShift64::new(0x1234_5678_ABCD_EF01);
        (0..CASES)
            .map(|_| {
                let (a, b) = gen_non_empty_span(&mut rng);
                let (c, d) = gen_non_empty_span(&mut rng);

                BinaryCase {
                    co_l: U64CO::try_new(a, b).unwrap(),
                    co_r: U64CO::try_new(c, d).unwrap(),
                    rg_l: a..b,
                    rg_r: c..d,
                    ri_l: Ri::new_closed_open(a, b),
                    ri_r: Ri::new_closed_open(c, d),
                }
            })
            .collect()
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
fn digest_range_opt(r: Option<Range<u64>>) -> u64 {
    r.as_ref().map_or(0, digest_range)
}

#[inline]
fn digest_range_one_two(x: RangeOneTwo<Range<u64>>) -> u64 {
    match x {
        RangeOneTwo::One(a) => digest_range(&a),
        RangeOneTwo::Two(a, b) => digest_range(&a) ^ digest_range(&b).rotate_left(7),
    }
}

#[inline]
fn digest_range_zero_one_two(x: RangeZeroOneTwo<Range<u64>>) -> u64 {
    match x {
        RangeZeroOneTwo::Zero => 0,
        RangeZeroOneTwo::One(a) => digest_range(&a),
        RangeZeroOneTwo::Two(a, b) => digest_range(&a) ^ digest_range(&b).rotate_left(11),
    }
}

#[inline]
fn digest_co_opt(x: Option<U64CO>) -> u64 {
    x.map_or(0, |v| mix(v.start()) ^ mix(v.end_excl()))
}

#[inline]
fn digest_co_one_two(x: OneTwo<U64CO>) -> u64 {
    match x {
        OneTwo::One(a) => mix(a.start()) ^ mix(a.end_excl()),
        OneTwo::Two(a, b) => {
            (mix(a.start()) ^ mix(a.end_excl()))
                ^ (mix(b.start()) ^ mix(b.end_excl())).rotate_left(7)
        }
    }
}

#[inline]
fn digest_co_zero_one_two(x: ZeroOneTwo<U64CO>) -> u64 {
    match x {
        ZeroOneTwo::Zero => 0,
        ZeroOneTwo::One(a) => mix(a.start()) ^ mix(a.end_excl()),
        ZeroOneTwo::Two(a, b) => {
            (mix(a.start()) ^ mix(a.end_excl()))
                ^ (mix(b.start()) ^ mix(b.end_excl())).rotate_left(11)
        }
    }
}

#[inline]
fn digest_ri(x: &Ri<u64>) -> u64 {
    if x.is_empty() {
        return 0;
    }

    let lo = x.lower().copied().unwrap_or(0);
    let hi = x.upper().copied().unwrap_or(0);
    let li = x.lower_inclusive() as u64;
    let ui = x.upper_inclusive() as u64;

    mix(lo) ^ mix(hi).rotate_left(5) ^ mix(li << 1 | ui)
}

#[inline]
fn digest_ri_opt(x: Option<Ri<u64>>) -> u64 {
    x.as_ref().map_or(0, digest_ri)
}

#[inline]
fn digest_ri_pair(x: RiPair<u64>) -> u64 {
    match x {
        RiPair::One(a) => digest_ri(&a),
        RiPair::Two(a, b) => digest_ri(&a) ^ digest_ri(&b).rotate_left(13),
    }
}

#[inline]
fn range_contains(r: &Range<u64>, x: u64) -> bool {
    r.start <= x && x < r.end
}

#[inline]
fn range_intersects(a: &Range<u64>, b: &Range<u64>) -> bool {
    !(a.end <= b.start || b.end <= a.start)
}

#[inline]
fn range_is_adjacent(a: &Range<u64>, b: &Range<u64>) -> bool {
    a.end == b.start || b.end == a.start
}

#[inline]
fn range_is_contiguous_with(a: &Range<u64>, b: &Range<u64>) -> bool {
    range_intersects(a, b) || range_is_adjacent(a, b)
}

#[inline]
fn range_intersection(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
    let start = a.start.max(b.start);
    let end = a.end.min(b.end);
    (start < end).then_some(start..end)
}

#[inline]
fn range_convex_hull(a: &Range<u64>, b: &Range<u64>) -> Range<u64> {
    a.start.min(b.start)..a.end.max(b.end)
}

#[inline]
fn range_between(a: &Range<u64>, b: &Range<u64>) -> Option<Range<u64>> {
    let (l, r) = if a.start <= b.start { (a, b) } else { (b, a) };
    (l.end < r.start).then_some(l.end..r.start)
}

#[inline]
fn range_union(a: &Range<u64>, b: &Range<u64>) -> RangeOneTwo<Range<u64>> {
    if range_is_contiguous_with(a, b) {
        RangeOneTwo::One(range_convex_hull(a, b))
    } else if a.start <= b.start {
        RangeOneTwo::Two(a.clone(), b.clone())
    } else {
        RangeOneTwo::Two(b.clone(), a.clone())
    }
}

#[inline]
fn range_difference(a: &Range<u64>, b: &Range<u64>) -> RangeZeroOneTwo<Range<u64>> {
    match range_intersection(a, b) {
        None => RangeZeroOneTwo::One(a.clone()),
        Some(inter) => {
            let left = (a.start < inter.start).then_some(a.start..inter.start);
            let right = (inter.end < a.end).then_some(inter.end..a.end);

            match (left, right) {
                (None, None) => RangeZeroOneTwo::Zero,
                (Some(x), None) | (None, Some(x)) => RangeZeroOneTwo::One(x),
                (Some(x), Some(y)) => RangeZeroOneTwo::Two(x, y),
            }
        }
    }
}

#[inline]
fn range_symmetric_difference(a: &Range<u64>, b: &Range<u64>) -> RangeZeroOneTwo<Range<u64>> {
    match range_intersection(a, b) {
        None => {
            if a.start <= b.start {
                RangeZeroOneTwo::Two(a.clone(), b.clone())
            } else {
                RangeZeroOneTwo::Two(b.clone(), a.clone())
            }
        }
        Some(inter) => {
            let hull = range_convex_hull(a, b);
            let left = (hull.start < inter.start).then_some(hull.start..inter.start);
            let right = (inter.end < hull.end).then_some(inter.end..hull.end);

            match (left, right) {
                (None, None) => RangeZeroOneTwo::Zero,
                (Some(x), None) | (None, Some(x)) => RangeZeroOneTwo::One(x),
                (Some(x), Some(y)) => RangeZeroOneTwo::Two(x, y),
            }
        }
    }
}

mod contains {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        let cases = unary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ mix(black_box(c.co).contains(black_box(c.x)) as u64)
            })
        });
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        let cases = unary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ mix(range_contains(black_box(&c.rg), black_box(c.x)) as u64)
            })
        });
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        let cases = unary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ mix(black_box(c.ri).contains(black_box(c.x)) as u64)
            })
        });
    }
}

mod intersects {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ mix(black_box(c.co_l).intersects(black_box(c.co_r)) as u64)
            })
        });
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ mix(range_intersects(black_box(&c.rg_l), black_box(&c.rg_r)) as u64)
            })
        });
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ mix(black_box(c.ri_l).intersects(black_box(c.ri_r)) as u64)
            })
        });
    }
}

mod intersection {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_co_opt(black_box(c.co_l).intersection(black_box(c.co_r)))
            })
        });
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_range_opt(range_intersection(black_box(&c.rg_l), black_box(&c.rg_r)))
            })
        });
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                let out = black_box(c.ri_l).intersection(black_box(c.ri_r));
                acc ^ digest_ri(&out)
            })
        });
    }
}

mod convex_hull {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                let out = black_box(c.co_l).convex_hull(black_box(c.co_r));
                acc ^ (mix(out.start()) ^ mix(out.end_excl()))
            })
        });
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_range(&range_convex_hull(black_box(&c.rg_l), black_box(&c.rg_r)))
            })
        });
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                let out = black_box(c.ri_l).convex_hull(black_box(c.ri_r));
                acc ^ digest_ri(&out)
            })
        });
    }
}

mod between {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_co_opt(black_box(c.co_l).between(black_box(c.co_r)))
            })
        });
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_range_opt(range_between(black_box(&c.rg_l), black_box(&c.rg_r)))
            })
        });
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                let out = black_box(c.ri_l).between(black_box(c.ri_r));
                acc ^ digest_ri(&out)
            })
        });
    }
}

mod union {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_co_one_two(black_box(c.co_l).union(black_box(c.co_r)))
            })
        });
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_range_one_two(range_union(black_box(&c.rg_l), black_box(&c.rg_r)))
            })
        });
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_ri_opt(black_box(c.ri_l).union(black_box(c.ri_r)))
            })
        });
    }
}

mod difference {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_co_zero_one_two(black_box(c.co_l).difference(black_box(c.co_r)))
            })
        });
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_range_zero_one_two(range_difference(
                    black_box(&c.rg_l),
                    black_box(&c.rg_r),
                ))
            })
        });
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_ri_pair(black_box(c.ri_l).difference(black_box(c.ri_r)))
            })
        });
    }
}

mod symmetric_difference {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_co_zero_one_two(
                    black_box(c.co_l).symmetric_difference(black_box(c.co_r)),
                )
            })
        });
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_range_zero_one_two(range_symmetric_difference(
                    black_box(&c.rg_l),
                    black_box(&c.rg_r),
                ))
            })
        });
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        let cases = binary_cases();
        bencher.bench_local(|| {
            cases.iter().fold(0_u64, |acc, c| {
                acc ^ digest_ri_pair(black_box(c.ri_l).symmetric_difference(black_box(c.ri_r)))
            })
        });
    }
}
