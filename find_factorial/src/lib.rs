pub fn factorial(num: u64) -> u64 {
    let mut result = 1;

    for i in 1..=num {
        result *= i;
    }

    result
}
