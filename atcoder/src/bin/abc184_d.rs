pub struct Solver {
    cache: HashMap<(usize, usize, usize), f64>,
}

#[allow(clippy::new_without_default)]
impl Solver {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    pub fn solve(&mut self, a: usize, b: usize, c: usize) -> f64 {
        debug!("solve({}, {}, {})", a, b, c);
        if [a, b, c].iter().any(|&x| x == 100) {
            return 0.0;
        }

        if let Some(&e) = self.cache.get(&(a, b, c)) {
            return e;
        }

        let af = a as f64;
        let bf = b as f64;
        let cf = c as f64;
        let zf = af + bf + cf;

        let p = (af / zf) * (self.solve(a + 1, b, c) + 1.0)
                   + (bf / zf) * (self.solve(a, b + 1, c) + 1.0)
                   + (cf / zf) * (self.solve(a, b, c + 1) + 1.0);

        self.cache.insert((a, b, c), p);
        p
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let a: usize = scanner.cin();
    let b: usize = scanner.cin();
    let c: usize = scanner.cin();

    let mut solver = Solver::new();
    let ans = solver.solve(a, b, c);
    // debug!("{:?}", solver.cache);
    println!("{}", ans);
}

use std::{io::Write, collections::HashMap};
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
