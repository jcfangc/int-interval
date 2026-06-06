use crate::traits::IntPrimitive;

#[test]
fn checked_from_accepts_representable_values() {
    assert_eq!(i8::checked_from(127_i16), Some(127_i8));
    assert_eq!(u8::checked_from(255_u16), Some(255_u8));
    assert_eq!(i128::checked_from(usize::MAX), Some(usize::MAX as i128));
}

#[test]
fn checked_from_rejects_unrepresentable_values() {
    assert_eq!(i8::checked_from(128_i16), None);
    assert_eq!(u8::checked_from(-1_i16), None);
    assert_eq!(usize::checked_from(-1_i32), None);
}

#[test]
fn checked_add_from_converts_then_adds_checked() {
    assert_eq!(10_i8.checked_add_from(5_u8), Some(15_i8));
    assert_eq!(120_i8.checked_add_from(10_u8), None);
    assert_eq!(0_u8.checked_add_from(-1_i8), None);
}

#[test]
fn checked_sub_from_converts_then_subtracts_checked() {
    assert_eq!(10_i8.checked_sub_from(5_u8), Some(5_i8));
    assert_eq!((-120_i8).checked_sub_from(7_u8), Some(-127_i8));
    assert_eq!((-120_i8).checked_sub_from(9_u8), None);
    assert_eq!(0_u8.checked_sub_from(1_u8), None);
}

#[test]
fn checked_mul_from_converts_then_multiplies_checked() {
    assert_eq!(12_i8.checked_mul_from(10_u8), Some(120_i8));
    assert_eq!(13_i8.checked_mul_from(10_u8), None);
    assert_eq!(10_u8.checked_mul_from(-1_i8), None);
}

#[test]
fn checked_div_from_converts_then_divides_checked() {
    assert_eq!(120_i8.checked_div_from(10_u8), Some(12_i8));
    assert_eq!(120_i8.checked_div_from(0_u8), None);
    assert_eq!(10_u8.checked_div_from(-1_i8), None);
}

#[test]
fn checked_rem_from_converts_then_remainders_checked() {
    assert_eq!(121_i8.checked_rem_from(10_u8), Some(1_i8));
    assert_eq!(121_i8.checked_rem_from(0_u8), None);
    assert_eq!(10_u8.checked_rem_from(-1_i8), None);
}

#[test]
fn checked_next_and_prev_respect_bounds() {
    assert_eq!(0_i8.checked_next(), Some(1_i8));
    assert_eq!(i8::MAX.checked_next(), None);

    assert_eq!(0_i8.checked_prev(), Some(-1_i8));
    assert_eq!(i8::MIN.checked_prev(), None);

    assert_eq!(0_u8.checked_prev(), None);
}

#[test]
fn saturating_next_and_prev_respect_bounds() {
    assert_eq!(0_i8.saturating_next(), 1_i8);
    assert_eq!(i8::MAX.saturating_next(), i8::MAX);

    assert_eq!(0_i8.saturating_prev(), -1_i8);
    assert_eq!(i8::MIN.saturating_prev(), i8::MIN);

    assert_eq!(0_u8.saturating_prev(), 0_u8);
}

#[test]
fn saturating_from_converts_then_saturates() {
    assert_eq!(120_i8.saturating_add_from(10_u8), Some(127_i8));
    assert_eq!((-120_i8).saturating_sub_from(20_u8), Some(-128_i8));
    assert_eq!(20_i8.saturating_mul_from(10_u8), Some(127_i8));

    assert_eq!(10_u8.saturating_add_from(-1_i8), None);
}

#[test]
fn checked_from_matches_standard_try_from() {
    let values = [-129_i16, -128, -1, 0, 1, 127, 128];

    for value in values {
        assert_eq!(i8::checked_from(value), i8::try_from(value).ok());
    }
}
