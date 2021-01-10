fn main() {
  solve();
}

fn solve() {
  let N: usize = {
    let mut line: String = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
  };
  call(N);
  println!();

}

fn call(N:usize){
  for i in 1..=N{
    let mut x = i;
    if x % 3 ==0 {
      print!(" {}", i);
    } else {
      loop {
        if x % 10 ==3 {
          print!(" {}", i);
          break;
        }
        x = x / 10;
        if x == 0 {
          break;
        }
      }
    }
  }
}
