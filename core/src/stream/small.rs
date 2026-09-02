use crate::DigitsStream;

pub struct SmallDigitsStream<T>(pub T);

macro_rules! impl_ints {
    ( $( $type:ty );* $(;)? ) => {
        $(
            impl DigitsStream for SmallDigitsStream<$type> {
                fn len_digits(&self) -> usize {
                    let val = self.0;
                    if val == 0 { return 0 };

                    val.ilog10() as usize + 1
                }

                fn get_digit(&self, index: usize) -> Option<usize> {
                    let len = self.len_digits();
 
                    if index >= len { None? }

                    let n = (10 as $type).pow((len - 1 - index) as u32);

                    Some(((self.0 / n) % 10) as usize)
                }

                fn from_slice(slice: &[u8]) -> Self {
                    let mut res = 0;
                    let slice_len = slice.len();

                    for i in 0..slice_len {
                        let digit = slice[i];

                        res *= 10;
                        res += digit as $type;
                    }
                    
                    Self(res)
                }
            }
        )*
    };
}

impl_ints!(u8; u16; u32; u64; u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_digits() {}

    #[test]
    fn int() {
        let cases = [
            (SmallDigitsStream::<u8>(123), [1, 2, 3usize], 3usize)
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