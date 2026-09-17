pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    let count = list.len() as f64;
    let mean = sum as f64 / count;
    mean
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted_list = list.to_vec();
    sorted_list.sort();
    let len = sorted_list.len();
    if len % 2 == 0 {
        let mid1 = sorted_list[len / 2 - 1];
        let mid2 = sorted_list[len / 2];
        (mid1 + mid2) / 2
    } else {
        sorted_list[len / 2]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    use std::collections::HashMap;
    let mut occurrences = HashMap::new();
    for &num in list {
        *occurrences.entry(num).or_insert(0) += 1;
    }
    let (mode, _) = occurrences.iter().max_by_key(|&(_, count)| count).unwrap();
    *mode
}
