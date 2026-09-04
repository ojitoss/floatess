use crate::DigitsStream;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmallDigitsStream<T>(pub T);

#[derive(Debug, PartialEq, Eq)]
pub enum SmallDigitsStreamError {
    HighThanNine { index: usize }
}

macro_rules! impl_unsigned {
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
                type Error = SmallDigitsStreamError;
                
                fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                    let mut res = 0;
                    let slice_len = value.len();
        
                    for i in 0..slice_len {
                        let digit = value[i];

                        if digit > 9 {
                            Err(SmallDigitsStreamError::HighThanNine { index: i })?
                        }
        
                        res *= 10;
                        res += digit as $type;
                    }
                    
                    Ok(Self(res)) 
                }
            }
        )*
    };
}

impl_unsigned!(u8; u16; u32; u64; u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_digits() {
        assert_eq!(SmallDigitsStream(0u8).len_digits(), 0);
        assert_eq!(SmallDigitsStream(123u8).len_digits(), 3);
    }

    #[test]
    fn from_slice() {
        assert_eq!(SmallDigitsStream::try_from(*&[0, 0, 0, 1].as_slice()), Ok(SmallDigitsStream::<u32>(1)));
        assert_eq!(SmallDigitsStream::try_from(*&[1, 0, 0, 0].as_slice()), Ok(SmallDigitsStream::<u32>(1000)));
        assert_eq!(SmallDigitsStream::try_from(*&[1, 2, 3].as_slice()), Ok(SmallDigitsStream::<u32>(123)));
        assert_eq!(SmallDigitsStream::<u8>::try_from(*&[1, 2, 10].as_slice()), Err(SmallDigitsStreamError::HighThanNine { index: 2 }));
    }
}