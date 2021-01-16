use std::io::*;
use std::str::FromStr;
use std::{collections::*, usize};

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

struct Node {
    prev: usize,
    next: usize,
    value: i64,
    is_end: bool,
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let q: i64 = sc.get();
    let mut v: Vec<Node> = Vec::new();
    v.push(Node {
        prev: 0,
        next: 0,
        value: 0,
        is_end: true,
    });
    let mut cursor: usize = 0;

    for i in 0..q {
        sc.new_line();
        let input = sc.get_as_vec::<i64>();
        if input[0] == 0 {
            let x: i64 = input[1];
            let prev = v[cursor].prev;
            let next = cursor;
            v.push(Node {
                prev: prev,
                next: cursor,
                value: x,
                is_end: false,
            });
            cursor = v.len() - 1;
            v[prev].next = cursor;
            v[next].prev = cursor;
        } else if input[0] == 1 {
            let d = input[1];
            if d > 0 {
                for i in 0..d {
                    cursor = v[cursor].next;
                }
            } else {
                for i in 0..-d {
                    cursor = v[cursor].prev;
                }
            }
        } else {
            let prev: usize = v[cursor].prev;
            let next: usize = v[cursor].next;
            v[prev].next = next;
            v[next].prev = prev;
            cursor = next;
        }
    }

    cursor = v[0].next;
    while cursor != 0 {
        println!("{}", v[cursor].value);
        cursor = v[cursor].next;
    }
}
