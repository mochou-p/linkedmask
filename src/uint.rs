// linkedmask/src/uint.rs

use core::{fmt::Binary, ops::BitOrAssign};


pub trait UnsignedInteger: Sized {
    type T: Binary + BitOrAssign;

    const MIN:  Self::T;
    const ONE:  Self;
    const BITS: u32;

    fn format(value: &Self::T)   -> String;
    fn shl   (&self, other: u32) -> Self::T;
}

macro_rules! impl_uint {
    ($($t:tt $fstring:tt),+) => {
        $(
            impl UnsignedInteger for $t {
                type T = Self;

                const MIN:  Self::T = Self::MIN;
                const ONE:  Self    = 1;
                const BITS: u32     = Self::BITS;

                #[inline]
                #[must_use]
                fn format(value: &Self::T) -> String {
                    format!($fstring, value)
                }

                #[inline]
                #[must_use]
                fn shl(&self, other: u32) -> Self::T {
                    self << other
                }
            }
        )+
    }
}

impl_uint!(
    u8   "{:08b}",
    u16  "{:016b}",
    u32  "{:032b}",
    u64  "{:064b}",
    u128 "{:0128b}"
);

#[cfg(target_pointer_width = "16")]
impl_uint!(usize "{:016}");
#[cfg(target_pointer_width = "32")]
impl_uint!(usize "{:032}");
#[cfg(target_pointer_width = "64")]
impl_uint!(usize "{:064}");

