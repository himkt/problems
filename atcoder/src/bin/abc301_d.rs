fn to_digit(s: &[char]) -> usize {
    let mut ret = 0;
    let mut pow = 1;
    for &c in s.iter().rev() {
        if c == '1' {
            ret += pow;
        }
        pow *= 2;
    }
    ret
}

fn to_binary(mut n: usize) -> Vec<char> {
    let mut ret = vec![];
    while n > 0 {
        if n % 2 == 1 {
            ret.push('1');
        }
        else {
            ret.push('0');
        }
        n /= 2;
    }
    ret.into_iter().rev().collect()
}

fn fillk(s: &[char], k: Option<usize>) -> Vec<char> {
    let mut ret: Vec<char> = s.to_vec();
    for i in 0..s.len() {
        if s[i] == '?' {
            ret[i] = '0';
        }
        else {
            ret[i] = s[i];
        }
    }
    if let Some(k) = k {
        ret[k] = '1';
    }
    ret
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let n: usize = scanner.cin();

    let s0: Vec<char> = fillk(&s, None);
    let n0: usize = to_digit(&s0);
    debug!("s={:?}, s0={:?}, n0={}", s, s0, n0);

    if n < n0 {
        println!("-1");
        return;
    }

    let p: Vec<char> = to_binary(n);
    let mut q = vec![];
    let mut s_rev = s.iter().rev();
    for _ in 0..p.len() {
        if let Some(&sri) = s_rev.next() {
            q.push(sri);
        }
        else {
            q.push('0');
        }
    }
    q.reverse();
    debug!("q={:?}", q);

    let mut ans: usize = 0;
    for i in 0..p.len() {
        if q[i] == '?' {
            let qi = fillk(&q, Some(i));
            let mi = to_digit(&qi);
            if mi <= n {
                q[i] = '1';
            }
            else {
                q[i] = '0';
            }
        }
        let qi = fillk(&q, None);
        let mi = to_digit(&qi);
        ans = mi;
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
        print!("[debug] ");
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}
