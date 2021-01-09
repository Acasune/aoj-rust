use std::io;

fn main() {
    let n = input();
    let cube = n * n * n;
    println!("{}", cube);
}

fn input() -> i32 {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("");

    n.trim().parse().unwrap()
}
