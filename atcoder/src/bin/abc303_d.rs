const INF: usize = 1_000_000_000_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let x: usize = scanner.cin();
    let y: usize = scanner.cin();
    let z: usize = scanner.cin();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let n: usize = s.len();

    let mut dp = ndarray!(INF; n + 1, 2);
    dp[0][0] = 0;

    for i in 1..=n {
        // (1) input
        // (2) shift + input
        // (3) caps lock + input

        if s[i - 1] == 'A' {
            // target: A
            dp[i][0] = dp[i][0].min(dp[i - 1][1] + z + y).min(dp[i - 1][0] + y);
            dp[i][1] = dp[i][1].min(dp[i - 1][0] + z + x).min(dp[i - 1][1] + x);
        }
        else {
            // target: a
            dp[i][0] = dp[i][0].min(dp[i - 1][0] + x).min(dp[i - 1][1] + z + x);
            dp[i][1] = dp[i][1].min(dp[i - 1][1] + y).min(dp[i - 1][0] + z + y);
        }
    }

    let ans = dp[n][0].min(dp[n][1]);
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
