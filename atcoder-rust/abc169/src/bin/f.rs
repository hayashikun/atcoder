use proconio::{fastout, input};

#[fastout]
fn main() {
    let m = 998244353;
    input! {
        n: usize,
        s: usize,
        aa: [usize; n]
    }

    let mut dp = vec![vec![0i128; s + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..(s + 1) {
            dp[i + 1][j] += dp[i][j] * 2 % m;
            if j >= aa[i] {
                dp[i + 1][j] += dp[i][j - aa[i]];
            }
            dp[i + 1][j] %= m;
        }
    }

    println!("{}", dp[n][s]);
}
