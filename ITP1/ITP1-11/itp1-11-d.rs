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

#[derive(Debug, Clone, Copy)]
struct Dice (Face,Face,Face,Face,Face,Face);

impl Dice {
  fn new(a:u32,b:u32,c:u32,d:u32,e:u32,f:u32) -> Dice {
    Dice(a,b,c,d,e,f)
  }

  fn rotate(&mut self, order: RollingDirection ) {
    match order {
      RollingDirection::E => {let tmp = self.0; self.0 = self.3; self.3 = self.5; self.5 = self.2; self.2 = tmp;},
      RollingDirection::W => {let tmp = self.0; self.0 = self.2; self.2 = self.5; self.5 = self.3; self.3 = tmp;},
      RollingDirection::S => {let tmp = self.0; self.0 = self.4; self.4 = self.5; self.5 = self.1; self.1 = tmp;},
      RollingDirection::N => {let tmp = self.0; self.0 = self.1; self.1 = self.5; self.5 = self.4; self.4 = tmp;},
    }
  }

  fn spin(&mut self) {
    self.rotate(RollingDirection::E);
    self.rotate(RollingDirection::E);
    self.rotate(RollingDirection::W);
  }

  fn is_equal(&mut self, other: &Dice) -> bool {
    for i in 0..4 {
      self.rotate(RollingDirection::N );
      for j in 0..4 {
        self.spin();
        if self._is_equal(other) {
          return true;
        }
      }
    }
    self.rotate(RollingDirection::W );
    for i in 0..4 {
      self.spin();
      if self._is_equal (other) {
        return true;
      }
    }
    self.rotate(RollingDirection::E );
    self.rotate(RollingDirection::E );
    for i in 0..4 {
      self.spin();
      if self._is_equal (other) {
        return true;
      }
    }
    return false;
  }

  fn _is_equal(&self, other: &Dice) -> bool {
    self.0 == other.0 && self.1==other.1 && self.2==other.2 && self.3==other.3 && self.4==other.4 && self.5==other.5
  }
}

fn main() {
  let cin = stdin();
  let cin = cin.lock();
  let mut sc = Scanner::new(cin);

  let mut dice:Vec<Dice> = Vec::new();

  sc.new_line();
  let n : usize = sc.get();

  for i in 0..n {
    sc.new_line();
    let v : Vec<u32> = sc.get_as_vec();
    dice.push(Dice::new(v[0],v[1],v[2],v[3],v[4],v[5]));
  }

  let mut flg = true;

  for i  in 0..n-1 {
    for j in i+1..n {
      let mut d1 = dice[i];
      let mut d2 = dice[j];
      if d1.is_equal(&mut d2) {
        flg=false;
      }
    }
  }

  println!("{}", if flg { "Yes" } else { "No" });

}
