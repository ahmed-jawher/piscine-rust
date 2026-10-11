pub fn collatz(n: u64) -> Option<usize> {
    if n == 0 {
        return None;
    }
    let mut steps = 0;
    let mut v = n;
    while v != 1 {
        v = if v % 2 == 0 {
            v / 2
        } else {
            v.checked_mul(3)?.checked_add(1)?
        };
        steps += 1;
    }
    Some(steps)
}
