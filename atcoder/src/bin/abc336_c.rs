const NUMS: [usize; 5] = [0, 2, 4, 6, 8];

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let mut n: usize = scanner.cin::<usize>() - 1;

    if n == 0 {
        println!("0");
        return;
    }

    let mut ans = vec![];
    while n > 0 {
        let p = n % 5;
        ans.push(p);
        debug!("p={}", p);
        n /= 5;
    }

    ans.reverse();
    debug!("{:?}", ans);

    let s: String = ans.iter().map(|&x| NUMS[x].to_string()).collect();
    println!("{}", s);
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
