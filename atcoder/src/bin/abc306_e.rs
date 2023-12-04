#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::create();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let q: usize = scanner.cin();
    let mut a = vec![0; n];
    let mut nu = k;

    let mut lower: BTreeMap<usize, usize> = BTreeMap::new();
    let mut upper: BTreeMap<usize, usize> = BTreeMap::new();
    let mut sum: usize = 0;

    for _ in 0..k {
        *upper.entry(0).or_insert(0) += 1;
    }
    for _ in k..n {
        *lower.entry(0).or_insert(0) += 1;
    }
    debug!("[init] upper={:?}, lower={:?}", upper, lower);

    for _ in 0..q {
        let xi: usize = scanner.cin::<usize>() - 1;
        let yi: usize = scanner.cin::<usize>();

        // delete a[x]
        if upper.contains_key(&a[xi]) {
            debug!("delete a[xi]={} from upper", a[xi]);
            upper.entry(a[xi]).and_modify(|x| *x -= 1);
            if upper[&a[xi]] == 0 {
                upper.remove(&a[xi]);
            }
            sum -= a[xi];
            nu -= 1;
        }
        else {
            debug!("delete a[xi]={} from lower", a[xi]);
            lower.entry(a[xi]).and_modify(|x| *x -= 1);
            if lower[&a[xi]] == 0 {
                lower.remove(&a[xi]);
            }
        }

        // add a[x]
        a[xi] = yi;
        *lower.entry(a[xi]).or_insert(0) += 1;

        // balance() [1]
        debug!("[start balance1] upper={:?}, lower={:?}", upper, lower);
        while nu < k {
            let (&lk, _) = lower.last_key_value().unwrap();
            *upper.entry(lk).or_insert(0) += 1;
            *lower.entry(lk).or_insert(0) -= 1;
            nu += 1;
            sum += lk;

            if lower[&lk] == 0 {
                lower.remove(&lk);
            }
        }
        debug!("[finish balance1] upper={:?}, lower={:?}", upper, lower);

        // balance() [2]
        debug!("[start balance2] upper={:?}, lower={:?}", upper, lower);
        while !lower.is_empty() && !upper.is_empty() {
            let (&lk, _) = lower.last_key_value().unwrap();
            let (&uk, _) = upper.first_key_value().unwrap();
            debug!("[in balance2] lk={}, uk={}", uk, lk);
            if lk > uk {
                *upper.entry(lk).or_insert(0) += 1;
                *upper.entry(uk).or_insert(0) -= 1;
                if upper[&uk] == 0 {
                    upper.remove(&uk);
                }

                *lower.entry(uk).or_insert(0) += 1;
                *lower.entry(lk).or_insert(0) -= 1;
                if lower[&lk] == 0 {
                    lower.remove(&lk);
                }
                sum += lk - uk;
                continue;
            }
            break;
        }
        debug!("[finish balance2] upper={:?}, lower={:?}", upper, lower);
        debug!("a={:?}", a);
        println!("{}", sum);
    }
}

use std::{io::Write, collections::BTreeMap};
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
