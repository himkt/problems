use std::collections::{BTreeSet, HashMap};

use proconio::input;


fn main () {
    input! { n: usize, xs: [usize; n] };
    let mut minmax = BTreeSet::new();
    minmax.insert(0);
    minmax.insert(xs[0]);

    let mut mp: HashMap<usize, usize> = HashMap::new();
    mp.insert(0, xs[0]);
    mp.insert(xs[0], xs[0]);
    let mut ans = 2 * xs[0];
    println!("{}", ans);

    for &x in &xs[1..] {
        let &lni = minmax.range(0..x).last().unwrap();

        if let Some(&rni) = minmax.range(x..=1_000_000_000).next() {
            let ldst = mp[&(lni as usize)];
            let rdst = mp[&(rni as usize)];

            let lndst = x - lni;
            let rndst = rni - x;

            if ldst > lndst {
                ans -= ldst - lndst;
                mp.insert(lni as usize, lndst);
            }
            if rdst > rndst {
                ans -= rdst - rndst;
                mp.insert(rni as usize, rndst);
            }

            let ndst = lndst.min(rndst);
            ans += ndst;
            mp.insert(x, ndst);
        }
        else {
            let ldst = mp[&(lni as usize)];
            let lndst = x - (lni as usize);
            if ldst > lndst {
                ans -= ldst - lndst;
                mp.insert(lni as usize, lndst);
            }
            ans += lndst;
            mp.insert(x, lndst);
        }

        println!("{}", ans);
        minmax.insert(x);
    }
}
