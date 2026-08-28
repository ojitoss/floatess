pub mod ops;

#[derive(Debug, PartialEq, Eq)]
pub struct Decimal<'a> {
    pub pre: u32,
    pub post: &'a [u8]
}

impl std::fmt::Display for Decimal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.post.len();

        let post = if len > 0 {
            let mut stack = String::new();

            for i in 0..len {
                let part = self.post[i];

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

impl std::str::FromStr for Decimal<'_> {
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
            post: Box::leak(post.into_iter().collect::<Vec<u8>>().into_boxed_slice())
        })
    }
}

impl Decimal<'_> {
    pub fn new(pre: u32, post: &[u8]) -> Self {
        let post = Vec::from(post);

        Self {
            pre,
            post: Box::leak(post.into_iter().collect::<Vec<u8>>().into_boxed_slice())
        }
    }
}