use std::ops::Add;

pub use crate::{DigitsStream, DigitsStreamUsable};

#[derive(Debug, PartialEq, Eq)]
pub struct PartialOp<T>(T);

impl<T: DigitsStream> Add for PartialOp<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{digits_storage, partial_ops::{PartialOp}};

    #[test]
    fn x() {
        assert_eq!(
            PartialOp(digits_storage::basic::Storage(Box::from([1, 3, 4])))
            + PartialOp(digits_storage::basic::Storage(Box::from([1]))),
            PartialOp(digits_storage::basic::Storage(Box::from([1, 3, 5])))
        );
    }
}