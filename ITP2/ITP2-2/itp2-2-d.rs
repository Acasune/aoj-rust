// TO-DO: This problem is not solved due to TLE

use std::{cmp::max, str::FromStr};
use std::{cmp::min, io::*};
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

fn main() {
    let cin = stdin();
    let cin = cin.lock();

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n: usize = sc.get();
    let q: usize = sc.get();
    let mut vv: Vec<Vec<i64>> = vec![Vec::new(); n];

    for i in 0..q {
        sc.new_line();
        let input = sc.get_as_vec::<i64>();
        if input[0] == 0 {
            vv[input[1] as usize].push(input[2]);
        } else if input[0] == 1 {
            for t in 0..vv[input[1] as usize].len() {
                if t == 0 {
                    write!(out, "{}", vv[input[1] as usize][t]);
                } else {
                    write!(out, " {}", vv[input[1] as usize][t]);
                }
            }
            writeln!(out, "");
        } else {
            let li = input[1] as usize;
            let ri = input[2] as usize;

            // ri++li

            if li > ri {
                // let (l, r) = vv.split_at_mut(ri + 1);
                // let mut tmp = &mut l[ri];
                // let mut tmp1 = &mut r[li - ri - 1];

                // tmp.splice(tmp.len().., tmp1.iter().cloned());
                // tmp1.clear();
                unsafe {
                    let tmp = vv[li].clone();
                    let mut tmp1 = &mut vv[ri];

                    tmp1.splice(tmp1.len().., tmp);
                    vv[li].clear()
                }
            } else {
                // let (l, r) = vv.split_at_mut(li + 1);
                // let mut tmp = &mut l[li];
                // let mut tmp1 = &mut r[ri - li - 1];

                // tmp1.splice(tmp1.len().., tmp.iter().cloned());
                // tmp.clear();
                unsafe {
                    let tmp = vv[li].clone();
                    let mut tmp1 = &mut vv[ri];

                    tmp1.splice(tmp1.len().., tmp);
                    vv[li].clear()
                }
            }
        }
    }
}
