#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();

    let mut lmax: usize = scanner.cin();
    let mut rmin: usize = scanner.cin();
    println!("0");

    for _ in 0..(n - 1) {
        let li: usize = scanner.cin();
        let ri: usize = scanner.cin();
        lmax = lmax.max(li);
        rmin = rmin.min(ri);
        if lmax <= rmin {
            println!("0");
        }
        else {
            let x: usize = (lmax + rmin) / 2;
            let y: usize = (lmax - x).max(x - rmin);
            println!("{}", y);
        }
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
        print!("[debug] ");
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}
