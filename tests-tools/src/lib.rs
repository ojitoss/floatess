use std::ops::{Add};
use std::fmt::Debug;

pub fn failed_template(s: &str) -> String {
    format!("\x1b[31mFailed in '{s}' case\x1b[0m")
}

pub struct CaseOp<'a, L, R, E> {
    pub lhs: L,
    pub rhs: R,
    pub expected: E,
    pub desc: &'a str
}

impl<'a, L, R, E> CaseOp<'a, L, R, E>
where
    L: Add<R, Output = E> + Debug,
    R: Debug,
    E: PartialEq + Debug
{
    pub fn add(self) {
        assert_eq!(self.lhs + self.rhs, self.expected, "{}", failed_template(self.desc))
    }
}