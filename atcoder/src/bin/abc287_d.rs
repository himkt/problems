fn eq(si: char, ti: char) -> bool {
    si == '?' || ti == '?' || si == ti
}

fn ans(match_cnt: usize, n: usize) {
    match match_cnt.eq(&n) {
        true  => println!("Yes"),
        false => println!("No")
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let t: Vec<char> = scanner.cin::<String>().chars().collect();

    let lt = t.len();
    let mut st: Vec<char> = s.iter().rev().take(lt).rev().cloned().collect();

    debug!("st={:?}", st);
    debug!("t={:?}", t);

    let mut match_cnt = 0;
    let mut matched = vec![false; lt];
    for i in 0..lt {
        if eq(st[i], t[i]) {
            match_cnt += 1;
            matched[i] = true;
        }
    }
    ans(match_cnt, lt);

    for i in 0..lt {
        st[i] = s[i];
        debug!("st={:?}, t={:?}, i={}, eq={}", st, t, i, eq(st[i], t[i]));

        if eq(st[i], t[i]) {
            if !matched[i] {
                match_cnt += 1;
            }
            matched[i] = true;
        }
        else {
            if matched[i] {
                match_cnt -= 1;
            }
            matched[i] = false;
        }

        ans(match_cnt, lt);
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
        println!($($arg)*);
    };
}
