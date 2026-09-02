use crate::DigitsStream;

#[derive(Debug, PartialEq, Clone)]
pub struct BasicDigitsStream<'a>(pub &'a [u8]);

impl<'a> DigitsStream for BasicDigitsStream<'a> {
    fn len_digits(&self) -> usize {
        self.0.len()
    }
    
    fn get_digit(&self, index: usize) -> Option<usize> {
        match self.0.get(index) {
            Some(n) => Some(*n as usize),
            None => None
        }
    }
}

impl<'a> TryFrom<&[u8]> for BasicDigitsStream<'a> {
    type Error = ();
    
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let slice = Vec::from(value);
        let owned_slice = Box::leak(slice.into_boxed_slice());
        
        Ok(BasicDigitsStream(owned_slice))
    }
}