pub fn number_logic(n: u32) -> bool {
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let num_digits = digits.len() as u32;
    let sum: u32 = digits.iter().map(|&d| d.pow(num_digits)).sum();
    sum == n
}
