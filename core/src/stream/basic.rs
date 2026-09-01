use crate::DigitStorage;

#[derive(Debug, PartialEq)]
pub struct BasicDecimalStream<'a>(pub &'a [u8]);

impl<'a> DigitStorage for BasicDecimalStream<'a> {
    fn len_digits(&self) -> usize {
        self.0.len()
    }
    
    fn get_digit(&self, index: usize) -> Option<usize> {
        match self.0.get(index) {
            Some(n) => Some(*n as usize),
            None => None
        }
    }
    
    fn from_slice(slice: &[u8]) -> Self {
        let slice = Vec::from(slice);
        let owned_slice = Box::leak(slice.into_boxed_slice());

        BasicDecimalStream(owned_slice)
    }
}