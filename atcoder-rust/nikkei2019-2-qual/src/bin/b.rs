#![allow(unused_imports)]

use itertools::enumerate;
use proconio::{fastout, input, marker::*};

const D: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        ds: [usize; n]
    };

    let max_d = ds.iter().max().unwrap();
    let mut c = vec![0; *max_d + 1];

    for d in ds.iter() {
        c[*d] += 1;
    }

    if ds[0] != 0 || c[0] > 1 {
        println!("{}", 0);
        return;
    }

    let mut ans = 1;
    for i in 0..*max_d {
        if c[i + 1] == 0 {
            println!("{}", 0);
            return;
        }
        ans *= mod_ex(c[i], c[i + 1], D);
        ans %= D;
    }

    println!("{}", ans % D);
}

// b^e % m
fn mod_ex(b: usize, e: usize, m: usize) -> usize {
    if e == 0 {
        return 1;
    }
    if b == 0 || b == 1 {
        return b;
    }
    let mut c = mod_ex(b, e / 2, m);
    c = (c * c) % m;
    if e % 2 == 1 {
        c = (c * b) % m;
    }
    return c;
}
