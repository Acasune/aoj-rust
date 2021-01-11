fn main() {

  let pi = std::f64::consts::PI;

  let mut line = String::new();
  std::io::stdin().read_line(&mut line).ok();
  let mut iter = line.split_whitespace();
  let a  = iter.next().unwrap().parse::<f64>().unwrap();
  let b  = iter.next().unwrap().parse::<f64>().unwrap();
  let c  = iter.next().unwrap().parse::<f64>().unwrap().to_radians();

  let s = 0.5 * a * b * c.sin();
  let h = b * c.sin();
  let s = a * h / 2.;
  let l = a + b + ((a - b * c.cos()).powi(2) + h.powi(2)).sqrt();

  println!("{:.12}", s);
  println!("{:.12}", l);
  println!("{:.12}", h);

}
