#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let _: usize = scanner.cin();
    let q: usize = scanner.cin();

    let mut history = vec![(1, 0)];
    for _ in 0..q {
        let qtype: usize = scanner.cin();

        if qtype == 1 {
            let direction: char = scanner.cin();
            let last = history.last().unwrap();
            let mut current = (last.0, last.1);

            match direction {
                'R' => {
                    current.0 += 1;
                }
                'L' => {
                    current.0 -= 1;
                }
                'U' => {
                    current.1 += 1;
                }
                'D' => {
                    current.1 -= 1;
                }
                _   => panic!()
            }
            history.push(current);
            debug!("[0] hist: {:?}", history);
        }
        else if qtype == 2 {
            let p: usize = scanner.cin();
            debug!("[1] hist: {:?}, p={}", history, p);
            if p < history.len() {
                // let coordinate = history.iter().rev().nth(p - 1).unwrap();
                let coordinate = history[history.len() - p];
                println!("{} {}", coordinate.0, coordinate.1);
            }
            else {
                println!("{} 0", p + 1 - history.len());
            }
        }
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
