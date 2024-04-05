pub fn prime_factorize(mut n: usize) -> Vec<usize> {
    let mut ret = vec![];
    let mut p = 2;

    while p * p <= n {
        if n % p == 0 {
            let mut k = 0;
            while n % p == 0 {
                k += 1;
                n /= p;
            }
            if k % 2 == 0 {
                continue;
            }
            ret.push(p);
        }

        p += 1;
    }

    if n != 1 {
        ret.push(n);
    }
    ret
}

pub fn solve(n: usize, a: &[usize]) -> usize {
    let mut b: Vec<Vec<usize>> = vec![];
    let mut bcnt: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut zcnt = 0;

    for &ai in a.iter() {
        if ai == 0 {
            zcnt += 1;
        }
        let ps = prime_factorize(ai);
        b.push(ps.clone());
        *bcnt.entry(ps).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 0..(n - 1) {
        let ps = b[i].clone();
        match a[i] {
            0 => ans += n - i - 1,  // 0 * v = 0 and 0 is a square number.
            _ => ans += bcnt[&ps] - 1,
        }
        bcnt.entry(ps).and_modify(|e| *e -= 1);

        if a[i] == 0 {
            zcnt -= 1;
        }
        else {
            ans += zcnt;
        }
    }

    ans
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    // let ans = solve_lazy(n, &a);
    let ans = solve(n, &a);
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
