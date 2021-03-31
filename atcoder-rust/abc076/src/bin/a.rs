#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        r: isize,
        g: isize
    };

    println!("{}", g * 2 - r);
}
