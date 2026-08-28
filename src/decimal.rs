pub mod ops;

use crate::digit_storage::DigitStorage;

#[derive(Debug, PartialEq, Eq)]
pub struct Decimal<T> {
    pub pre: u32,
    pub post: T
}

impl<'a> std::fmt::Display for Decimal<&'a [u8]> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.post.len_digits();

        let post = if len > 0 {
            let mut stack = String::new();

            for i in 0..len {
                let part = self.post.get_digit(i).unwrap();

                stack.push_str(&part.to_string());
            }

            stack
        } else { 
            "0".to_string() 
        };

        write!(f, "{}.{}", self.pre, post)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DecimalFromStrErr {
    InvalidDigit,
    InvalidDoubledDot
}

impl<'a> std::str::FromStr for Decimal<&'a [u8]> {
    type Err = DecimalFromStrErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pre = String::new();
        let mut post = Vec::<u8>::new();
        
        enum State {
            Pre,
            Post
        }
        
        let mut state = State::Pre;
        
        for ch in s.chars() {
            if !ch.is_ascii_digit() && ch != '.' {
                Err(DecimalFromStrErr::InvalidDigit)?
            }
        
            match state {
                State::Pre => {
                    if ch == '.' {
                        state = State::Post;
        
                        continue;
                    }
        
                    pre.push(ch);
                }
                State::Post => {
                    if ch == '.' {
                        Err(DecimalFromStrErr::InvalidDoubledDot)?
                    }
        
                    post.push(ch.to_string().parse().unwrap());
                }
            }
        }
        
        Ok(Self { 
            pre: pre.parse().unwrap(),
            post: Box::leak(post.into_boxed_slice())
        })
    }
}

impl<'a> Decimal<&'a [u8]> {
    pub fn new<'b>(pre: u32, post: &'b [u8]) -> Self {
        let post = Vec::from(post);

        Self {
            pre,
            post: Box::leak(post.into_iter().collect::<Vec<u8>>().into_boxed_slice())
        }
    }
}