const DIV: usize = 998244353;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

fn encode(x: usize, y: usize) -> usize {
    x * 10000 + y
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let mut union_find = UnionFind::new(10_000_000);

    let mut s = vec![vec!['.'; w]; h];
    for i in 0..h {
        let si: Vec<char> = scanner.cin::<String>().chars().collect();
        s[i] = si;
    }

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }
            for direction in DIRECTIONS {
                match direction {
                    Direction::Up => {
                        if i == 0 {
                            continue;
                        }
                        let ni = i - 1;
                        let nj = j;
                        if s[ni][nj] == '.' {
                            continue;
                        }
                        union_find.unite(encode(i, j), encode(ni, nj));
                    }
                    Direction::Right => {
                        if j == w - 1 {
                            continue;
                        }
                        let ni = i;
                        let nj = j + 1;
                        if s[ni][nj] == '.' {
                            continue;
                        }
                        union_find.unite(encode(i, j), encode(ni, nj));
                    }
                    Direction::Down => {
                        if i == h - 1 {
                            continue;
                        }
                        let ni = i + 1;
                        let nj = j;
                        if s[ni][nj] == '.' {
                            continue;
                        }
                        union_find.unite(encode(i, j), encode(ni, nj));
                    }
                    Direction::Left => {
                        if j == 0 {
                            continue;
                        }
                        let ni = i;
                        let nj = j - 1;
                        if s[ni][nj] == '.' {
                            continue;
                        }
                        union_find.unite(encode(i, j), encode(ni, nj));
                    }
                }
            }
        }
    }

    let mut connected_components = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }
            connected_components.insert(union_find.parent(encode(i, j)));
        }
    }
    let num_connected_components = connected_components.len();

    let mut num_increased_components = 0;
    let mut num_decreased_components = 0;
    let mut num_red_points = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            num_red_points += 1;

            let mut st = HashSet::new();
            for direction in DIRECTIONS {

                match direction {
                    Direction::Up => {
                        if i == 0 {
                            continue;
                        }
                        let ni = i - 1;
                        let nj = j;
                        if s[ni][nj] == '.' {
                            continue;
                        }
                        st.insert(union_find.parent(encode(ni, nj)));
                    }
                    Direction::Right => {
                        if j == w - 1 {
                            continue;
                        }
                        let ni = i;
                        let nj = j + 1;
                        if s[ni][nj] == '.' {
                            continue;
                        }
                        st.insert(union_find.parent(encode(ni, nj)));
                    }
                    Direction::Down => {
                        if i == h - 1 {
                            continue;
                        }
                        let ni = i + 1;
                        let nj = j;
                        if s[ni][nj] == '.' {
                            continue;
                        }
                        st.insert(union_find.parent(encode(ni, nj)));
                    }
                    Direction::Left => {
                        if j == 0 {
                            continue;
                        }
                        let ni = i;
                        let nj = j - 1;
                        if s[ni][nj] == '.' {
                            continue;
                        }
                        st.insert(union_find.parent(encode(ni, nj)));
                    }
                }
            }

            debug!("({}, {}) st={:?}", i, j, st);
            let num_components = st.len();

            if st.is_empty() {
                num_increased_components += 1;
            }
            else {
                num_decreased_components += num_components - 1;
            }
        }
    }

    let num_tot_connected_components = num_red_points * num_connected_components;
    let inv = modinv(num_red_points, DIV);
    debug!("num_connected_components: {}", num_connected_components);
    debug!("num_tot_connected_components: {}", num_tot_connected_components);
    debug!("num_decreased_components: {}", num_decreased_components);
    debug!("num_red_points: {}", num_red_points);
    debug!("({} + {} - {}) / {}", num_tot_connected_components, num_increased_components, num_decreased_components, num_red_points);

    let numerator = (DIV + num_tot_connected_components + num_increased_components - num_decreased_components) % DIV;
    println!("{}", (numerator * inv % DIV));
}

use std::{io::Write, vec, collections::HashSet};
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

pub fn modpow(mut a: usize, mut n: usize, m: usize) -> usize {
    let mut ans = 1;

    while n > 0 {
        if n & 1 == 1 {
            ans = ans * a % m;
        }

        a = a * a % m;
        n >>= 1;
    }

    ans
}

pub fn phi(mut n: usize) -> usize {
    let mut res = n;

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            res = res / i * (i - 1);
            while n % i == 0 {
                n /= i;
            }
        }
        i += 1;
    }

    if n != 1 {
        res = res / n * (n - 1);
    }

    res
}

pub fn modinv(a: usize, p: usize) -> usize {
    let m = phi(p);
    modpow(a, m - 1, p)
}
