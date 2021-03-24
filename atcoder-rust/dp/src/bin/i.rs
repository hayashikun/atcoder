use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        pp: [f64; n]
    }

    let mut dp = vec![0.0; n + 1];
    dp[0] = 1.0 - pp[0];
    dp[1] = pp[0];

    for i in 1..n {
        let mut ndp = vec![0.0; n + 1];
        ndp[0] = dp[0] * (1.0 - pp[i]);
        for j in 1..=(i + 1) {
            ndp[j] = dp[j] * (1.0 - pp[i]) + dp[j - 1] * pp[i];
        }
        dp = ndp;
    }

    let mut ans = 0.0;
    for i in (n / 2 + 1)..=n {
        ans += dp[i];
    }

    println!("{}", ans);
}
