use std::ops::Add;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
}

impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
        }
    }
}

impl<T: Add<Output = T> + PartialOrd + Copy> Iterator for StepIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.current > self.end {
            return None;
        }
        let value = self.current;
        self.current = self.current + self.step;
        Some(value)
    }
}
