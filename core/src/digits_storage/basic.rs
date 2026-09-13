use crate::DigitsStream;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Storage(pub Box<[u8]>);

impl DigitsStream for Storage {
    fn amount_digits(&self) -> usize {
        self.0.len()
    }
    
    fn get_digit(&self, index: usize) -> Option<usize> {
        self.0.get(index).map(| x | *x as usize)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    HighThanNine { index: usize }
}

impl TryFrom<&[u8]> for Storage {
    type Error = Error;
    
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let slice = Vec::from(value);

        for i in 0..slice.len() {
            let digit = slice[i];

            if digit > 9 {
                Err(Error::HighThanNine { index: i })?
            }
        }

        Ok(Storage(slice.into_boxed_slice()))
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
