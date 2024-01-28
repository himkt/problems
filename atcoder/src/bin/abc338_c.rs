#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let q: Vec<usize> = scanner.vec(n);
    let a: Vec<usize> = scanner.vec(n);
    let b: Vec<usize> = scanner.vec(n);

    let mut ans = 0;
    for na in 0..=1_000_000 {
        let mut possible = true;
        let mut nb = 1_000_000_000;

        for i in 0..n {
            let da = a[i] * na;
            if q[i] < da {
                possible = false;
                break;
            }
            if b[i] == 0 {
                continue;
            }
            nb = std::cmp::min(nb, (q[i] - da) / b[i]);
        }

        if !possible {
            continue;
        }

        debug!("na={}, nb={}", na, nb);
        ans = std::cmp::max(ans, na + nb);
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
