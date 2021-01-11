use std::io;

fn main() {
  let mut W = String::new();
  io::stdin().read_line(&mut W).expect("");
  let W = W.trim().to_lowercase();

  let mut cnt =0;

  loop {
    let mut S = String::new();
    std::io::stdin().read_line(&mut S).ok();
    let mut S = S.trim();
    if S == "END_OF_TEXT" {
      break;
    }

    let v: Vec<&str> = S.split(" ").collect();
    for w in v {
      if W == w.to_lowercase() {
        cnt += 1;
      }
    }

  }
    println!("{}", cnt);

}
