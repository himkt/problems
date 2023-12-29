#[allow(clippy::needless_range_loop)]
fn swap_col(mat: &[Vec<usize>], j1: usize, j2: usize) -> Vec<Vec<usize>> {
    let mut ret: Vec<Vec<usize>> = mat.into();
    let n: usize = mat.len();
    for i in 0..n {
        ret[i].swap(j1, j2);
    }
    ret
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();

    let a: Vec<Vec<usize>> = (0..h)
        .map(|_| scanner.vec(w))
        .collect();

    let b: Vec<Vec<usize>> = (0..h)
        .map(|_| scanner.vec(w))
        .collect();

    let mut dist = HashMap::new();
    dist.insert(a.clone(), 0);

    let mut queue = VecDeque::new();
    queue.push_back((a, 0));

    while let Some((u, cost)) = queue.pop_front() {
        for i in 0..(h - 1) {
            let mut v = u.clone();
            v.swap(i, i + 1);
            if dist.contains_key(&v) {
                continue;
            }
            dist.insert(v.clone(), cost + 1);
            queue.push_back((v, cost + 1));
        }
        for j in 0..(w - 1) {
            let v = swap_col(&u.clone(), j, j + 1);
            if dist.contains_key(&v) {
                continue;
            }
            dist.insert(v.clone(), cost + 1);
            queue.push_back((v, cost + 1));
        }
    }

    if let Some(ans) = dist.get(&b) {
        println!("{}", ans);
    }
    else {
        println!("-1");
    }
}

use std::{io::Write, collections::{VecDeque, HashMap}};
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
