use std::io::*;

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    loop {
      let line = lines.next().unwrap().unwrap();
      let mut args = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
      let n = args.next().unwrap();
      let x = args.next().unwrap();
  
      if n == 0 && x == 0 {
          break;
      }

      let mut cnt = 0;
      for i in 1..=n {
        for j in i+1..=n {
          for k in j+1..=n {
            if i + j + k == x{
              cnt = cnt + 1;
            }
          }
        }
      }
      println!("{}", cnt);
    }
}
