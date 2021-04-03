#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        x: usize,
        y: usize
    };

    println!("{}", y / x);
}
