use core::fmt;
use core::ops::{Add};

#[derive(Debug, PartialEq, Eq)]
pub struct Decimal<'a> {
    pub pre: u32,
    pub post: &'a [u8]
}

impl fmt::Display for Decimal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.post.len();

        let post = if len > 0 {
            let mut stack = String::new();
            let mut i = len - 1;

            loop {
                // usize can't be lower than zero, so this had the same function than had a 'i < 0' but using an overflow
                if i == usize::MAX { break };
    
                let part = self.post[i];
    
                stack.push_str(&part.to_string());
                i = i.wrapping_sub(1);
            }

            stack
        } else { "0".to_string() };

        write!(f, "{}.{}", self.pre, post)
    }
}

impl<'a> Add for Decimal<'a> {
    type Output = Decimal<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut add_one_next = false;
        let len = self.post.len();
        let mut res = Vec::<u8>::with_capacity(len);

        for i in 0..len {
            let left = self.post[i];
            let right = rhs.post[i];

            let sum = left + right + if add_one_next { 1 } else { 0 };
            let low_ten = sum < 10;
            let to_push = if low_ten { sum } else { sum - 10 };

            res.push(to_push);

            // to keep the last sum to 'pre' prop
            if i < len - 1 {
                add_one_next = !low_ten;
            }
        }

        Self { 
            pre: self.pre + rhs.pre + if add_one_next { 1 } else { 0 },
            post: Box::leak(res.into_boxed_slice())
         }
    }
}

impl<'a> Decimal<'a> {
    pub fn str_to_decimal(string: &'a str) -> Self {
        let mut pre = String::new();
        let mut post = Vec::<u8>::new();

        enum State {
            Pre,
            Post
        }

        let mut state = State::Pre;

        for ch in string.chars() {
            let ch_u8 = ch as u8;

            if (ch_u8 < b'0' || ch_u8 > b'9') && ch_u8 != b'.' {
                panic!("{}", format!("'{ch}' is not valid digit"))
            }

            match state {
                State::Pre => {
                    if ch_u8 == b'.' {
                        state = State::Post;

                        continue;
                    }

                    pre.push(ch);
                }
                State::Post => {
                    if ch_u8 == b'.' {
                        panic!("insert more than one '.'")
                    }

                    post.push(ch.to_string().parse().unwrap());
                }
            }
        }

        Self { 
            pre: pre.parse().unwrap(),
            post: Box::leak(post.into_iter().rev().collect::<Vec<u8>>().into_boxed_slice())
         }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let cases = [
            ("1.45", "1.45", Decimal { pre: 1, post: &[5, 4] }),
            ("1.", "1.0", Decimal { pre: 1, post: &[] }),
            ("1", "1.0", Decimal { pre: 1, post: &[] })
        ];

        for (input_num, expected_str, expected_decimal) in cases {
            let decimal = Decimal::str_to_decimal(input_num);

            assert_eq!(decimal, expected_decimal);
            assert_eq!(format!("{decimal}"), expected_str);
        }
    }
}