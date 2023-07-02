const DIV: usize = 1_000_000_007;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let p: usize = scanner.cin();

    let mut ans = p - 1;
    let power = modpow(p - 2, n - 1, DIV);
    ans *= power;
    ans %= DIV;

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

pub fn modpow(mut a: usize, mut n: usize, m: usize) -> usize {
    let mut ans = 1;

    while n > 0 {
        if n & 1 == 1 {
            ans = ans * a % m;
        }

        a = a * a % m;
        n >>= 1;
    }

    ans
}
