#![allow(unused_imports)]

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        ss: [usize; n]
    };

    let s_max = ss.iter().max().unwrap();
    let s_min = ss.iter().min().unwrap();

    if s_max == s_min {
        println!("{}", -1);
        return;
    }

    let p = b as f64 / (s_max - s_min) as f64;
    let s_sum: usize = ss.iter().sum();
    let q = a as f64 - p * s_sum as f64 / n as f64;
    println!("{} {}", p, q);
}
