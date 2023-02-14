#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let s: Vec<Vec<char>> = (0..n)
        .map(|_| scanner.cin::<String>().chars().collect())
        .collect();

    fn dfs(ans: &mut Vec<usize>, s: &[Vec<char>], n: usize, ids: Vec<usize>, l: usize) {
        debug!("called dfs ids={:?}, l={}", ids, l);

        for &id in ids.iter() {
            ans[id] = l;
        }

        let mut groups = vec![vec![]; 26];
        for chari in 0..26 {
            let char = (b'a' + chari) as char;
            for &i in ids.iter() {
                if s[i].len() <= l {
                    continue;
                }
                if s[i][l] == char {
                    let t = char as usize - 'a' as usize;
                    groups[t].push(i);
                }
            }
        }

        for group in groups {
            if group.len() < 2 {
                continue;
            }
            dfs(ans, s, n, group, l + 1);
        }
    }

    let mut ans = vec![0; n];
    let ids: Vec<usize> = (0..n).collect();
    dfs(&mut ans, &s, n, ids, 0);

    for ansi in ans {
        println!("{}", ansi);
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
