use std::{io::*, usize};

fn main() {
  let stdin = stdin();
  let mut lines = stdin.lock().lines();
  let N = lines.next().unwrap().unwrap().parse::<u64>().unwrap();

  let mut buildings = vec![vec![vec![0; 10];3]; 4];

  for i in 0..N {
    let line = lines.next().unwrap().unwrap();
    let mut args = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let b = args.next().unwrap() as usize;
    let f = args.next().unwrap() as usize;
    let r = args.next().unwrap() as usize;
    let v = args.next().unwrap();

    buildings[b-1][f-1][r-1] += v;

  }

  for b in 0..4 {
    for f in 0..3 {
      for r in 0..10 {
        print!(" {}", buildings[b][f][r]);
      }
      println!("");
    }
    if b != 3 {
      println!("####################");
    }

  }


}
