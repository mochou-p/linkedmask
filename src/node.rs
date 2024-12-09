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

    pub fn bitor_assign_n_shl(&mut self, shiftee: u128, offset: u128) {
        if offset >= U::BITS {
            self.next_option
                .get_or_insert_with(Default::default)
                .bitor_assign_n_shl(shiftee, offset - U::BITS);
        } else {
            let mut ptr  = self;
            let mut frag = shiftee << offset;

            while frag > U::MAX {
                ptr.value  |= U::from_u128(frag & U::MAX);
                ptr         = ptr.next_option.get_or_insert_with(Default::default);
                frag      >>= U::BITS;
            }

            ptr.value |= U::from_u128(frag);
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
            self.value |= U::from_u128(rhs & U::MAX);
        } else {
            self.value |= U::from_u128(rhs);
        }
    }
}

