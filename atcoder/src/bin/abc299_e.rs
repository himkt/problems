const INF: usize = 1_000_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let u: usize = scanner.cin::<usize>() - 1;
        let v: usize = scanner.cin::<usize>() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let k: usize = scanner.cin();
    let mut constraints = vec![];
    for _ in 0..k {
        let p: usize = scanner.cin::<usize>() - 1;
        let d: usize = scanner.cin();
        constraints.push((p, d));
    }

    let mut dist = ndarray!(INF; n, n);
    for s in 0..n {
        let mut queue = VecDeque::new();
        queue.push_back((s, 0));

        while let Some((u, d)) = queue.pop_front() {
            if dist[s][u] < d {
                continue;
            }

            dist[s][u] = d;
            for &v in graph[u].iter() {
                queue.push_back((v, d + 1));
            }
        }
    }
    for row in &dist {
        debug!("{:?}", row);
    }

    // construct
    let mut colors = vec![1; n];
    for &(u, d) in constraints.iter() {
        for v in 0..n {
            if dist[u][v] < d {
                colors[v] = 0;
            }
        }
    }
    debug!("colors={:?}", colors);

    // verify (1)
    if colors.iter().sum::<usize>() == 0 {
        println!("No");
        return;
    }

    // verify (2)
    for &(u, d) in constraints.iter() {
        let min = (0..n)
            .filter(|&v| colors[v] == 1)
            .map(|v| dist[u][v])
            .min()
            .unwrap();

        if d != min {
            println!("No");
            return;
        }
    }

    println!("Yes");
    println!("{}", colors.iter().map(|&x| x.to_string()).collect::<String>());
}

use std::{io::Write, collections::VecDeque};
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
