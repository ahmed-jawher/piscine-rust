#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            None
        } else {
            let next_val = if self.v % 2 == 0 {
                self.v / 2
            } else {
                3 * self.v + 1
            };
            let current = self.v;
            self.v = next_val;
            Some(Collatz { v: current })
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        if n == 0 {
            Self { v: 0 }
        } else {
            Self { v: n }
        }
    }
}

pub fn collatz(n: u64) -> usize {
    if n <= 1 {
        return 0;
    }
    Collatz::new(n).count()
}
