pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = chars.len();
    while i > 0 {
        i -= 1;
        if chars[i] == '+' {
            chars.remove(i);
            if i < chars.len() {
                chars.remove(i);
            }
        }
    }
    i = 0;
    while i < chars.len() {
        if chars[i] == '-' {
            if i > 0 {
                chars.remove(i - 1);
                i -= 1;
            }
            chars.remove(i);
        } else {
            i += 1;
        }
    }

    *s = chars.into_iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for i in 0..v.len() {
        if v[i].contains('+') {
            let operands: Vec<&str> = v[i].split('+').collect();
            let num1: i32 = operands[0].trim().parse().unwrap();
            let num2: i32 = operands[1].trim().parse().unwrap();
            v[i] = (num1 + num2).to_string();
        } else if v[i].contains('-') {
            let operands: Vec<&str> = v[i].split('-').collect();
            let num1: i32 = operands[0].trim().parse().unwrap();
            let num2: i32 = operands[1].trim().parse().unwrap();
            v[i] = (num1 - num2).to_string();
        }
    }
}
