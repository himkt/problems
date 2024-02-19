#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let events: Vec<_> = (0..n)
        .map(|_| {
            let t: usize = scanner.cin();
            let x: usize = scanner.cin();
            (t, x)
        })
        .collect();

    let mut used = vec![0; n];
    let mut requests = HashMap::new();
    for (_i, &(ti, xi)) in events.iter().rev().enumerate() {
        let i = n - _i - 1;

        match ti {
            1 => {
                if requests.contains_key(&xi) {
                    requests.entry(xi).and_modify(|x| *x -= 1);
                    used[i] = 1;
                    if requests[&xi] == 0 {
                        requests.remove(&xi);
                    }
                }
            }
            2 => {
                *requests.entry(xi).or_insert(0) += 1;
            }
            _ => panic!()
        }
    }

    if !requests.is_empty() {
        println!("-1");
        return;
    }

    let mut cur_k: usize = 0;
    let mut max_k: usize = 0;
    for i in 0..n {
        if events[i].0 == 2 {
            cur_k -= 1;
            continue;
        }
        if used[i] == 1 {
            cur_k += 1;
            max_k = cmp::max(max_k, cur_k);
        }
    }

    let s = (0..n)
        .filter(|&i| events[i].0 == 1)
        .map(|i| used[i].to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", max_k);
    println!("{}", s);
}

use std::{cmp, collections::HashMap, io::Write};
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
