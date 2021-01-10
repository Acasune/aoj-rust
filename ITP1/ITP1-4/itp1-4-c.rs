fn main() {
  for i in 1.. {
      let mut buf = String::new();
      std::io::stdin().read_line(&mut buf).unwrap();

      let mut iter = buf.split_whitespace();
      let x: i64 = iter.next().unwrap().parse().unwrap();
      let op: char = iter.next().unwrap().parse().unwrap();
      let y: i64 = iter.next().unwrap().parse().unwrap();


      if op == '?' {
          break;
      }
      match op {
        '+' => println!("{}",x + y),
        '-' => println!("{}",x - y),
        '*' => println!("{}",x * y),
        '/' => println!("{}",x / y),
        _ => print!("Never happen")
      }
  }
}
