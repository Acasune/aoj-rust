use std::io;

fn main() {
  loop {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut s = s.trim().to_string();
    if s == "-" {
      break;
    }

    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n: usize = n.trim().parse().unwrap();

    for _ in 0..n {
        let mut h = String::new();
        std::io::stdin().read_line(&mut h).ok();
        let h: usize = h.trim().parse().unwrap();
        s = format!("{}{}", &s[h..], &s[..h]);
    }
    println!("{}", s);

  }

}
