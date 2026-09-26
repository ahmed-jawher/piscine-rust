pub fn scytale_cipher(message: &str, i: usize) -> String {
    let chars: Vec<char> = message.chars().collect();
    if i == 0 || chars.is_empty() {
        return message.to_string();
    }

    let rows = chars.len().div_ceil(i);
    let mut result = String::with_capacity(rows * i);

    for col in 0..i {
        for row in 0..rows {
            result.push(*chars.get(row * i + col).unwrap_or(&' '));
        }
    }

    result.trim_end().to_string()
}
