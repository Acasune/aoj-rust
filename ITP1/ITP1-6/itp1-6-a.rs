use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().nth(1).unwrap().unwrap();
    let s = line.split_whitespace().rev().collect::<Vec<_>>().join(" ");
    println!("{}", s);
}
