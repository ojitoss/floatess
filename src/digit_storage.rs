pub trait DigitStorage {
    fn len_digits(&self) -> usize;

    fn get_digit(&self, index: usize) -> Option<usize>;
}

impl DigitStorage for [u8] {
    fn len_digits(&self) -> usize {
        self.len() as usize
    }

    fn get_digit(&self, index: usize) -> Option<usize> {
        match self.get(index) {
            Some(n) => Some(*n as usize),
            None => None
        }
    }
}

impl DigitStorage for u8 {
    fn len_digits(&self) -> usize {
        if *self == 0 { return 0 };

        self.ilog10() as usize + 1
    }

    fn get_digit(&self, index: usize) -> Option<usize> {
        let len = self.len_digits();

        if index > len { None? }

        let n = 10u8.pow((len - 1 - index) as u32);

        Some(((*self / n) % 10) as usize)
    }
}