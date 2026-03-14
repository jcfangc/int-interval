#![no_std]
#[cfg(test)]
extern crate std;

mod res;
mod u8;
mod u16;
mod u32;
mod u64;
mod u128;
mod usize;
mod i8;
mod i16;
mod i32;
mod i64;
mod i128;
mod isize;

pub use res::{OneTwo, ZeroOneTwo};
pub use u8::U8CO;
pub use u16::U16CO;
pub use u32::U32CO;
pub use u64::U64CO;
pub use u128::U128CO;
pub use usize::UsizeCO;
pub use i8::I8CO;
pub use i16::I16CO;
pub use i32::I32CO;
pub use i64::I64CO;
pub use i128::I128CO;
pub use isize::IsizeCO;
