pub trait DigitStorage<T> {
    fn len_digits(&self) -> u32;

    fn get_digit(&self, index: usize) -> Option<T>;
}

impl DigitStorage<u8> for [u8] {
    fn len_digits(&self) -> u32 {
        self.len() as u32
    }

    fn get_digit(&self, index: usize) -> Option<u8> {
        match self.get(index) {
            Some(n) => Some(*n),
            None => None
        }
    }
}

impl DigitStorage<u8> for u8 {
    fn len_digits(&self) -> u32 {
        if *self == 0 { return 0 };

        self.ilog10() + 1
    }

    fn get_digit(&self, index: usize) -> Option<u8> {
        let len = self.len_digits();

        if index > len as usize { None? }

        let n = 10u8.pow(len - 1 - index as u32);

        Some((*self / n) % 10)
    }
}