#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();

    let mut graph = Graph::new(n, true);
    let mut num_self_loops = 0;
    for u in 0..n {
        let v: usize = scanner.cin::<usize>() - 1;
        graph.connect_unweighted(u, v);
        if u == v {
            num_self_loops += 1;
        }
    }

    let mut scc = StronglyConnectedComponent::new(graph);
    scc.scc();

    let mut cnt: HashMap<usize, usize> = HashMap::new();
    for &component_id in scc.topological_ranks.iter() {
        *cnt.entry(component_id).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (_, &c) in cnt.iter() {
        if c == 1 {
            continue;
        }
        ans += c;
    }
    ans += num_self_loops;
    println!("{}", ans);
}

use std::{collections::HashMap, io::Write};
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

use std::collections::VecDeque;

pub struct StronglyConnectedComponent {
    forward_graph: Graph,
    forward_visited: Vec<bool>,
    forward_visited_nodes: VecDeque<usize>,
    backward_graph: Graph,
    backward_visited: Vec<bool>,
    topological_ranks: Vec<usize>,
}

impl StronglyConnectedComponent {
    pub fn new(graph: Graph) -> Self {
        let n = graph.n;
        let forward_graph = graph;
        let mut backward_graph = Graph::new(n, true);

        for u in 0..n {
            for &(v, _) in forward_graph.graph[u].iter() {
                backward_graph.connect_unweighted(v, u);
            }
        }

        Self {
            forward_graph,
            forward_visited: vec![false; n],
            forward_visited_nodes: VecDeque::new(),
            backward_graph,
            backward_visited: vec![false; n],
            topological_ranks: vec![0; n],
        }
    }

    pub fn scc(&mut self) -> usize {
        for u in 0..self.forward_graph.n {
            if self.forward_visited[u] {
                continue;
            }

            self.fdfs(u);
        }

        let mut topological_rank = 0;
        while let Some(u) = self.forward_visited_nodes.pop_back() {
            if self.backward_visited[u] {
                continue;
            }

            self.rdfs(u, topological_rank);
            topological_rank += 1;
        }

        topological_rank
    }

    fn fdfs(&mut self, u: usize) {
        self.forward_visited[u] = true;

        for i in 0..self.forward_graph.graph[u].len() {
            let (v, _) = self.forward_graph.graph[u][i];

            if self.forward_visited[v] {
                continue;
            }

            self.fdfs(v);
        }

        self.forward_visited_nodes.push_back(u);
    }

    fn rdfs(&mut self, u: usize, topological_rank: usize) {
        self.backward_visited[u] = true;
        self.topological_ranks[u] = topological_rank;

        for i in 0..self.backward_graph.graph[u].len() {
            let (v, _) = self.backward_graph.graph[u][i];

            if self.backward_visited[v] {
                continue;
            }

            self.rdfs(v, topological_rank);
        }
    }
}
