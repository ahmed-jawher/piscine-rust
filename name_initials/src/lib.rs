pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|name| {
            let mut out = String::new();
            for word in name.split_whitespace() {
                if let Some(c) = word.chars().next() {
                    if !out.is_empty() {
                        out.push(' ');
                    }
                    out.extend(c.to_uppercase());
                    out.push('.');
                }
            }
            out
        })
        .collect()
}
