use core::ops::Add;
use crate::Decimal;

impl<'a> Add for Decimal<'a> {
    type Output = Decimal<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut add_one_next = false;
        let self_len = self.post.len();
        let self_len_minus_one = self_len - 1;
        let mut res = Vec::<u8>::with_capacity(self_len);

        for i in 0..self_len {
            let left = self.post[i];
            let right = rhs.post[i];

            let sum = left + right + add_one_next as u8;
            let low_ten = sum < 10;
            let to_push = if low_ten { sum } else { sum - 10 };

            res.push(to_push);

            // to keep the last sum to 'pre' prop
            if i < self_len_minus_one {
                add_one_next = !low_ten;
            }
        }

        Self { 
            pre: self.pre + rhs.pre + add_one_next as u32,
            post: Box::leak(res.into_boxed_slice())
         }
    }
}