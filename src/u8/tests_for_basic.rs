use super::*;

mod about_midpoint {
    use super::*;

    mod unit_tests {
        use super::*;

        fn span(lo: u8, hi: u8) -> U8CO {
            U8CO::try_new(lo, hi).unwrap()
        }

        #[test]
        fn midpoint_basic() {
            let iv = span(10, 20);
            assert_eq!(iv.midpoint(), 15);

            let iv2 = span(0, 2);
            assert_eq!(iv2.midpoint(), 1);

            let iv3 = span(0, u8::MAX);
            assert_eq!(iv3.midpoint(), (0 + ((u8::MAX - 0) / 2)));
        }

        #[test]
        fn checked_from_midpoint_len_basic() {
            let mid = 10;
            let len = 5;
            let iv = U8CO::checked_from_midpoint_len(mid, len).unwrap();
            assert_eq!(iv.len(), len);
            assert_eq!(iv.midpoint(), mid);

            // len=0 returns None
            assert!(U8CO::checked_from_midpoint_len(mid, 0).is_none());

            // overflow start returns None
            assert!(U8CO::checked_from_midpoint_len(0, 5).is_none());

            // overflow end returns None
            assert!(U8CO::checked_from_midpoint_len(u8::MAX, 10).is_none());
        }

        #[test]
        fn saturating_from_midpoint_len_basic() {
            let mid = 10;
            let len = 5;
            let iv = U8CO::saturating_from_midpoint_len(mid, len).unwrap();
            assert_eq!(iv.len(), len);
            assert_eq!(iv.midpoint(), mid);

            // len=0 returns None
            assert!(U8CO::saturating_from_midpoint_len(mid, 0).is_none());

            // saturating start
            let iv2 = U8CO::saturating_from_midpoint_len(0, 5).unwrap();
            assert_eq!(iv2.start(), 0);
            assert_eq!(iv2.len(), 5);

            // saturating end
            let iv3 = U8CO::saturating_from_midpoint_len(u8::MAX, 10).unwrap();
            assert!(iv3.end_excl() <= u8::MAX);
        }
    }

    mod prop_tests {
        use std::{vec, vec::Vec};

        use super::*;
        use proptest::prelude::*;

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

        proptest! {
            #[test]
            fn midpoint_rounds_down(x in mixed_scalar(), y in mixed_scalar()) {
                let lo = x.min(y);
                let hi = x.max(y).saturating_add(1); // ensure non-empty
                if let Some(iv) = U8CO::try_new(lo, hi) {
                    let mp = iv.midpoint();
                    // midpoint in interval and closer to start
                    prop_assert!(mp >= iv.start() && mp <= iv.end_incl());
                    prop_assert_eq!(mp, iv.start() + (iv.len() / 2));
                }
            }

            #[test]
            fn checked_from_midpoint_len_inverse(mid in mixed_scalar(), len in 1u8..20) {
                if let Some(iv) = U8CO::checked_from_midpoint_len(mid, len) {
                    prop_assert_eq!(iv.len(), len);
                    prop_assert_eq!(iv.midpoint(), mid);
                }
            }

            #[test]
            fn saturating_from_midpoint_len_safety(mid in mixed_scalar(), len in 1u8..20) {
                if let Some(iv) = U8CO::saturating_from_midpoint_len(mid, len) {
                    // start < end_excl invariant
                    prop_assert!(iv.start() < iv.end_excl());
                    prop_assert_eq!(iv.midpoint(), iv.start() + (iv.len()/2));
                }
            }
        }
    }
}
