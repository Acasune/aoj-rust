use std::io;

fn main() {

  let mut n = String::new();
  std::io::stdin().read_line(&mut n).ok();
  let n: i32 = n.trim().parse().unwrap();

  let mut left =0;
  let mut right =0;

  for _ in 0..n {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let v: Vec<&str> = s.split_whitespace().collect();
    if v[0] > v[1] {
      left += 3;
    } else if v[0] == v[1] {
      left += 1;
      right += 1;
    } else {
      right += 3;
    }
  }
  println!("{} {}", left, right);


}
