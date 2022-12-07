use std::io;

/// This program converts any standard input to a format that can be understood by
/// a LOUDBOT compatible agent such as [LOUDCRAB](https://github.com/ceejbot/LOUDCRAB).

fn main() {
    let input_lines = io::stdin().lines();
    let mut input = String::new();
    for line in input_lines {
        input += &line.unwrap();
    }
    println!("{}", input.to_uppercase());
}