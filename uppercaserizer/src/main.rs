use std::io::BufRead;

fn main() {
    for line in std::io::stdin().lock().lines() {
        println!("{}", line.unwrap().to_uppercase());
    }
    std::process::exit(0);
}
