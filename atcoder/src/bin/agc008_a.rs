const INF: i64 = 1_000_000_000_000;

fn dist(mut x: i64, mut y: i64, b0: bool, bn: bool) -> Option<i64> {
    match (b0, bn) {
        (true, true) => {
            x *= -1;
            y *= -1;
            if x <= y {
                return Some(y - x + 2);
            }
        },
        (true, false) => {
            x *= -1;
            if x <= y {
                return Some(y - x + 1);
            }
        },
        (false, true) => {
            y *= -1;
            if x <= y {
                return Some(y - x + 1);
            }
        },
        (false, false) => {
            if x <= y {
                return Some(y - x);
            }
        }
    }

    if b0 && bn {
    }

    None
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let x: i64 = scanner.cin();
    let y: i64 = scanner.cin();

    let mut ans = INF;
    for &b0 in [true, false].iter() {
        for &bn in [true, false].iter() {
            debug!("b0={}, bn={}, dist={:?}", b0, bn, dist(x, y, b0, bn));
            if let Some(c) = dist(x, y, b0, bn) {
                ans = ans.min(c);
            }
        }
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
