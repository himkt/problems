use proconio::input;


fn main() {
    input! { t: usize };

    for _ in 0..t {
        input! { n: usize, m: usize, k: usize, _s: String }
        let s: Vec<char> = _s.chars().collect();
        let mut g = vec![vec![]; n];
        for _ in 0..m {
            input! { u: usize, v: usize };
            g[u - 1].push(v - 1);
        }

        let mut dp = vec![vec!['x'; n]; 2 * k + 1];
        for j in 0..n {
            dp[2 * k][j] = s[j];
        }
        for i in (0..(2 * k)).rev() {
            for j in 0..n {
                // Alice
                if i % 2 == 0 {
                    dp[i][j] = if g[j].iter().any(|&x| dp[i + 1][x] == 'A') {
                        'A'
                    }
                    else {
                        'B'
                    };
                }
                // Bob
                else {
                    dp[i][j] = if g[j].iter().any(|&x| dp[i + 1][x] == 'B') {
                        'B'
                    }
                    else {
                        'A'
                    };
                }
            }
        }
        let ans = if dp[0][0] == 'A' { "Alice" } else { "Bob" };
        println!("{}", ans);
    }
}
