#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        let values = [
            (1000, RomanDigit::M),
            (900, RomanDigit::C),
            (500, RomanDigit::D),
            (400, RomanDigit::C),
            (100, RomanDigit::C),
            (90, RomanDigit::X),
            (50, RomanDigit::L),
            (40, RomanDigit::X),
            (10, RomanDigit::X),
            (9, RomanDigit::I),
            (5, RomanDigit::V),
            (4, RomanDigit::I),
            (1, RomanDigit::I),
        ];

        let mut result = Vec::new();

        for (value, digit) in values.iter() {
            while num >= *value {
                match (*value, digit) {
                    (900, _) => {
                        result.push(RomanDigit::C);
                        result.push(RomanDigit::M);
                        num -= 900;
                    }
                    (400, _) => {
                        result.push(RomanDigit::C);
                        result.push(RomanDigit::D);
                        num -= 400;
                    }
                    (90, _) => {
                        result.push(RomanDigit::X);
                        result.push(RomanDigit::C);
                        num -= 90;
                    }
                    (40, _) => {
                        result.push(RomanDigit::X);
                        result.push(RomanDigit::L);
                        num -= 40;
                    }
                    (9, _) => {
                        result.push(RomanDigit::I);
                        result.push(RomanDigit::X);
                        num -= 9;
                    }
                    (4, _) => {
                        result.push(RomanDigit::I);
                        result.push(RomanDigit::V);
                        num -= 4;
                    }
                    _ => {
                        result.push(*digit);
                        num -= *value;
                    }
                }
            }
        }

        RomanNumber(result)
    }
}

impl std::fmt::Display for RomanNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in &self.0 {
            write!(f, "{:?}", digit)?;
        }
        Ok(())
    }
}
