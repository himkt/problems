#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let n: usize = s.len();

    if s.len() == 2 {
        println!("dict");
        return;
    }

    let mut lb = 0;
    let mut rb = 0;
    for i in 1..(n-1) {
        if s[i] == '{' {
            lb += 1;
        }
        else if s[i] == '}' {
            rb += 1;
        }
        else if s[i] == ':' {
            debug!("column");
            if rb == lb {
                println!("dict");
                return;
            }
        }
        else if s[i] == ',' {
            if lb == rb {
                println!("set");
                return;
            }
        }
        else {
            // integer
            continue;
        }
    }

    println!("set");
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
