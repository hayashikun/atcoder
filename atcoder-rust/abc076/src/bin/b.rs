#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};
use nalgebra::min;

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize
    };


    let mut a = 1;
    for _ in 0..n {
        a = min(a * 2, a + k);
    }

    println!("{}", a);
}
