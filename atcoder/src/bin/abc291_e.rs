pub struct TopologicalSort {
    graph: Graph,
}

#[allow(clippy::needless_range_loop)]
impl TopologicalSort {
    pub fn new(graph: Graph) -> Self {
        TopologicalSort { graph }
    }

    pub fn sort(&mut self) -> Option<Vec<usize>> {
        let mut ans: Vec<usize> = vec![];
        let mut s = std::collections::BinaryHeap::new();
        let mut degrees = self.graph.in_degrees.clone();

        for v in 0..self.graph.n {
            if degrees[v] == 0 {
                s.push(std::cmp::Reverse(v));
            }
        }

        if s.len() > 1 {
            return None;
        }

        while let Some(std::cmp::Reverse(v)) = s.pop() {
            ans.push(v);

            for &(nv, _) in self.graph.graph[v].iter() {
                if degrees[nv] == 0 {
                    continue;
                }

                degrees[nv] -= 1;

                if degrees[nv] == 0 {
                    s.push(std::cmp::Reverse(nv));
                    if s.len() > 1 {
                        return None;
                    }
                }
            }
        }

        if ans.len() == degrees.len() {
            Some(ans)
        } else {
            Some(vec![])
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut edges = HashSet::new();
    let mut graph = Graph::new(n, true);
    for _ in 0..m {
        let x: usize = scanner.cin::<usize>() - 1;
        let y: usize = scanner.cin::<usize>() - 1;
        if !edges.contains(&(x, y)) {
            graph.connect_unweighted(x, y);
            edges.insert((x, y));
        }
    }

    let mut sorter = TopologicalSort::new(graph);
    if let Some(order) = sorter.sort() {
        let mut ans = vec![0; n];
        for (v, &idx) in order.iter().enumerate() {
            ans[idx] = v + 1;
        }
        println!("Yes");
        for v in ans {
            println!("{}", v);
        }
    }
    else {
        println!("No");
    }
}

use std::{io::Write, collections::HashSet};
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
        print!("[debug] ");
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

#[derive(Clone, Debug)]
pub struct Graph {
    pub n: usize,
    pub graph: Vec<Vec<(usize, usize)>>,
    pub rev: Vec<Vec<usize>>,
    pub in_degrees: Vec<usize>,
    pub out_degrees: Vec<usize>,
    pub directed: bool,
}

impl Graph {
    pub fn new(n: usize, directed: bool) -> Self {
        let graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
        let in_degrees = vec![0; n];
        let out_degrees = vec![0; n];
        let rev = vec![vec![]; n];
        Self {
            n,
            graph,
            rev,
            in_degrees,
            out_degrees,
            directed,
        }
    }

    pub fn connect(&mut self, from: usize, to: usize, weight: usize) {
        self.graph[from].push((to, weight));
        self.out_degrees[from] += 1;
        self.in_degrees[to] += 1;

        if !self.directed {
            self.graph[to].push((from, weight));
            self.out_degrees[to] += 1;
            self.in_degrees[from] += 1;
        }
    }

    pub fn connect_unweighted(&mut self, from: usize, to: usize) {
        self.graph[from].push((to, 1));
        self.out_degrees[from] += 1;
        self.in_degrees[to] += 1;

        if !self.directed {
            self.graph[to].push((from, 1));
            self.out_degrees[to] += 1;
            self.in_degrees[from] += 1;
        }
    }

    pub fn connect_with_residual(&mut self, from: usize, to: usize, weight: usize) {
        assert!(
            self.directed,
            "connect_with_residual only works in directed graph."
        );

        self.graph[from].push((to, weight));
        self.out_degrees[from] += 1;
        self.in_degrees[to] += 1;

        self.graph[to].push((from, 0));
        self.out_degrees[to] += 1;
        self.in_degrees[from] += 1;

        self.rev[from].push(self.graph[to].len() - 1);
        self.rev[to].push(self.graph[from].len() - 1);
    }

    pub fn in_degree(&self, u: usize) -> usize {
        self.graph[u].len()
    }

    pub fn out_degree(&self, u: usize) -> usize {
        self.graph[u].len()
    }

    pub fn connected(&self, u: usize, v: usize) -> bool {
        self.graph[u].iter().filter(|&(k, _)| &v == k).count() > 0
    }
}
