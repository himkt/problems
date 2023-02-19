const MOD: i128 = 1_000_000_000 + 7;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let a: Vec<i128> = scanner.vec(n);

    let mut sum0 = a[0];
    let mut sum1 = 0;
    debug!("sum0={}, sum1={}", sum0, sum1);

    let mut dp = vec![vec![0; 2]; n];
    dp[0][0] = 1;
    dp[0][1] = 0;

    for i in 1..n {
        let (s0, s1) = (sum0, sum1);
        sum0 = (s0 + s1) % MOD;
        sum0 += a[i] * (dp[i - 1][0] + dp[i - 1][1]) % MOD;
        sum1 = (s0 - (a[i] * dp[i - 1][0]) % MOD) % MOD;
        debug!("sum0={}, sum1={}", sum0, sum1);

        dp[i][0] = (dp[i - 1][0] + dp[i - 1][1]) % MOD;
        dp[i][1] = dp[i - 1][0] % MOD;
    }

    let ans = (sum0 + sum1) % MOD;
    println!("{}", ans);

    for row in dp.iter() {
        debug!("{:?}", row);
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
