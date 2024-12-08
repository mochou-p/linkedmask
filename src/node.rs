// linkedmask/src/node.rs

use super::color;


pub(super) struct Node {
    value:       u8,
    next_option: Option<Box<Self>>
}

impl Node {
    pub(super) fn add(&mut self, n: u16) {
        if n >= 8 {
            self.next_option.get_or_insert_with(Default::default).add(n - 8);
        } else {
            self.value |= 1 << n;
        }
    }

    #[must_use]
    pub(super) fn get_string(&self, indicator: bool) -> String {
        format!(
            "{}{}{:08b}{}",
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
            self.value,
            color::RESET
        )
    }
}

impl Default for Node {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self { value: 0, next_option: None }
    }
}

