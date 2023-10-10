const ZMAX: usize = 100_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let mut dp = vec![vec![!0; ZMAX + 1]; n];
    let mut ztot = 0;

    let x0: usize = scanner.cin();
    let y0: usize = scanner.cin();
    let z0: usize = scanner.cin();
    ztot += z0;

    if x0 > y0 {
        dp[0][z0] = 0;
    }
    else {
        // win
        let tot = x0 + y0;
        let dom = (tot / 2) + 1;
        dp[0][z0] = dom - x0;

        // lose
        dp[0][0] = 0;
    }

    for i in 0..(n - 1) {
        let x: usize = scanner.cin();
        let y: usize = scanner.cin();
        let z: usize = scanner.cin();
        ztot += z;

        for k in 0..=ZMAX {
            if k + z > ZMAX {
                continue;
            }
            if dp[i][k] == !0 {
                continue;
            }

            if x > y {
                dp[i + 1][k + z] = dp[i + 1][k + z].min(dp[i][k]);
            }
            else {
                let tot = x + y;
                let dom = (tot / 2) + 1;
                let cost = dom - x;
                // pay to win
                dp[i + 1][k + z] = dp[i + 1][k + z].min(dp[i][k] + cost);
                // leave lose
                dp[i + 1][k] = dp[i + 1][k].min(dp[i][k]);
            }
        }
    }

    let dom = (ztot / 2) + 1;
    let mut ans = !0;
    for z in dom..=ZMAX {
        ans = ans.min(dp[n - 1][z]);
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
