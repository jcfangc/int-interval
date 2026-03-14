use std::{ops::Range, sync::OnceLock};

use divan::{Bencher, black_box};
use int_interval::{OneTwo, U64CO, ZeroOneTwo};
use rust_intervals::{Interval as Ri, Pair as RiPair};

const CASES: usize = 16_384;
const DOMAIN: u64 = 1 << 20;
const BASE_LEN_MIN: u64 = 8;
const BASE_LEN_MAX: u64 = 512;

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

    #[inline]
    fn gen_range(&mut self, lo: u64, hi_excl: u64) -> u64 {
        debug_assert!(lo < hi_excl);
        lo + (self.next() % (hi_excl - lo))
    }
}

#[inline]
fn main() {
    divan::main();
}

#[inline]
fn mk_case(a: u64, b: u64, c: u64, d: u64) -> BinaryCase {
    debug_assert!(a < b);
    debug_assert!(c < d);

    BinaryCase {
        co_l: U64CO::try_new(a, b).unwrap(),
        co_r: U64CO::try_new(c, d).unwrap(),
        rg_l: a..b,
        rg_r: c..d,
        ri_l: Ri::new_closed_open(a, b),
        ri_r: Ri::new_closed_open(c, d),
    }
}

#[inline]
fn sample_base_len(rng: &mut XorShift64) -> u64 {
    rng.gen_range(BASE_LEN_MIN, BASE_LEN_MAX + 1)
}

#[inline]
fn clamp_base_start(max_end_excl: u64, len: u64, rng: &mut XorShift64) -> u64 {
    debug_assert!(len < max_end_excl);
    rng.gen_range(0, max_end_excl - len)
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
fn digest_range_opt(x: Option<Range<u64>>) -> u64 {
    x.as_ref().map_or(0, digest_range)
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

// -----------------------------------------------------------------------------
// std::ops::Range adapters
// -----------------------------------------------------------------------------

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

// -----------------------------------------------------------------------------
// scenario generators
// -----------------------------------------------------------------------------

fn build_overlap_heavy() -> Vec<BinaryCase> {
    let mut rng = XorShift64::new(0xA11C_E001);
    (0..CASES)
        .map(|_| {
            let len1 = sample_base_len(&mut rng);
            let len2 = sample_base_len(&mut rng);

            let max_len = len1.max(len2);
            let start1 = clamp_base_start(DOMAIN - 2, max_len + 2, &mut rng);
            let end1 = start1 + len1;

            let overlap = rng.gen_range(1, len1.min(len2) + 1);
            let start2_lo = start1.saturating_add(overlap).saturating_sub(len2);
            let start2_hi = end1.saturating_sub(1);
            let start2 = if start2_lo < start2_hi {
                rng.gen_range(start2_lo, start2_hi + 1)
            } else {
                start2_lo
            };
            let end2 = start2 + len2;

            mk_case(start1, end1, start2, end2)
        })
        .collect()
}

fn build_adjacent_heavy() -> Vec<BinaryCase> {
    let mut rng = XorShift64::new(0xA11C_E002);
    (0..CASES)
        .map(|_| {
            let len1 = sample_base_len(&mut rng);
            let len2 = sample_base_len(&mut rng);
            let total = len1 + len2;

            let start1 = clamp_base_start(DOMAIN - 1, total + 1, &mut rng);
            let end1 = start1 + len1;
            let start2 = end1;
            let end2 = start2 + len2;

            mk_case(start1, end1, start2, end2)
        })
        .collect()
}

fn build_disjoint_near() -> Vec<BinaryCase> {
    let mut rng = XorShift64::new(0xA11C_E003);
    (0..CASES)
        .map(|_| {
            let len1 = sample_base_len(&mut rng);
            let len2 = sample_base_len(&mut rng);
            let gap = rng.gen_range(1, 9);
            let total = len1 + gap + len2;

            let start1 = clamp_base_start(DOMAIN - 1, total + 1, &mut rng);
            let end1 = start1 + len1;
            let start2 = end1 + gap;
            let end2 = start2 + len2;

            mk_case(start1, end1, start2, end2)
        })
        .collect()
}

fn build_disjoint_far() -> Vec<BinaryCase> {
    let mut rng = XorShift64::new(0xA11C_E004);
    (0..CASES)
        .map(|_| {
            let len1 = sample_base_len(&mut rng);
            let len2 = sample_base_len(&mut rng);

            let start1 = clamp_base_start(DOMAIN / 3, len1 + 1, &mut rng);
            let end1 = start1 + len1;

            let far_lo = (DOMAIN * 2 / 3).min(DOMAIN - len2 - 1);
            let start2 = rng.gen_range(far_lo, DOMAIN - len2);
            let end2 = start2 + len2;

            mk_case(start1, end1, start2, end2)
        })
        .collect()
}

fn build_containment_heavy() -> Vec<BinaryCase> {
    let mut rng = XorShift64::new(0xA11C_E005);
    (0..CASES)
        .map(|_| {
            let outer_len = rng.gen_range(16, BASE_LEN_MAX + 64);
            let outer_start = clamp_base_start(DOMAIN - 1, outer_len + 1, &mut rng);
            let outer_end = outer_start + outer_len;

            let inner_len = rng.gen_range(1, outer_len);
            let inner_start = rng.gen_range(outer_start, outer_end - inner_len + 1);
            let inner_end = inner_start + inner_len;

            if rng.next() & 1 == 0 {
                mk_case(outer_start, outer_end, inner_start, inner_end)
            } else {
                mk_case(inner_start, inner_end, outer_start, outer_end)
            }
        })
        .collect()
}

static OVERLAP_HEAVY: OnceLock<Vec<BinaryCase>> = OnceLock::new();
static ADJACENT_HEAVY: OnceLock<Vec<BinaryCase>> = OnceLock::new();
static DISJOINT_NEAR: OnceLock<Vec<BinaryCase>> = OnceLock::new();
static DISJOINT_FAR: OnceLock<Vec<BinaryCase>> = OnceLock::new();
static CONTAINMENT_HEAVY: OnceLock<Vec<BinaryCase>> = OnceLock::new();

#[inline]
fn overlap_heavy() -> &'static [BinaryCase] {
    OVERLAP_HEAVY.get_or_init(build_overlap_heavy)
}

#[inline]
fn adjacent_heavy() -> &'static [BinaryCase] {
    ADJACENT_HEAVY.get_or_init(build_adjacent_heavy)
}

