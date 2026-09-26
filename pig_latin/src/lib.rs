pub fn pig_latin(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let is_vowel = |c: char| "aeiou".contains(c);

    let mut split = match chars.iter().position(|&c| is_vowel(c)) {
        Some(i) => i,
        None => chars.len(),
    };

    // consonant + "qu": the "u" moves along with the "q"
    if split >= 2 && chars[split - 1] == 'q' && chars[split] == 'u' {
        split += 1;
    }

    let head: String = chars[..split].iter().collect();
    let tail: String = chars[split..].iter().collect();
    format!("{}{}ay", tail, head)
}
