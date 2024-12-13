// linkedmask/src/binary.rs

pub trait BinaryHelpers {
    fn not(&mut self);

    fn    count_1s(&self) -> u128;
    fn    count_0s(&self) -> u128;
    fn  leading_1s(&self) -> u128;
    fn  leading_0s(&self) -> u128;
    fn trailing_1s(&self) -> u128;
    fn trailing_0s(&self) -> u128;
}

macro_rules! impl_bh {
    ($($t:tt),+) => {
        $(
            impl BinaryHelpers for $t {
                #[inline]
                fn not(&mut self) {
                    *self = !*self
                }

                #[inline]
                #[must_use]
                fn count_1s(&self) -> u128 {
                    u128::from(self.count_ones())
                }

                #[inline]
                #[must_use]
                fn count_0s(&self) -> u128 {
                    u128::from(self.count_zeros())
                }

                #[inline]
                #[must_use]
                fn leading_1s(&self) -> u128 {
                    u128::from(self.leading_ones())
                }

                #[inline]
                #[must_use]
                fn leading_0s(&self) -> u128 {
                    u128::from(self.leading_zeros())
                }

                #[inline]
                #[must_use]
                fn trailing_1s(&self) -> u128 {
                    u128::from(self.trailing_ones())
                }

                #[inline]
                #[must_use]
                fn trailing_0s(&self) -> u128 {
                    u128::from(self.trailing_zeros())
                }
            }
        )+
    }
}

impl_bh!(u8, u16, u32, u64, u128, usize);

