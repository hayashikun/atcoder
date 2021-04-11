#![allow(unused_imports)]

use libm::{ceil, sqrt};
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        r: usize,
        x: usize,
        y: usize
    };

    let d = sqrt((x * x + y * y) as f64);
    let s = d / r as f64;
    if s == 0.0 {
        println!("{}", 0);
    } else if s < 1.0 {
        println!("{}", 2);
    } else {
        println!("{}", ceil(s));
    }
}
