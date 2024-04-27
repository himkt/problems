#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut ret = vec![];
    for ai in a {
        ret.push(ai);
        debug!("{:?}", ret);

        loop {
            let k = ret.len();
            if k <= 1 {
                break;
            }
            if ret[k - 2] != ret[k - 1] {
                break;
            }
            if ret[k - 2] == ret[k - 1] {
                let ak = ret[k - 1];
                ret.pop();
                ret.pop();
                ret.push(ak + 1);
            }
            debug!("[internal] {:?}", ret);
        }
        debug!("=> {:?}", ret);
    }

    println!("{}", ret.len());
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
