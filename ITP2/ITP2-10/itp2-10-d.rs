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
    let n: usize = sc.get();
    let mut bits = 0u64;
    let mut masks: Vec<u64> = vec![0u64; n];

    for i in 0..n {
        sc.new_line();
        let v = sc.get_as_vec::<u64>();
        for j in 1..v.len() {
            masks[i] |= 1u64 << v[j];
        }
    }

    sc.new_line();
    let q: usize = sc.get();

    for _ in 0..q {
        sc.new_line();
        let v = sc.get_as_vec::<u64>();
        if v[0] == 0 {
            println!("{}", if bits & (1u64 << v[1]) != 0 { 1 } else { 0 })
        } else if v[0] == 1 {
            bits |= masks[v[1] as usize];
        } else if v[0] == 2 {
            bits &= !masks[v[1] as usize];
        } else if v[0] == 3 {
            bits ^= masks[v[1] as usize];
        } else if v[0] == 4 {
            println!(
                "{}",
                if !bits & masks[v[1] as usize] == 0 {
                    1
                } else {
                    0
                }
            )
        } else if v[0] == 5 {
            println!(
                "{}",
                if bits & masks[v[1] as usize] != 0 {
                    1
                } else {
                    0
                }
            )
        } else if v[0] == 6 {
            println!(
                "{}",
                if bits & masks[v[1] as usize] == 0 {
                    1
                } else {
                    0
                }
            )
        } else if v[0] == 7 {
            // print!("{:032b}\n", bits);
            let mut cnt: u64 = 0;
            for s in 0..64 as u64 {
                if (((bits & masks[v[1] as usize]) >> s) & 1u64) == 1 {
                    cnt += 1;
                }
            }
            println!("{}", cnt);
        } else {
            println!("{}", bits & masks[v[1] as usize]);
        }
    }
}
