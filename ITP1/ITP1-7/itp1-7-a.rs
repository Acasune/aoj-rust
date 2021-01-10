use std::{io::*, usize};

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    loop {
      let line = lines.next().unwrap().unwrap();
      let mut args = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
      let m = args.next().unwrap();
      let f = args.next().unwrap();
      let r = args.next().unwrap();
  
      if m == -1 && f == -1 {
          break;
      }

      if m * f < 0 {
          println!("F");
      } else if m + f >= 80 {
          println!("A");
      } else if m + f >= 65 {
        println!("B");
      } else if m + f >= 50 {
        println!("C");
      } else if m + f >= 50 || r >= 50 {
        println!("C");
      } else if m + f >= 30 {
        println!("D");
      } else {
        println!("F");
      }
    }
}
