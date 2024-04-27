#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();

    let mut board: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for i in 0..h {
        let s: Vec<char> = scanner.cin::<String>().chars().collect();
        for j in 0..w {
            if s[j] == '.' {
                board[i][j] = 2;
            }
            else {
                board[i][j] = 0;
            }
        }
    }

    // debug!("[before]");
    // for i in 0..h {
    //     for j in 0..w {
    //         print!("{} ", board[i][j])
    //     }
    //     println!();
    // }

    for i in 0..h {
        for j in 0..w {
            if board[i][j] == 0 {
                continue;
            }

            // i - 1, j
            if i > 0 && board[i - 1][j] == 0 {
                board[i][j] = 1;
                continue;
            }
            // i, j + 1
            if j + 1 < w && board[i][j + 1] == 0 {
                board[i][j] = 1;
                continue;
            }
            // i + 1, j
            if i + 1 < h && board[i + 1][j] == 0 {
                board[i][j] = 1;
                continue;
            }
            // i, j - 1
            if j > 0 && board[i][j - 1] == 0 {
                board[i][j] = 1;
                continue;
            }
        }
    }

    // debug!("[after]");
    // for i in 0..h {
    //     for j in 0..w {
    //         print!("{} ", board[i][j])
    //     }
    //     println!();
    // }

    let mut ans: usize = 0;
    let mut visited = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if board[i][j] == 0 {
                continue;
            }
            if board[i][j] == 1 {
                ans = std::cmp::max(ans, 1);
                continue;
            }
            if visited[i][j] {
                continue;
            }

            let mut queue = VecDeque::new();
            let mut score = 1;
            let mut used_inside_s = HashSet::new();
            queue.push_back((i, j));
            visited[i][j] = true;

            debug!("start from ({}, {})", i, j);
            while let Some((i, j)) = queue.pop_front() {
                debug!("visit ({}, {})", i, j);

                // i - 1, j
                if i > 0 {
                    if board[i - 1][j] == 1 && !used_inside_s.contains(&(i - 1, j)) {
                        used_inside_s.insert((i - 1, j));
                        score += 1;
                    }
                    if board[i - 1][j] == 2 && !visited[i - 1][j] {
                        queue.push_back((i - 1, j));
                        visited[i - 1][j] = true;
                        score += 1;
                    }
                }

                // i, j + 1
                if j + 1 < w {
                    if board[i][j + 1] == 1 && !used_inside_s.contains(&(i, j + 1)) {
                        used_inside_s.insert((i, j + 1));
                        score += 1;
                    }
                    if board[i][j + 1] == 2 && !visited[i][j + 1] {
                        queue.push_back((i, j + 1));
                        visited[i][j + 1] = true;
                        score += 1;
                    }
                }

                // i + 1, j
                if i + 1 < h {
                    if board[i + 1][j] == 1 && !used_inside_s.contains(&(i + 1, j)) {
                        used_inside_s.insert((i + 1, j));
                        score += 1;
                    }
                    if board[i + 1][j] == 2 && !visited[i + 1][j] {
                        queue.push_back((i + 1, j));
                        visited[i + 1][j] = true;
                        score += 1;
                    }
                }

                // i, j - 1
                if j > 0 {
                    if board[i][j - 1] == 1 && !used_inside_s.contains(&(i, j - 1)) {
                        used_inside_s.insert((i, j - 1));
                        score += 1;
                    }
                    if board[i][j - 1] == 2 && !visited[i][j - 1] {
                        queue.push_back((i, j - 1));
                        visited[i][j - 1] = true;
                        score += 1;
                    }
                }
            }

            debug!(" => score={}", score);
            ans = std::cmp::max(ans, score);
        }
    }
    println!("{}", ans);
}

use std::{collections::{HashSet, VecDeque}, io::Write};
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
