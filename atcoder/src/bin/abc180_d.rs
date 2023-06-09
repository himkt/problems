fn upper_bound1(x: u128, a: u128, xu: u128) -> (u128, u128) {
    let mut cnt = 0;
    let mut xc = x;

    loop {
        xc *= a;
        cnt += 1;
        if xc * a >= xu {
            break;
        }
    }

    (xc, cnt)
}

fn upper_bound2(x: u128, b: u128, xu: u128) -> (u128, u128) {
    let xd = xu - x;
    let cnt = xd / b;
    (x + b * cnt, cnt)
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let mut x: u128 = scanner.cin();
    let y: u128 = scanner.cin();
    let a: u128 = scanner.cin();
    let b: u128 = scanner.cin();
    let mut exp: u128 = 0;

    loop {
        let x1 = a * x;
        let x2 = x + b;
        if x1 >= y && x2 >= y {
            break;
        }

        if x1 <= x2 {
            let upper_bound = x2.min(y - 1);
            let (xn, cnt) = upper_bound1(x, a, upper_bound);
            exp += cnt;
            x = xn;
        }
        else {
            let upper_bound = x1.min(y - 1);
            let (xn, cnt) = upper_bound2(x, b, upper_bound);
            exp += cnt;
            x = xn;
        }
    }

    println!("{}", exp);
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
