mod impl_unsiged;
mod impl_basics;

pub trait DigitStorage {
    fn len_digits(&self) -> usize;

    fn get_digit(&self, index: usize) -> Option<usize>;

    fn from_slice(slice: &[u8]) -> Self;
}