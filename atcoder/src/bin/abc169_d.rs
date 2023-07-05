fn prime_factorize(mut n: usize) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    let mut p = 2;

    while p * p <= n {
        if n % p == 0 {
            let mut k = 0;
            while n % p == 0 {
                k += 1;
                n /= p;
            }
            ret.push((p, k));
        }

        p += 1;
    }

    if n != 1 {
        ret.push((n, 1));
    }
    ret
}

fn calc_num_divisions(n: &mut usize, mut p: usize, k: usize) -> usize {
    let mut ret = 0;
    let op = p;

    while p <= *n && ret < k && *n % p == 0 {
        *n /= p;
        p *= op;
        ret += 1;
    }

    ret
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let mut ans = 0;
    let mut n: usize = scanner.cin();
    for (p, k) in prime_factorize(n) {
        let ai = calc_num_divisions(&mut n, p, k);
        ans += ai;
    }
    println!("{}", ans);
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
        print!("[debug] ");
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}
