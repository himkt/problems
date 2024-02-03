fn calculate_volume(corns: &[(i64, i64, i64)], a: i64, b: i64) -> f64 {

    fn _volume(r: f64, h: f64) -> f64 {
        let vb = r * r * std::f64::consts::PI;
        vb * h * (1.0 / 3.0)
    }

    let mut ans = 0.0;

    for &(x, r, h) in corns {
        debug!("(x({}), r({}), h({})", x, r, h);
        let u = x + h;

        if u <= a || b <= x {
            debug!("skipped");
            continue;
        }

        let tx = std::cmp::max(x, a);
        let th = (u - tx) as f64;
        let ratio = th / h as f64;
        let tr = r as f64 * ratio;

        let mut volume = _volume(tr, th);

        if b < x + h {
            let dh = (x + h - b) as f64;
            let ratio = dh / h as f64;
            let dr = r as f64 * ratio;
            let dvolume = _volume(dr, dh);
            volume -= dvolume;
        }

        debug!("{}", volume);
        ans += volume;
    }

    ans
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();

    let n: usize = scanner.cin();
    let q: usize = scanner.cin();

    let mut corns = vec![];
    for _ in 0..n {
        let x: i64 = scanner.cin();
        let r: i64 = scanner.cin();
        let h: i64 = scanner.cin();
        corns.push((x, r, h));
    }

    for _ in 0..q {
        let a: i64 = scanner.cin();
        let b: i64 = scanner.cin();
        let ans = calculate_volume(&corns, a, b);
        println!("{}", ans);
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
