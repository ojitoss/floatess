pub mod small;
pub mod basic;
pub mod whitout;

use std::fmt::{Debug, Display};

pub trait DigitsStream {
    fn len_digits(&self) -> usize;

    fn get_digit(&self, index: usize) -> Option<usize>;
}

#[derive(Clone, PartialEq, Eq)]
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

impl<T: DigitsStream + Debug> Debug for DigitsStreamUsable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = format!("{}", self);

        write!(f, "({:?} -> {})", self.0, x)
    }
}