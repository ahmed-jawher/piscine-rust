pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_value = f64::exp(c as f64);
    let ln_value = match c {
        0 => std::f64::NEG_INFINITY,
        _ => (c.abs() as f64).ln(),
    };

    (c, exp_value, ln_value)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values = a
        .split_whitespace()
        .map(|word| word.parse::<f64>().unwrap_or(0.0))
        .map(f64::exp)
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let exp_str = exp_values.join(" ");
    (a, exp_str)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_values = b
        .iter()
        .map(|&x| (x.abs() as f64).ln())
        .collect::<Vec<f64>>();
    (b, ln_values)
}
