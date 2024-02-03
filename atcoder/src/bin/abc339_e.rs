#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let d: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut rmaxq = SegmentTree::new(Mode::RangeGet(Op::Max));
    rmaxq.update_one(a[0], 1);

    let mut ans = 1;
    let mut dp = vec![1; n];

    for i in 1..n {
        let ai = a[i];
        let lower = if ai >= d { ai - d } else { 0 };
        let upper = ai + d + 1;
        let max = rmaxq.get_range(lower, upper);
        debug!("query=[{}, {}] => {}", lower, upper, max);
        if max == SegmentTree::MIN {
            rmaxq.update_one(a[i], 1);
            continue;
        }
        dp[i] = max + 1;
        rmaxq.update_one(a[i], dp[i]);
        debug!("{:?}", dp);
        ans = std::cmp::max(ans, dp[i]);
    }

    debug!("{:?}", dp);
    println!("{}", ans);
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


#[derive(Debug, Clone)]
pub struct SegmentTree {
    data: Vec<i64>,
    mode: Mode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    RangeUpdate(Op),
    RangeGet(Op),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Max,
    Min,
    Add,
}

// Segment tree implementation. All operations are 0-origin.
// Note that a half-open interval [l, r) is used as a range representation.
impl SegmentTree {
    const SEQ_LEN: usize = 1 << 20;
    const MAX: i64 = 1_000_000_000_000;
    const MIN: i64 = -1_000_000_000_000;

    pub fn new(mode: Mode) -> Self {
        let default = match &mode {
            Mode::RangeGet(op) => SegmentTree::default(op),
            Mode::RangeUpdate(op) => SegmentTree::default(op),
        };

        Self {
            data: vec![default; 2 * SegmentTree::SEQ_LEN],
            mode,
        }
    }

    /// Return an appropriate default value for the given operation.
    pub fn default(op: &Op) -> i64 {
        match op {
            Op::Add => 0,
            Op::Max => SegmentTree::MIN,
            Op::Min => SegmentTree::MAX,
        }
    }

    /// Get an i-th element of from the tree.
    pub fn get_one(&mut self, mut index: usize) -> i64 {
        index += SegmentTree::SEQ_LEN;
        let mut ret = 0;

        if let Mode::RangeUpdate(op) = &self.mode {
            let operator = match op {
                Op::Add => |ret: &mut i64, v: i64| *ret += v,
                _ => panic!(),
            };

            operator(&mut ret, self.data[index]);
            while index > 0 {
                index /= 2;
                operator(&mut ret, self.data[index]);
            }
        } else {
            panic!("Unsupported");
        }

        ret
    }

    fn range_query_recursive(
        &self,
        op: &Op,
        ql: usize,
        qr: usize,
        sl: usize,
        sr: usize,
        pos: usize,
    ) -> i64 {
        if qr <= sl || sr <= ql {
            return SegmentTree::default(op);
        }

        if ql <= sl && sr <= qr {
            return self.data[pos];
        }

        fn add(l: i64, r: i64) -> i64 {
            l + r
        }
        fn max(l: i64, r: i64) -> i64 {
            l.max(r)
        }
        fn min(l: i64, r: i64) -> i64 {
            l.min(r)
        }

        let sm = (sl + sr) / 2;
        let lv = self.range_query_recursive(op, ql, qr, sl, sm, pos * 2);
        let rv = self.range_query_recursive(op, ql, qr, sm, sr, pos * 2 + 1);
        let operate = match op {
            Op::Add => add,
            Op::Max => max,
            Op::Min => min,
        };
        operate(lv, rv)
    }

    /// Run a range query.
    pub fn get_range(&self, l: usize, r: usize) -> i64 {
        if let Mode::RangeGet(op) = &self.mode {
            self.range_query_recursive(op, l, r, 0, SegmentTree::SEQ_LEN, 1)
        } else {
            panic!("Unsupported");
        }
    }

    /// Update an i-th element to `value`.
    pub fn update_one(&mut self, mut index: usize, value: i64) {
        index += SegmentTree::SEQ_LEN;

        fn add_assign_one(ret: &mut i64, v: i64) {
            *ret += v;
        }
        fn max_assign_one(ret: &mut i64, v: i64) {
            *ret = v;
        }
        fn min_assign_one(ret: &mut i64, v: i64) {
            *ret = v;
        }
        fn add_assign(ret: &mut i64, l: i64, r: i64) {
            *ret = l + r;
        }
        fn max_assign(ret: &mut i64, l: i64, r: i64) {
            *ret = l.max(r);
        }
        fn min_assign(ret: &mut i64, l: i64, r: i64) {
            *ret = l.min(r);
        }

        if let Mode::RangeGet(op) = &self.mode {
            let operate_and_assign_one = match op {
                Op::Add => add_assign_one,
                Op::Max => max_assign_one,
                Op::Min => min_assign_one,
            };
            operate_and_assign_one(&mut self.data[index], value);

            let operate_and_assign = match op {
                Op::Add => add_assign,
                Op::Max => max_assign,
                Op::Min => min_assign,
            };

            while index > 0 {
                index /= 2;
                let lv = self.data[index * 2];
                let rv = self.data[index * 2 + 1];
                operate_and_assign(&mut self.data[index], lv, rv);
            }
        }
    }

    /// Add `value` to the range `[l, r)`.
    pub fn update_range(&mut self, mut l: usize, mut r: usize, value: i64) {
        if let Mode::RangeUpdate(op) = &self.mode {
            let operate_and_assign_one = match op {
                Op::Add => |ret: &mut i64, v: i64| *ret += v,
                _ => panic!(),
            };

            l += SegmentTree::SEQ_LEN;
            r += SegmentTree::SEQ_LEN;

            while l < r {
                if l % 2 == 1 {
                    operate_and_assign_one(&mut self.data[l], value);
                    l += 1;
                }
                l /= 2;

                if r % 2 == 1 {
                    operate_and_assign_one(&mut self.data[r - 1], value);
                    r -= 1;
                }
                r /= 2;
            }
        } else {
            panic!("Unsupported");
        }
    }
}
