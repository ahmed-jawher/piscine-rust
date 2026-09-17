pub fn is_pangram(s: &str) -> bool {
    let mut letters = [false; 26];
    let mut count = 0;
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            let index = c.to_ascii_lowercase() as usize - 'a' as usize;
            if !letters[index] {
                letters[index] = true;
                count += 1;
            }
        }
    }
    println!("{:?}", count == 26);
    count == 26
}
