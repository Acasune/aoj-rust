fn main() {
  for i in 1.. {
      let mut buf = String::new();
      std::io::stdin().read_line(&mut buf).unwrap();

      let mut iter = buf.split_whitespace();
      let x: usize = iter.next().unwrap().parse().unwrap();
      let y: usize = iter.next().unwrap().parse().unwrap();

      if x == 0 && y == 0 {
          break;
      }
      println!("{} {}", std::cmp::min(x,y), std::cmp::max(x,y));
  }
}
