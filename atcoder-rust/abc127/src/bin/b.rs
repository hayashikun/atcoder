#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input!{
        r: usize,
        d: usize,
        mut x: usize
    };

    for _ in 0..10 {
        x = r * x - d;
        println!("{}", x);
    }
}
