#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let l: usize = scanner.cin();

    let aprices = scanner.vec::<usize>(n);
    let bprices = scanner.vec::<usize>(m);
    let mut bindices: Vec<_> = (0..m).collect();
    bindices.sort_by_key(|&i| Reverse(bprices[i]));

    let mut bad_meals_by_a = HashMap::new();
    for _ in 0..l {
        let aindex: usize = scanner.cin::<usize>() - 1;
        let bindex: usize = scanner.cin::<usize>() - 1;
        bad_meals_by_a.entry(aindex).or_insert(vec![]).push(bindex);
    }

    let mut ans: usize = 0;
    for aindex in 0..n {
        for &bindex in bindices.iter() {
            if bad_meals_by_a.contains_key(&aindex) && bad_meals_by_a[&aindex].contains(&bindex) {
                continue;
            }
            let candidate = aprices[aindex] + bprices[bindex];
            ans = std::cmp::max(ans, candidate);
            break;
        }
    }

    println!("{}", ans);
}

use std::{cmp::Reverse, collections::HashMap, io::Write};
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
