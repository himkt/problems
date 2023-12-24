fn show(a: &[usize]) {
    for ai in a {
        println!("{}", ai);
    }
}

#[allow(clippy::needless_range_loop)]
#[allow(clippy::comparison_chain)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin::<usize>() - 1;
    let a: Vec<usize> = scanner.vec(n);

    let mut lower = 0;
    let mut upper = (n * (n + 1) / 2) - 1;

    for i in 0..n {
        debug!("i={}", i);
        let mut small = vec![];
        let mut large = vec![];

        for j in (i + 1)..n {
            match a[i].cmp(&a[j]) {
                std::cmp::Ordering::Less => large.push(a[j]),
                std::cmp::Ordering::Greater => small.push(a[j]),
                _ => panic!(),
            }
        }

        let new_lower_r = lower + small.len();
        let new_upper_l = upper - large.len();

        let mut ar = None;
        if k < new_lower_r {
            small.sort();
            ar = Some(small[k - lower]);
            debug!("find in lower, small={:?}, ar={:?}", small, ar);
        }
        else if new_upper_l < k {
            large.sort();
            ar = Some(large[k - new_upper_l - 1]);
            debug!("find in upper, large={:?}, ar={:?}", large, ar);
        }

        if let Some(ar) = ar {
            let j = a.iter().position(|&v| v == ar).unwrap();
            let mut b = a.clone();
            debug!("i={}, j={}", i, j);
            b[i..=j].reverse();
            show(&b);
            return;
        }

        lower = new_lower_r;
        upper = new_upper_l;
    }

    show(&a);
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
