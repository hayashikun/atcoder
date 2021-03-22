#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize
    };

    if a >= 13 {
        println!("{}", b);
    } else if a >= 6 {
        println!("{}", b / 2);
    } else {
        println!("0");
    }
}
