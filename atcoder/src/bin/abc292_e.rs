#[allow(clippy::never_loop)]
#[allow(clippy::needless_range_loop)]
fn traverse(n: usize, u: usize, graph: &[Vec<usize>]) -> Vec<bool> {
    let mut used = vec![false; n];
    let mut queue = VecDeque::new();
    queue.push_back(u);

    while let Some(u) = queue.pop_front() {
        for &v in &graph[u] {
            if used[v] {
                continue;
            }
            used[v] = true;
            queue.push_back(v);
        }
    }

    used
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut graph = vec![vec![]; n];
    let mut edges_set = HashSet::new();
    for _ in 0..m {
        let u: usize = scanner.cin::<usize>() - 1;
        let v: usize = scanner.cin::<usize>() - 1;
        graph[u].push(v);
        edges_set.insert((u, v));
    }

    let mut reachable: Vec<Vec<bool>> = vec![vec![]; n];
    for u in 0..n {
        let reachable_u = traverse(n, u, &graph);
        reachable[u] = reachable_u;
    }

    let mut ans = 0;
    for u in 0..n {
        for v in 0..n {
            if u == v {
                continue;
            }
            if !reachable[u][v] {
                continue;
            }
            if edges_set.contains(&(u, v)) {
                continue;
            }
            debug!("insert ({}, {})", u, v);
            ans += 1;
        }
    }

    println!("{}", ans);
}

use std::{io::Write, collections::{HashSet, VecDeque}};
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

#[macro_export]
macro_rules! ndarray {
    // ndarray!(val; *shape)
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}
