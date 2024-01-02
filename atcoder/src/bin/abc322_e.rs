const INF: usize = 1_000_000_000_000_000;

fn add_state(state: &Vec<usize>, diffs: &[usize], clip: usize) -> Vec<usize> {
    let mut ret = state.clone();
    let n: usize = state.len();
    for i in 0..n {
        ret[i] += diffs[i];
        ret[i] = ret[i].min(clip);
    }
    ret
}

#[allow(clippy::needless_range_loop)]
#[allow(clippy::collapsible_else_if)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let p: usize = scanner.cin();

    let mut costs: Vec<usize> = vec![0; n];
    let mut a: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n {
        costs[i] = scanner.cin();
        a[i] = scanner.vec::<usize>(k);
    }

    let mut ans = INF;

    let mut dp: Vec<HashMap<Vec<usize>, usize>> = vec![HashMap::new(); n + 1];
    dp[0].insert(vec![0; k], 0);

    for i in 0..n {
        debug!("i={}", i);

        let row = dp[i].clone();
        for (state, &cost) in row.iter() {
            let current_state = state.clone();
            let new_state = add_state(state, &a[i], p);

            // 使わない
            let new_cost = dp[i + 1]
                .get(&current_state)
                .map_or(cost, |&x| cost.min(x));

            dp[i + 1]
                .entry(current_state)
                .and_modify(|x| *x = new_cost)
                .or_insert(cost);

            // 使う
            debug!("new_state={:?}", new_state);
            let new_cost = dp[i + 1]
                .get(&new_state)
                .map_or(cost + costs[i], |&x| x.min(cost + costs[i]));

            if new_state.iter().all(|&x| x >= p) {
                debug!("fulfilled");
                ans = ans.min(cost + costs[i]);
            }
            else {
                dp[i + 1].insert(new_state, new_cost);
            }

            // IMPORTANT PROVE
            assert!(dp[i + 1].len() < 10_000);
        }

    }
    debug!("==dp==");
    for row in dp.iter() {
        debug!("{:?}", row);
    }

    if ans == INF {
        println!("-1");
    }
    else {
        print!("{}", ans);
    }
}

use std::{io::Write, collections::HashMap};
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
