#![allow(unused_imports)]

use std::f64::consts::PI;

use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        rr: [i32; n]
    };

    let mut s = 0;
    let mut pm = 1;
    for r in rr.into_iter().sorted().rev() {
        s += pm * r * r;
        pm = -pm;
    }

    println!("{}", s as f64 * PI);
}
