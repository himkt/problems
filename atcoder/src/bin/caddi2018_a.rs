#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let p: usize = scanner.cin();

    let mut ans = 1;
    for (d, f) in prime_factorize(p) {
        debug!("d={}, f={}", d, f);
        let power = f / n;
        if power >= 1 {
            ans *= d.pow(power as u32) ;
        }
    }
    println!("{}", ans);
}

fn prime_factorize(mut p: usize) -> Vec<(usize, usize)> {
    let mut cnt = HashMap::new();
    let mut i = 2;
    while i * i <= p {
        while p % i == 0 {
            *cnt.entry(i).or_insert(0) += 1;
            p /= i;
        }
        i += 1;
    }

    if p > 1 {
        *cnt.entry(p).or_insert(0) += 1;
    }
    cnt.into_iter().collect()
}

use std::{io::Write, collections::HashMap};
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
