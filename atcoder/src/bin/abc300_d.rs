pub fn eratosthenes_sieve(n: usize) -> Vec<bool> {
    let mut s = vec![true; n];
    s[0] = false;
    s[1] = false;

    for i in 2..n {
        if i * i > n {
            break;
        }
        if s[i] {
            for k in 2..(n + i - 1) / i {
                s[k * i] = false
            }
        }
    }

    s
}

pub fn upper_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if !prop(range.start) {
        return range.end;
    }

    let mut ok = range.start;
    let mut ng = range.end;

    while ng - ok > 1 {
        let middle = ok + (ng - ok) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
}

fn judge(a: usize, b: usize, c: usize, n: usize) -> bool {
    let ax = a as u128;
    let bx = b as u128;
    let cx = c as u128;
    let nx = n as u128;
    ax * ax * bx * cx * cx <= nx
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();

    let is_prime = eratosthenes_sieve(1_000_000);
    let prime_tables: Vec<usize> = (2usize..1_000_000)
        .filter(|&x| is_prime[x]).collect();
    let l: usize = prime_tables.len();

    let mut ans = 0;
    for i in 0..l {
        for j in (i + 1)..(l - 1) {
            let k = upper_bound((j + 1)..l, &|x| {
                judge(prime_tables[i], prime_tables[j], prime_tables[x], n)
            });
            if k == l {
                break;
            }
            debug!("i={}, j={}, k={}, a={}, b={}, c={} => {}", i, j, k, prime_tables[i], prime_tables[j], prime_tables[k], k - j);
            ans += k - j;
        }
    }

    println!("{}", ans);
}

use std::io::Write;
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
