fn check(a: VecDeque<usize>) -> bool {
    let n: usize = a.len();
    for i in 0..(n - 1) {
        if a[i] + 1 != a[i + 1] {
            return false;
        }
    }
    true
}

fn solve(mut a: VecDeque<usize>, rev0: bool, revn: bool) -> Option<usize> {
    let n: usize = a.len();

    if rev0 {
        a = a.into_iter().rev().collect();
    }

    let target = match revn {
        true => n,
        false => 1,
    };

    let mut num_op = 0;
    if rev0 {
        num_op += 1;
    }
    if revn {
        num_op += 1;
    }

    debug!("a={:?}, target={}", a, target);
    while a[0] != target {
        debug!("{:?}", a);
        let t = a.pop_front().unwrap();
        a.push_back(t);
        num_op += 1;
    }

    if revn {
        a = a.into_iter().rev().collect();
    }

    if check(a) {
        Some(num_op)
    }
    else {
        None
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let a: VecDeque<usize> = scanner.vec::<usize>(n).into_iter().collect();

    let mut ans = 1_000_000_000;
    for &rev0 in &[true, false] {
        for &revn in &[true, false] {
            debug!("===");
            if let Some(v) = solve(a.clone(), rev0, revn) {
                debug!("rev0={}, revn={}, v={}", rev0, revn, v);
                ans = ans.min(v);
            }
            else {
                debug!("rev0={}, revn={}, v={}", rev0, revn, "INF");
            }
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
