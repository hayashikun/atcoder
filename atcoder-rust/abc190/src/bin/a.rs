#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        a: usize,
        b: usize,
        c: usize
    };

    let s = if a == b {
        if c == 0 {
            "Aoki"
        } else {
            "Takahashi"
        }
    } else {
        if a > b {
            "Takahashi"
        } else {
            "Aoki"
        }
    };
    println!("{}", s);
}
