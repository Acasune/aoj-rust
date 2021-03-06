﻿use std::collections::Bound::Included;
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
    let q = sc.get::<usize>();

    let mut hs: BTreeSet<i64> = BTreeSet::new();

    for _ in 0..q {
        sc.new_line();
        let v = sc.get_as_vec::<i64>();
        if v[0] == 0 {
            hs.insert(v[1]);
            println!("{}", hs.len());
        } else if v[0] == 1 {
            let output = if hs.contains(&v[1]) { 1 } else { 0 };
            println!("{}", output);
        } else if v[0] == 2 {
            hs.remove(&v[1]);
        } else {
            for &i in hs.range((Included(&v[1]), Included(&v[2]))) {
                println!("{}", &i);
            }
        }
    }
}
