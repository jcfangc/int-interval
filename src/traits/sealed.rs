use core::{
    fmt::Debug,
    iter::Sum,
    num::ParseIntError,
    ops::{Add, Div, Mul, Rem, Sub},
    str::FromStr,
};

pub trait Int:
    Copy
    + Ord
    + Eq
    + Debug
    + Send
    + Sync
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Sum<Self>
    + Default
    + FromStr<Err = ParseIntError>
{
    fn zero() -> Self;
    fn one() -> Self;

    fn min_value() -> Self;
    fn max_value() -> Self;

    fn as_f32(self) -> f32;
    fn as_f64(self) -> f64;

    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn checked_rem(self, rhs: Self) -> Option<Self>;

    fn saturating_add(self, rhs: Self) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;
    fn saturating_mul(self, rhs: Self) -> Self;

    #[inline]
    fn checked_next(self) -> Option<Self> {
        self.checked_add(Self::one())
    }

    #[inline]
    fn parse_decimal(src: &str) -> Result<Self, ParseIntError> {
        Self::from_str(src)
    }
}

pub trait Unsigned: Int {}

macro_rules! impl_int {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Int for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }

                #[inline]
                fn one() -> Self {
                    1
                }

                #[inline]
                fn min_value() -> Self {
                    <$ty>::MIN
                }

                #[inline]
                fn max_value() -> Self {
                    <$ty>::MAX
                }

                #[inline]
                fn as_f32(self) -> f32 {
                    self as f32
                }

                #[inline]
                fn as_f64(self) -> f64 {
                    self as f64
                }

                #[inline]
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    <$ty>::checked_add(self, rhs)
                }

                #[inline]
                fn checked_sub(self, rhs: Self) -> Option<Self> {
                    <$ty>::checked_sub(self, rhs)
                }

                #[inline]
                fn checked_mul(self, rhs: Self) -> Option<Self> {
                    <$ty>::checked_mul(self, rhs)
                }

                #[inline]
                fn checked_div(self, rhs: Self) -> Option<Self> {
                    <$ty>::checked_div(self, rhs)
                }

                #[inline]
                fn checked_rem(self, rhs: Self) -> Option<Self> {
                    <$ty>::checked_rem(self, rhs)
                }

                #[inline]
                fn saturating_add(self, rhs: Self) -> Self {
                    <$ty>::saturating_add(self, rhs)
                }

                #[inline]
                fn saturating_sub(self, rhs: Self) -> Self {
                    <$ty>::saturating_sub(self, rhs)
                }

                #[inline]
                fn saturating_mul(self, rhs: Self) -> Self {
                    <$ty>::saturating_mul(self, rhs)
                }
            }
        )*
    };
}

macro_rules! impl_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(impl Unsigned for $ty {})*
    };
}

impl_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
);

impl_unsigned!(u8, u16, u32, u64, u128, usize);
