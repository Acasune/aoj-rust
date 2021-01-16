use std::io::*;
use std::str::FromStr;
use std::iter::*;
struct Scanner<R: Read> {
  reader: R,
}

#[allow(dead_code)]
impl<R: Read> Scanner<R> {
  fn new(reader: R) -> Scanner<R> {
      Scanner { reader: reader }
  }

  fn safe_read<T: FromStr>(&mut self) -> Option<T> {
      let token = self.reader.by_ref().bytes().map(|c| c.unwrap() as char)
          .skip_while(|c| c.is_whitespace())
          .take_while(|c| !c.is_whitespace())
          .collect::<String>();
      if token.is_empty() {
          None
      } else {
          token.parse::<T>().ok()
      }
  }

  fn read<T: FromStr>(&mut self) -> T {
      if let Some(s) = self.safe_read() {
          s
      } else {
          writeln!(stderr(), "Terminated with EOF").unwrap();
          std::process::exit(0);
      }
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
}

fn main() {
  let cin = stdin();
  let cin = cin.lock();
  let mut sc = Scanner::new(cin);

  let a : u32 = sc.read();
  let b : u32 = sc.read();
  let c : u32 = sc.read();
  let d : u32 = sc.read();
  let e : u32 = sc.read();
  let f : u32 = sc.read();

  let roll : String = sc.read();

  let mut die = Dice::new(a,b,c,d,e,f);
  for ch in roll.chars() {
    die = match ch {
      'E' => die.rotate(RollingDirection::E),
      'N' => die.rotate(RollingDirection::N),
      'S' => die.rotate(RollingDirection::S),
      'W' => die.rotate(RollingDirection::W),
      _   => {panic!("Unreachable!!")},
    };
  }
  println!("{}", die.0);

}
