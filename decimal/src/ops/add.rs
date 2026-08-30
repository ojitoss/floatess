use core::ops::Add;
use floatess::digit_storage::DigitStorage;
use crate::{Decimal};

impl<T: DigitStorage> Add for Decimal<T> {
    type Output = Decimal<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut add_one_next = false;
        let max_len = usize::max(self.post.len_digits(), rhs.post.len_digits());
        let mut res = vec![0; max_len];

        for i in (0..max_len).rev() {
            let left = self.post.get_digit(i);
            let right = rhs.post.get_digit(i);

            if left.is_none() || right.is_none() {
                let value = if right.is_none() { left } else { right }.unwrap();

                res[i] = value as u8;

                continue;
            }

            let left = left.unwrap();
            let right = right.unwrap();

            let sum = left + right + add_one_next as usize;
            let low_ten = sum < 10;
            let value = sum - (10 * (!low_ten as usize));

            res[i] = value as u8;

            add_one_next = !low_ten;
        }

        if *res.last().unwrap() == 0 {
            let mut poped;

            loop {
                poped = res.pop().unwrap();
                if res.is_empty() || poped != 0 { break; }
            }

            if poped != 0 { res.push(poped); }
        }

        Self { 
            pre: self.pre + rhs.pre + add_one_next as u32,
            post: T::from_slice(&res)
         }
    }
}