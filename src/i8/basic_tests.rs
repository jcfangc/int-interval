use super::*;

mod unit_tests {
    use super::*;

    const MIN: i8 = i8::MIN;
    const MAX: i8 = i8::MAX;
    const NEG_ONE: i8 = -1;
    const ZERO: i8 = 0;
    const ONE: i8 = 1;

    mod len {
        use super::*;

        #[test]
        fn len_of_singleton_is_one() {
            let iv = I8CO::try_new(ZERO, ONE).unwrap();
            assert_eq!(iv.len(), 1);

            let iv = I8CO::try_new(MIN, MIN + ONE).unwrap();
            assert_eq!(iv.len(), 1);

            let iv = I8CO::try_new(MAX - ONE, MAX).unwrap();
            assert_eq!(iv.len(), 1);
        }

        #[test]
        fn len_matches_basic_examples() {
            let cases = [
                (MIN, MIN + ONE, 1),
                (MIN, MIN + 8, 8),
                (-5, ZERO, 5),
                (ZERO, ONE, 1),
                (ZERO, 10, 10),
                (5, 9, 4),
                (MAX - ONE, MAX, 1),
                (MIN, MAX, u8::MAX),
            ];

            for (start, end_excl, expect) in cases {
                let iv = I8CO::try_new(start, end_excl).unwrap();
                assert_eq!(iv.len(), expect, "case [{start}, {end_excl})");
            }
        }

        #[test]
        fn len_matches_iteration_count() {
            let cases = [
                (MIN, MIN + ONE),
                (MIN, MIN + 8),
                (-100, -90),
                (-5, ZERO),
                (NEG_ONE, ONE),
                (ZERO, ONE),
                (ZERO, 10),
                (10, 20),
                (MAX - ONE, MAX),
            ];

            for (start, end_excl) in cases {
                let iv = I8CO::try_new(start, end_excl).unwrap();
                assert_eq!(
                    iv.len(),
                    iv.iter().count() as u8,
                    "case [{start}, {end_excl})"
                );
            }
        }

        #[test]
        fn len_equals_end_minus_start_in_order_space() {
            const SIGN_MASK: u8 = 1u8 << (i8::BITS - 1);

            let cases = [
                (MIN, MIN + ONE),
                (MIN, NEG_ONE),
                (MIN, ZERO),
                (MIN, MAX),
                (NEG_ONE, ZERO),
                (NEG_ONE, ONE),
                (ZERO, ONE),
                (ZERO, MAX),
                (MAX - ONE, MAX),
            ];

            for (start, end_excl) in cases {
                let iv = I8CO::try_new(start, end_excl).unwrap();

                let expect = ((end_excl as u8) ^ SIGN_MASK) - ((start as u8) ^ SIGN_MASK);

                assert_eq!(iv.len(), expect, "case [{start}, {end_excl})");
            }
        }

        #[test]
        fn len_is_monotonic_under_interval_extension() {
            let small = I8CO::try_new(-5, 5).unwrap();
            let large = I8CO::try_new(-10, 10).unwrap();

            assert!(small.len() < large.len());

            let small = I8CO::try_new(MIN, -120).unwrap();
            let large = I8CO::try_new(MIN, -100).unwrap();

            assert!(small.len() < large.len());

            let small = I8CO::try_new(100, 110).unwrap();
            let large = I8CO::try_new(100, MAX).unwrap();

            assert!(small.len() < large.len());
        }

        #[test]
        fn len_of_maximal_i8_interval_is_255() {
            let iv = I8CO::try_new(MIN, MAX).unwrap();
            assert_eq!(iv.len(), u8::MAX);
        }
    }

    mod midpoint_api {
        use super::*;

        #[test]
        fn midpoint_basic() {
            let iv = span(0, 5);
            assert_eq!(iv.midpoint(), 2);

            let iv = span(-5, 5);
            assert_eq!(iv.midpoint(), 0);

            let iv = span(MIN, MAX);
            assert_eq!(iv.midpoint(), MIN + ((iv.len() / 2) as i8));
        }

