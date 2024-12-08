// linkedmask/src/lib.rs

mod color;
mod node;

use core::fmt::{Display, Formatter, Result as FmtResult};

use node::Node;


pub struct LinkedMask {
    data: Node
}

impl LinkedMask {
    #[inline]
    #[must_use]
    #[expect(clippy::new_without_default, reason = "default would be empty")]
    pub fn new() -> Self {
        Self { data: Node::default() }
    }

    #[inline]
    pub fn add(&mut self, n: u16) {
        self.data.add(n);
    }
}

impl Display for LinkedMask {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.data.get_string(true))
    }
}

