use std::io;

fn main() {
  loop {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    if s == "0\n" {
      break;
    }

    let mut ret = 0;

    for c in s.chars() {
      let x = match c {
        t @ '0'..='9' => t as i32 - '0' as i32,
        t => 0
      };
      ret += x;
    };

    println!("{}", ret);
  }
}
