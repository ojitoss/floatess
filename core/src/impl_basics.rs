use crate::DigitStorage;

impl DigitStorage for &[u8] {
    fn len_digits(&self) -> usize {
        self.len()
    }
    
    fn get_digit(&self, index: usize) -> Option<usize> {
        match self.get(index) {
            Some(n) => Some(*n as usize),
            None => None
            }
    }
    
    fn from_slice(slice: &[u8]) -> Self {
        let slice = Vec::from(slice);
            
            Box::leak(slice.into_boxed_slice())
    }
}