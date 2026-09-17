use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut char_count = HashMap::new();
    for ch in s1.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }
    for ch in s2.chars() {
        match char_count.get_mut(&ch) {
            Some(count) => {
                *count -= 1;
                if *count == 0 {
                    char_count.remove(&ch);
                }
            }
            None => return false,
        }
    }

    char_count.is_empty()
}
