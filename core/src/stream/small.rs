use crate::DigitsStream;

#[derive(Debug, Clone, Copy)]
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
            }
            
            impl TryFrom<&[u8]> for SmallDigitsStream<$type> {
                type Error = ();
                
                fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                    let mut res = 0;
                    let slice_len = value.len();
        
                    for i in 0..slice_len {
                        let digit = value[i];
        
                        res *= 10;
                        res += digit as $type;
                    }
                    
                    Ok(Self(res)) 
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
    fn amount_digits() {
        assert_eq!(SmallDigitsStream(0u8).len_digits(), 0);
        assert_eq!(SmallDigitsStream(123u8).len_digits(), 3);
    }

    #[test]
    fn int() {
        let cases = [
            (SmallDigitsStream::<u8>(123), [1, 2, 3usize])
        ];

        for (input, expected_digits) in cases {
            for i in 0..expected_digits.len() {
                let expected_digit = expected_digits[i]; 

                assert_eq!(input.get_digit(i).unwrap(), expected_digit)
            }
        }
    }
}