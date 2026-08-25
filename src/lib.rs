use core::fmt;

#[derive(Debug)]
pub struct Decimal<'a> {
    pub pre: u32,
    pub post: &'a [u8]
}

impl fmt::Display for Decimal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut post = String::new();

        let mut i = self.post.len() - 1;
        loop {
            // usize can be less than zero but this had the same function using a overflow
            if i == usize::MAX { break };

            let part = self.post[i];

            post.push_str(&part.to_string());
            i = i.wrapping_sub(1);
        }

        write!(f, "{}.{}", self.pre.to_string(), post)
    }
}

impl<'a> Decimal<'a> {
    pub fn str_to_decimal(string: &'a str) -> Decimal<'a> {
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

        Decimal { 
            pre: pre.parse().unwrap(),
            post: Box::leak(post.into_iter().rev().collect::<Vec<u8>>().into_boxed_slice())
         }
    }
}