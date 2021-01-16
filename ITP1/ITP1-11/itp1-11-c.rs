use std::{io::*, u32, vec};
use std::str::FromStr;

#[allow(unused_macros)]
macro_rules! scan {
  ($e:expr; $t:ty) => {
    $e.get::<$t>()
  };
  ($e:expr; $($t:ty), *) => {
    ($($e.get::<$t>(),)*)
  }
}

struct Scanner<R: BufRead> {
  reader: R,
  iter: std::vec::IntoIter<String>
}

#[allow(dead_code)]
impl<R: BufRead> Scanner<R> {
  fn new(reader: R) -> Scanner<R> {
    Scanner { reader, iter: vec![String::new()].into_iter() }
  }
  fn new_line(&mut self) {
    let mut line = String::new();
    self.reader.read_line(&mut line).ok();
    self.iter = line.trim().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>().into_iter();
  }
  fn get<T: FromStr>(&mut self) -> T {
    self.iter.next().unwrap().parse().ok().unwrap()
  }
  fn get_as_vec<T: FromStr>(&mut self) -> Vec<T> {
    self.iter.clone().map(|v| v.parse().ok().unwrap()).collect()
  }
  fn get_line(&mut self) -> String {
    let mut line = String::new();
    self.reader.read_line(&mut line).ok();
    line.trim().to_string()
  }
}

enum RollingDirection {
  E,
  W,
  S,
  N,
}

type Face = u32;

#[derive(Debug)]
struct Dice (Face,Face,Face,Face,Face,Face);

impl Dice {
  fn new(a:u32,b:u32,c:u32,d:u32,e:u32,f:u32) -> Dice {
    Dice(a,b,c,d,e,f)
  }
  fn rotate(&self, order: RollingDirection ) -> Dice {
    let (a,b,c,d,e,f) = (self.0,self.1,self.2,self.3,self.4,self.5);
    match order {
      RollingDirection::E => Dice(d,b,a,f,e,c),
      RollingDirection::W => Dice(c,b,f,a,e,d),
      RollingDirection::S => Dice(e,a,c,d,f,b),
      RollingDirection::N => Dice(b,f,c,d,a,e),
    }
  }
  fn spin(&self) -> Dice {
    return self.rotate(RollingDirection::E).
        rotate(RollingDirection::S).
        rotate(RollingDirection::W)

  }
  fn is_equal(&self, other: &Dice) -> bool {
    self.0 == other.0 && self.1==other.1 && self.2==other.2 && self.3==other.3 && self.4==other.4 && self.5==other.5
  }
}

fn main() {
  let cin = stdin();
  let cin = cin.lock();
  let mut sc = Scanner::new(cin);

  sc.new_line();
  let v1 : Vec<u32> = sc.get_as_vec();
  sc.new_line();
  let v2 : Vec<u32> = sc.get_as_vec();

  for i in v1.iter() {
    if !v2.contains(i) {
      println!("No");
      return;
    }
  }

  let d1 = Dice::new(v1[0],v1[1],v1[2],v1[3],v1[4],v1[5]);
  let mut d2 = Dice::new(v2[0],v2[1],v2[2],v2[3],v2[4],v2[5]);

  let mut flg =d2.is_equal(&d1);

  for i in 0..4 {
    d2 = d2.rotate(RollingDirection::N );
    for j in 0..4 {
      d2 = d2.spin();
      if d2.is_equal (&d1) {
        flg = true;
        break;
      }
    }
  }
  d2 = d2.rotate(RollingDirection::W );
  for i in 0..4 {
    d2 = d2.spin();
    if d2.is_equal (&d1) {
      flg = true;
      break;
    }
  }
  d2 = d2.rotate(RollingDirection::E )
    .rotate(RollingDirection::E );
  for i in 0..4 {
    d2 = d2.spin();
    if d2.is_equal (&d1) {
      flg = true;
      break;
    }
  }

  println!("{}", if flg { "Yes" } else { "No" });

}
