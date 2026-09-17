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
