use std::io::*;

fn main() {
  let stdin = stdin();
  let mut lines = stdin.lock().lines();
  let N = lines.next().unwrap().unwrap().parse::<u64>().unwrap();

  let mut cards = vec![vec![-1; 13]; 4];

  for i in 0..N {
    let line = lines.next().unwrap().unwrap();
    let mut args = line.split_whitespace();
    let s = args.next().unwrap();
    let r = args.next().unwrap().parse::<usize>().unwrap();
    match s {
      "S" => {cards[0][r-1]=1;}
      "H" => {cards[1][r-1]=1;}
      "C" => {cards[2][r-1]=1;}
      "D" => {cards[3][r-1]=1;}
      _ => {}
    }
  }
  for s in 0..4 {
    for r in 0..13 {
      if cards[s][r] != 1 {
        if s==0 {
          println!("{} {}","S",r+1);
        }
        if s==1{
          println!("{} {}","H",r+1);
        }
        if s==2 {
          println!("{} {}","C",r+1);
        }
        if s==3 {
          println!("{} {}","D",r+1);
        }
      }
    }
  }


}
