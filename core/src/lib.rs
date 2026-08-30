pub trait DigitStorage {
    fn len_digits(&self) -> usize;

    fn get_digit(&self, index: usize) -> Option<usize>;

    fn from_slice(slice: &[u8]) -> Self;
}

macro_rules! impl_ints_slices {
    ( $( $type:ty );* $(;)? ) => {
        $(
            impl DigitStorage for &[$type] {
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
                    let slice = Vec::from(slice).into_iter()
                        .map(| x | x as $type)
                        .collect::<Vec<$type>>();
            
                    Box::leak(slice.into_boxed_slice())
                }
            }
        )*
    };
}

impl_ints_slices!(
    u8; u16; u32; u64; u128; usize;
    i8; i16; i32; i64; i128; isize
);

macro_rules! impl_ints {
    ( $( $type:ty );* $(;)? ) => {
        $(
            impl DigitStorage for $type {
                fn len_digits(&self) -> usize {
                    if *self == 0 { return 0 };

                    self.ilog10() as usize + 1
                }

                fn get_digit(&self, index: usize) -> Option<usize> {
                    let len = self.len_digits();

                    if index >= len { None? }

                    let n = (10 as $type).pow((len - 1 - index) as u32);

                    Some(((*self / n) % 10) as usize)
                }

                fn from_slice(slice: &[u8]) -> Self {
                    let mut string = String::with_capacity(slice.len());

                    for digit in slice {
                        string.push_str(&digit.to_string());
                    }

                    string.parse::<$type>().unwrap()
                }
            }
        )*
    };
}

impl_ints!(
    u8; u16; u32; u64; u128; usize;
    i8; i16; i32; i64; i128; isize
);

impl DigitStorage for () {
    fn len_digits(&self) -> usize { 0 }

    fn get_digit(&self, _index: usize) -> Option<usize> { None }

    fn from_slice(_slice: &[u8]) -> Self {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int() {
        let cases = [
            (123u8, [1, 2, 3usize], 3usize)
        ];

        for (input, expected_digits, expected_len) in cases {
            for i in 0..expected_digits.len() {
                let expected_digit = expected_digits[i]; 

                assert_eq!(input.get_digit(i).unwrap(), expected_digit)
            }

            assert_eq!(input.len_digits(), expected_len)
        }
    }
}