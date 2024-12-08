// linkedmask/src/lib.rs

mod color;
mod node;
mod uint;

use core::{any::type_name, fmt::{Display, Formatter, Result as FmtResult}};

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
    pub fn add(&mut self, n: u32) {
        self.data.add(n);
    }
}

impl<U> Display for LinkedMask<U>
where
    U: UnsignedInteger
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let (string, count) = self.data.get_string(true);

        write!(
            f,
            "{}x{count}: {string}{}",
            type_name::<U>(),
            color::RESET
        )
    }
}

