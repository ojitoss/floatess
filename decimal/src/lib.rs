pub mod ops;

use std::fmt::{Debug, Display};

use floatess::{DigitsStream,  stream::{SmallDigitsStream, DigitsStreamUsable}};

#[derive(PartialEq, Eq)]
pub struct Decimal<T> {
    pub pre: u32,
    pub post: DigitsStreamUsable<T>
}

impl<T: DigitsStream> Display for Decimal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let post = format!("{}", self.post);

        write!(f, "{}.{}", self.pre, post)
    }
}

impl<T: DigitsStream + Debug> Debug for Decimal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Decimal [ pre: {:?}, post: {:?} ]", self.pre, self.post)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DecimalFromStrErr {
    InvalidDigit,
    InvalidDoubledDot
}

impl<'a, T: DigitsStream + TryFrom<&'a [u8]>> std::str::FromStr for Decimal<T> 
where 
    <T as TryFrom<&'a [u8]>>::Error: Debug 
{
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
            post: DigitsStreamUsable(T::try_from(Box::leak(post.into_boxed_slice())).unwrap())
        })
    }
}

impl<T: DigitsStream + Clone> Decimal<T> {
    pub fn new(pre: u32, post: T) -> Self {
        Self {
            pre,
            post: DigitsStreamUsable(post)
        }
    }

    pub fn get_decimal_part_as_digits_stream(&self) -> T { 
        self.post.0.clone() 
    }

    pub fn get_int_part_as_digits_stream(&self) -> SmallDigitsStream<u32> {
        SmallDigitsStream(self.pre)
    }
}