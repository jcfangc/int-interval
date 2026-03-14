use super::*;

mod unit_tests {
    use super::*;

    fn span(a: u8, b: u8) -> U8CO {
        U8CO::try_new(a, b).unwrap()
    }

    #[test]
    fn overlapping() {
        let a = span(2, 6);
        let b = span(4, 8);

        match a.symmetric_difference(b) {
            ZeroOneTwo::Two(l, r) => {
                assert_eq!(l, span(2, 4));
                assert_eq!(r, span(6, 8));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn disjoint() {
        let a = span(1, 3);
        let b = span(5, 7);

        match a.symmetric_difference(b) {
            ZeroOneTwo::Two(l, r) => {
                assert_eq!(l, a);
                assert_eq!(r, b);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn containment() {
        let a = span(1, 10);
        let b = span(4, 6);

        match a.symmetric_difference(b) {
            ZeroOneTwo::Two(l, r) => {
                assert_eq!(l, span(1, 4));
                assert_eq!(r, span(6, 10));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn equal_spans() {
        let a = span(3, 7);
        let b = span(3, 7);

        assert_eq!(a.symmetric_difference(b), ZeroOneTwo::Zero);
    }

    #[test]
    fn adjacent_are_not_merged() {
        let a = span(1, 3);
        let b = span(3, 5);

        match a.symmetric_difference(b) {
            ZeroOneTwo::Two(l, r) => {
                assert_eq!(l, a);
                assert_eq!(r, b);
            }
            _ => panic!(),
        }
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
        prop_oneof![
            3 => edge_scalar(),
            7 => any::<u8>(),
        ]
    }

    fn span_strategy() -> impl Strategy<Value = U8CO> {
        (mixed_scalar(), mixed_scalar()).prop_filter_map("non-empty span", |(a, b)| span(a, b))
    }

    fn contains_zot(xs: ZeroOneTwo<U8CO>, p: u8) -> bool {
        match xs {
            ZeroOneTwo::Zero => false,
            ZeroOneTwo::One(z) => z.contains(p),
            ZeroOneTwo::Two(l, r) => l.contains(p) || r.contains(p),
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 64,
            .. ProptestConfig::default()
        })]

        #[test]
        fn xor_matches_definition(
            x in span_strategy(),
            y in span_strategy(),
            p in mixed_scalar(),
        ) {
            let actual = contains_zot(x.symmetric_difference(y), p);
            let expected = x.contains(p) ^ y.contains(p);

            prop_assert_eq!(actual, expected);
        }

        #[test]
        fn commutative_as_set(
            x in span_strategy(),
            y in span_strategy(),
            p in mixed_scalar(),
        ) {
            let lhs = contains_zot(x.symmetric_difference(y), p);
            let rhs = contains_zot(y.symmetric_difference(x), p);

            prop_assert_eq!(lhs, rhs);
        }

        #[test]
        fn xor_never_contains_overlap(
            x in span_strategy(),
            y in span_strategy(),
            p in mixed_scalar(),
        ) {
            if x.contains(p) && y.contains(p) {
                prop_assert!(!contains_zot(x.symmetric_difference(y), p));
            }
        }

        #[test]
        fn xor_two_parts_do_not_overlap(
            x in span_strategy(),
            y in span_strategy(),
        ) {
            match x.symmetric_difference(y) {
                ZeroOneTwo::Two(l, r) => {
                    prop_assert!(!l.intersects(r));
                    prop_assert!(!r.intersects(l));
                }
                _ => {}
            }
        }

        #[test]
        fn xor_equals_union_of_differences(
            x in span_strategy(),
            y in span_strategy(),
            p in mixed_scalar(),
        ) {
            let xor = contains_zot(x.symmetric_difference(y), p);

            let d1 = contains_zot(x.difference(y), p);
            let d2 = contains_zot(y.difference(x), p);

            prop_assert_eq!(xor, d1 || d2);
        }
    }
}
