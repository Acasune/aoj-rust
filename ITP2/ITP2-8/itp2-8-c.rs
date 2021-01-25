use std::collections::Bound::Included;
use std::{collections::BTreeMap, io::*, str::FromStr};

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
    iter: std::vec::IntoIter<String>,
}

#[allow(dead_code)]
impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Scanner<R> {
        Scanner {
            reader,
            iter: vec![String::new()].into_iter(),
        }
    }
    fn new_line(&mut self) {
        let mut line = String::new();
        self.reader.read_line(&mut line).ok();
        self.iter = line
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .into_iter();
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

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let q = sc.get::<usize>();

    let mut btm = BTreeMap::new();

    for _ in 0..q {
        sc.new_line();
        let v = sc.get_as_vec::<String>();
        let cmd: i64 = v[0].parse::<i64>().unwrap();
        if cmd == 0 {
            btm.insert(v[1].clone(), v[2].clone());
        } else if cmd == 1 {
            match btm.get(&v[1]) {
                Some(x) => {
                    println!("{}", x);
                }
                None => {
                    println!("0")
                }
            }
        } else if cmd == 2 {
            btm.remove(&v[1]);
        } else {
            for (s, i) in btm.range(v[1].to_owned()..=v[2].to_owned()) {
                println!("{} {}", s, i);
            }
        }
    }
}
