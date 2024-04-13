#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let q: usize = scanner.cin();

    let mut ret = n;
    let mut g: Vec<HashSet<usize>> = vec![HashSet::new(); n];

    for _ in 0..q {
        let qtype: usize = scanner.cin();
        match qtype {
            1 => {
                let u: usize = scanner.cin::<usize>() - 1;
                let v: usize = scanner.cin::<usize>() - 1;
                if g[u].is_empty() {
                    ret -= 1;
                }
                if g[v].is_empty() {
                    ret -= 1;
                }
                g[u].insert(v);
                g[v].insert(u);
            }
            2 => {
                let v: usize = scanner.cin::<usize>() - 1;
                if !g[v].is_empty() {
                    ret += 1;
                }

                let us = g[v].clone();
                for u in us {
                    if g[u].remove(&v) && g[u].is_empty() {
                        ret += 1;
                    }
                }
                g[v] = HashSet::new();
            }
            _ => panic!()
        }
        println!("{}", ret);
    }
}

use std::{collections::HashSet, io::Write};
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
    buf: String,
}
impl Scanner {
    pub fn create() -> Self {
        Scanner {
            buffer: std::collections::VecDeque::new(),
            buf: String::new(),
        }
    }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() {
            return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap();
        }
        self.buf.truncate(0);
        std::io::stdin().read_line(&mut self.buf).ok();
        self.buf
            .to_owned()
            .split_whitespace()
            .for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
    pub fn flush(&self) {
        std::io::stdout().flush().unwrap();
    }
}

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        println!();
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        print!("[debug] ");
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}
