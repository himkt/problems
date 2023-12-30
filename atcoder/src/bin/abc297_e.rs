#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let mut set = BTreeSet::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);
    for &ai in a.iter() {
        set.insert(ai);
    }

    let mut cnt = 0;
    while let Some(u) = set.pop_first() {
        debug!("cnt={}, u={}", cnt, u);
        cnt += 1;
        if cnt == k {
            println!("{}", u);
            return;
        }
        for &v in a.iter() {
            set.insert(u + v);
        }
    }
}

use std::{io::Write, collections::BTreeSet};
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
