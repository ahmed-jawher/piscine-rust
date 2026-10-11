pub fn first_fifty_even_square() -> Vec<i32> {
    (1..=50).map(|n| 2 * n).map(|x| x * x).collect()
}
