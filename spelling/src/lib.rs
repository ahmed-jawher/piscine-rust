const ONES: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

pub fn spell(n: u64) -> String {
    match n {
        0..=19 => ONES[n as usize].to_string(),
        20..=99 => match n % 10 {
            0 => TENS[(n / 10) as usize].to_string(),
            r => format!("{}-{}", TENS[(n / 10) as usize], ONES[r as usize]),
        },
        100..=999 => with_rest(format!("{} hundred", spell(n / 100)), n % 100),
        1_000..=999_999 => with_rest(format!("{} thousand", spell(n / 1_000)), n % 1_000),
        _ => with_rest(format!("{} million", spell(n / 1_000_000)), n % 1_000_000),
    }
}

fn with_rest(head: String, rest: u64) -> String {
    if rest == 0 {
        head
    } else {
        format!("{} {}", head, spell(rest))
    }
}
