use proconio::input;

fn main() {
    input! { n: usize, items: [(usize, usize, usize); n] };

    let limit = 500 * 500;
    let mut dp = vec![vec![0usize; limit + 1]; n];
    dp[0][items[0].0] = items[0].1;
    dp[0][0] = items[0].2;

    for i in 1..n {
        for k in 0..(limit + 1) {
            // body
            if dp[i - 1][k] > 0 {
                dp[i][k] = dp[i][k].max(dp[i - 1][k] + items[i].2);
            }
            // head
            if k >= items[i].0 && dp[i - 1][k - items[i].0] > 0 {
                dp[i][k] = dp[i][k].max(dp[i - 1][k - items[i].0] + items[i].1);
            }
        }
    }

    let wall: usize = items.iter().map(|item| item.0).sum();
    println!("{}", &dp[n - 1][0..=(wall / 2)].iter().max().unwrap());
}
