pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn flip(c: char) -> char {
    match c {
        '.' => '#',
        '#' => '.',
        _   => panic!(),
    }
}

fn next_direction(c: char, d: &Direction) -> Direction {
    match c {
        '#' => {
            match d {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        },
        '.' => {
            match d {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            }
        }
        _ => panic!(),
    }
}

fn debug_board(board: &[Vec<char>]) {
    debug!();
    for row in board {
        debug!("{:?}", row);
    }
    debug!();
}

fn next_position(i: usize, j: usize, h: usize, w: usize, d: &Direction) -> (usize, usize) {
    match d {
        Direction::Up => {
            match i == 0 {
                true  => (h - 1, j),
                false => (i - 1, j),
            }
        }
        Direction::Right => {
            match j == w - 1 {
                true  => (i, 0),
                false => (i, j + 1),
            }
        }
        Direction::Down => {
            match i == h - 1 {
                true  => (0, j),
                false => (i + 1, j),
            }
        }
        Direction::Left => {
            match j == 0 {
                true  => (i, w - 1),
                false => (i, j - 1),
            }
        }
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let n: usize = scanner.cin();

    let mut board = ndarray!('.'; h, w);
    let mut i = 0;
    let mut j = 0;
    let mut d = Direction::Up;

    for _ in 0..n {
        board[i][j] = flip(board[i][j]);
        d = next_direction(board[i][j], &d);
        (i, j) = next_position(i, j, h, w, &d);
        debug_board(&board);
    }

    for row in board {
        let si: String = row.iter().collect();
        println!("{}", si);
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
