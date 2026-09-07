use crate::DigitsStream;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Storage<'a>(pub &'a [u8]);

impl<'a> DigitsStream for Storage<'a> {
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

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    HighThanNine { index: usize }
}

impl<'a> TryFrom<&[u8]> for Storage<'a> {
    type Error = Error;
    
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let slice = Vec::from(value);

        for i in 0..slice.len() {
            let digit = slice[i];

            if digit > 9 {
                Err(Error::HighThanNine { index: i })?
            }
        }

        let owned_slice = Box::leak(slice.into_boxed_slice());

        Ok(Storage(owned_slice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn erros() {
        assert_eq!(Storage::try_from(*&[1, 2, 3, 10].as_slice()), Err(Error::HighThanNine { index: 3 }));
    }
}