use std::io;

fn main() {

  let mut n = String::new();
  std::io::stdin().read_line(&mut n).ok();
  let n: f64 = n.trim().parse().unwrap();

  let mut t1 = String::new();
  io::stdin().read_line(&mut t1).unwrap();
  let v1: Vec<f64> = t1.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<_>();

  let mut t2 = String::new();
  io::stdin().read_line(&mut t2).unwrap();
  let v2: Vec<f64> = t2.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<_>();

  let xy: Vec<_> = v1.iter().zip(v2.iter()).collect();

  for &p in [1.0, 2.0, 3.0].iter() {
    println!("{:.09}", xy.iter().fold(0.0, |acc, &(x ,y)| acc + (x - y).abs().powf(p)).powf(1./p));
  }
  println!("{:.09}", xy.iter().fold(0.0, |acc, &(x, y)| if acc > (x - y).abs() { acc } else { (x - y).abs() }));

}
