#[derive(PartialEq, Debug)]
pub enum Direction {
    Forward,
    Bckward,
}

const MAX: usize = 1_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];

    for _ in 0..m {
        let u: usize = scanner.cin::<usize>() - 1;
        let v: usize = scanner.cin::<usize>() - 1;
        g[u].push(v);
        g[v].push(u);
    }

    let mut ans = 0;

    let mut used = vec![false; n];
    let mut stack = VecDeque::new();
    stack.push_front((0, Direction::Bckward));
    stack.push_front((0, Direction::Forward));

    while let Some((u, direction)) = stack.pop_front() {
        debug!("u={}, direction={:?}", u, direction);
        debug!("{:?}", stack);
        if direction == Direction::Bckward {
            used[u] = false;
            continue;
        }
        else {
            ans += 1;
            if ans == MAX {
                break;
            }
        }
        used[u] = true;

        for &v in g[u].iter() {
            if used[v] {
                continue;
            }
            stack.push_front((v, Direction::Bckward));
            stack.push_front((v, Direction::Forward));
        }
    }

    println!("{}", ans);
}

use std::{io::Write, collections::VecDeque};
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
