#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let _: usize = scanner.cin();
    let _: usize = scanner.cin();

    let n: usize = scanner.cin();
    let mut berries: Vec<(usize, usize)> = vec![];
    for _ in 0..n {
        let p: usize = scanner.cin();
        let q: usize = scanner.cin();
        berries.push((p, q));
    }

    let a: usize = scanner.cin();
    let a_barriers: Vec<usize> = scanner.vec(a);

    let b: usize = scanner.cin();
    let b_barriers: Vec<usize> = scanner.vec(b);

    debug!("a_barriers: {:?}", a_barriers);
    debug!("b_barriers: {:?}", b_barriers);
    let mut mp: HashMap<(usize, usize), usize> = HashMap::new();
    for (p, q) in berries {
        debug!("p={}, q={}", p, q);
        let a_index = upper_bound(
            0..a,
            &|i| a_barriers[i] < p,
        );
        let b_index = upper_bound(
            0..b,
            &|i| b_barriers[i] < q,
        );
        debug!("a_index: {}, b_index: {}", a_index, b_index);

        let query = (a_index, b_index);
        let cnt = mp.get(&query).map_or(0, |&v| v);
        mp.insert(query, cnt + 1);
    }

    debug!("{:?}", mp);
    let mut num_berries: Vec<usize> = mp.values().cloned().collect();
    num_berries.sort();

    let num_blocks = (a + 1) * (b + 1);
    let min = match mp.len() < num_blocks {
        true  => 0,
        false => num_berries[0],
    };
    let max = num_berries[num_berries.len() - 1];
    println!("{} {}", min, max);
}

use std::{io::Write, collections::HashMap};
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
