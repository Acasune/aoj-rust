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

fn binary_search(t: i64, v: &Vec<i64>) -> bool {
    let mut l: usize = 0;
    let mut r: usize = v.len() - 1;
    while r >= l {
        let m = (r + l) / 2;
        if v[m] == t {
            return true;
        }
        if t < v[m] {
            if m == 0 {
                return false;
            }
            r = m - 1;
        } else if t > v[m] {
            l = m + 1;
        }
    }
    v[min(r, l)] == t
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();

    sc.new_line();
    let v = sc.get_as_vec::<i64>();

    sc.new_line();
    let q = sc.get::<usize>();

    for _ in 0..q {
        sc.new_line();
        let k = sc.get::<i64>();
        if binary_search(k, &v) {
            println!("1");
        } else {
            println!("0");
        }
    }
}
