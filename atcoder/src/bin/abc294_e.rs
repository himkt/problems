#[allow(clippy::needless_range_loop)]
#[allow(clippy::comparison_chain)]
fn main() {
    let mut scanner = Scanner::default();
    let _: usize = scanner.cin();
    let n1: usize = scanner.cin();
    let n2: usize = scanner.cin();

    let mut p1: VecDeque<(usize, usize)> = (0..n1).map(|_| {
        let vi: usize = scanner.cin();
        let li: usize = scanner.cin();
        (vi, li)
    }).collect();
    let mut p2: VecDeque<(usize, usize)> = (0..n2).map(|_| {
        let vi: usize = scanner.cin();
        let li: usize = scanner.cin();
        (vi, li)
    }).collect();

    let mut ans = 0;
    while !p1.is_empty() && !p2.is_empty() {
        let (v1, l1) = p1[0];
        let (v2, l2) = p2[0];

        if v1 == v2 {
            ans += l1.min(l2);
        }

        if l1 == l2 {
            p1.pop_front();
            p2.pop_front();
        }
        else if l1 > l2 {
            p1[0].1 -= l2;
            p2.pop_front();
        }
        else if l1 < l2 {
            p1.pop_front();
            p2[0].1 -= l1;
        }
    }

    println!("{}", ans);
}

use std::{io::Write, collections::VecDeque};
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
    buf: String,
}
impl Scanner {
    pub fn default() -> Self {
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
        println!($($arg)*);
    };
}
