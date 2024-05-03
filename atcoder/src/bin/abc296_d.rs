#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: u128 = scanner.cin();
    let m: u128 = scanner.cin();
    if n * n < m {
        println!("-1");
        return;
    }
    if m <= n {
        println!("{}", m);
        return;
    }

    let mut ans = u128::MAX;
    let u = (m as f64).sqrt().ceil() as u128;
    let l = std::cmp::min(n + 1, m);

    for p in 1..=u {
        let q = lower_bound(1..l, &|x| p * x >= m);
        if q == l {
            continue;
        }
        ans = std::cmp::min(ans, p * q);
    }

    if ans == u128::MAX {
        println!("-1");
        return;
    }
    println!("{}", ans);
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

pub fn lower_bound(range: std::ops::Range<u128>, prop: &dyn Fn(u128) -> bool) -> u128 {
    if prop(range.start) {
        return range.start;
    }

    let mut ng = range.start;
    let mut ok = range.end;

    while ok - ng > 1 {
        let middle = ng + (ok - ng) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
}
