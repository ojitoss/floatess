use crate::DigitsStream;

#[derive(Debug, PartialEq, Eq)]
// If in the array, a nibble of 4 bits was exaclty '0', is like had no value, example [0x40] has len to 1
// and get_digit(1) give None instead of len of 2 and index 1 was Some(0), this is because than no need a use in
// the struct a boolean prop
pub struct Storage(pub Box<[u8]>);

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    HighThanNine { index: usize }
}

impl DigitsStream for Storage {
    fn amount_digits(&self) -> usize {
        let len = self.0.len() * 2;

        match self.0.last() {
            Some(last) if last & 0x0F == 0 => len - 1,
            _ => len,
        }
    }

    fn get_digit(&self, index: usize) -> Option<usize> {
        if index >= self.amount_digits() {
            return None;
        }

        let byte = self.0[index / 2];
        let nibble = if index % 2 == 0 { byte >> 4 } else { byte & 0x0F };

        if nibble == 0 { None } else { Some(nibble as usize - 1) }
    }
}

impl TryFrom<&[u8]> for Storage {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut nibbles: Vec<u8> = Vec::with_capacity(value.len() * 2);

        for i in 0..value.len() {
            let digit = value[i];

            if digit > 9 {
                Err(Error::HighThanNine { index: i })?
            }

            nibbles.push(digit + 1);
        }

        if nibbles.len() % 2 != 0 {
            nibbles.push(0);
        }

        let bytes: Vec<u8> = nibbles
            .chunks_exact(2)
            .map(|chunk| (chunk[0] << 4) | chunk[1])
            .collect();

        Ok(Storage(bytes.into_boxed_slice()))
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
        assert_eq!(Storage(Box::new([0x40])).amount_digits(), 1);
        assert_eq!(Storage(Box::new([0x21])).amount_digits(), 2);
        assert_eq!(Storage(Box::new([0x20])).amount_digits(), 1);
        assert_eq!(Storage(Box::new([0x01])).amount_digits(), 2);
    }

    #[test]
    fn get_digit() {
        assert_eq!(Storage(Box::new([0x23])).get_digit(0), Some(1));
        assert_eq!(Storage(Box::new([0x23])).get_digit(1), Some(2));
        assert_eq!(Storage(Box::new([0x23])).get_digit(2), None);
        assert_eq!(Storage(Box::new([0x40])).get_digit(0), Some(3));
        assert_eq!(Storage(Box::new([0x40])).get_digit(1), None);
        assert_eq!(Storage(Box::new([0x21])).get_digit(0), Some(1));
        assert_eq!(Storage(Box::new([0x21])).get_digit(1), Some(0));
        assert_eq!(Storage(Box::new([0x23, 0x22])).get_digit(3), Some(1));
    }

    #[test]
    fn from_slice() {
        assert_eq_from_slice!(Storage;
            [1, 2]; == Ok(Storage(Box::new([0x23]))),
            [1, 2, 4]; == Ok(Storage(Box::new([0x23, 0x50]))),
            [1, 2, 1, 1]; == Ok(Storage(Box::new([0x23, 0x22]))),
            [1, 0]; == Ok(Storage(Box::new([0x21])))
        );

        assert_eq_from_slice!(Storage;
            [1, 2, 10]; == Err(Error::HighThanNine { index: 2 })
        )
    }
}