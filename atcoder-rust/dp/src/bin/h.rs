#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h]
    };

    let d = 10isize.pow(9) + 7;

    let mut dp = vec![vec![-1; w + 1]; h + 1];

    for (i, l) in grid.into_iter().enumerate() {
        for (j, c) in l.into_iter().enumerate() {
            if c == '#' {
                dp[i + 1][j + 1] = 0;
            }
        }
    }

    for i in 0..=h {
        for j in 0..=w {
            dp[i][j] = if dp[i][j] == 0 {
                0
            } else if i == 0 || j == 0 {
                0
            } else if i == 1 && j == 1 {
                1
            } else {
                (dp[i - 1][j] + dp[i][j - 1]) % d
            };
        }
    }

    println!("{}", dp[h][w]);
}
