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
    + TryFrom<u8>
    + TryFrom<u16>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<u128>
    + TryFrom<usize>
    + TryFrom<i8>
    + TryFrom<i16>
    + TryFrom<i32>
    + TryFrom<i64>
    + TryFrom<i128>
    + TryFrom<isize>
{
    // ============================================================
    // Constants
    // ============================================================

    fn zero() -> Self;
    fn one() -> Self;

    fn min_value() -> Self;
    fn max_value() -> Self;

    // ============================================================
    // Numeric conversion
    // ============================================================

    fn as_f32(self) -> f32;
    fn as_f64(self) -> f64;

    #[inline]
    fn checked_from<T>(value: T) -> Option<Self>
    where
        Self: Sized + TryFrom<T>,
    {
        Self::try_from(value).ok()
    }

    // ============================================================
    // Checked same-type arithmetic
    // ============================================================

    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn checked_sub(self, rhs: Self) -> Option<Self>;
    fn checked_mul(self, rhs: Self) -> Option<Self>;
    fn checked_div(self, rhs: Self) -> Option<Self>;
    fn checked_rem(self, rhs: Self) -> Option<Self>;

    // ============================================================
    // Checked converted arithmetic
    // ============================================================

    #[inline]
    fn checked_add_from<T>(self, rhs: T) -> Option<Self>
    where
        Self: Sized + TryFrom<T>,
    {
        self.checked_add(Self::checked_from(rhs)?)
    }

    #[inline]
    fn checked_sub_from<T>(self, rhs: T) -> Option<Self>
    where
        Self: Sized + TryFrom<T>,
    {
        self.checked_sub(Self::checked_from(rhs)?)
    }

    #[inline]
    fn checked_mul_from<T>(self, rhs: T) -> Option<Self>
    where
        Self: Sized + TryFrom<T>,
    {
        self.checked_mul(Self::checked_from(rhs)?)
    }

    #[inline]
    fn checked_div_from<T>(self, rhs: T) -> Option<Self>
    where
        Self: Sized + TryFrom<T>,
    {
        self.checked_div(Self::checked_from(rhs)?)
    }

    #[inline]
    fn checked_rem_from<T>(self, rhs: T) -> Option<Self>
    where
        Self: Sized + TryFrom<T>,
    {
        self.checked_rem(Self::checked_from(rhs)?)
    }

    // ============================================================
    // Saturating same-type arithmetic
    // ============================================================

    fn saturating_add(self, rhs: Self) -> Self;
    fn saturating_sub(self, rhs: Self) -> Self;
    fn saturating_mul(self, rhs: Self) -> Self;

    // ============================================================
    // Saturating converted arithmetic
    // ============================================================

    #[inline]
    fn saturating_add_from<T>(self, rhs: T) -> Option<Self>
    where
        Self: Sized + TryFrom<T>,
    {
        Some(self.saturating_add(Self::checked_from(rhs)?))
    }

    #[inline]
    fn saturating_sub_from<T>(self, rhs: T) -> Option<Self>
    where
        Self: Sized + TryFrom<T>,
    {
        Some(self.saturating_sub(Self::checked_from(rhs)?))
    }

    #[inline]
    fn saturating_mul_from<T>(self, rhs: T) -> Option<Self>
    where
        Self: Sized + TryFrom<T>,
    {
        Some(self.saturating_mul(Self::checked_from(rhs)?))
    }

    // ============================================================
    // Unit stepping
    // ============================================================

    #[inline]
    fn checked_next(self) -> Option<Self> {
        self.checked_add(Self::one())
    }

    #[inline]
    fn checked_prev(self) -> Option<Self> {
        self.checked_sub(Self::one())
    }

    #[inline]
    fn saturating_next(self) -> Self {
        self.saturating_add(Self::one())
    }

    #[inline]
    fn saturating_prev(self) -> Self {
        self.saturating_sub(Self::one())
    }

    // ============================================================
    // Parsing
    // ============================================================

    #[inline]
    fn parse_decimal(src: &str) -> Result<Self, ParseIntError> {
        Self::from_str(src)
    }
}

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

pub trait Unsigned: Int {}

macro_rules! impl_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(impl Unsigned for $ty {})*
    };
}

impl_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
);

impl_unsigned!(u8, u16, u32, u64, u128, usize);
