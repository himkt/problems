#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Rh,
    Lf,
    Dw,
}

fn reach_wall(board: &[Vec<char>], cur: (usize, usize), direction: &Direction) -> bool {
    let (i, j): (usize, usize) = cur;
    match direction {
        Direction::Up => board[i - 1][j] == '#',
        Direction::Rh => board[i][j + 1] == '#',
        Direction::Dw => board[i + 1][j] == '#',
        Direction::Lf => board[i][j - 1] == '#',
    }
}

fn scan(board: &[Vec<char>], cur: (usize, usize), direction: &Direction) -> Vec<((usize, usize), Direction)> {
    let mut ret = vec![];
    let (i, j): (usize, usize) = cur;

    if board[i - 1][j] == '.' && direction != &Direction::Dw {
        ret.push(((i - 1, j), Direction::Up));
    }
    if board[i][j + 1] == '.' && direction != &Direction::Lf {
        ret.push(((i, j + 1), Direction::Rh));
    }
    if board[i + 1][j] == '.' && direction != &Direction::Up {
        ret.push(((i + 1, j), Direction::Dw));
    }
    if board[i][j - 1] == '.' && direction != &Direction::Rh {
        ret.push(((i, j - 1), Direction::Lf));
    }

    ret
}

fn inv_dir(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Dw,
        Direction::Rh => Direction::Lf,
        Direction::Dw => Direction::Up,
        Direction::Lf => Direction::Rh,
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.cin();
    let _: usize = scanner.cin();
    let board: Vec<Vec<char>> = (0..n).map(|_| {
        scanner.cin::<String>().chars().collect()
    }).collect();

    let mut stack = VecDeque::new();
    let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();
    let mut visited_undirectional: HashSet::<(usize, usize)> = HashSet::new();

    visited.insert(((1, 1), Direction::Up));
    visited.insert(((1, 1), Direction::Rh));
    visited.insert(((1, 1), Direction::Dw));
    visited.insert(((1, 1), Direction::Lf));
    visited_undirectional.insert((1, 1));

    if board[1][2] == '.' {
        stack.push_front(((1, 2), Direction::Rh));
    }
    if board[2][1] == '.' {
        stack.push_front(((2, 1), Direction::Dw));
    }

    while let Some((cur, dir)) = stack.pop_front() {
        debug!("visit: {:?}, dir={:?}", cur, dir);
        visited_undirectional.insert(cur);
        let nexts = if reach_wall(&board, cur, &dir) {
            debug!("reached wall");
            scan(&board, cur, &dir)
        }
        else {
            let (i, j): (usize, usize) = cur;
            match dir {
                Direction::Up => vec![((i - 1, j), Direction::Up)],
                Direction::Rh => vec![((i, j + 1), Direction::Rh)],
                Direction::Dw => vec![((i + 1, j), Direction::Dw)],
                Direction::Lf => vec![((i, j - 1), Direction::Lf)],
            }
        };

        visited.insert((cur, dir));
        for (next_pos, next_dir) in nexts {
            if visited.contains(&(next_pos, next_dir)) || visited.contains(&(next_pos, inv_dir(&next_dir))) {
                continue;
            }
            stack.push_front((next_pos, next_dir));
        }
    }

    debug!("{:?}", visited_undirectional);
    println!("{}", visited_undirectional.len());
}

use std::{io::Write, collections::{VecDeque, HashSet}};
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
