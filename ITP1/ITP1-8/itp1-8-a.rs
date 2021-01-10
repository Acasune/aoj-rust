use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("");

  for c in s.chars() {
    let x = match c {
      t @ 'a'..='z' => t.to_uppercase().to_string(),
      t @ 'A'..='Z' => t.to_lowercase().to_string(),
      t  => t.to_string(),
    };

    print!("{}", x);
    
  }
}
