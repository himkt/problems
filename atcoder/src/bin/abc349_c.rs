#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let t: Vec<char> = scanner.cin::<String>().to_lowercase().chars().collect();

    let mut cur_t = 0;
    let mut score = 0;

    for i in 0..s.len() {
        if s[i] == t[cur_t] {
            cur_t += 1;
            score += 1;
        }
        if score == 3 {
            println!("Yes");
            return;
        }
    }

    if score == 2 && t[2] == 'x' {
            println!("Yes");
    }
    else {
        println!("No");
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
