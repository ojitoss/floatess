pub mod ops;

use floatess::DigitStorage;

#[derive(Debug, PartialEq, Eq)]
pub struct Decimal<T> {
    pub pre: u32,
    pub post: T
}

impl<T: DigitStorage> std::fmt::Display for Decimal<T> {
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

impl<T: DigitStorage> std::str::FromStr for Decimal<T> {
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
            post: T::from_slice(&post)
        })
    }
}

impl<T: DigitStorage> Decimal<T> {
    pub fn new(pre: u32, post: T) -> Self {
        Self {
            pre,
            post
        }
    }

    pub fn amount_decimal_digits(&self) -> usize {
        self.post.len_digits()
    }

    pub fn amount_int_digits(&self) -> usize {
        self.pre.len_digits()
    }
}