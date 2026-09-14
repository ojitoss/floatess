use crate::DigitsStream;

#[derive(Debug, PartialEq, Eq)]
// If in the array, a nibble of 4 bits was exaclty '1', is like had no value, example [0x01] has len to 1
// and get_digit(1) give None instead of len of 2 and index 1 was Some(0), this is because than no need a use in
// the struct a boolean prop
pub struct Storage(pub Box<[u8]>);

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    HighThanNine { index: usize }
}

impl DigitsStream for Storage {
    fn amount_digits(&self) -> usize {
        0
    }

    fn get_digit(&self, index: usize) -> Option<usize> {
        None
    }
}

impl TryFrom<&[u8]> for Storage {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Err(Error::HighThanNine { index: 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tests_tools::assert_eq_from_slice;

    #[test]
    fn amount_digits() {
        assert_eq!(Storage(Box::new([0x23])).amount_digits(), 2);
        assert_eq!(Storage(Box::new([0x23, 0x40])).amount_digits(), 3);
        assert_eq!(Storage(Box::new([0x23, 0x22])).amount_digits(), 4);
    }

    #[test]
    fn from_slice() {
        assert_eq_from_slice!(Storage;
            [1, 2]; == Ok(Storage(Box::new([0x23]))),
            [1, 2, 4]; == Ok(Storage(Box::new([0x24, 0x50]))),
            [1, 2, 1, 1]; == Ok(Storage(Box::new([0x22, 0x22])))
        );

        assert_eq_from_slice!(Storage;
            [1, 2, 10]; == Err(Error::HighThanNine { index: 2 })
        )
    }
}