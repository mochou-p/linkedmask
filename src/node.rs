// linkedmask/src/node.rs

use core::ops::BitOrAssign;

use super::{color, uint::UnsignedInteger};

#[cfg(feature = "serde_")]
use serde::{Serialize, Deserialize};


#[cfg_attr(feature = "serde_", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
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
    pub fn from(slice: &[U::T]) -> Self {
        let (first, rest) = slice.split_first().expect("unexpected slice error");

        Self { value: *first, next_option: (!rest.is_empty()).then(|| Box::new(Self::from(rest))) }
    }

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
            let mut ptr   = self;
            let mut chunk = shiftee << offset;

            while chunk > U::MAX {
                ptr.value  |= U::from_u128(chunk & U::MAX);
                ptr         = ptr.next_option.get_or_insert_with(Default::default);
                chunk     >>= U::BITS;
            }

            ptr.value |= U::from_u128(chunk);
        }
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<U::T> {
        self.next_option.as_ref().map_or_else(|| vec![self.value], |next| {
            let mut vec = vec![self.value];
            vec.extend(next.to_vec());
            vec
        })
    }

    #[must_use]
    pub fn get(&self, index: u128) -> Option<&U::T> {
        match (index, &self.next_option) {
            (0, _         ) => Some(&self.value),
            (_, Some(next)) => next.get(index - 1),
            _               => None
        }
    }

    #[must_use]
    pub fn get_mut(&mut self, index: u128) -> Option<&mut U::T> {
        match (index, &mut self.next_option) {
            (0, _         ) => Some(&mut self.value),
            (_, Some(next)) => next.get_mut(index - 1),
            _               => None
        }
    }

    #[must_use]
    pub fn count_ones(&self) -> u128 {
        U::count_ones(&self.value) + self.next_option.as_ref().map_or(0, |next| next.count_ones())
    }

    #[must_use]
    pub fn count_zeros(&self) -> u128 {
        U::count_zeros(&self.value) + self.next_option.as_ref().map_or(0, |next| next.count_zeros())
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

