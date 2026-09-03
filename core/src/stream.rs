mod small;
mod basic;

use std::{fmt::Display};

pub use small::SmallDigitsStream;
pub use basic::BasicDigitsStream;

pub trait DigitsStream {
    fn len_digits(&self) -> usize;

    fn get_digit(&self, index: usize) -> Option<usize>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitsStreamUsable<T>(pub T);

impl<T: DigitsStream> Display for DigitsStreamUsable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.0.len_digits();

        let post = if len > 0 {
            let mut stack = String::new();

            for i in 0..len {
                let part = self.0.get_digit(i).unwrap();

                stack.push_str(&part.to_string());
            }

            stack
        } else { 
            "0".to_string() 
        };

        write!(f, "{post}")
    }
}

pub struct WithoutDigitsStream;

impl DigitsStream for WithoutDigitsStream {
    fn len_digits(&self) -> usize { 0 }

    fn get_digit(&self, _index: usize) -> Option<usize> { None }
}

impl TryFrom<&[u8]> for WithoutDigitsStream {
    type Error = ();

    fn try_from(_value: &[u8]) -> Result<Self, Self::Error> {
        Ok(WithoutDigitsStream)
    }
}
