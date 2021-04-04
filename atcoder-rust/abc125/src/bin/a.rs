#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        t: usize,
    };

    println!("{}", (t / a) * b);
}
