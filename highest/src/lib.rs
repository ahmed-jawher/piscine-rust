#[derive(Debug)]
pub struct Numbers<const N: usize> {
    numbers: [u32; N],
}

impl<const N: usize> Numbers<N> {
    pub const fn new(numbers: [u32; N]) -> Self {
        Numbers { numbers }
    }

    pub fn inner(&self) -> &[u32] {
        &self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Option<[u32; 3]> {
        let mut sorted = self.numbers.to_vec();
        sorted.sort_by(|a, b| b.cmp(a));
        sorted.get(..3)?.try_into().ok()
    }
}
