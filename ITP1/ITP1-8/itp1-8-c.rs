use std::{io::Read, usize};

fn main() {
  let mut s = String::new();
  std::io::stdin().read_to_string(&mut s).unwrap();
  let mut v = vec![0; 256];

  for c in s.bytes().map(|c| c as usize) {
    v[c] += 1;
  };

  for c  in b'a'..b'z'+1 {
    println!("{} : {}", c as char, (v[c as usize]+v[c as usize ^ 32]));
  }

}
