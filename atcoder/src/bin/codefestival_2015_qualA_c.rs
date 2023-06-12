#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let t: usize = scanner.cin();

    let mut a = vec![];
    let mut b = vec![];

    let mut atot = 0;
    let mut btot = 0;
    for _ in 0..n {
        let ai: usize = scanner.cin();
        let bi: usize = scanner.cin();
        a.push(ai);
        b.push(bi);
        atot += ai;
        btot += bi;
    }

    if t < btot {
        println!("-1");
        return;
    }

    if atot <= t {
        println!("0");
        return;
    }

    let mut arange = (0..n).collect::<Vec<usize>>();
    arange.sort_by_key(|&x| a[x] - b[x]);
    arange.reverse();
    debug!("{:?}", arange);

    let mut ans: usize = 0;
    for i in arange {
        atot -= a[i] - b[i];
        ans += 1;
        debug!("copy {} assignment", i);
        if atot <= t {
            break;
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

#[macro_export]
macro_rules! ndarray {
    // ndarray!(val; *shape)
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}
