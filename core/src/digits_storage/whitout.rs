use crate::DigitsStream;

pub struct Storage;

impl DigitsStream for Storage {
    fn len_digits(&self) -> usize { 0 }

    fn get_digit(&self, _index: usize) -> Option<usize> { None }
}

impl TryFrom<&[u8]> for Storage {
    type Error = ();

    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Storage)
    }
}