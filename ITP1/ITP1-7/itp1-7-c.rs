use std::{io::*, usize};

fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  let vec: Vec<usize> = s.split_whitespace().map(|a| a.parse().unwrap()).collect();
  let R = vec[0];
  let C = vec[1];
  let mut sheet: Vec<Vec<i32>> = Vec::new();
  for _ in 0..R {
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).ok();
      let mut row: Vec<i32> = s.split_whitespace().map(|a| a.parse().unwrap()).collect();
      row.push(0);
      sheet.push(row);
  }
  sheet.push(vec![0;C+1]);

  for i in 0..R {
    let mut cnt =0;
    for j in 0..C {
      cnt = cnt + sheet[i][j];
    }
    sheet[i][C]=cnt
  }
  for j in 0..C {
    let mut cnt =0;
    for i in 0..R {
      cnt = cnt + sheet[i][j];
    }
    sheet[R][j]=cnt
  }
  for j in 0..C {
    sheet[R][C]=sheet[R][C]+sheet[R][j];
  }

  for i in 0..=R {
    for j in 0..=C {
      if j == 0 {
        print!("{}", sheet[i][j]);
      } else {
        print!(" {}", sheet[i][j]);
      }
    }
    println!("");
  }

}
