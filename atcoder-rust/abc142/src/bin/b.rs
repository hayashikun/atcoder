#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        hh: [usize; n]
    };

    let c = hh.into_iter().filter(|&h| h >= k).count();
    println!("{}", c);
}
