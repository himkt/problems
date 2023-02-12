const INF: usize = 1_000_000_000;

fn min(a: usize, b: usize) -> usize {
    a.min(b)
}

#[allow(clippy::if_same_then_else)]
#[allow(clippy::collapsible_else_if)]
#[allow(clippy::never_loop)]
fn has_isolate(a: &[Vec<usize>], h: usize, w: usize, i: usize, flip0: bool, flip1: bool, flip2: bool) -> bool {
    for j in 0..w {
        let mut ok = false;
        debug!("(i,j)=({},{})", i, j);

        for (ni, nj) in neighbors(h, w, i, j) {
            debug!("(ni,nj) = ({},{})", ni, nj);
            if i == ni {
                ok = ok || a[i][j] == a[ni][nj];
            }
            else {
                // 上
                if i == ni + 1 {
                    match flip1.eq(&flip0) {
                        true  => {
                            ok = ok || a[i][j] == a[ni][nj];
                        }
                        false => {
                            ok = ok || a[i][j] != a[ni][nj];
                        }
                    }
                }
                // 下
                else {
                    match flip1.eq(&flip2) {
                        true  => {
                            ok = ok || a[i][j] == a[ni][nj];
                        }
                        false => {
                            ok = ok || a[i][j] != a[ni][nj];
                        }
                    }
                }
            }
        }
        if ok {
            continue;
        }
        // neighbors を全てチェックしても隣接する要素が見つからない
        // => 孤立している
        return true;
    }
    false
}

fn neighbors(h: usize, w: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    if i > 0 {
        ret.push((i - 1, j));
    }
    if j < w - 1 {
        ret.push((i, j + 1));
    }
    if i < h - 1 {
        ret.push((i + 1, j));
    }
    if j > 0 {
        ret.push((i, j - 1));
    }
    ret
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let a: Vec<Vec<usize>> = (0..h).map(|_| scanner.vec::<usize>(w)).collect();

    let mut dp = ndarray![INF; h, 2, 2];
    dp[0][0][0] = 0;
    dp[0][0][1] = 1;
    dp[0][1][0] = 1;
    dp[0][1][1] = 2;

    for i in 1..h {
        for j in 0..2 {
            for k in 0..2 {
                for l in 0..2 {
                    // j: whether (i-2)-th row is flipped or not.
                    // k: whether (i-1)-th row is flipped or not.
                    // l: whether i-th row is flipped or not.

                    let ret = has_isolate(&a, h, w, i - 1, j == 1, k == 1, l == 1);
                    debug!("i={}, j={}, k={}, l={}, has_isolated={}", i, j, k, l, ret);
                    if ret {
                        continue
                    }

                    dp[i][k][l] = min(dp[i][k][l], dp[i - 1][j][k] + l);
                }
            }
        }
    }

    for row in dp.iter() {
        debug!("{:?}", row);
    }

    let mut ans = INF;
    for j in 0..2 {
        for k in 0..2 {
            if has_isolate(&a, h, w, h - 1, j == 1, k == 1, false) {
                continue;
            }
            debug!("[final] j={}, k={}", j, k);
            ans = min(ans, dp[h - 1][j][k]);
        }
    }
    match ans == INF {
        true => println!("-1"),
        false => println!("{}", ans)
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

#[macro_export]
macro_rules! ndarray {
    // ndarray!(val; *shape)
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}
