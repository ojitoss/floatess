mod small;
mod basic;

pub use small::SmallDigitsStream;
pub use basic::BasicDigitsStream;

pub trait DigitsStream {
    fn len_digits(&self) -> usize;

    fn get_digit(&self, index: usize) -> Option<usize>;
}

struct WithoutDigitsStream;

impl DigitsStream for WithoutDigitsStream {
    fn len_digits(&self) -> usize { 0 }

    fn get_digit(&self, _index: usize) -> Option<usize> { None }
}

impl TryFrom<&[u8]> for WithoutDigitsStream {
    type Error = ();

    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {
        Ok(WithoutDigitsStream)
    }
}
