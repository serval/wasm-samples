/// This program converts any standard input to a format that can be understood by
/// a LOUDBOT compatible agent such as [LOUDCRAB](https://github.com/ceejbot/LOUDCRAB).
use std::io;

fn main() {
    let input_lines = io::stdin().lines();
    let output: Vec<String> = input_lines.flatten().map(|v| v.to_uppercase()).collect();
    println!("{}", output.join("\n"));
}
