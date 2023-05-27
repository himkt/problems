#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();

    let mut cumr = vec![0; n + 1];
    let mut cumg = vec![0; n + 1];
    let mut cumb = vec![0; n + 1];
    for i in 0..n {
        match s[i] {
            'R' => cumr[i + 1] += 1,
            'G' => cumg[i + 1] += 1,
            'B' => cumb[i + 1] += 1,
            _   => panic!(),
        };
    }
    for i in 1..=n {
        cumr[i] += cumr[i - 1];
        cumg[i] += cumg[i - 1];
        cumb[i] += cumb[i - 1];
    }

    debug!("{:?}", cumr);
    debug!("{:?}", cumg);
    debug!("{:?}", cumb);

    let mut ans: usize = 0;
    for i in 0..n {
        for j in (i + 1)..(n - 1) {
            if s[i] == s[j] {
                continue;
            }
            debug!("Si={}, Sj={}, i={}, j={}", s[i], s[j], i, j);
            let k = 2 * j - i;

            if s[i] != 'R' && s[j] != 'R' {
                let rest = cumr[n] - cumr[j];
                ans += rest;
                if k < n && s[k] == 'R' {
                    ans -= 1;
                }
            }
            if s[i] != 'G' && s[j] != 'G' {
                let rest = cumg[n] - cumg[j];
                ans += rest;
                if k < n && s[k] == 'G' {
                    ans -= 1;
                }
            }
            if s[i] != 'B' && s[j] != 'B' {
                let rest = cumb[n] - cumb[j];
                ans += rest;
                if k < n && s[k] == 'B' {
                    ans -= 1;
                }
            }
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
