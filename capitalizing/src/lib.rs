pub fn capitalize_first(input: &str) -> String {
    if let Some(first) = input.chars().next() {
        let c = first.to_ascii_uppercase().to_string();
        let r = input
            .chars()
            .skip(1)
            .collect::<String>()
            .to_ascii_lowercase();
        return format!("{}{}", c, r);
    }
    String::new()
}
pub fn title_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            capitalize_next = true;
            result.push(c);
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}
pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|s| {
            if s.is_ascii_uppercase() {
                s.to_ascii_lowercase()
            } else {
                s.to_ascii_uppercase()
            }
        })
        .collect()
}
