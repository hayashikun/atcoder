#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize
    };

    let c = a + b;

    if c >= 15 && b >= 8 {
        println!("1")
    } else if c >= 10 && b >= 3 {
        println!("2")
    } else if c >= 3 {
        println!("3")
    } else {
        println!("4")
    }
}
