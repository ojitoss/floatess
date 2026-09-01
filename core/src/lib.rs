pub mod impl_unsiged;
pub mod impl_basics;

pub trait DigitStorage {
    fn len_digits(&self) -> usize;

    fn get_digit(&self, index: usize) -> Option<usize>;

    fn from_slice(slice: &[u8]) -> Self;
}

impl DigitStorage for () {
    fn len_digits(&self) -> usize { 0 }

    fn get_digit(&self, _index: usize) -> Option<usize> { None }

    fn from_slice(_slice: &[u8]) -> Self {}
}