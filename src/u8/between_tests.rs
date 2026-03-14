use super::*;

mod unit_tests {
    use super::*;

    fn span(lo: u8, hi: u8) -> U8CO {
        U8CO::try_new(lo, hi).unwrap()
    }

    #[test]
    fn gap_exists() {
        let a = span(1, 3);
        let b = span(6, 8);
        assert_eq!(a.between(b), Some(span(3, 6)));
    }

    #[test]
    fn touching() {
        let a = span(1, 3);
        let b = span(3, 6);
        assert_eq!(a.between(b), None);
    }

    #[test]
    fn overlapping() {
        let a = span(1, 5);
        let b = span(3, 7);
        assert_eq!(a.between(b), None);
    }

    #[test]
    fn contained() {
        let a = span(1, 8);
        let b = span(3, 5);
        assert_eq!(a.between(b), None);
    }

    #[test]
    fn reversed_order_gap_exists() {
        let a = span(6, 8);
        let b = span(1, 3);
        assert_eq!(a.between(b), Some(span(3, 6)));
    }
}

mod prop_tests {
    use std::{vec, vec::Vec};

    use super::*;
    use proptest::prelude::*;

    fn span(a: u8, b: u8) -> Option<U8CO> {
        let lo = a.min(b);
        let hi = a.max(b);
        U8CO::try_new(lo, hi)
    }

    fn edge_values() -> Vec<u8> {
        let mut v = vec![u8::MIN, u8::MAX, 0, 1];

        if u8::MIN < u8::MAX {
            v.push(u8::MIN.saturating_add(1));
            v.push(u8::MAX.saturating_sub(1));
        }

        v.sort_unstable();
        v.dedup();
        v
    }

    fn edge_scalar() -> impl Strategy<Value = u8> {
        prop::sample::select(edge_values())
    }

    fn mixed_scalar() -> impl Strategy<Value = u8> {
        prop_oneof! {
            3 => edge_scalar(),
            7 => any::<u8>(),
        }
    }

    fn span_strategy() -> impl Strategy<Value = U8CO> {
        (mixed_scalar(), mixed_scalar())
            .prop_filter_map("non-empty half-open interval", |(a, b)| span(a, b))
    }

    proptest! {
        #[test]
        fn between_not_overlap(
            x in span_strategy(),
            y in span_strategy(),
            p in mixed_scalar(),
        ) {
            let mid = x.between(y);

            if let Some(mid) = mid {
                prop_assert!(!mid.intersects(x));
                prop_assert!(!mid.intersects(y));

                if mid.contains(p) {
                    prop_assert!(!x.contains(p));
                    prop_assert!(!y.contains(p));
                }
            }
        }

        #[test]
        fn between_correct_bounds(
            x in span_strategy(),
            y in span_strategy(),
        ) {
            let (left, right) = if x.start() <= y.start() { (x, y) } else { (y, x) };
            let expected = U8CO::try_new(left.end_excl(), right.start());

            prop_assert_eq!(x.between(y), expected);
        }

        #[test]
        fn between_symmetric(
            x in span_strategy(),
            y in span_strategy(),
        ) {
            prop_assert_eq!(x.between(y), y.between(x));
        }

        #[test]
        fn between_is_gap_of_hull_minus_union_when_present(
            x in span_strategy(),
            y in span_strategy(),
            p in mixed_scalar(),
        ) {
            if let Some(mid) = x.between(y) {
                let hull = x.convex_hull(y);

                if mid.contains(p) {
                    prop_assert!(hull.contains(p));
                    prop_assert!(!x.contains(p));
                    prop_assert!(!y.contains(p));
                }
            }
        }

        #[test]
        fn between_exists_iff_disjoint_and_not_adjacent(
            x in span_strategy(),
            y in span_strategy(),
        ) {
            prop_assert_eq!(
                x.between(y).is_some(),
                !x.intersects(y) && !x.is_adjacent(y),
            );
        }
    }
}
