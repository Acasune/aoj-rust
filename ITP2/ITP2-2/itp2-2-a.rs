﻿use std::str::FromStr;
use std::{collections::VecDeque, io::*};

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
    let n: usize = sc.get();
    let q: usize = sc.get();
    let mut vv: Vec<VecDeque<i64>> = vec![VecDeque::new(); n];

    for _ in 0..q {
        sc.new_line();
        let input = sc.get_as_vec::<i64>();
        if input[0] == 0 {
            vv[input[1] as usize].push_back(input[2]);
        } else if input[0] == 1 {
            let tmp = vv[input[1] as usize].back();
            match tmp {
                Some(t) => {
                    println!("{}", t);
                }
                None => {}
            }
        } else {
            vv[input[1] as usize].pop_back();
        }
    }
}
