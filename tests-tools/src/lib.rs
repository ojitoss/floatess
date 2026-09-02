use std::ops::{Add};
use std::fmt::Debug;

pub fn failed_template(s: &str) -> String {
    format!("\x1b[31mFailed in '{s}' case\x1b[0m")
}

pub struct CaseOp<'a, Lhs, Rhs, Exp> {
    pub lhs: Lhs,
    pub rhs: Rhs,
    pub expected: Exp,
    pub desc: &'a str
}

impl<'a, L, R, E> CaseOp<'a, L, R, E>
where
    L: Add<R, Output = E> + Debug,
    R: Debug,
    E: PartialEq + Debug
{
    pub fn add<F>(self, callback: F) where F: Fn(&L, &R, &E, &str) -> String {
        let desc = callback(&self.lhs, &self.rhs, &self.expected, self.desc);

        assert_eq!(self.lhs + self.rhs, self.expected, "{}", failed_template(&desc));
    }
}