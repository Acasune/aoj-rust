﻿use std::{cmp::max, str::FromStr, vec};
use std::{cmp::min, io::*};

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

fn rotate(v: &mut Vec<i64>, b: usize, m: usize, e: usize) {
    let mut tmp_v: Vec<i64> = vec![0; e - b];
    for k in 0..e - b {
        tmp_v[b + ((k + (e - m)) % (e - b)) - b] = v[b + k];
    }
    for i in 0..e - b {
        v[b + i] = tmp_v[i];
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    sc.new_line();
    let mut v = sc.get_as_vec::<i64>();
    sc.new_line();
    let q = sc.get::<usize>();

    for _ in 0..q {
        sc.new_line();
        let b = sc.get::<usize>();
        let m = sc.get::<usize>();
        let e = sc.get::<usize>();

        rotate(&mut v, b, m, e);
    }

    for i in 0..n {
        if i == 0 {
            print!("{}", v[i]);
        } else {
            print!(" {}", v[i]);
        }
    }
    println!("")
}
