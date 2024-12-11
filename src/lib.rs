// linkedmask/src/lib.rs

#![expect(clippy::module_name_repetitions, reason = "structure preference")]

mod binary;
mod color;
mod node;
mod uint;

use core::{any::type_name, fmt::{Display, Formatter, Result as FmtResult}, ops::{BitOrAssign, Index, IndexMut}};

use {node::Node, uint::UnsignedInteger};

#[cfg(feature = "serde_")]
use serde::{Serialize, Deserialize};


#[cfg_attr(feature = "serde_", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct LinkedMask<U>
where
    U: UnsignedInteger
{
    data_option: Option<Node<U>>
}

impl<U> LinkedMask<U>
where
    U: UnsignedInteger
{
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { data_option: None }
    }

    #[inline]
    #[must_use]
    pub fn from<const N: usize>(array: [U::T; N]) -> Self {
        Self { data_option: (!array.is_empty()).then(|| Node::from(&array)) }
    }

    #[inline]
    pub fn bitor_assign_n_shl(&mut self, shiftee: u128, offset: u128) {
        self.data_option
            .get_or_insert_with(Default::default)
            .bitor_assign_n_shl(shiftee, offset);
    }

    #[inline]
    pub fn bitor_assign_one_shl(&mut self, nth_bit: u128) {
        self.bitor_assign_n_shl(1, nth_bit);
    }

    #[inline]
    #[must_use]
    pub fn to_vec(&self) -> Vec<U::T> {
        self.data_option.as_ref().map_or_else(Vec::new, Node::to_vec)
    }

    #[inline]
    #[must_use]
    pub fn into_vec(self) -> Vec<U::T> {
        self.to_vec()
    }

    #[inline]
    #[must_use]
    pub fn get(&self, index: u128) -> Option<&U::T> {
        self.data_option.as_ref().and_then(|data| data.get(index))
    }

    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, index: u128) -> Option<&mut U::T> {
        self.data_option.as_mut().and_then(|data| data.get_mut(index))
    }

    #[inline]
    #[must_use]
    pub fn count_ones(&self) -> u128 {
        self.data_option.as_ref().map_or(0, Node::count_ones)
    }

    #[inline]
    #[must_use]
    pub fn count_zeros(&self) -> u128 {
        self.data_option.as_ref().map_or(0, Node::count_zeros)
    }

    #[inline]
    #[must_use]
    pub fn leading_ones(&self) -> u128 {
        self.data_option.as_ref().map_or(0, Node::leading_ones)
    }

    #[inline]
    #[must_use]
    pub fn leading_zeros(&self) -> u128 {
        self.data_option.as_ref().map_or(0, Node::leading_zeros)
    }

    #[inline]
    #[must_use]
    pub fn trailing_ones(&self) -> u128 {
        self.data_option.as_ref().map_or(0, Node::trailing_ones)
    }

    #[inline]
    #[must_use]
    pub fn trailing_zeros(&self) -> u128 {
        self.data_option.as_ref().map_or(0, Node::trailing_zeros)
    }
}

impl<U> Default for LinkedMask<U>
where
    U: UnsignedInteger
{
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self { data_option: Some(Node::default()) }
    }
}

impl<U> FromIterator<U::T> for LinkedMask<U>
where
    U: UnsignedInteger
{
    #[inline]
    #[must_use]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = U::T>
    {
        let vec = iter.into_iter().collect::<Vec<U::T>>();

        Self { data_option: (!vec.is_empty()).then(|| Node::from(&vec)) }
    }
}

impl<U> Index<u128> for LinkedMask<U>
where
    U: UnsignedInteger
{
    type Output = U::T;

    #[inline]
    #[must_use]
    fn index(&self, index: u128) -> &Self::Output {
        self.get(index).unwrap_or_else(|| panic!("linkedmask has less parts than {}", index + 1))
    }
}

impl<U> IndexMut<u128> for LinkedMask<U>
where
    U: UnsignedInteger
{
    #[inline]
    #[must_use]
    fn index_mut(&mut self, index: u128) -> &mut Self::Output {
        self.get_mut(index).unwrap_or_else(|| panic!("linkedmask has less parts than {}", index + 1))
    }
}

impl<U> BitOrAssign<u128> for LinkedMask<U>
where
    U: UnsignedInteger
{
    #[inline]
    fn bitor_assign(&mut self, rhs: u128) {
        *self.data_option
            .get_or_insert_with(Default::default) |= rhs;
    }
}

impl<U> Display for LinkedMask<U>
where
    U: UnsignedInteger
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if let Some(data) = &self.data_option {
            let (string, count) = data.get_string(true);

            write!(f, "{}x{count}: {string}{}", type_name::<U>(), color::RESET)
        } else {
            write!(f, "{}x0: empty", type_name::<U>())
        }
    }
}

