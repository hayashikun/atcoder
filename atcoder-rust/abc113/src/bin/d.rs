#![allow(unused_imports)]

use std::cmp::max;

use nalgebra::min;
use proconio::{fastout, input, marker::*};

const D: usize = 1000000007;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize
    };

    let mut dp = vec![vec![-1; h]; w];
    let ans = rec(&h, &w, &k, 1, h, &mut dp);
    println!("{}", ans % D);
}

fn rec(h: &usize, w: &usize, k: &usize, x: usize, y: usize, dp: &mut Vec<Vec<isize>>) -> usize {
    if y == 0 {
        return if x == *k { 1 } else { 0 };
    }
    if dp[x - 1][y - 1] >= 0 {
        return dp[x - 1][y - 1] as usize;
    }
    let mut sum = 0;
    let lpc = comb(max(x, 2) - 2, false);
    let rpc = comb(w - min(x + 1, *w), false);

    if 1 < x {
        sum += comb(max(x, 3) - 3, false) * rpc * rec(h, w, k, x - 1, y - 1, dp);
    }
    if x < *w {
        sum += comb(w - min(x + 2, *w), false) * lpc * rec(h, w, k, x + 1, y - 1, dp);
    }
    sum += lpc * rpc * rec(h, w, k, x, y - 1, dp);
    sum %= D;
    dp[x - 1][y - 1] = sum as isize;
    return sum;
}

fn comb(n: usize, l: bool) -> usize {
    if n == 0 {
        return 1;
    }
    if l {
        return comb(n - 1, false);
    }
    return comb(n - 1, true) + comb(n - 1, false);
}
