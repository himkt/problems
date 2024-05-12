const MOD: usize = 100_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);
    let mut tot = a.clone();
    for i in 1..n {
        tot[i] += tot[i - 1];
    }

    let mut ans = 0;
    for i in 0..(n - 1) {
        let k = n - i - 1;
        let v1 = k * a[i];
        let v2 = tot[n - 1] - tot[i];
        ans += v1 + v2;
    }

    let mut tree = SegmentTree::new(Mode::RangeGet(Op::Add));
    tree.update_one(a[n - 1], 1);

    let mut discount = 0;
    for i in (0..n - 1).rev() {
        let ret = tree.get_range(MOD - a[i], MOD + 1);
        discount += ret as usize * MOD;
        tree.update_one(a[i], 1);
    }

    ans -= discount;
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

pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> Option<usize> {
    if prop(range.start) {
        return Some(range.start);
    }

    let mut ng = range.start;
    let mut ok = range.end;

    while ok - ng > 1 {
        let middle = ng + (ok - ng) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    match ok.eq(&range.end) {
        true  => None,
        false => Some(ok),
    }
}

#[derive(Debug, Clone)]
pub struct SegmentTree {
    data: Vec<i32>,
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
    const SEQ_LEN: usize = 1 << 27;
    const MAX: i32 = 1_000_000_000;
    const MIN: i32 = -1_000_000_000;

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
    pub fn default(op: &Op) -> i32 {
        match op {
            Op::Add => 0,
            Op::Max => SegmentTree::MIN,
            Op::Min => SegmentTree::MAX,
        }
    }

    /// Get an i-th element of from the tree.
    pub fn get_one(&mut self, mut index: usize) -> i32 {
        index += SegmentTree::SEQ_LEN;
        let mut ret = 0;

        if let Mode::RangeUpdate(op) = &self.mode {
            let operator = match op {
                Op::Add => |ret: &mut i32, v: i32| *ret += v,
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
    ) -> i32 {
        if qr <= sl || sr <= ql {
            return SegmentTree::default(op);
        }

        if ql <= sl && sr <= qr {
            return self.data[pos];
        }

        fn add(l: i32, r: i32) -> i32 {
            l + r
        }
        fn max(l: i32, r: i32) -> i32 {
            l.max(r)
        }
        fn min(l: i32, r: i32) -> i32 {
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
    pub fn get_range(&self, l: usize, r: usize) -> i32 {
        if let Mode::RangeGet(op) = &self.mode {
            self.range_query_recursive(op, l, r, 0, SegmentTree::SEQ_LEN, 1)
        } else {
            panic!("Unsupported");
        }
    }

    /// Update an i-th element to `value`.
    pub fn update_one(&mut self, mut index: usize, value: i32) {
        index += SegmentTree::SEQ_LEN;

        fn add_assign_one(ret: &mut i32, v: i32) {
            *ret += v;
        }
        fn max_assign_one(ret: &mut i32, v: i32) {
            *ret = v;
        }
        fn min_assign_one(ret: &mut i32, v: i32) {
            *ret = v;
        }
        fn add_assign(ret: &mut i32, l: i32, r: i32) {
            *ret = l + r;
        }
        fn max_assign(ret: &mut i32, l: i32, r: i32) {
            *ret = l.max(r);
        }
        fn min_assign(ret: &mut i32, l: i32, r: i32) {
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
    pub fn update_range(&mut self, mut l: usize, mut r: usize, value: i32) {
        if let Mode::RangeUpdate(op) = &self.mode {
            let operate_and_assign_one = match op {
                Op::Add => |ret: &mut i32, v: i32| *ret += v,
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
