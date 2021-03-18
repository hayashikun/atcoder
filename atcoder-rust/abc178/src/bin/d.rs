#![allow(unused_imports)]

use std::cmp::max;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: usize
    };

    if s < 3 {
        println!("{}", 0);
        return;
    }

    let d = 10usize.pow(9) + 7;

    let mut dp = vec![0; s + 1];
    dp[0] = 1;

    for i in 3..=s {
        dp[i] = (dp[i - 1] + dp[i - 3]) % d;
    }

    println!("{}", dp[s]);
}
