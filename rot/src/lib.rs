pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_alphabetic() {
            let base = if c.is_lowercase() { b'a' } else { b'A' };
            let key = key.rem_euclid(26) as u8;
            let offset = (c as u8 - base + key as u8) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }
    result
}
