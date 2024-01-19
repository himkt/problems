fn f(x: f64, p: f64) -> f64 {
    x + p / 2.0_f64.powf(x / 1.5)
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let p: f64 = scanner.cin();

    let mut l = 0.0_f64;
    let mut r = 1e8_f64;

    for _ in 0..1000 {
        let cl = l + (r - l) / 3.0_f64;
        let cr = l + (2.0_f64 * (r - l)) / 3.0_f64;
        let fl = f(cl, p);
        let fr = f(cr, p);
        debug!("l={}, r={}, fcl={}, fcr={}", l, r, fl, fr);

        if fl < fr {
            r = cr;
        }
        else {
            l = cl;
        }
    }

    println!("{}", f(l, p));
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
