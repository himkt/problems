#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let mut a: Vec<Vec<char>> = vec![vec!['.'; n]; n];
    let mut b: Vec<Vec<char>> = vec![vec!['.'; n]; n];
    for i in 0..n {
        let s: Vec<char> = scanner.cin::<String>().chars().collect();
        for j in 0..n {
            a[i][j] = s[j];
        }
    }
    for i in 0..n {
        let s: Vec<char> = scanner.cin::<String>().chars().collect();
        for j in 0..n {
            b[i][j] = s[j];
        }
    }

    for i in 0..n {
        for j in 0..n {
            if a[i][j] != b[i][j] {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }

    panic!();
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
