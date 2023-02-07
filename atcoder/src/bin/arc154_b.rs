#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let s: Vec<char> = scanner
        .cin::<String>()
        .chars()
        .rev()
        .collect();
    let t: Vec<char> = scanner
        .cin::<String>()
        .chars()
        .rev()
        .collect();

    let mut cnts = HashMap::new();
    let mut cntt = HashMap::new();
    for i in 0..n {
        *cnts.entry(s[i]).or_insert(0) += 1;
        *cntt.entry(t[i]).or_insert(0) += 1;
    }

    if cnts != cntt {
        println!("-1");
        return;
    }

    debug!("s={:?}, t={:?}", s, t);

    let mut l = 0;
    let mut j = 0;
    for i in 0..n {
        debug!("i={}, j={}, l={}", i, j, l);
        if j >= n {
            break;
        }
        while j < n && s[i] != t[j] {
            debug!("j++;");
            j += 1;
        }
        if j == n {
            break;
        }
        debug!("j++;");
        debug!("l++;");
        j += 1;
        l += 1;
    }

    println!("{}", n - l);
}

use std::{io::Write, collections::HashMap};
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
