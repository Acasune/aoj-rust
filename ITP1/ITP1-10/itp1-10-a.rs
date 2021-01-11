fn main() {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).ok();
  let mut iter = line.split_whitespace();
  let a  = iter.next().unwrap().parse::<f64>().unwrap();
  let b  = iter.next().unwrap().parse::<f64>().unwrap();
  let c  = iter.next().unwrap().parse::<f64>().unwrap();
  let d  = iter.next().unwrap().parse::<f64>().unwrap();

  println!("{:.8}", ((b-d)*(b-d)+(c-a)*(c-a)).powf(0.5));

}
