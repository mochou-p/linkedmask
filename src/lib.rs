// linkedmask/src/lib.rs

mod color;
mod node;
mod uint;

use core::{any::type_name, fmt::{Display, Formatter, Result as FmtResult}, ops::BitOrAssign};

use {node::Node, uint::UnsignedInteger};


pub struct LinkedMask<U>
where
    U: UnsignedInteger
{
    data: Node<U>
}

impl<U> LinkedMask<U>
where
    U: UnsignedInteger
{
    #[inline]
    #[must_use]
    #[expect(clippy::new_without_default, reason = "default would be empty")]
    pub fn new() -> Self {
        Self { data: Node::<U>::default() }
    }

    #[inline]
    pub fn bitor_assign_one_shl(&mut self, nth_bit: u128) {
        self.data.bitor_assign_one_shl(nth_bit);
    }
}

impl<U> BitOrAssign<u128> for LinkedMask<U>
where
    U: UnsignedInteger
{
    #[inline]
    fn bitor_assign(&mut self, rhs: u128) {
        self.data |= rhs;
    }
}

impl<U> Display for LinkedMask<U>
where
    U: UnsignedInteger
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let (string, count) = self.data.get_string(true);

        write!(f, "{}x{count}: {string}{}", type_name::<U>(), color::RESET)
    }
}

