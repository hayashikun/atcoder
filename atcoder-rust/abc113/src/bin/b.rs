#![allow(unused_imports)]

use itertools::Itertools;
use num::abs;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: isize,
        t: isize,
        a: isize,
        hh: [isize; n]
    };
    let a = 1000 * a;
    let t = 1000 * t;

    let (i, _) = hh
        .into_iter()
        .map(|h| abs(t - 6 * h - a))
        .enumerate()
        .min_by_key(|&(_, d)| d)
        .unwrap();

    println!("{}", i + 1);
}
