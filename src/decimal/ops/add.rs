use core::ops::Add;
use crate::Decimal;

impl<'a> Add for Decimal<'a> {
    type Output = Decimal<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut add_one_next = false;
        let max_len = usize::max(self.post.len(), rhs.post.len());
        let mut res = Vec::<u8>::with_capacity(max_len);

        for i in (0..max_len).rev() {
            let left = self.post.get(i);
            let right = rhs.post.get(i);

            if left.is_none() || right.is_none() {
                let value = if right.is_none() { *left.unwrap() } else { *right.unwrap() };

                res.push(value);

                continue;
            }

            let left = left.unwrap();
            let right = right.unwrap();

            let sum = left + right + add_one_next as u8;
            let low_ten = sum < 10;
            let to_push = sum - (10 * (!low_ten as u8));

            res.push(to_push);

            add_one_next = !low_ten;
        }

        Self { 
            pre: self.pre + rhs.pre + add_one_next as u32,
            post: Box::leak(res.into_iter().rev().collect::<Vec<u8>>().into_boxed_slice())
         }
    }
}