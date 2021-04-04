#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        vv: [isize; n],
        cc: [isize; n]
    };

    let s: isize = (0..n).map(|i| vv[i] - cc[i]).filter(|&d| d > 0).sum();
    println!("{}", s);
}
