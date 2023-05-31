fn product(a: &Vec<usize>) -> usize {
    let mut ret = 1;
    for ai in a {
        ret *= ai;
    }
    ret
}

fn judge(a: &Vec<usize>, y: usize) -> bool {
    for &ai in a {
        if gcd(ai, y) == 1 {
            return false;
        }
    }
    true
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let x: Vec<usize> = scanner.vec(n);

    let prime_numbers: Vec<usize> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let bitset = bitset(prime_numbers);

    let mut ans = !0;
    for candidate in bitset {
        if candidate.is_empty() {
            continue;
        }
        let y = product(&candidate);
        if judge(&x, y) {
            ans = ans.min(y);
            debug!("{:?} -> {}", &candidate, y);
            debug!("!=> {}", ans);
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
        println!($($arg)*);
    };
}

pub struct Bitset<T: Copy> {
    curr: usize,
    array: Vec<T>,
    len: usize,
}

impl<T: Copy> Iterator for Bitset<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.curr == (1 << self.len) {
            return None;
        }

        let mut ret = Vec::<T>::new();
        for (i, &ai) in self.array.iter().enumerate() {
            if (self.curr >> i & 1) == 1 {
                ret.push(ai);
            }
        }

        self.curr += 1;
        Some(ret)
    }
}

pub fn bitset<T: Copy>(a: Vec<T>) -> Bitset<T> {
    let len = a.len();
    Bitset {
        curr: 0,
        array: a,
        len,
    }
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
