// linkedmask/src/node.rs

use super::{color, uint::UnsignedInteger};


pub(super) struct Node<U>
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
    pub(super) fn add(&mut self, n: u32) {
        if n >= U::BITS {
            self.next_option.get_or_insert_with(Default::default).add(n - U::BITS);
        } else {
            self.value |= U::ONE.shl(n);
        }
    }

    #[must_use]
    pub(super) fn get_string(&self, indicator: bool) -> String {
        format!(
            "{}{}{}{}",
            if let Some(next) = &self.next_option {
                next.get_string(!indicator)
            } else {
                String::new()
            },
            if indicator {
                color::DARK
            } else {
                color::LIGHT
            },
            U::format(&self.value),
            color::RESET
        )
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

