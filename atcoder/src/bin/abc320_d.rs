#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let a: usize = scanner.cin::<usize>() - 1;
        let b: usize = scanner.cin::<usize>() - 1;
        let x: i128 = scanner.cin();
        let y: i128 = scanner.cin();

        g[a].push((b, x, y));
        g[b].push((a, -x, -y));
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut ret = HashMap::new();
    queue.push_back((0, (0, 0)));
    ret.insert(0, (0, 0));
    visited.insert(0);

    while let Some((u, (x, y))) = queue.pop_front() {
        debug!("u={}, ({}, {})", u, x, y);

        for &(v, dx, dy) in &g[u] {
            if visited.contains(&v) {
                continue;
            }
            ret.insert(v, (x + dx, y + dy));
            queue.push_back((v, (x + dx, y + dy)));
            visited.insert(v);
        }
    }

    for u in 0..n {
        if ret.contains_key(&u) {
            let (x, y) = ret[&u];
            println!("{} {}", x, y);
        }
        else {
            println!("undecidable");
        }
    }
}

use std::{io::Write, collections::{HashSet, VecDeque, HashMap}};
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
