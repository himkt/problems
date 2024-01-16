fn conv(a: &[usize]) -> Vec<usize> {
    let n: usize = a.len();
    let mut dp = vec![1; n];
    for i in 1..n {
        dp[i] = cmp::min(dp[i - 1] + 1, a[i]);
    }
    dp
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin::<usize>();
    let a: Vec<usize> = scanner.vec(n);
    let b: Vec<usize> = a.clone().into_iter().rev().collect();
    debug!("a={:?}", a);
    debug!("b={:?}", b);

    let ldp: Vec<usize> = conv(&a);
    let rdp: Vec<usize> = conv(&b).into_iter().rev().collect();
    debug!("ldp={:?}", ldp);
    debug!("rdp={:?}", rdp);

    let mut ans = 1;
    for i in 1..(n - 1) {
        ans = cmp::max(ans, cmp::min(ldp[i], rdp[i]));
    }
    println!("{}", ans);
}

use std::{io::Write, cmp};
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
