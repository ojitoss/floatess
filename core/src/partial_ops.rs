use std::fmt::Debug;
use std::ops::Add;

pub use crate::{DigitsStream, DigitsStreamUsable};

#[derive(Debug, PartialEq, Eq)]
pub struct PartialOp<T>(T);

impl<'a, T: DigitsStream + TryFrom<&'a [u8]>> Add for PartialOp<T>
where
    <T as TryFrom<&'a [u8]>>::Error: Debug,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs_len = self.0.amount_digits();
        let rhs_len = rhs.0.amount_digits();
        let max_len = usize::max(lhs_len, rhs_len);
        let mut res = vec![0u8; max_len];
        let mut carry = false;

        for offset in 0..max_len {
            let left = if offset < lhs_len {
                self.0.get_digit(lhs_len - 1 - offset).unwrap()
            } else {
                0
            };

            let right = if offset < rhs_len {
                rhs.0.get_digit(rhs_len - 1 - offset).unwrap()
            } else {
                0
            };

            let sum = left + right + carry as usize;
            res[max_len - 1 - offset] = (sum % 10) as u8;

            carry = sum >= 10;
        }

        if carry {
            res.insert(0, 1);
        }

        Self(T::try_from(Box::leak(res.into_boxed_slice())).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::{digits_storage, partial_ops::PartialOp};

    #[test]
    fn add() {
        assert_eq!(
            PartialOp(digits_storage::basic::Storage(Box::from([1, 3, 4])))
                + PartialOp(digits_storage::basic::Storage(Box::from([1]))),
            PartialOp(digits_storage::basic::Storage(Box::from([1, 3, 5])))
        );
    }
}
