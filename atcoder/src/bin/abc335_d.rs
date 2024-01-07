#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();

    let mut board = ndarray!("T".to_string(); n, n);
    let mut cur = 1;

    let mut x = 0;
    let mut y = 0;
    let mut direction = 'R';

    loop {
        board[x][y] = cur.to_string();
        debug!("x={}, y={}, direction={}", x, y, direction);

        match direction {
            'R' => {
                if y + 1 < n && board[x][y + 1] == "T" {
                    y += 1;
                }
                else {
                    direction = 'D';
                    x += 1;
                }
            }
            'D' => {
                if x + 1 < n && board[x + 1][y] == "T" {
                    x += 1;
                }
                else {
                    direction = 'L';
                    y -= 1;
                }
            }
            'L' => {
                if y > 0 && board[x][y - 1] == "T" {
                    y -= 1;
                }
                else {
                    direction = 'U';
                    x -= 1;
                }
            }
            'U' => {
                if x > 0 && board[x - 1][y] == "T" {
                    x -= 1;
                }
                else {
                    direction = 'R';
                    y += 1;
                }
            }
            _   => panic!()
        }

        cur += 1;
        if cur == n * n {
            break;
        }
    }

    for row in board.iter() {
        println!("{}", row.join(" "));
    }
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

#[macro_export]
macro_rules! ndarray {
    // ndarray!(val; *shape)
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}
