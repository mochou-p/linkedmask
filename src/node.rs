// linkedmask/src/node.rs

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
    pub fn add(&mut self, n: u32) {
        if n >= U::BITS {
            self.next_option.get_or_insert_with(Default::default).add(n - U::BITS);
        } else {
            self.value |= U::ONE.shl(n);
        }
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

