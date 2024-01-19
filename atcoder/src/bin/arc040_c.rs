fn fill(board: &mut Vec<Vec<char>>, r: usize, c: usize) {
    let n = board.len();
    for j in 0..=c {
        board[r][j] = 'o';
    }
    if r >= n - 1 {
        return;
    }
    for j in c..n {
        board[r + 1][j] = 'o';
    }
}

fn show(board: &[Vec<char>]) {
    for row in board.iter() {
        let si = row.iter().cloned().collect::<String>();
        debug!("{}", si);
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let mut board = vec![vec!['n'; n]; n];

    let mut points = BinaryHeap::new();
    for i in 0..n {
        let row: Vec<char> = scanner.cin::<String>().chars().collect();
        board[i] = row;
        for j in 0..n {
            if board[i][j] == '.' {
                points.push((Reverse(i), j));
            }
        }
    }

    let mut ans: usize = 0;
    while let Some((Reverse(r), c)) = points.pop() {
        debug!("point=({}, {})", r, c);
        if board[r][c] == 'o' {
            debug!("skipped");
            continue;
        }
        debug!("");
        debug!("=== begin ===");
        show(&board);
        debug!("===  end  ===");
        debug!("");
        fill(&mut board, r, c);
        ans += 1;
    }

    println!("{}", ans);
}

use std::{io::Write, collections::BinaryHeap, cmp::Reverse};
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
