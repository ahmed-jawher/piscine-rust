pub fn first_subword(s: String) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    if let Some(first) = chars.next() {
        result.push(first);
        for c in chars {
            if c.is_uppercase() || c == '_' {
                break;
            }
            result.push(c);
        }
    }

    result
}
