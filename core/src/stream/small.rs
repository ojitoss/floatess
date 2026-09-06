use crate::DigitsStream;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmallDigitsStream<T>(pub T);

#[derive(Debug, PartialEq, Eq)]
pub enum SmallDigitsStreamError {
    HighThanNine { index: usize },
    OverflowDigitsAmount,
    OverflowInSpecificDigit
}

trait RangeLimit<const N: usize>: Sized {
    const VALID_DIGITS: [u8; N];
    const AMOUNT_VALID_DIGITS: usize = N;
}

macro_rules! impl_unsigned {
    ( $( $type:ty => { 
        DIGITS => $DIGITS:expr,
        AMOUNT => $AMOUNT:literal 
    });* $(;)? ) => {
        $(
            impl RangeLimit<$AMOUNT> for $type {
                const VALID_DIGITS: [u8; $AMOUNT] = $DIGITS;
            }

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

                    if slice_len > <$type as RangeLimit<$AMOUNT>>::AMOUNT_VALID_DIGITS {
                        Err(SmallDigitsStreamError::OverflowDigitsAmount)?
                    }

                    if slice_len == <$type as RangeLimit<$AMOUNT>>::AMOUNT_VALID_DIGITS {
                        for i in 0..slice_len {
                            #[allow(non_snake_case)]
                            let DIGIT = <$type as RangeLimit<$AMOUNT>>::VALID_DIGITS[i];
                            let digit = value[i];

                            if digit > DIGIT {
                                Err(SmallDigitsStreamError::OverflowInSpecificDigit)?
                            }
                        }
                    }
        
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

impl_unsigned!(
    u8 => {
        DIGITS => [2,5,5],
        AMOUNT => 3
    }; 
    u16 => {
        DIGITS => [6,5,5,3,5],
        AMOUNT => 5
    }; 
    u32 => {
        DIGITS => [4,2,9,4,9,6,7,2,9,5],
        AMOUNT => 10
    }; 
    u64 => {
        DIGITS => [1,8,4,4,6,7,4,4,0,7,3,7,0,9,5,5,1,6,1,5],
        AMOUNT => 20
    }; 
    u128 => {
        DIGITS => [3,4,0,2,8,2,3,6,6,9,2,0,9,3,8,4,6,3,4,6,3,3,7,4,6,0,7,4,3,1,7,6,8,2,1,1,4,5,5],
        AMOUNT => 39
    }
);

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
        assert_eq!(SmallDigitsStream::<u32>::try_from(*&[0, 0, 0, 1].as_slice()), Ok(SmallDigitsStream(1)));
        assert_eq!(SmallDigitsStream::<u32>::try_from(*&[1, 0, 0, 0].as_slice()), Ok(SmallDigitsStream(1000)));
        assert_eq!(SmallDigitsStream::<u32>::try_from(*&[1, 2, 3].as_slice()), Ok(SmallDigitsStream(123)));
        assert_eq!(SmallDigitsStream::<u8>::try_from(*&[1, 2, 10].as_slice()), Err(SmallDigitsStreamError::HighThanNine { index: 2 }));
    }
}