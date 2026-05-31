use core::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, Sub},
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
    + Sum<Self>
    + Default
{
    fn as_f32(self) -> f32;
    fn as_f64(self) -> f64;
}

pub trait Unsigned: Int {}

macro_rules! impl_int {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Int for $ty {
                #[inline]
                fn as_f32(self) -> f32 {
                    self as f32
                }

                #[inline]
                fn as_f64(self) -> f64 {
                    self as f64
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
