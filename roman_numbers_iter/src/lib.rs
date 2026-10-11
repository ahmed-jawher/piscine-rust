pub use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

const VALUES: [(u32, &[RomanDigit]); 13] = [
    (1000, &[M]),
    (900, &[C, M]),
    (500, &[D]),
    (400, &[C, D]),
    (100, &[C]),
    (90, &[X, C]),
    (50, &[L]),
    (40, &[X, L]),
    (10, &[X]),
    (9, &[I, X]),
    (5, &[V]),
    (4, &[I, V]),
    (1, &[I]),
];

impl RomanDigit {
    fn value(self) -> u32 {
        match self {
            Nulla => 0,
            I => 1,
            V => 5,
            X => 10,
            L => 50,
            C => 100,
            D => 500,
            M => 1000,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }
        let mut digits = Vec::new();
        for (value, symbols) in VALUES {
            while num >= value {
                digits.extend_from_slice(symbols);
                num -= value;
            }
        }
        RomanNumber(digits)
    }
}

impl From<&RomanNumber> for u32 {
    fn from(number: &RomanNumber) -> Self {
        let values: Vec<u32> = number.0.iter().map(|d| d.value()).collect();
        values
            .iter()
            .enumerate()
            .map(|(i, &v)| match values.get(i + 1) {
                Some(&next) if next > v => -(v as i64),
                _ => v as i64,
            })
            .sum::<i64>() as u32
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let current = u32::from(&*self);
        *self = RomanNumber::from(current + 1);
        Some(self.clone())
    }
}
