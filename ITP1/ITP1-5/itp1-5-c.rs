
fn main() {
  solve();
}

fn solve() {
  loop {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();

    if h==0 && w==0{
      break;
    };
    draw_rect(h, w);
    print!("\n")

  }
}

fn draw_rect(H: usize, W: usize){
  for h in 0..H {
    for w in 0..W{
      let dot =
        if (w + h) % 2 ==0 {
          "#"
        }
        else {
          "."
        };
        print!("{}", dot);
      }
    print!("\n");
  }
}
