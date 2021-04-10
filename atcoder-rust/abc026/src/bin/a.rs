#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize
    };

    let b = a / 2;
    println!("{}", b * b);
}
