use std::collections::Bound::Included;
use std::{collections::BTreeSet, io::*, str::FromStr};

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
    let q: u64 = sc.get();
    let mut bits: u64 = 0u64;

    for _ in 0..q {
        sc.new_line();
        let v = sc.get_as_vec::<u64>();
        if v[0] == 0 {
            println!("{}", if bits & (1u64 << v[1]) != 0 { 1 } else { 0 })
        } else if v[0] == 1 {
            bits |= 1u64 << v[1];
        } else if v[0] == 2 {
            bits &= !(1u64 << v[1]);
        } else if v[0] == 3 {
            bits ^= 1u64 << v[1];
        } else if v[0] == 4 {
            println!("{}", if bits == !0u64 { 1 } else { 0 })
        } else if v[0] == 5 {
            println!("{}", if bits != 0u64 { 1 } else { 0 })
        } else if v[0] == 6 {
            println!("{}", if bits == 0u64 { 1 } else { 0 })
        } else if v[0] == 7 {
            println!("{}", bits.count_ones());
        } else {
            println!("{}", bits);
        }
    }
}
