#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let mut uf = UnionFind::new(n);
    for _ in 0..m {
        let a: usize = scanner.cin();
        let b: usize = scanner.cin();
        uf.unite(a - 1, b - 1);
    }

    let mut bad_combinations: HashMap<usize, HashSet<usize>> = HashMap::new();

    let k: usize = scanner.cin();
    for _ in 0..k {
        let x: usize = scanner.cin();
        let y: usize = scanner.cin();
        let mut px = uf.parent(x - 1);
        let mut py = uf.parent(y - 1);

        if px > py {
            std::mem::swap(&mut px, &mut py);
        }

        if let Some(pbx) = bad_combinations.get_mut(&px) {
            pbx.insert(py);
        } else {
            let mut set = HashSet::new();
            set.insert(py);
            bad_combinations.insert(px, set);
        }
    }

    let q: usize = scanner.cin();
    for _ in 0..q {
        let a: usize = scanner.cin();
        let b: usize = scanner.cin();
        let mut pa = uf.parent(a - 1);
        let mut pb = uf.parent(b - 1);

        if pa > pb {
            std::mem::swap(&mut pa, &mut pb);
        }

        if !bad_combinations.contains_key(&pa) {
            println!("Yes");
            continue;
        }

        if bad_combinations[&pa].contains(&pb) {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}

use std::{io::Write, collections::{HashMap, HashSet}};
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

#[derive(Debug, Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1usize; n],
        }
    }

    pub fn parent(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            self.parents[x] = self.parent(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.parent(x);
        let mut py = self.parent(y);

        if px == py {
            return;
        }

        if self.sizes[px] < self.sizes[py] {
            std::mem::swap(&mut px, &mut py);
        }

        self.sizes[px] += self.sizes[py];
        self.parents[py] = px;
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x = self.parent(x);
        self.sizes[x]
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        let px = self.parent(x);
        let py = self.parent(y);
        px == py
    }
}
