// linkedmask/src/uint.rs

use core::{fmt::Debug, ops::BitOrAssign};

#[cfg(feature = "serde_")]
use serde::{Serialize, Deserialize};


pub trait UnsignedInteger: Sized {
    #[cfg(feature = "serde_")]
    type T: BitOrAssign + Clone + Copy + Debug + Serialize + for<'de> Deserialize<'de>;
    #[cfg(not(feature = "serde_"))]
    type T: BitOrAssign + Clone + Copy + Debug;

    const MIN:  Self::T;
    const MAX:  u128;
    const BITS: u128;

    fn   from_u128(value: u128)     -> Self::T;
    fn      format(value: &Self::T) -> String;
    fn  count_ones(value: &Self::T) -> u128;
    fn count_zeros(value: &Self::T) -> u128;
}

macro_rules! impl_uint {
    ($($t:tt $fstring:tt),+) => {
        $(
            impl UnsignedInteger for $t {
                type T = Self;

                const MIN:  Self::T = Self::MIN;
                const MAX:  u128    = Self::MAX  as u128;
                const BITS: u128    = Self::BITS as u128;

                #[inline]
                #[must_use]
                fn from_u128(value: u128) -> Self::T {
                    $t::try_from(value).expect("unexpected overflow")
                }

                #[inline]
                #[must_use]
                fn format(value: &Self::T) -> String {
                    format!($fstring, value)
                }

                #[inline]
                #[must_use]
                fn count_ones(value: &Self::T) -> u128 {
                    u128::from(value.count_ones())
                }

                #[inline]
                #[must_use]
                fn count_zeros(value: &Self::T) -> u128 {
                    u128::from(value.count_zeros())
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

