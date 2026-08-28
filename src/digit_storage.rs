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