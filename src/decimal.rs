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

impl<'a> std::str::FromStr for Decimal<'a> {
    type Err = ();

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
                panic!("{}", format!("'{ch}' is not valid digit"))
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
                        panic!("insert more than one '.'")
                    }
        
                    post.push(ch.to_string().parse().unwrap());
                }
            }
        }
        
        Ok(Self { 
            pre: pre.parse().unwrap(),
            post: Box::leak(post.into_iter().rev().collect::<Vec<u8>>().into_boxed_slice())
        })
    }
}

impl Decimal<'_> {
    pub fn new(pre: u32, post: &[u8]) -> Self {
        let post = Vec::from(post);

        Self {
            pre,
            post: Box::leak(post.into_iter().rev().collect::<Vec<u8>>().into_boxed_slice())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    fn failed_template(s: &str) -> String {
        format!("\x1b[31mFailed in '{s}' case\x1b[0m")
    }

    #[test]
    fn from_str() {
        let cases = [
            (
                "1.45", "1.45", Decimal::new(1, &[4, 5]), 
                "Regular parse"
            ),
            (
                "1.", "1.0", Decimal::new(1, &[]), 
                "Without post dot digit autocompleter"
            ),
            (
                "1", "1.0", Decimal::new(1, &[]), 
                "Whitout none decimal part"
            )
        ];

        for (input_num, expected_str, expected_decimal, description) in cases {
            let decimal  = Decimal::from_str(input_num).unwrap();

            assert_eq!(decimal, expected_decimal, "{}", failed_template(description));
            assert_eq!(format!("{decimal}"), expected_str, "{}", failed_template(description));
        }
    }

    #[test]
    fn add() {
        let cases = [
            (
                Decimal::new(2, &[4, 4]),
                Decimal::new(1, &[4, 4]),
                Decimal::new(3, &[8, 8]),
                "Standar sum (formatted is: 2.44 + 1.44 = 3.88)"
            ),
            (
                Decimal::new(2, &[5, 5]),
                Decimal::new(1, &[4, 5]),
                Decimal::new(4, &[0, 0]),
                "Check carry (formatted is: 2.55 + 1.45 = 4.00)"
            )
        ];

        for (left, rigth, expected, description) in cases {
            assert_eq!(left + rigth, expected, "{}", failed_template(description));
        }
    }
}