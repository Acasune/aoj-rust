use std::{cmp::min, io::*, str::FromStr};

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

fn lower_bound(t: i64, v: &Vec<i64>) -> usize {
    let mut l: usize = 0;
    let mut r: usize = v.len();
    while r > l {
        let m = (r + l) / 2;
        if t <= v[m] {
            if m == 0 {
                return m;
            }
            r = m;
        } else {
            if m >= v.len() - 1 {
                return v.len();
            }
            l = m + 1;
        }
    }
    return l;
}

fn upper_bound(t: i64, v: &Vec<i64>) -> usize {
    let mut l: usize = 0;
    let mut r: usize = v.len();
    while r > l {
        let m = (r + l) / 2;
        if t < v[m] {
            if m == 0 {
                return m;
            }
            r = m;
        } else {
            if m >= v.len() - 1 {
                return v.len();
            }
            l = m + 1;
        }
    }
    return l;
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();

    sc.new_line();
    let a = sc.get_as_vec::<i64>();

    sc.new_line();
    let q = sc.get::<usize>();

    for i in 0..q {
        sc.new_line();
        let key = sc.get::<i64>();

        println!("{} {}", lower_bound(key, &a), upper_bound(key, &a))
    }
}
