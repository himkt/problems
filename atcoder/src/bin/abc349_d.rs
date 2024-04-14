#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let l: usize = scanner.cin();
    let r: usize = scanner.cin();

    let mut m = 0;
    let mut li = l;
    let mut hist = vec![];
    loop {
        let mut i = 0;
        loop {
            let ci = i + 1;
            let p = 2usize.pow(ci as u32);
            let j = li / p;
            if p * j == li && p * (j + 1) <= r {
                i += 1;
                continue;
            }
            break;
        }
        let p = 2usize.pow(i as u32);
        let j = li / p;

        let v0 = li;
        li = 2usize.pow(i as u32) * (j + 1);
        let v1 = li;
        hist.push((v0, v1));
        m += 1;
        if li == r {
            break;
        }
    }

    println!("{}", m);
    for (v0, v1) in hist {
        println!("{} {}", v0, v1);
    }
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
