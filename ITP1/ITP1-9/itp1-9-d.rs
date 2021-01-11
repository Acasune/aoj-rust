
fn main() {

  let mut w = String::new();
  std::io::stdin().read_line(&mut w).ok();
  let mut w = w.trim().to_string();

  let mut n = String::new();
  std::io::stdin().read_line(&mut n).ok();
  let n: i32 = n.trim().parse().unwrap();

  for _ in 0..n {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    let mut iter = line.split_whitespace();
    let command = iter.next().unwrap();
    match command {
        "print" => {
            let a = iter.next().unwrap().parse::<usize>().unwrap();
            let b = iter.next().unwrap().parse::<usize>().unwrap();
            println!("{}", &w[a..b+1])
        },
        "reverse" => {
            let a = iter.next().unwrap().parse::<usize>().unwrap();
            let b = iter.next().unwrap().parse::<usize>().unwrap();
            let rev = w.drain(a..b+1).rev().collect::<String>();
            w.insert_str(a, &rev);
        },
        "replace" => {
            let a = iter.next().unwrap().parse::<usize>().unwrap();
            let b = iter.next().unwrap().parse::<usize>().unwrap();
            let p = iter.next().unwrap();
            w.drain(a..b+1);
            w.insert_str(a, p);
        },
        _ => {},
    }

  }
}
