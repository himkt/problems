const MOD: usize = 1_000_000_007;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let mut board: Vec<Vec<char>> = vec![];
    for _ in 0..h {
        board.push(scanner.cin::<String>().chars().collect());
    }

    let mut dp = ndarray!(0; h + 1, w + 1);
    let mut dpl = ndarray!(0; h + 1, w + 1);
    let mut dpu = ndarray!(0; h + 1, w + 1);
    let mut dplu = ndarray!(0; h + 1, w + 1);

    for i in 1..=h {
        for j in 1..=w {
            if i == 1 && j == 1 {
                dp[i][j] = 1;
            }

            dp[i][j] += dpu[i - 1][j];
            dp[i][j] += dpl[i][j - 1];
            dp[i][j] += dplu[i - 1][j - 1];
            dp[i][j] %= MOD;

            dpu[i][j] = (dpu[i - 1][j] + dp[i][j]) % MOD;
            dpl[i][j] = (dpl[i][j - 1] + dp[i][j]) % MOD;
            dplu[i][j] = (dplu[i - 1][j - 1] + dp[i][j]) % MOD;

            if board[i - 1][j - 1] == '#' {
                dp[i][j] = 0;
                dpl[i][j] = 0;
                dpu[i][j] = 0;
                dplu[i][j] = 0;
            }
        }
    }

    debug!("{:?}", dp);
    debug!("{:?}", dpu);
    debug!("{:?}", dpl);
    debug!("{:?}", dplu);

    println!("{}", dp[h][w]);
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