#[inline]
fn disjoint_near() -> &'static [BinaryCase] {
    DISJOINT_NEAR.get_or_init(build_disjoint_near)
}

#[inline]
fn disjoint_far() -> &'static [BinaryCase] {
    DISJOINT_FAR.get_or_init(build_disjoint_far)
}

#[inline]
fn containment_heavy() -> &'static [BinaryCase] {
    CONTAINMENT_HEAVY.get_or_init(build_containment_heavy)
}

// -----------------------------------------------------------------------------
// bench helpers
// -----------------------------------------------------------------------------

#[inline]
fn bench_intersection_co(cases: &[BinaryCase]) -> u64 {
    cases.iter().fold(0, |acc, c| {
        acc ^ digest_co_opt(black_box(c.co_l).intersection(black_box(c.co_r)))
    })
}

#[inline]
fn bench_intersection_std(cases: &[BinaryCase]) -> u64 {
    cases.iter().fold(0, |acc, c| {
        acc ^ digest_range_opt(range_intersection(black_box(&c.rg_l), black_box(&c.rg_r)))
    })
}

#[inline]
fn bench_intersection_ri(cases: &[BinaryCase]) -> u64 {
    cases.iter().fold(0, |acc, c| {
        let out = black_box(c.ri_l).intersection(black_box(c.ri_r));
        acc ^ digest_ri(&out)
    })
}

#[inline]
fn bench_union_co(cases: &[BinaryCase]) -> u64 {
    cases.iter().fold(0, |acc, c| {
        acc ^ digest_co_one_two(black_box(c.co_l).union(black_box(c.co_r)))
    })
}

#[inline]
fn bench_union_std(cases: &[BinaryCase]) -> u64 {
    cases.iter().fold(0, |acc, c| {
        acc ^ digest_range_one_two(range_union(black_box(&c.rg_l), black_box(&c.rg_r)))
    })
}

#[inline]
fn bench_union_ri(cases: &[BinaryCase]) -> u64 {
    cases.iter().fold(0, |acc, c| {
        acc ^ digest_ri_opt(black_box(c.ri_l).union(black_box(c.ri_r)))
    })
}

#[inline]
fn bench_difference_co(cases: &[BinaryCase]) -> u64 {
    cases.iter().fold(0, |acc, c| {
        acc ^ digest_co_zero_one_two(black_box(c.co_l).difference(black_box(c.co_r)))
    })
}

#[inline]
fn bench_difference_std(cases: &[BinaryCase]) -> u64 {
    cases.iter().fold(0, |acc, c| {
        acc ^ digest_range_zero_one_two(range_difference(black_box(&c.rg_l), black_box(&c.rg_r)))
    })
}

#[inline]
fn bench_difference_ri(cases: &[BinaryCase]) -> u64 {
    cases.iter().fold(0, |acc, c| {
        acc ^ digest_ri_pair(black_box(c.ri_l).difference(black_box(c.ri_r)))
    })
}

