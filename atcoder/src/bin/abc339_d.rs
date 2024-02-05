pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn next_position(i: usize, j: usize, n: usize, direction: &Direction, board: &[Vec<char>]) -> (usize, usize) {
    match direction {
        Direction::Up => {
            if i > 0 && board[i - 1][j] != '#' {
                (i - 1, j)
            } else {
                (i, j)
            }
        },
        Direction::Right => {
            if j < n - 1 && board[i][j + 1] != '#' {
                (i, j + 1)
            } else {
                (i, j)
            }
        },
        Direction::Down => {
            if i < n - 1 && board[i + 1][j] != '#' {
                (i + 1, j)
            } else {
                (i, j)
            }
        },
        Direction::Left => {
            if j > 0 && board[i][j - 1] != '#' {
                (i, j - 1)
            } else {
                (i, j)
            }
        },
    }

}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let mut board = ndarray!('x'; n, n);

    for i in 0..n {
        let s: Vec<char> = scanner.cin::<String>().chars().collect();
        board[i] = s;
    }

    for row in board.iter() {
        debug!("{:?}", row);
    }

    let mut players = vec![];
    for i in 0..n {
        for j in 0..n {
            if board[i][j] == 'P' {
                players.push((i, j));
            }
        }
    }
    assert_eq!(players.len(), 2);

    let p0 = players[0];
    let p1 = players[1];

    let mut queue = VecDeque::new();
    queue.push_back(((p0.0, p0.1, p1.0, p1.1), 0));

    let mut dist = HashMap::new();
    dist.insert((p0.0, p0.1, p1.0, p1.1), 0);

    while let Some(((i0, j0, i1, j1), d)) = queue.pop_front() {
        debug!("p1=({}, {}), p2=({}, {}), d={}", i1, j1, i1, j1, d);
        if (i0, j0) == (i1, j1) {
            println!("{}", d);
            return;
        }

        for direction in [Direction::Up, Direction::Left, Direction::Down, Direction::Right].iter() {
            let (ni0, nj0) = next_position(i0, j0, n, direction, &board);
            let (ni1, nj1) = next_position(i1, j1, n, direction, &board);
            if dist.contains_key(&(ni0, nj0, ni1, nj1)) {
                continue;
            }
            dist.insert((ni0, nj0, ni1, nj1), d + 1);
            queue.push_back(((ni0, nj0, ni1, nj1), d + 1));
        }
    }

    println!("-1");
}

use std::{collections::{HashMap, VecDeque}, io::Write};
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
