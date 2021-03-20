#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    };

    println!("{}", b - c);
}
