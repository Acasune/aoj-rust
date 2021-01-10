fn main() {
  for i in 1.. {
      let mut x = String::new();
      std::io::stdin().read_line(&mut x).unwrap();
      let x: i32 = x.trim().parse().unwrap();
      if x == 0 {
          break;
      }
      println!("Case {}: {}", i, x);
  }
}
