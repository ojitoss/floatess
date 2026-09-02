pub mod stream;

pub trait DigitsStream {
    fn len_digits(&self) -> usize;

    fn get_digit(&self, index: usize) -> Option<usize>;

    fn from_slice(slice: &[u8]) -> Self;
}

impl DigitsStream for () {
    fn len_digits(&self) -> usize { 0 }

    fn get_digit(&self, _index: usize) -> Option<usize> { None }

    fn from_slice(_slice: &[u8]) -> Self {}
}