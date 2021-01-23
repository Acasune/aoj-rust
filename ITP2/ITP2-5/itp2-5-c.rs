use std::{io::*, str::FromStr};

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

fn permutation(
    n: usize,
    base: &mut Vec<i64>,
    p: &mut Vec<i64>,
    used: &mut Vec<bool>,
    all: &mut Vec<Vec<i64>>,
) {
    if p.len() == n {
        all.push(p.to_vec());
        return;
    }
    for i in 0..n {
        if !used[i] {
            p.push(base[i].clone());
            used[i] = true;
            permutation(n, base, p, used, all);
            p.pop();
            used[i] = false;
        }
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();

    sc.new_line();
    let mut b: Vec<i64> = sc.get_as_vec();

    let mut vv: Vec<Vec<i64>> = Vec::new();
    let mut p: Vec<i64> = Vec::new();
    let mut used = vec![false; n];

    permutation(n, &mut b, &mut p, &mut used, &mut vv);
    vv.sort();

    let b_i: usize = vv.iter().position(|r| r == &b).unwrap();

    if b_i > 0 {
        for i in 0..n {
            if i == 0 {
                print!("{}", vv[b_i - 1][i]);
            } else {
                print!(" {}", vv[b_i - 1][i]);
            }
        }
        println!("");
    }

    for i in 0..n {
        if i == 0 {
            print!("{}", vv[b_i][i]);
        } else {
            print!(" {}", vv[b_i][i]);
        }
    }
    println!("");
    if b_i < vv.len() - 1 {
        for i in 0..n {
            if i == 0 {
                print!("{}", vv[b_i + 1][i]);
            } else {
                print!(" {}", vv[b_i + 1][i]);
            }
        }
        println!("");
    }
}
