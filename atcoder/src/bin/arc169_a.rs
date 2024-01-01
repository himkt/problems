#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let a: Vec<i128> = scanner.vec(n);
    let p: Vec<usize> = scanner.vec::<usize>(n - 1)
        .iter()
        .map(|&x| x - 1)
        .collect();

    // distance from A_0.
    let mut deps: Vec<usize> = vec![0; n];
    for i in 1..n {
        let src = i;
        let dst = p[src - 1];
        deps[src] = deps[dst] + 1;
    }

    debug!("{:?}", deps);
    let mut diffs_by_deps = vec![0; n];
    for u in 0..n {
        diffs_by_deps[deps[u]] += a[u];
    }

    debug!("{:?}", diffs_by_deps);
    let nonzero = diffs_by_deps
        .iter()
        .rev()
        .find(|&&x| x != 0);

    let ans;
    if let Some(val) = nonzero {
        ans = match val.cmp(&0) {
            std::cmp::Ordering::Less => "-",
            std::cmp::Ordering::Greater => "+",
            std::cmp::Ordering::Equal => panic!("val must not be zero."),
        };
    }
    else {
        ans = "0";
    }
    println!("{}", ans);
}

use std::io::Write;
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
