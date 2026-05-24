pub trait Int {}
pub trait Unsigned {}

macro_rules! impl_int {
        ($($ty:ty),* $(,)?) => {
            $(impl Int for $ty {})*
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
