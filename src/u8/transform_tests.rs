use super::*;

mod unit_tests {
    use super::*;
    use core::u8;

    fn span(lo: u8, hi: u8) -> U8CO {
        U8CO::try_new(lo, hi).unwrap()
    }

    #[test]
    fn scale_basic() {
        let a = span(2, 5);
        let scaled = a.scale(3).unwrap();
        assert_eq!(scaled, span(6, 15));
    }

    #[test]
    fn shift_basic() {
        let a = span(1, 4);
        let shifted = a.shift(5).unwrap();
        assert_eq!(shifted, span(6, 9));
    }

    #[test]
    fn saturating_arithmetic() {
        let a = span(u8::MAX.saturating_sub(5), u8::MAX);

        let scaled = a.scale(2);
        assert!(scaled.is_none());

        let shifted = a.shift(10);
        assert!(shifted.is_none());
    }
}

mod prop_tests {
    use super::*;
    use proptest::prelude::*;
    use std::vec;

    fn edge_scalar() -> impl Strategy<Value = u8> {
        prop::sample::select(vec![0, 1, u8::MAX, u8::MAX.saturating_sub(1)])
    }

    fn mixed_scalar() -> impl Strategy<Value = u8> {
        prop_oneof! {
            3 => edge_scalar(),
            7 => any::<u8>(),
        }
    }

    fn span_strategy() -> impl Strategy<Value = U8CO> {
        (mixed_scalar(), mixed_scalar()).prop_filter_map("non-empty interval", |(a, b)| {
            let lo = a.min(b);
            let hi = a.max(b);
            U8CO::try_new(lo, hi)
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig { cases: 64, .. ProptestConfig::default() })]

        #[test]
        fn scale_preserves_order(x in span_strategy(), factor in mixed_scalar()) {
            if let Some(scaled) = x.scale(factor) {
                prop_assert!(scaled.start() < scaled.end_excl());
            }
        }

        #[test]
        fn shift_preserves_order(x in span_strategy(), offset in mixed_scalar()) {
            if let Some(shifted) = x.shift(offset) {
                prop_assert!(shifted.start() < shifted.end_excl());
            }
        }
    }
}
