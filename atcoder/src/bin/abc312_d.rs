const MOD: usize = 998244353;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let n: usize = s.len();

    let mut dp = ndarray!(0; n + 1, n + 1, 2);
    dp[0][0][0] = 1;

    for i in 0..n {
        for j in 0..=n {
            let sum = (dp[i][j][0] + dp[i][j][1]) % MOD;

            if s[i] == '?' {
                if j < n {
                    dp[i + 1][j + 1][0] += sum;
                    dp[i + 1][j + 1][0] %= MOD;
                }
                if j > 0 {
                    dp[i + 1][j - 1][1] += sum;
                    dp[i + 1][j - 1][1] %= MOD;
                }
            }
            else if s[i] == '(' && j < n {
                dp[i + 1][j + 1][0] += sum;
                dp[i + 1][j + 1][0] %= MOD;
            }
            else if s[i] == ')' && j > 0 {
                dp[i + 1][j - 1][1] += sum;
                dp[i + 1][j - 1][1] %= MOD;
            }
        }
    }

    debug!("{:?}", dp);
    println!("{}", dp[n][0][1]);
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
