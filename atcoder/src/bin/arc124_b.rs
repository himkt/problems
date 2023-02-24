#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let mut a: Vec<usize> = scanner.vec(n);
    a.sort_unstable();
    let b: Vec<usize> = scanner.vec(n);

    let mut ans = vec![];

    for i in 0..n {
        let mut bb = b.clone();
        let x = a[0] ^ b[i];
        for j in 0..n {
            bb[j] ^= x;
        }
        bb.sort_unstable();

        if a == bb {
            debug!("x={}", x);
            debug!("a={:?}", a);
            debug!("b={:?}", bb);
            ans.push(x);
        }
    }

    ans.sort_unstable();
    ans.dedup();
    println!("{}", ans.len());
    for v in ans {
        println!("{}", v);
    }
}

use std::io::Write;
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
