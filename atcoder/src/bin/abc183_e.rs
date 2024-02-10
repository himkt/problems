// WA
const MOD: usize = 1_000_000_007;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();

    let mut board = vec![];
    for _ in 0..h {
        board.push(scanner.chars());
    }

    let mut dp = ndarray!(0; h, w);
    let mut cumsum = ndarray!(0; 3, h, w);

    dp[0][0] = 1;
    for k in 0..3 {
        cumsum[k][0][0] = 1;
    }

    for i in 0..h {
        for j in 0..w {
            debug!("i={}, j={}", i, j);
            if i == 0 && j == 0 {
                continue;
            }

            if board[i][j] == '#' {
                dp[i][j] = 0;
                for k in 0..3 {
                    cumsum[k][i][j] = 0;
                }
                continue;
            }

            if i > 0 {
                dp[i][j] += cumsum[0][i - 1][j];
                dp[i][j] %= MOD;
            }
            if j > 0 {
                dp[i][j] += cumsum[1][i][j - 1];
                dp[i][j] %= MOD;
            }
            if i > 0 && j > 0 {
                dp[i][j] += cumsum[2][i - 1][j - 1];
                dp[i][j] %= MOD;
            }

            for k in 0..3 {
                cumsum[k][i][j] = dp[i][j];
            }

            if i > 0 {
                cumsum[0][i][j] += cumsum[0][i - 1][j];
                cumsum[0][i][j] %= MOD;
            }
            if j > 0 {
                cumsum[1][i][j] += cumsum[1][i][j - 1];
                cumsum[1][i][j] %= MOD;
            }
            if i > 0 && j > 0 {
                cumsum[2][i][j] += cumsum[2][i - 1][j - 1];
                cumsum[2][i][j] %= MOD;
            }

        }
    }

    debug!("# dp");
    for row in dp.iter() {
        debug!("{:?}", row);
    }

    debug!("# cumsum");
    for k in 0..3 {
        debug!("k={}", k);
        for row in cumsum[k].iter() {
            debug!("{:?}", row);
        }
    }

    println!("{}", dp[h-1][w-1]);
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
    pub fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
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
