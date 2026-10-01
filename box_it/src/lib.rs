pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace()
        .map(|num_str| {
            let value = if let Some(without_k) = num_str.strip_suffix('k') {
                without_k.parse::<f64>().unwrap() * 1000.0
            } else {
                num_str.parse::<f64>().unwrap()
            };
            Box::new(value.round() as u32)
        })
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|b| *b).collect()
}
