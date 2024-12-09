// linkedmask/src/node.rs

use core::ops::BitOrAssign;

use super::{color, uint::UnsignedInteger};


pub struct Node<U>
where
    U: UnsignedInteger
{
    value:       U::T,
    next_option: Option<Box<Self>>
}

impl<U> Node<U>
where
    U: UnsignedInteger
{
    #[must_use]
    pub fn get_string(&self, indicator: bool) -> (String, u16) {
        let mut count = 1;

        let string = self.next_option.as_ref().map_or_else(String::new, |next| {
            let (string_, count_) = next.get_string(!indicator);
            count += count_;
            string_
        });

        (
            format!(
                "{string}{}{}",
                if indicator {
                    color::DARK
                } else {
                    color::LIGHT
                },
                U::format(&self.value)
            ),
            count
        )
    }

    pub fn bitor_assign_one_shl(&mut self, nth_bit: u128) {
        if nth_bit >= U::BITS {
            self.next_option
                .get_or_insert_with(Default::default)
                .bitor_assign_one_shl(nth_bit - U::BITS);
        } else {
            self.value |= U::from_u128(1 << nth_bit);
        }
    }
}

impl<U> Default for Node<U>
where
    U: UnsignedInteger
{
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self { value: U::MIN, next_option: None }
    }
}

impl<U> BitOrAssign<u128> for Node<U>
where
    U: UnsignedInteger
{
    fn bitor_assign(&mut self, rhs: u128) {
        if rhs >= U::MAX {
            **self.next_option.get_or_insert_with(Default::default) |= rhs >> U::BITS;
        } else {
            self.value |= U::from_u128(rhs);
        }
    }
}

