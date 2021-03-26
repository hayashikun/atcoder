#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

const D: usize = 1_000_000_007;

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

#[fastout]
fn main() {
    input! {
        n: usize
    };

    if n < 2 {
        println!("{}", 0);
        return;
    }

    let a = mod_ex(10, n, D) + 2 * D;
    let b = mod_ex(9, n, D);
    let c = mod_ex(8, n, D);
    println!("{}", (a - 2 * b + c) % D);
}