        #[test]
        fn checked_from_midpoint_len_basic() {
            // odd length
            let mid = 0;
            let len = 5;
            let iv = I8CO::checked_from_midpoint_len(mid, len).unwrap();
            assert_eq!(iv.len(), len);
            assert_eq!(iv.midpoint(), mid);

            // even length
            let mid = 10;
            let len = 4;
            let iv = I8CO::checked_from_midpoint_len(mid, len).unwrap();
            assert_eq!(iv.len(), len);
            assert_eq!(iv.midpoint(), mid);

            // len=0 returns None
            assert!(I8CO::checked_from_midpoint_len(mid, 0).is_none());

            // overflow start
            assert!(I8CO::checked_from_midpoint_len(MIN, 10).is_none());

            // overflow end
            assert!(I8CO::checked_from_midpoint_len(MAX, 10).is_none());
        }

        #[test]
        fn saturating_from_midpoint_len_basic() {
            let mid = 0;
            let len = 5;
            let iv = I8CO::saturating_from_midpoint_len(mid, len).unwrap();
            assert_eq!(iv.len(), len);
            assert_eq!(iv.midpoint(), mid);

            // len=0 returns None
            assert!(I8CO::saturating_from_midpoint_len(mid, 0).is_none());

            // saturate start
            let iv = I8CO::saturating_from_midpoint_len(MIN, 10).unwrap();
            assert!(iv.start() >= MIN);

            // saturate end
            let iv = I8CO::saturating_from_midpoint_len(MAX, 10).unwrap();
            assert!(iv.end_excl() <= MAX);
        }
    }
}

mod prop_tests {
    use proptest::prelude::*;

    use super::*;

    mod len {
        use super::*;

        proptest! {
            #[test]
            fn prop_len_is_positive_for_valid_intervals(start in any::<i8>(), end in any::<i8>()) {
                if let Some(iv) = I8CO::try_new(start, end) {
                    prop_assert!(iv.len() >= 1);
                }
            }

            #[test]
            fn prop_len_matches_order_space_difference(start in any::<i8>(), end in any::<i8>()) {
                if let Some(iv) = I8CO::try_new(start, end) {
                    const SIGN_MASK: u8 = 1u8 << (i8::BITS - 1);
                    let expect = ((end as u8) ^ SIGN_MASK) - ((start as u8) ^ SIGN_MASK);
                    prop_assert_eq!(iv.len(), expect);
                }
            }
        }
    }

    mod midpoint_api {
        use super::*;

        fn mixed_scalar() -> impl Strategy<Value = i8> {
            prop_oneof![3 => prop::sample::select(&[i8::MIN, i8::MAX, 0, -1, 1]), 7 => any::<i8>()]
        }

        proptest! {
            #[test]
            fn prop_midpoint_in_bounds(start in any::<i8>(), end in any::<i8>()) {
                if let Some(iv) = I8CO::try_new(start, end) {
                    let mp = iv.midpoint();
                    prop_assert!(mp >= iv.start());
                    prop_assert!(mp <= iv.end_incl());
                }
            }

            #[test]
            fn prop_checked_from_midpoint_len_inverse(mid in mixed_scalar(), len in 1u8..=255) {
                if let Some(iv) = I8CO::checked_from_midpoint_len(mid, len) {
                    prop_assert_eq!(iv.len(), len);
                    prop_assert_eq!(iv.midpoint(), mid);
                }
            }

            #[test]
            fn prop_saturating_from_midpoint_len_safety(mid in mixed_scalar(), len in 1u8..=255) {
                if let Some(iv) = I8CO::saturating_from_midpoint_len(mid, len) {
                    prop_assert!(iv.start() < iv.end_excl());
                    prop_assert_eq!(iv.midpoint(), iv.start() + (iv.len()/2) as i8);
                }
            }
        }
    }
}
