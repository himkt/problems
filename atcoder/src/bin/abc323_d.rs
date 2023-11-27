#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();

    let mut slime_sizes: BTreeSet<usize> = BTreeSet::new();
    let mut cnt: HashMap<usize, usize> = HashMap::new();

    for _ in 0..n {
        let si: usize = scanner.cin();
        let ci: usize = scanner.cin();
        slime_sizes.insert(si);
        cnt.insert(si, ci);
    }

    while let Some(si) = slime_sizes.pop_first() {
        debug!("{:?}, cnt={:?}", slime_sizes, cnt);

        let ci = cnt[&si];
        let nsi = 2 * si;
        let nci = ci / 2;

        if nci > 0 {
            *cnt.entry(nsi).or_insert(0) += nci;
            cnt.entry(si).and_modify(|v| *v -= 2 * nci);
            slime_sizes.insert(nsi);
        }

        // if si slime is remaining
        if ci >= 3 {
            slime_sizes.insert(si);
        }
    }

    let ans: usize = cnt.values().sum();
    println!("{}", ans);
}

use std::{io::Write, collections::{BTreeSet, HashMap}};
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