#[inline]
fn bench_symmetric_difference_co(cases: &[BinaryCase]) -> u64 {
    cases.iter().fold(0, |acc, c| {
        acc ^ digest_co_zero_one_two(black_box(c.co_l).symmetric_difference(black_box(c.co_r)))
    })
}

#[inline]
fn bench_symmetric_difference_std(cases: &[BinaryCase]) -> u64 {
    cases.iter().fold(0, |acc, c| {
        acc ^ digest_range_zero_one_two(range_symmetric_difference(
            black_box(&c.rg_l),
            black_box(&c.rg_r),
        ))
    })
}

#[inline]
fn bench_symmetric_difference_ri(cases: &[BinaryCase]) -> u64 {
    cases.iter().fold(0, |acc, c| {
        acc ^ digest_ri_pair(black_box(c.ri_l).symmetric_difference(black_box(c.ri_r)))
    })
}

// -----------------------------------------------------------------------------
// overlap_heavy
// -----------------------------------------------------------------------------

mod overlap_heavy_intersection {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_co(overlap_heavy()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_std(overlap_heavy()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_ri(overlap_heavy()));
    }
}

mod overlap_heavy_union {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_union_co(overlap_heavy()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_union_std(overlap_heavy()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_union_ri(overlap_heavy()));
    }
}

mod overlap_heavy_difference {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_co(overlap_heavy()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_std(overlap_heavy()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_ri(overlap_heavy()));
    }
}

mod overlap_heavy_symmetric_difference {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_co(overlap_heavy()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_std(overlap_heavy()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_ri(overlap_heavy()));
    }
}

// -----------------------------------------------------------------------------
// adjacent_heavy
// -----------------------------------------------------------------------------

mod adjacent_heavy_intersection {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_co(adjacent_heavy()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_std(adjacent_heavy()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_ri(adjacent_heavy()));
    }
}

mod adjacent_heavy_union {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_union_co(adjacent_heavy()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_union_std(adjacent_heavy()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_union_ri(adjacent_heavy()));
    }
}

mod adjacent_heavy_difference {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_co(adjacent_heavy()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_std(adjacent_heavy()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_ri(adjacent_heavy()));
    }
}

mod adjacent_heavy_symmetric_difference {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_co(adjacent_heavy()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_std(adjacent_heavy()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_ri(adjacent_heavy()));
    }
}

// -----------------------------------------------------------------------------
// disjoint_near
// -----------------------------------------------------------------------------

mod disjoint_near_intersection {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_co(disjoint_near()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_std(disjoint_near()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_ri(disjoint_near()));
    }
}

mod disjoint_near_union {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_union_co(disjoint_near()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_union_std(disjoint_near()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_union_ri(disjoint_near()));
    }
}

mod disjoint_near_difference {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_co(disjoint_near()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_std(disjoint_near()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_ri(disjoint_near()));
    }
}

mod disjoint_near_symmetric_difference {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_co(disjoint_near()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_std(disjoint_near()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_ri(disjoint_near()));
    }
}

// -----------------------------------------------------------------------------
// disjoint_far
// -----------------------------------------------------------------------------

mod disjoint_far_intersection {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_co(disjoint_far()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_std(disjoint_far()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_ri(disjoint_far()));
    }
}

mod disjoint_far_union {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_union_co(disjoint_far()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_union_std(disjoint_far()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_union_ri(disjoint_far()));
    }
}

mod disjoint_far_difference {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_co(disjoint_far()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_std(disjoint_far()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_ri(disjoint_far()));
    }
}

mod disjoint_far_symmetric_difference {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_co(disjoint_far()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_std(disjoint_far()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_ri(disjoint_far()));
    }
}

// -----------------------------------------------------------------------------
// containment_heavy
// -----------------------------------------------------------------------------

mod containment_heavy_intersection {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_co(containment_heavy()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_std(containment_heavy()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_intersection_ri(containment_heavy()));
    }
}

mod containment_heavy_union {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_union_co(containment_heavy()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_union_std(containment_heavy()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_union_ri(containment_heavy()));
    }
}

mod containment_heavy_difference {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_co(containment_heavy()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_std(containment_heavy()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_difference_ri(containment_heavy()));
    }
}

mod containment_heavy_symmetric_difference {
    use super::*;

    #[divan::bench]
    fn co(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_co(containment_heavy()));
    }

    #[divan::bench]
    fn std_range(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_std(containment_heavy()));
    }

    #[divan::bench]
    fn rust_intervals(bencher: Bencher) {
        bencher.bench_local(|| bench_symmetric_difference_ri(containment_heavy()));
    }
}
