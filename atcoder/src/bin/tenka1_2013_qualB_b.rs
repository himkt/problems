#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let q: usize = scanner.cin();
    let l: usize = scanner.cin();

    let mut current_size = 0;
    let mut stack = VecDeque::new();
    for _ in 0..q {
        let query: String = scanner.cin();
        debug!("[before] query={}, {:?}, size={}", query, stack, current_size);

        match query.as_str() {
            "Push" => {
                let n: usize = scanner.cin();
                let m: i64 = scanner.cin();

                if current_size + n > l {
                    println!("FULL");
                    return;
                }

                stack.push_front((m, n));
                current_size += n;
            }
            "Pop"  => {
                let mut n: usize = scanner.cin();

                if n > current_size {
                    println!("EMPTY");
                    return;
                }

                while n > 0 {
                    if stack.is_empty() {
                        println!("EMPTY");
                        return;
                    }

                    if n > stack[0].1 {
                        n -= stack[0].1;
                        current_size -= stack[0].1;
                        stack.pop_front();
                        continue;
                    }

                    let rest = stack[0].1 - n;
                    current_size -= n;
                    stack[0].1 = rest;

                    if stack[0].1 == 0 {
                        stack.pop_front();
                    }

                    break;
                }
            }
            "Top"  => {
                if stack.is_empty() {
                    println!("EMPTY");
                    return;
                }
                println!("{}", stack[0].0);
            }
            "Size" => {
                println!("{}", current_size);
            }
            _      => panic!()
        }

        debug!("[after] {:?}, size={}", stack, current_size);
    }

    println!("SAFE");
}

use std::{collections::VecDeque, io::Write};
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
