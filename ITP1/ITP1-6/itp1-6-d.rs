fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  let vec: Vec<usize> = s.split_whitespace().map(|a| a.parse().unwrap()).collect();
  let N = vec[0];
  let M = vec[1];
  let mut a: Vec<Vec<i32>> = Vec::new();
  let mut b: Vec<i32> = Vec::new();
  for _ in 0..N {
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).ok();
      let row: Vec<i32> = s.split_whitespace().map(|a| a.parse().unwrap()).collect();
      a.push(row);
  }
  for _ in 0..M {
      let mut s = String::new();
      std::io::stdin().read_line(&mut s).ok();
      let v: i32 = s.trim().parse().unwrap();
      b.push(v);
  }
  let mut c: Vec<i32> = vec![0; N];
  for i in 0..N {
      for j in 0..M {
          c[i] += a[i][j] * b[j];
      }
  }
  for i in 0..N {
      println!("{}", c[i]);
  }
}
