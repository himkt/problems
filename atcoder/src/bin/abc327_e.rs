#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let p: Vec<f64> = scanner.vec::<f64>(n)
        .into_iter()
        .rev()
        .collect();

    let mut dp = ndarray!(0f64; n, n);
    for i in 0..n {
        dp[i][0] = p[i];
    }
    for i in 1..n {
        dp[i][0] = dp[i][0].max(dp[i - 1][0]);
    }

    let mut powers = vec![0.0f64; n + 1];
    for i in 0..n {
        powers[i] = 0.9_f64.powi(i as i32);
    }
    debug!("powers={:?}", powers);

    let mut cum_powers = powers.clone();
    for i in 1..n {
        cum_powers[i] += cum_powers[i - 1];
    }
    debug!("cum_powers={:?}", cum_powers);

    for k in 1..n {
        for i in k..n {
            let val = powers[k] * p[i];
            dp[i][k] = dp[i - 1][k].max(dp[i - 1][k - 1] + val);
        }
    }

    debug!("dp table:");
    for row in dp.iter() {
        debug!("{:?}", row);
    }

    let mut candidates = vec![];
    for k in 0..n {
        let kf = (k + 1) as f64;
        let kfsqrt = 1200.0 / kf.sqrt();
        let candidate = (dp[n - 1][k] / cum_powers[k]) - kfsqrt;
        debug!("k={}, kf={}, kfsqrt={}, val={}", k, kf, kfsqrt, candidate);
        candidates.push(candidate);
    }

    let mut ans = candidates[0];
    for &candidate in &candidates[1..] {
        ans = ans.max(candidate);
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

#[macro_export]
macro_rules! ndarray {
    // ndarray!(val; *shape)
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}
