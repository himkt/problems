const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];

#[allow(clippy::nonminimal_bool)]
fn len_descent(s: &Vec<char>) -> usize {
    let mut ret = 1;
    let n: usize = s.len();
    for i in (0..(n - 1)).rev() {
        if !(s[i] > s[i + 1]) {
            break
        }
        ret += 1;
    }
    ret
}

#[allow(clippy::needless_range_loop)]
fn find_smallest(c: char, s: &[char], i: usize) -> char {
    let mut ret = *ALPHABET.last().unwrap();
    let n: usize = s.len();

    for k in i..n {
        debug!("s[{}]={}", k, s[k]);
        if c < s[k] && s[k] < ret {
            ret = s[k];
        }
    }

    ret
}

#[allow(clippy::needless_range_loop)]
fn succ(mut s: Vec<char>) -> Option<Vec<char>> {
    if s.len() < 26 {
        for &c in ALPHABET.iter() {
            if !s.contains(&c) {
                s.push(c);
                return Some(s);
            }
        }
    }

    let n: usize = s.len();
    let l = len_descent(&s);
    let i = n - l;

    if i == 0 {
        return None;
    }

    let q: char = find_smallest(s[i - 1], &s, i);
    debug!("i={}, smallest={}", i, q);

    if i == 1 {
        return Some(vec![q]);
    }

    let mut ret = vec![];
    for i in 0..(i-1) {
        ret.push(s[i]);
    }
    ret.push(q);

    Some(ret)
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();

    debug!("input: {:?}", s);
    if let Some(ans) = succ(s) {
        let ans: String = ans.iter().collect();
        println!("{}", ans);
    }
    else {
        println!("-1");
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
