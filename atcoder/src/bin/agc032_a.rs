fn find_rightest(b: &[usize]) -> Option<usize> {
    let mut i = None;
    for (j, &bj) in b.iter().enumerate() {
        debug!("j={}, bj={}", j, bj);
        if (j + 1) == bj {
            i = Some(j);
            debug!("i={:?}", i);
        }
    }

    i.map(|i: usize| i + 1)
}

fn construct(b: &[usize], i: usize) -> Vec<usize> {
    let mut ret = vec![];
    for (j, &bj) in b.iter().enumerate() {
        if i == (j + 1) {
            continue;
        }
        ret.push(bj);
    }
    ret
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let mut b: Vec<usize> = scanner.vec(n);

    let mut ans = vec![];
    for _ in 0..n {
        if let Some(j) = find_rightest(&b) {
            b = construct(&b, j);
            debug!("=> j={}", j);
            ans.push(j);
        }
        else {
            println!("-1");
            return;
        }
    }

    for j in ans.iter().rev() {
        println!("{}", j);
    }
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
