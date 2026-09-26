pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A') as usize;
    let width = 2 * n + 1;

    let top: Vec<String> = (0..=n)
        .map(|i| {
            let mut row = vec![' '; width];
            let letter = (b'A' + i as u8) as char;
            row[n - i] = letter;
            row[n + i] = letter;
            row.into_iter().collect()
        })
        .collect();

    let mut diamond = top.clone();
    diamond.extend(top.into_iter().rev().skip(1));
    diamond
}
