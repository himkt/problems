#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let mut k: usize = scanner.cin();

    let mut g = vec![!0; n];
    for i in 0..n {
        let ai: usize = scanner.cin();
        g[i] = ai - 1;
    }

    let mut used = vec![false; n];
    let mut cursor = 0;
    while !used[cursor] {
        used[cursor] = true;
        cursor = g[cursor];
    }
    let loop_from = cursor;

    let mut cursor = 0;
    let mut num_nodes_before_loop = 0;
    while cursor != loop_from {
        cursor = g[cursor];
        num_nodes_before_loop += 1;
    }

    debug!("num_nodes_before_loop={}", num_nodes_before_loop);
    debug!("loop_from={}", loop_from);

    let mut cursor = loop_from;
    let mut num_nodes_in_loop = 0;
    let mut used = vec![false; n];
    while !used[cursor] {
        used[cursor] = true;
        cursor = g[cursor];
        num_nodes_in_loop += 1;
    }

    let mut cursor = 0;
    debug!("cursor={}", cursor);
    debug!("k={}", k);

    while cursor != loop_from {
        cursor = g[cursor];
        k -= 1;
        if k == 0 {
            break;
        }
    }

    if k > 0 {
        cursor = loop_from;
        let kk = k % num_nodes_in_loop;
        for _ in 0..kk {
            cursor = g[cursor];
        }
    }

    println!("{}", cursor + 1);
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
