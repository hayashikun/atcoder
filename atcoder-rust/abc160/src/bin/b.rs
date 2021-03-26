#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        x: usize
    };

    let ans = (x / 500) * 1000 + (x % 500) / 5 * 5;
    println!("{}", ans);
}
