use std::io::{self, Write};

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let correct_answer = "The letter e";
    let mut attempts = 0;

    loop {
        println!("{}", riddle);
        let mut input = String::new();
        print!("");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        attempts += 1;
        if input.trim() == correct_answer {
            break;
        }
    }

    println!("Number of trials: {}", attempts);
}
