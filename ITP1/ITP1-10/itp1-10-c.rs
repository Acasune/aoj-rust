use std::io;

fn main() {

  loop {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let n: f64 = n.trim().parse().unwrap();
    if n == 0. {
      break;
    }

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let v: Vec<f64> = t.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<_>();
    let sum:f64 = v.iter().sum();
    let mu:f64 = sum/n;
    let sd:f64 = v.iter().map(|x| (x-mu).powi(2)).sum();
    let ans = (sd/n).sqrt();
    println!("{}", ans);

  }


}
