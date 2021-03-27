#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
        xy: [(usize, usize); n]
    };

    let mut ans = false;
    for (x, y) in xy.into_iter() {
        if x < s && y > d {
            ans = true;
            break;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" })
}
