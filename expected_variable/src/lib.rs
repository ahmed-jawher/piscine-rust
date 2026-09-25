pub fn edit_distance(source: &str, target: &str) -> usize {
    let source = source.as_bytes();
    let target = target.as_bytes();
    let source_len = source.len();
    let target_len = target.len();
    let mut matrix = vec![vec![0; target_len + 1]; source_len + 1];

    for i in 0..=source_len {
        matrix[i][0] = i;
    }
    for j in 0..=target_len {
        matrix[0][j] = j;
    }

    for i in 1..=source_len {
        for j in 1..=target_len {
            if source[i - 1] == target[j - 1] {
                matrix[i][j] = matrix[i - 1][j - 1];
            } else {
                matrix[i][j] = std::cmp::min(
                    matrix[i - 1][j] + 1,
                    std::cmp::min(matrix[i][j - 1] + 1, matrix[i - 1][j - 1] + 1),
                );
            }
        }
    }

    matrix[source_len][target_len]
}

fn is_camel_or_snake(s: &str) -> bool {
    !s.is_empty()
        && s
            .split('_')
            .all(|word| !word.is_empty() && word.chars().all(|c| c.is_ascii_alphanumeric()))
}

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    let compared = compared.to_lowercase();
    let expected = expected.to_lowercase();

    if !is_camel_or_snake(&compared) || expected.is_empty() {
        return None;
    }

    let distance = edit_distance(&compared, &expected) as f64;
    let alikeness = (1.0 - distance / expected.len() as f64) * 100.0;

    if alikeness > 50.0 {
        Some(format!("{}%", alikeness.round()))
    } else {
        None
    }
}
