#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let _: usize = scanner.cin();
    let m: usize = scanner.cin();
    let mut relations: Vec<(usize, usize)> = (0..m)
        .map(|_| {
            let a: usize = scanner.cin();
            let b: usize = scanner.cin();
            (a, b)
        })
        .collect();
    relations.sort_by_key(|&(_, b)| b);
    debug!("relations: {:?}", relations);

    let mut ans = 1;
    let mut last = relations[0].1;
    for &(a, b) in &relations[1..] {
        debug!("current last={}", last);
        if a < last {
            continue;
        }
        debug!("divide {}", b);
        ans += 1;
        last = b;
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
