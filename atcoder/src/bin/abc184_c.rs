fn dist(r1: i128, c1: i128, r2: i128, c2: i128) -> i128 {
    (r1 - r2).abs() + (c1 - c2).abs()
}

fn reach(r1: i128, c1: i128, r2: i128, c2: i128) -> bool {
    if r1 + c1 == r2 + c2 {
        return true;
    }
    if r1 - c1 == r2 - c2 {
        return true;
    }
    if dist(r1, c1, r2, c2) <= 3 {
        return true;
    }
    false
}

fn vert2(r1: i128, c1: i128, r2: i128, c2: i128) -> bool {
    dist(r1, c1, r2, c2) <= 6
}

fn align_r2(r1: i128, c1: i128, r2: i128, c2: i128) -> bool {
    let d = r2 - r1;
    let (r3, c3) = (r2, c1 + d);
    let (r4, c4) = (r2, c1 - d);

    if reach(r3, c3, r2, c2) {
        return true;
    }
    if reach(r4, c4, r2, c2) {
        return true;
    }

    false
}

fn align_c2(r1: i128, c1: i128, r2: i128, c2: i128) -> bool {
    let d = c2 - c1;
    let (r3, c3) = (r1 + d, c2);
    let (r4, c4) = (r1 - d, c2);

    if reach(r3, c3, r2, c2) {
        return true;
    }
    if reach(r4, c4, r2, c2) {
        return true;
    }

    false
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let r1: i128 = scanner.cin();
    let c1: i128 = scanner.cin();
    let r2: i128 = scanner.cin();
    let c2: i128 = scanner.cin();

    if r1 == r2 && c1 == c2 {
        println!("0");
        return;
    }
    if reach(r1, c1, r2, c2) {
        println!("1");
        return;
    }
    if align_r2(r1, c1, r2, c2) {
        println!("2");
        return;
    }
    if align_c2(r1, c1, r2, c2) {
        println!("2");
        return;
    }
    let e1 = (r1 + c1) % 2;
    let e2 = (r2 + c2) % 2;
    if e1 == e2 {
        println!("2");
        return;
    }
    if vert2(r1, c1, r2, c2) {
        println!("2");
        return;
    }
    println!("3");
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
