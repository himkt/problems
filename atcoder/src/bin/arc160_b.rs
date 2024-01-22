const MOD: usize = 998_244_353;

fn multiply(a: usize, b: usize) -> usize {
    (a * b) % MOD
}

fn solve(n: usize) -> usize {
    let mut ans = 0;

    // [1]
    ans += (n as f64).sqrt() as usize;
    ans %= MOD;

    // [2]
    let mut y = 1;
    while y * y < n {
        ans += multiply(3, (n / y) - y);
        ans %= MOD;
        y += 1;
    }

    // [3]
    let mut y = 2;
    while y * y <= n {
        ans += multiply(3, y - 1);
        ans %= MOD;
        y += 1;
    }

    // [4]
    let mut y = 2;
    while y * y < n {
        let cmb = multiply((n / y) - y , y - 1);
        ans += multiply(6, cmb);
        ans %= MOD;
        y += 1;
    }

    ans
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let t: usize = scanner.cin();
    for _ in 0..t {
        let n: usize = scanner.cin();
        println!("{}", solve(n));
    }
}

use std::io::Write;
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
