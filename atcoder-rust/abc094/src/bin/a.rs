#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        a: usize,
        b: usize,
        x: usize
    };

    if a + b < x || a > x {
        println!("NO");
    } else {
        println!("YES");
    }
}
