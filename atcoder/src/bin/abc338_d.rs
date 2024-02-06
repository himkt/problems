#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();

    let m: usize = scanner.cin();
    let mut x: Vec<usize> = vec![0; m];
    for i in 0..m {
        let xi: usize = scanner.cin::<usize>() - 1;
        x[i] = xi;
    }

    fn fd(u: usize, v: usize) -> i128 {
        (v - u) as i128
    }
    fn rd(u: usize, v: usize, n: usize) -> i128 {
        n as i128 - fd(u, v)
    }

    debug!("n={}", n);
    let mut cumsum = vec![0; n + 1];

    for i in 1..m {
        let mut u = x[i - 1];
        let mut v = x[i];
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }

        let fdi = fd(u, v);
        let rdi = rd(u, v, n);
        debug!("fdi={}, rdi={}", fdi, rdi);

        // forward
        cumsum[u] += rdi;
        cumsum[v] -= rdi;

        // backward
        cumsum[0] += fdi;
        cumsum[u] -= fdi;
        cumsum[v] += fdi;
        cumsum[n] -= fdi;
    }

    for i in 1..=n {
        cumsum[i] += cumsum[i - 1];
    }
    debug!("cumsum={:?}", cumsum);

    let mut minimum = cumsum[0];
    let mut t = 0;
    for i in 1..n {
        if cumsum[i] < minimum {
            minimum = cumsum[i];
            t = i;
        }
    }
    debug!("minimum={}, t={}", minimum, t);

    let mut ans = 0;
    for i in 1..m {
        let mut u = x[i - 1];
        let mut v = x[i];
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        if u <= t && t < v {
            ans += rd(u, v, n);
        }
        else {
            ans += fd(u, v);
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
