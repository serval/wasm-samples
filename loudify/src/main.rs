/// This program converts any standard input to a format that can be understood by
/// a LOUDBOT compatible agent such as [LOUDCRAB](https://github.com/ceejbot/LOUDCRAB).
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    _ = io::stdin().read_to_string(&mut input);
    println!("{}", input.to_uppercase());
}
