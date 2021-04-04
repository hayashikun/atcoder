#![allow(unused_imports)]

use std::cmp::max;

use num_integer::gcd;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };

    let mut f = vec![0; n - 1];
    f[0] = aa[0];
    for i in 1..(n - 1) {
        f[i] = gcd(f[i - 1], aa[i]);
    }

    let mut b = vec![0; n - 1];
    b[n - 2] = aa[n - 1];
    for i in 1..(n - 1) {
        b[n - 2 - i] = gcd(b[n - 1 - i], aa[n - i - 1]);
    }

    let mut ans = max(f[n - 2], b[0]);
    for i in 0..(n - 2) {
        let a = gcd(f[i], b[i + 1]);
        if a > ans {
            ans = a;
        }
    }

    println!("{}", ans);
}
